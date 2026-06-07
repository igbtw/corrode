// Report building pipeline: orchestrates scanning, reading, and analysis.

use std::fs;
use std::path::Path;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::analysis::{dependencies, metrics, project};
use crate::filesystem::scanner::{count_directories, scan_directory};
use crate::models::{AnalysisReport, FileEntry, ProjectType};

pub fn analyse(path: &str) -> Result<AnalysisReport, Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let start = Instant::now();

    let path_obj = Path::new(path);
    let is_directory = path_obj.is_dir();

    let project_type = if is_directory {
        project::detect_project_type(path)
    } else {
        ProjectType::Unknown
    };

    let files: Vec<String> = if is_directory {
        pb.set_message("Scanning project...");
        scan_directory(path)
    } else {
        pb.set_message("Reading file...");
        vec![path.to_string()]
    };

    let directory_count = if is_directory {
        count_directories(path)
    } else {
        0
    };

    let mut entries: Vec<FileEntry> = Vec::with_capacity(files.len());

    for file_path in &files {
        pb.set_message(format!("Reading {}...", file_path));

        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => {
                pb.println(format!("  Skipping '{}' (not valid UTF-8)", file_path));
                continue;
            }
        };

        let line_count = contents.lines().count();
        let extension = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        let size_bytes = fs::metadata(file_path)?.len();

        entries.push(FileEntry {
            path: file_path.clone(),
            contents,
            extension,
            line_count,
            size_bytes,
        });
    }

    let language_map = metrics::build_language_map(&entries);
    let entry_point = project::detect_entry_point(&files, &project_type);
    let dependencies_list = dependencies::detect_dependencies(&entries, &project_type);
    let total_lines = metrics::calculate_total_lines(&entries);
    let duration = start.elapsed();

    let directory_stats = metrics::compute_directory_stats(&entries);
    let depth_map = metrics::compute_depth_map(path, &entries);
    let size_distribution = metrics::compute_size_distribution(&entries);

    let max_depth = depth_map.last().map(|(d, _)| *d).unwrap_or(0);
    let hotspots = metrics::compute_hotspots(&entries, path);
    let code_entries: Vec<&FileEntry> = entries.iter().filter(|f| metrics::is_code_extension(&f.extension)).collect();
    let code_total_lines: usize = code_entries.iter().map(|f| f.line_count).sum();
    let architecture = metrics::compute_architecture(&code_entries, max_depth);

    let largest_code_ratio = if code_entries.len() > 1 {
        let max_lines = code_entries.iter().map(|f| f.line_count).max().unwrap_or(0);
        if code_total_lines > 0 {
            (max_lines as f64 / code_total_lines as f64).min(1.0)
        } else {
            0.0
        }
    } else {
        0.0
    };

    let complexity = metrics::compute_complexity(
        code_entries.len(),
        code_total_lines,
        max_depth,
        directory_count,
        largest_code_ratio,
    );

    let warnings = metrics::compute_warnings(&entries, max_depth);
    let code_metrics = metrics::compute_code_metrics(&entries);

    pb.finish_and_clear();
    println!("✓ Scanning project...");

    Ok(AnalysisReport {
        architecture,
        code_metrics,
        complexity,
        dependencies: dependencies_list,
        depth_map,
        directory_count,
        directory_stats,
        duration,
        entry_point,
        files: entries,
        hotspots,
        language_map,
        project_root: path.to_string(),
        project_type,
        size_distribution,
        total_lines,
        warnings,
    })
}

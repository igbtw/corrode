// Report building pipeline: orchestrates scanning, reading, and analysis.

use std::fs;
use std::path::{Component, Path};
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::analysis::classification::is_code_extension;
use crate::analysis::{
    architecture, complexity, dependencies, health, hotspots, metrics, project, warnings,
};
use crate::filesystem::scanner::{count_directories, scan_directory};
use crate::models::{
    AnalysisReport, ArchitectureReport, DependencyReport, FileEntry, FileReport, ProjectInfo,
    ProjectType,
};

pub fn analyse(path: &str) -> Result<AnalysisReport, Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let start = Instant::now();

    let path_obj = Path::new(path);
    let is_directory = path_obj.is_dir();

    // Phase 1: Detect project type from manifest files in the root.
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

    // Phase 2: Read every file into memory (contents kept for future AST passes).
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

    // Phase 3: Compute all metrics from in-memory entries.
    let language_map = metrics::build_language_map(&entries);
    let entry_point = project::detect_entry_point(&files, &project_type);
    let dependencies_list = dependencies::detect_dependencies(&entries, &project_type);
    let total_lines = metrics::calculate_total_lines(&entries);
    let duration = start.elapsed();

    let directory_stats = metrics::compute_directory_stats(&entries);
    let depth_map = metrics::compute_depth_map(path, &entries);
    let size_distribution = metrics::compute_size_distribution(&entries);

    let max_depth = depth_map.last().map(|(d, _)| *d).unwrap_or(0);
    let hotspot_list = hotspots::compute_hotspots(&entries, path);
    let code_entries: Vec<&FileEntry> =
        entries.iter().filter(|f| is_code_extension(&f.extension)).collect();
    let code_total_lines: usize = code_entries.iter().map(|f| f.line_count).sum();
    let arch_metrics = architecture::compute_architecture(&code_entries, max_depth);

    // Largest-file-to-total-code ratio penalises monolithic files.
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

    let warning_list = warnings::compute_warnings(&entries, max_depth);
    let code_metrics = metrics::compute_code_metrics(&entries);

    // Boolean health signals derived from file listing.
    let has_tests = entries.iter().any(|f| {
        Path::new(&f.path)
            .components()
            .any(|c| matches!(c, Component::Normal(name) if name == "tests"))
    });
    let has_readme = entries.iter().any(|f| {
        Path::new(&f.path)
            .file_name()
            .and_then(|n| n.to_str())
            == Some("README.md")
    });

    let complexity_score = complexity::compute_complexity(
        code_total_lines,
        max_depth,
        directory_count,
        &hotspot_list,
        &size_distribution,
        largest_code_ratio,
    );

    let health_score = health::compute_health(
        &warning_list,
        &hotspot_list,
        has_tests,
        has_readme,
        largest_code_ratio,
    );

    pb.finish_and_clear();
    eprintln!("✓ Scanning project...");

    Ok(AnalysisReport {
        project: ProjectInfo {
            project_type,
            entry_point,
            project_root: path.to_string(),
            duration,
        },
        files: FileReport {
            entries,
            directory_count,
            total_lines,
            language_map,
            directory_stats,
            depth_map,
            size_distribution,
        },
        dependencies: DependencyReport {
            list: dependencies_list,
        },
        architecture: ArchitectureReport {
            metrics: arch_metrics,
            hotspots: hotspot_list,
            code_metrics,
        },
        quality: crate::models::QualityReport {
            complexity: complexity_score,
            health: health_score,
            warnings: warning_list,
        },
    })
}

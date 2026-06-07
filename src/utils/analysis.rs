// Core pipeline: orchestrates scanning, reading, and report assembly.
// Steps:
//   1. Detect project type from manifest files
//   2. Walk the directory (or read a single file)
//   3. Read each file, count lines, record metadata
//   4. Detect entry point
//   5. Return an AnalysisReport with all collected data

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::filesystem::scanner::{
    count_directories, detect_entry_point, detect_project_type, scan_directory,
};
use crate::models::{classify_extension, AnalysisReport, FileEntry, ProjectType};

// Main entry point. If path is a directory, scans it recursively.
// If it's a file, analyses just that file. Non-UTF-8 files are
// skipped with a warning instead of aborting.
pub fn analyse(path: &str) -> Result<AnalysisReport, Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let start = Instant::now();

    let path_obj = Path::new(path);
    let is_directory = path_obj.is_dir();

    let project_type = if is_directory {
        detect_project_type(path)
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
    let mut language_map: HashMap<String, usize> = HashMap::new();
    let mut file_categories: HashMap<String, usize> = HashMap::new();

    for file_path in &files {
        pb.set_message(format!("Reading {}...", file_path));

        // read_to_string fails on binary files — skip gracefully
        // instead of crashing the whole analysis.
        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => {
                pb.println(format!("  Skipping '{}' (not valid UTF-8)", file_path));
                continue;
            }
        };

        let line_count = contents.lines().count();

        // Lowercase extension used for language grouping.
        let extension = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !extension.is_empty() {
            *language_map.entry(extension.clone()).or_insert(0) += 1;
            let category = classify_extension(&extension);
            *file_categories.entry(category.to_string()).or_insert(0) += 1;
        }

        let size_bytes = fs::metadata(file_path)?.len();

        entries.push(FileEntry {
            path: file_path.clone(),
            contents,
            extension,
            line_count,
            size_bytes,
        });
    }

    let entry_point = detect_entry_point(&files, &project_type);

    let total_lines: usize = entries.iter().map(|e| e.line_count).sum();
    let duration = start.elapsed();

    pb.finish_and_clear();
    println!("✓ Scanning project...");

    Ok(AnalysisReport {
        project_type,
        entry_point,
        files: entries,
        directory_count,
        total_lines,
        language_map,
        project_root: path.to_string(),
        duration,
        file_categories,
    })
}

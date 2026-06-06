// Core analysis logic — reads files, gathers metadata, and returns
// an `AnalysisReport` ready for output formatting.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use indicatif::{ProgressBar, ProgressStyle};

use crate::filesystem::scanner::{count_directories, scan_directory};
use crate::models::{AnalysisReport, FileEntry};

/// Analyzes a file or directory at `path`.
///
/// - If `path` is a **file**: reads it and returns a report with a
///   single `FileEntry`.
/// - If `path` is a **directory**: scans recursively with `walkdir`,
///   reads every file found, and builds a full `AnalysisReport`
///   with counts, language map, and duration.
///
/// Non-UTF-8 files are skipped with a warning instead of aborting.
pub fn analyse(path: &str) -> Result<AnalysisReport, Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let start = Instant::now();

    let path_obj = Path::new(path);
    let is_directory = path_obj.is_dir();

    let files: Vec<String> = if is_directory {
        pb.set_message("Scanning project...");
        scan_directory(path)
    } else {
        pb.set_message("Reading file...");
        vec![path.to_string()]
    };

    // Count directories with the same exclusion rules.
    let directory_count = if is_directory {
        count_directories(path)
    } else {
        0
    };

    let mut entries: Vec<FileEntry> = Vec::with_capacity(files.len());
    let mut language_map: HashMap<String, usize> = HashMap::new();

    for file_path in &files {
        pb.set_message(format!("Reading {}...", file_path));

        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => {
                pb.println(format!("  ⚠ Skipping '{}' (not valid UTF-8)", file_path));
                continue;
            }
        };

        let line_count = contents.lines().count();

        // Track language by file extension.
        let ext = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if !ext.is_empty() {
            *language_map.entry(ext).or_insert(0) += 1;
        }

        entries.push(FileEntry {
            path: file_path.clone(),
            contents,
            line_count,
        });
    }

    let total_lines: usize = entries.iter().map(|e| e.line_count).sum();
    let duration = start.elapsed();

    pb.finish_and_clear();
    println!("✓ Scanning project...");

    Ok(AnalysisReport {
        files: entries,
        directory_count,
        total_lines,
        language_map,
        duration,
    })
}

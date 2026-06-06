// Core analysis logic — reads files and directories, reports progress.

use std::fs;
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

use crate::filesystem::scanner::scan_directory;
use crate::models::FileEntry;

/// Analyzes a file or directory at `path`.
///
/// - If `path` is a **file**: reads it and returns a single `FileEntry`.
/// - If `path` is a **directory**: scans it recursively with `walkdir`,
///   reads every file found, and returns a `Vec<FileEntry>`.
///
/// Shows a spinner while working. Non-UTF-8 files are skipped with
/// a warning instead of aborting the whole analysis.
pub fn analyse(path: &str) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let path_obj = Path::new(path);

    let files: Vec<String> = if path_obj.is_dir() {
        pb.set_message("Scanning directory...");
        scan_directory(path)
    } else {
        pb.set_message("Reading file...");
        vec![path.to_string()]
    };

    let mut entries = Vec::with_capacity(files.len());
    for file_path in &files {
        pb.set_message(format!("Reading {}...", file_path));

        // `read_to_string` fails on binary files (images, .o, etc.)
        // because they are not valid UTF-8. Instead of crashing, we
        // skip the file and show a warning via `pb.println` (which
        // prints above the spinner without disrupting it).
        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => {
                pb.println(format!(
                    "  ⚠ Skipping '{}' (not valid UTF-8)",
                    file_path
                ));
                continue;
            }
        };

        entries.push(FileEntry {
            path: file_path.clone(),
            contents,
        });
    }

    pb.finish_and_clear();
    let file_count = entries.len();
    let noun = if file_count == 1 { "file" } else { "files" };
    println!("✓ Read {} {}.", file_count, noun);

    Ok(entries)
}

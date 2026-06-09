// Filesystem discovery via walkdir, respecting skip lists.

use walkdir::WalkDir;

use crate::filesystem::filters::{is_minified_file, is_skipped_dir, is_skipped_extension};

pub fn scan_directory(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !is_skipped_dir(&name)
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            if !e.file_type().is_file() {
                return false;
            }
            let ext = e.path().extension().and_then(|e| e.to_str()).unwrap_or("");
            if is_skipped_extension(ext) {
                return false;
            }
            let name = e.file_name().to_string_lossy();
            if is_minified_file(&name) {
                return false;
            }
            true
        })
        .map(|e| e.path().display().to_string())
        .collect()
}

pub fn count_directories(path: &str) -> usize {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !is_skipped_dir(&name)
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .count()
        // Subtract the root directory itself from the count.
        .saturating_sub(1)
}

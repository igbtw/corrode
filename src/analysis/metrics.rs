// Aggregate statistics computed from loaded FileEntry values.
// These are the pure data-aggregation functions that don't have
// a clear single-subject home (unlike hotspots/architecture/complexity/warnings).

use std::collections::HashMap;
use std::path::Path;

use crate::analysis::classification::classify_extension;
use crate::models::{CodeMetrics, DirectoryStat, FileEntry, SizeDistribution};

pub fn build_language_map(files: &[FileEntry]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for file in files {
        if !file.extension.is_empty() {
            *map.entry(file.extension.clone()).or_insert(0) += 1;
        }
    }
    map
}

pub fn calculate_total_lines(files: &[FileEntry]) -> usize {
    files.iter().map(|f| f.line_count).sum()
}

pub fn compute_directory_stats(files: &[FileEntry]) -> Vec<DirectoryStat> {
    let mut dirs: HashMap<String, (usize, usize, u64)> = HashMap::new();
    for file in files {
        let parent = Path::new(&file.path)
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        let entry = dirs.entry(parent).or_default();
        entry.0 += 1;
        entry.1 += file.line_count;
        entry.2 += file.size_bytes;
    }
    let mut sorted: Vec<DirectoryStat> = dirs
        .into_iter()
        .map(|(path, (file_count, total_lines, total_bytes))| DirectoryStat {
            path,
            file_count,
            total_lines,
            total_bytes,
        })
        .collect();
    // Sort by LOC descending; use file count as tiebreaker.
    sorted.sort_by(|a, b| {
        b.total_lines
            .cmp(&a.total_lines)
            .then_with(|| b.file_count.cmp(&a.file_count))
    });
    sorted.truncate(5);
    sorted
}

pub fn compute_depth_map(root: &str, files: &[FileEntry]) -> Vec<(usize, usize)> {
    let root_path = Path::new(root);
    let mut depths: HashMap<usize, usize> = HashMap::new();
    for entry in files {
        let depth = Path::new(&entry.path)
            .strip_prefix(root_path)
            .map(|rel| rel.components().count().saturating_sub(1))
            .unwrap_or(0);
        *depths.entry(depth).or_insert(0) += 1;
    }
    let mut sorted: Vec<_> = depths.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    sorted
}

pub fn compute_size_distribution(files: &[FileEntry]) -> SizeDistribution {
    let noise = files.iter().filter(|f| f.line_count < 10).count();
    let small = files
        .iter()
        .filter(|f| (10..100).contains(&f.line_count))
        .count();
    let medium = files
        .iter()
        .filter(|f| (100..500).contains(&f.line_count))
        .count();
    let large = files.iter().filter(|f| f.line_count >= 500).count();
    SizeDistribution {
        noise,
        small,
        medium,
        large,
    }
}

/// Count files and LOC by category (code, config, docs).
pub fn compute_code_metrics(files: &[FileEntry]) -> CodeMetrics {
    let mut cm = CodeMetrics {
        code_files: 0,
        code_lines: 0,
        config_files: 0,
        config_lines: 0,
        docs_files: 0,
        docs_lines: 0,
    };

    for f in files {
        match classify_extension(&f.extension) {
            "code" => {
                cm.code_files += 1;
                cm.code_lines += f.line_count;
            }
            "config" => {
                cm.config_files += 1;
                cm.config_lines += f.line_count;
            }
            "docs" => {
                cm.docs_files += 1;
                cm.docs_lines += f.line_count;
            }
            _ => {}
        }
    }

    cm
}

/// Returns the top N code files sorted by line count descending.
pub fn top_code_files<'a>(entries: &'a [FileEntry], n: usize) -> Vec<&'a FileEntry> {
    let mut code: Vec<&FileEntry> = entries
        .iter()
        .filter(|f| classify_extension(&f.extension) == "code")
        .collect();
    code.sort_by(|a, b| b.line_count.cmp(&a.line_count));
    code.truncate(n);
    code
}

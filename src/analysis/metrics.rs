// Aggregate statistics computed from loaded FileEntry values.

use std::collections::HashMap;
use std::path::{Component, Path};

use crate::models::{
    ArchitectureMetrics, CodeMetrics, ComplexityReport, DirectoryStat, FileEntry, Hotspot,
    SizeDistribution,
};

/// Extensions treated as non-code for architecture warnings and metrics.
const NON_CODE_EXTS: &[&str] = &[
    "lock", "md", "txt", "json", "svg", "png", "jpg", "jpeg", "ico", "pdf", "gif",
];

pub(crate) fn is_code_extension(ext: &str) -> bool {
    !NON_CODE_EXTS.contains(&ext)
}

fn classify_extension(ext: &str) -> &'static str {
    match ext {
        "rs" | "go" | "py" | "rb" | "js" | "ts" | "java" | "kt" | "swift" | "c" | "h"
        | "cpp" | "hpp" | "cs" | "scala" | "ex" | "exs" | "php" | "r" | "dart" | "lua"
        | "sh" | "bash" | "zsh" | "fish" | "pl" | "pm" => "code",
        "toml" | "json" | "yaml" | "yml" | "ini" | "cfg" | "conf" | "xml" | "lock"
        | "gradle" | "sbt" | "mk" | "cmake" => "config",
        "md" | "txt" | "rst" | "adoc" | "org" => "docs",
        _ => "other",
    }
}

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

// ── Hotspots ─────────────────────────────────────────────────────────────────

/// Groups code files by parent directory, computes percentage of total code LOC.
pub fn compute_hotspots(files: &[FileEntry], project_root: &str) -> Vec<Hotspot> {
    let code_files: Vec<_> = files.iter().filter(|f| is_code_extension(&f.extension)).collect();
    if code_files.is_empty() {
        return Vec::new();
    }

    let code_total: usize = code_files.iter().map(|f| f.line_count).sum();
    let root = Path::new(project_root);

    let mut groups: HashMap<String, usize> = HashMap::new();
    for file in &code_files {
        let parent = Path::new(&file.path).parent().map(|p| {
            p.strip_prefix(root)
                .unwrap_or(p)
                .display()
                .to_string()
        });

        let key = match parent {
            Some(ref p) if !p.is_empty() && p != "." && p != project_root => {
                format!("{}/", p)
            }
            _ => String::from("(root)"),
        };

        *groups.entry(key).or_insert(0) += file.line_count;
    }

    let mut sorted: Vec<Hotspot> = groups
        .into_iter()
        .map(|(path, loc)| Hotspot {
            percentage: loc as f64 / code_total as f64 * 100.0,
            path,
            total_lines: loc,
        })
        .collect();

    sorted.sort_by(|a, b| b.total_lines.cmp(&a.total_lines));

    let mut hotspots: Vec<Hotspot> = Vec::with_capacity(6);
    let mut other_lines = 0usize;

    for (i, spot) in sorted.into_iter().enumerate() {
        if i < 5 {
            hotspots.push(spot);
        } else {
            other_lines += spot.total_lines;
        }
    }

    if !hotspots.is_empty() && other_lines > 0 {
        hotspots.push(Hotspot {
            path: String::from("other"),
            total_lines: other_lines,
            percentage: other_lines as f64 / code_total as f64 * 100.0,
        });
    } else if hotspots.is_empty() && other_lines > 0 {
        hotspots.push(Hotspot {
            path: String::from("other"),
            total_lines: other_lines,
            percentage: 100.0,
        });
    }

    hotspots
}

// ── Architecture Metrics ─────────────────────────────────────────────────────

/// Compute architecture metrics from code files only.
pub fn compute_architecture(files: &[&FileEntry], max_depth: usize) -> ArchitectureMetrics {
    let file_count = files.len();
    if file_count == 0 {
        return ArchitectureMetrics {
            avg_file_size: 0.0,
            avg_loc_per_file: 0.0,
            max_depth: 0,
            median_loc_per_file: 0.0,
        };
    }

    let total_lines: usize = files.iter().map(|f| f.line_count).sum();
    let total_bytes: u64 = files.iter().map(|f| f.size_bytes).sum();

    let avg_loc = total_lines as f64 / file_count as f64;
    let avg_size = total_bytes as f64 / file_count as f64;

    let median_loc = {
        let mut lines: Vec<usize> = files.iter().map(|f| f.line_count).collect();
        lines.sort_unstable();
        let mid = lines.len() / 2;
        if lines.len() % 2 == 0 {
            (lines[mid - 1] + lines[mid]) as f64 / 2.0
        } else {
            lines[mid] as f64
        }
    };

    ArchitectureMetrics {
        avg_file_size: avg_size,
        avg_loc_per_file: avg_loc,
        max_depth,
        median_loc_per_file: median_loc,
    }
}

// ── Complexity Score ─────────────────────────────────────────────────────────

/// Weighted heuristic score (0–100) estimating project complexity.
// Uses code files for LOC and file count; depth and dir count are structural.
// Sqrt scaling gives sub-linear growth with project size.
// Weights: files 25%, directories 15%, LOC 35%, depth 15%, largest file ratio 10%.
pub fn compute_complexity(
    code_file_count: usize,
    code_total_lines: usize,
    max_depth: usize,
    directory_count: usize,
    largest_code_ratio: f64,
) -> ComplexityReport {
    let file_score = ((code_file_count as f64).sqrt() / 20.0).min(1.0) * 25.0;
    let dir_score = ((directory_count as f64).sqrt() / 10.0).min(1.0) * 15.0;
    let loc_score = ((code_total_lines as f64).sqrt() / 200.0).min(1.0) * 35.0;
    let depth_score = ((max_depth as f64).sqrt() / 3.0).min(1.0) * 15.0;
    let largest_score = (largest_code_ratio * 10.0).min(10.0);

    let raw = file_score + dir_score + loc_score + depth_score + largest_score;
    let score = (raw as u8).min(100);

    let rating = match score {
        0..=20 => "Tiny",
        21..=40 => "Small",
        41..=60 => "Medium",
        61..=80 => "Large",
        81..=100 => "Massive",
        _ => unreachable!(),
    }
    .to_string();

    ComplexityReport { rating, score }
}

// ── Code Metrics ─────────────────────────────────────────────────────────────

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

// ── Health Warnings ──────────────────────────────────────────────────────────

/// Check for common warning conditions based on project data.
pub fn compute_warnings(files: &[FileEntry], max_depth: usize) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();

    if files.is_empty() {
        return warnings;
    }

    // Largest code file > 5% of code LOC
    let code_files: Vec<_> = files.iter().filter(|f| is_code_extension(&f.extension)).collect();
    if let Some(max_lines) = code_files.iter().map(|f| f.line_count).max() {
        let code_total: usize = code_files.iter().map(|f| f.line_count).sum();
        if code_total > 0 {
            let ratio = max_lines as f64 / code_total as f64;
            if ratio > 0.05 {
                warnings.push(format!(
                    "Largest code file represents {:.0}% of code LOC",
                    ratio * 100.0
                ));
            }
        }
    }

    // Markdown files > 20%
    let md_count = files.iter().filter(|f| f.extension == "md").count();
    if md_count > 0 {
        let md_ratio = md_count as f64 / files.len() as f64;
        if md_ratio > 0.20 {
            warnings.push(format!(
                "Markdown files represent {:.0}% of project files",
                md_ratio * 100.0
            ));
        }
    }

    // JSON files > 100
    let json_count = files.iter().filter(|f| f.extension == "json").count();
    if json_count > 100 {
        warnings.push(format!("More than {} JSON files detected", json_count));
    }

    // Max depth > 8
    if max_depth > 8 {
        warnings.push("Project depth exceeds 8 levels".to_string());
    }

    // Missing README.md
    let has_readme = files.iter().any(|f| {
        Path::new(&f.path)
            .file_name()
            .and_then(|n| n.to_str())
            == Some("README.md")
    });
    if !has_readme {
        warnings.push("No README.md found".to_string());
    }

    // Missing tests directory
    let has_tests = files.iter().any(|f| {
        Path::new(&f.path).components().any(|c| {
            matches!(c, Component::Normal(name) if name == "tests")
        })
    });
    if !has_tests {
        warnings.push("No tests/ directory detected".to_string());
    }

    warnings
}

// Health warnings based on project data.

use std::path::{Component, Path};

use crate::analysis::classification::is_code_extension;
use crate::models::FileEntry;

/// Check for common warning conditions based on project data.
pub fn compute_warnings(files: &[FileEntry], max_depth: usize) -> Vec<String> {
    let mut warnings: Vec<String> = Vec::new();

    if files.is_empty() {
        return warnings;
    }

    // Largest code file > 5% of code LOC
    let code_files: Vec<_> = files
        .iter()
        .filter(|f| is_code_extension(&f.extension))
        .collect();
    if let Some(max_lines) = code_files.iter().map(|f| f.line_count).max() {
        let code_total: usize = code_files.iter().map(|f| f.line_count).sum();
        if code_total > 0 {
            let ratio = max_lines as f64 / code_total as f64;
            if ratio > 0.05 {
                warnings.push(format!(
                    "Largest code file represents {:.0}% of code LOC — consider splitting into smaller modules",
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
        warnings.push("Project depth exceeds 8 levels — consider flattening the directory structure".to_string());
    }

    // README.md is the canonical project overview; warn if absent.
    let has_readme = files.iter().any(|f| {
        Path::new(&f.path)
            .file_name()
            .and_then(|n| n.to_str())
            == Some("README.md")
    });
    if !has_readme {
        warnings.push("No README.md found — add a project overview for new contributors".to_string());
    }

    // Missing tests directory — checks if any path component is called "tests"
    // to catch both root-level tests/ and src/tests/ or tests/foo/bar.
    let has_tests = files.iter().any(|f| {
        Path::new(&f.path).components().any(|c| {
            matches!(c, Component::Normal(name) if name == "tests")
        })
    });
    if !has_tests {
        warnings.push("No tests/ directory detected — add test coverage to improve maintainability".to_string());
    }

    warnings
}

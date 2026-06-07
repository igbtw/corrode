// Dependency detection from in-memory file contents.
// Currently supports Cargo.toml (Rust). Designed for future
// support of package.json, pyproject.toml, go.mod, Gemfile.

use crate::models::{FileEntry, ProjectType};

// Scans already-loaded files for the project's manifest file
// and extracts dependency names. No additional disk I/O.
pub fn detect_dependencies(files: &[FileEntry], project_type: &ProjectType) -> Vec<String> {
    match project_type {
        ProjectType::Rust => parse_cargo_deps(files),
        _ => Vec::new(),
    }
}

// Parse [dependencies] from Cargo.toml. Simple line-by-line:
//   - looks for [dependencies] section
//   - stops at the next [section]
//   - ignores comments and blank lines
//   - extracts the crate name before '=', '{', or whitespace
fn parse_cargo_deps(files: &[FileEntry]) -> Vec<String> {
    let cargo = match files.iter().find(|f| f.path.ends_with("Cargo.toml")) {
        Some(f) => f,
        None => return Vec::new(),
    };

    let mut deps = Vec::new();
    let mut in_deps = false;

    for line in cargo.contents.lines() {
        let line = line.trim();

        if line.starts_with("[dependencies]") {
            in_deps = true;
            continue;
        }

        if !in_deps {
            continue;
        }

        // Next TOML section — stop.
        if line.starts_with('[') {
            break;
        }

        // Skip comments and blanks.
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Extract crate name before '=', '{', or whitespace.
        let name = line
            .split_once('=')
            .map(|(n, _)| n.trim())
            .or_else(|| line.split_whitespace().next())
            .unwrap_or("");

        if !name.is_empty() && !name.starts_with('#') {
            deps.push(name.to_string());
        }
    }

    deps
}

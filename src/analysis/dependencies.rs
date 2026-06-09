// Dependency detection from in-memory file contents.
// Supports Cargo.toml (Rust) currently; the match dispatch makes it
// straightforward to add package.json, pyproject.toml, go.mod, Gemfile.

use crate::models::{FileEntry, ProjectType};

// Reads already-loaded file contents so no extra disk I/O is needed.
pub fn detect_dependencies(files: &[FileEntry], project_type: &ProjectType) -> Vec<String> {
    match project_type {
        ProjectType::Rust => parse_cargo_deps(files),
        _ => Vec::new(),
    }
}

// Line-oriented Cargo.toml parser. Not a full TOML parser — just enough
// to extract crate names from [dependencies]. Stops at the next section.
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

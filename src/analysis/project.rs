// Project type and entry point detection from manifest files.

use std::path::Path;

use crate::models::ProjectType;

// Known entry-point paths per project type. Looked up in order.
const ENTRY_POINTS: &[(ProjectType, &[&str])] = &[
    (ProjectType::Rust, &["src/main.rs", "src/lib.rs"]),
    (
        ProjectType::Node,
        &["index.js", "src/index.js", "index.ts", "src/index.ts"],
    ),
    (ProjectType::Go, &["main.go", "cmd/main.go"]),
    (
        ProjectType::Python,
        &["main.py", "app.py", "src/main.py", "src/app.py"],
    ),
    (ProjectType::Ruby, &["main.rb", "app.rb", "bin/rails"]),
];

// Check for known manifest files in the project root.
// Returns Unknown if no match is found or if the path is a file.
pub fn detect_project_type(path: &str) -> ProjectType {
    let root = Path::new(path);
    if !root.is_dir() {
        return ProjectType::Unknown;
    }

    if root.join("Cargo.toml").exists() {
        ProjectType::Rust
    } else if root.join("package.json").exists() {
        ProjectType::Node
    } else if root.join("go.mod").exists() {
        ProjectType::Go
    } else if root.join("pyproject.toml").exists() || root.join("setup.py").exists() {
        ProjectType::Python
    } else if root.join("Gemfile").exists() {
        ProjectType::Ruby
    } else {
        ProjectType::Unknown
    }
}

// Scans `files` for an entry that ends with one of the known
// entry point paths for the given project type.
// Normalizes separators to forward slashes for cross-platform matching.
pub fn detect_entry_point(files: &[String], project_type: &ProjectType) -> Option<String> {
    let candidates = ENTRY_POINTS
        .iter()
        .find(|(pt, _)| pt == project_type)
        .map(|(_, candidates)| *candidates)?;

    for file_path in files {
        let normalised = file_path.replace('\\', "/");
        for candidate in candidates {
            if normalised.ends_with(candidate) {
                return Some(candidate.to_string());
            }
        }
    }

    None
}

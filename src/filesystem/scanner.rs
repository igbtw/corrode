// File discovery and project detection via walkdir.
// Scans the filesystem respecting skip lists, then detects
// project type and entry point from well-known manifest files.

use std::path::Path;

use walkdir::WalkDir;

use crate::models::ProjectType;

// Build artifacts, VCS data, vendored deps, generated output dirs.
pub const SKIP_DIRS: &[&str] = &[
    "target", ".git", "node_modules", ".cargo",
    "dist", "build", "out", "coverage",
    ".next", ".nuxt", "vendor", ".cache",
];

// Binary / cache / media formats that would fail or pollute UTF-8 stats.
pub const SKIP_EXTENSIONS: &[&str] = &[
    "sqlite", "sqlite-wal", "db", "cache",
    "bin", "exe", "dll", "so", "dylib",
    "map", "log", "pdf", "zip", "tar", "gz",
    "jpg", "png", "svg", "ico",
];

// Recursively walk `path`, skipping SKIP_DIRS and SKIP_EXTENSIONS.
// Returns a flat list of valid file paths.
pub fn scan_directory(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        // filter_entry runs before walkdir descends — returning false
        // prevents the entire subtree from being traversed.
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !SKIP_DIRS.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            if !e.file_type().is_file() {
                return false;
            }
            let ext = e.path().extension().and_then(|e| e.to_str()).unwrap_or("");
            if SKIP_EXTENSIONS.contains(&ext) {
                return false;
            }
            let name = e.file_name().to_string_lossy();
            if name.ends_with(".min.js") || name.ends_with(".min.css") {
                return false;
            }
            true
        })
        .map(|e| e.path().display().to_string())
        .collect()
}

// Same walk as scan_directory, but counts directories instead
// of collecting file paths. The root is subtracted from the total.
pub fn count_directories(path: &str) -> usize {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !SKIP_DIRS.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .count()
        .saturating_sub(1)
}

// Check for known manifest files in the project root.
// Returns Unknown if no match is found or if the path is a file.
pub fn detect_project_type(path: &str) -> ProjectType {
    let root = Path::new(path);
    if !root.is_dir() {
        return ProjectType::Unknown;
    }

    // Ordered by likelihood — first match wins.
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

// Common entry-point paths per project type. The first match
// in the scanned file list is used.
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

// Scans `files` for an entry that ends with one of the known
// entry point paths for the given project type.
// Path separators are normalized to forward slashes for cross-platform matching.
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

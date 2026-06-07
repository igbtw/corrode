// ─────────────────────────────────────────────────────────────
// Filesystem scanner
// ─────────────────────────────────────────────────────────────
//
// Provides three public functions built on top of the `walkdir`
// crate:
//
//   1. scan_directory      — recursive file discovery
//   2. count_directories   — count subdirectories
//   3. detect_project_type — identify the language/framework by
//                            checking for manifest files
//
// All three respect the same exclusion lists (SKIP_DIRS and
// SKIP_EXTENSIONS) to keep results focused on source code.

use std::path::Path;

use walkdir::WalkDir;

use crate::models::ProjectType;

/// Directories that are skipped entirely during traversal.
/// Build artifacts, version control data, and vendored
/// dependencies are not relevant for code analysis.
const SKIP_DIRS: &[&str] = &["target", ".git", "node_modules", ".cargo"];

/// File extensions that are never valid UTF-8 source code.
/// Binary formats, databases, and cache files that would
/// either fail to read or pollute language statistics.
const SKIP_EXTENSIONS: &[&str] = &[
    "sqlite", "sqlite-wal", "db", "cache", "bin", "exe", "dll", "so", "dylib",
];

/// Walks a directory recursively and returns the paths of every
/// file found, excluding:
///
///   - Directories listed in SKIP_DIRS (entire subtrees skipped)
///   - Files whose extension is listed in SKIP_EXTENSIONS
pub fn scan_directory(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        // filter_entry runs BEFORE walkdir descends into a directory.
        // If the entry is a directory whose name matches SKIP_DIRS,
        // we return false and the entire subtree is skipped.
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !SKIP_DIRS.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            // Keep only regular files whose extension is not in
            // the skip list.
            if !e.file_type().is_file() {
                return false;
            }
            let ext = e
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            !SKIP_EXTENSIONS.contains(&ext)
        })
        .map(|e| e.path().display().to_string())
        .collect()
}

/// Counts the number of subdirectories under `path` (excluding
/// the root directory itself), using the same SKIP_DIRS rules
/// as `scan_directory`.  Returns 0 for single-file paths.
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
        .saturating_sub(1) // exclude the root
}

/// Detects the project type (Rust, Node, Go, …) by checking for
/// well-known manifest files in the root of `path`.
///
/// Returns `ProjectType::Unknown` when nothing recognizable is
/// found or when `path` is a single file, not a directory.
pub fn detect_project_type(path: &str) -> ProjectType {
    let root = Path::new(path);
    if !root.is_dir() {
        return ProjectType::Unknown;
    }

    // Ordered by likelihood — the first match wins.
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

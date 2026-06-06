// Filesystem scanner — walks directories to find source files.
//
// `filter_entry` is a method on `WalkDir` that decides, for each
// entry (file or directory), whether to descend into it.
// Directories like `target/` and `.git/` contain non-UTF-8 binary
// data and are irrelevant for code analysis, so we skip them.

use walkdir::WalkDir;

/// Names of directories to skip during traversal.
/// These contain build artifacts, version control data,
/// or vendored dependencies — not user source code.
const SKIP_DIRS: &[&str] = &["target", ".git", "node_modules", ".cargo"];

/// Walks a directory recursively and returns all file paths found.
/// Skips binary/vendor directories automatically.
pub fn scan_directory(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        // filter_entry runs BEFORE descending into a directory.
        // If the entry is a directory whose name is in SKIP_DIRS,
        // we return false and walkdir skips the entire subtree.
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !SKIP_DIRS.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect()
}

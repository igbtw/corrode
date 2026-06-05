// ─────────────────────────────────────────────────────────────
// Filesystem scanner — walks directories to find source files
// ─────────────────────────────────────────────────────────────
//
// TODO: implement recursive directory traversal to discover
// all source files in a project (e.g. using the `walkdir` crate).

/// Scans the directory at `path` for source files.
/// Currently a stub that just prints the path.
pub fn scan_directory(path: &str) {
    println!("Scanning {}", path);
}

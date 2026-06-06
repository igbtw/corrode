// Data models — types that represent a codebase's structure.

use std::collections::HashMap;
use std::time::Duration;

/// A single source file discovered during scanning, with its
/// full text content and metadata. `contents` is preserved for
/// future JSON / verbose output.
pub struct FileEntry {
    pub path: String,
    /// Raw file content. Kept for JSON / verbose output;
    /// unused by the summary formatter.
    #[allow(dead_code)]
    pub contents: String,
    /// Number of lines in this file (contents.lines().count()).
    pub line_count: usize,
}

/// The complete result of a project analysis, ready to be
/// formatted by any output module (summary, JSON, Markdown…).
pub struct AnalysisReport {
    /// Every file that was successfully read.
    pub files: Vec<FileEntry>,
    /// Number of directories scanned (excluding skipped ones).
    pub directory_count: usize,
    /// Sum of line_count across all files.
    pub total_lines: usize,
    /// Extension → file count  (e.g. "rs" → 8, "toml" → 2).
    pub language_map: HashMap<String, usize>,
    /// Wall-clock time the analysis took.
    pub duration: Duration,
}

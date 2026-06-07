// ─────────────────────────────────────────────────────────────
// Data models
// ─────────────────────────────────────────────────────────────
//
// Core types that flow through the entire analysis pipeline:
//
//   FileEntry        — a single source file with metadata
//   ProjectType      — detected language/framework (enum)
//   AnalysisReport   — the aggregated result of an analysis run

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

/// A single source file discovered during scanning.
///
/// `contents` is preserved for future parser/analysis stages
/// (syn, tree-sitter, …).  Several fields are unused by the
/// terminal summary today but will be consumed by JSON output
/// and `--verbose` mode — the `#[allow(dead_code)]` attribute
/// suppresses false-positive compiler warnings.
#[allow(dead_code)]
pub struct FileEntry {
    /// Absolute or relative path to the file.
    pub path: String,
    /// Full text content of the file.
    pub contents: String,
    /// File extension (lowercase, no dot), e.g. "rs", "toml".
    pub extension: String,
    /// Number of lines in the file.
    pub line_count: usize,
    /// File size on disk in bytes.
    pub size_bytes: u64,
}

/// Detectable project types based on manifest files found in the
/// project root directory.  Each variant corresponds to a known
/// language or framework ecosystem.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Rust,
    Node,
    Go,
    Python,
    Ruby,
    Unknown,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::Node => write!(f, "Node"),
            ProjectType::Go => write!(f, "Go"),
            ProjectType::Python => write!(f, "Python"),
            ProjectType::Ruby => write!(f, "Ruby"),
            ProjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// The complete result of a project analysis, ready to be
/// consumed by any output formatter (terminal summary, JSON,
/// Markdown, …).
pub struct AnalysisReport {
    /// Detected project type (e.g. Rust, Node, Unknown).
    pub project_type: ProjectType,
    /// Every file that was successfully read.
    pub files: Vec<FileEntry>,
    /// Number of directories scanned (excluding skipped ones).
    pub directory_count: usize,
    /// Sum of `line_count` across all files.
    pub total_lines: usize,
    /// File extension → file count  (e.g. "rs" → 8, "toml" → 2).
    pub language_map: HashMap<String, usize>,
    /// Wall-clock duration of the analysis.
    pub duration: Duration,
}

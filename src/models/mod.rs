// Core types that flow through the pipeline:
//   FileEntry      — one file with path, contents, extension, line count, size
//   ProjectType    — enum of known ecosystems (Rust, Node, Go, Python, Ruby)
//   AnalysisReport — the final result of an analysis run

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

/// A source file with metadata.
// `contents` is kept so future syn/tree-sitter parsers can
// analyse ASTs without re-reading from disk.
#[allow(dead_code)]
pub struct FileEntry {
    pub path: String,
    pub contents: String,
    pub extension: String,
    pub line_count: usize,
    pub size_bytes: u64,
}

/// Detected language/framework based on manifest files found
// in the project root (Cargo.toml, package.json, etc.).
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

// Carries all data needed by any output formatter
// (terminal summary, JSON, Markdown, etc.).
pub struct AnalysisReport {
    pub project_type: ProjectType,
    pub entry_point: Option<String>,
    pub files: Vec<FileEntry>,
    pub directory_count: usize,
    pub total_lines: usize,
    pub language_map: HashMap<String, usize>,
    // Used by depth-map to compute relative depth.
    pub project_root: String,
    pub duration: Duration,
}

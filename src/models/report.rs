// Data types produced by the analysis pipeline.

use std::collections::HashMap;
use std::time::Duration;

use super::ProjectType;

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

/// Aggregated stats for a single directory.
pub struct DirectoryStat {
    pub path: String,
    pub file_count: usize,
    pub total_lines: usize,
    #[allow(dead_code)]
    pub total_bytes: u64,
}

/// File-size buckets used in verbose output.
pub struct SizeDistribution {
    pub noise: usize,
    pub small: usize,
    pub medium: usize,
    pub large: usize,
}

/// A top-level directory with its share of total project LOC.
pub struct Hotspot {
    pub path: String,
    pub total_lines: usize,
    pub percentage: f64,
}

/// Aggregate structural metrics.
pub struct ArchitectureMetrics {
    pub max_depth: usize,
    pub avg_loc_per_file: f64,
    pub median_loc_per_file: f64,
    pub avg_file_size: f64,
}

/// Complexity score (0–100) with a human-readable rating.
pub struct ComplexityReport {
    pub score: u8,
    pub rating: String,
}

/// Breakdown of file counts and LOC by category (code, config, docs).
pub struct CodeMetrics {
    pub code_files: usize,
    pub code_lines: usize,
    pub config_files: usize,
    pub config_lines: usize,
    pub docs_files: usize,
    pub docs_lines: usize,
}

/// Carries all data needed by any output formatter
// (terminal summary, JSON, Markdown, etc.).
pub struct AnalysisReport {
    pub project_type: ProjectType,
    pub entry_point: Option<String>,
    pub files: Vec<FileEntry>,
    pub directory_count: usize,
    pub total_lines: usize,
    pub language_map: HashMap<String, usize>,
    #[allow(dead_code)]
    pub project_root: String,
    pub duration: Duration,
    pub dependencies: Vec<String>,
    pub directory_stats: Vec<DirectoryStat>,
    pub depth_map: Vec<(usize, usize)>,
    pub size_distribution: SizeDistribution,
    pub hotspots: Vec<Hotspot>,
    pub architecture: ArchitectureMetrics,
    pub complexity: ComplexityReport,
    pub warnings: Vec<String>,
    pub code_metrics: CodeMetrics,
}

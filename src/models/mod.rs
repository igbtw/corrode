// Core types that flow through the pipeline.

pub mod report;

pub use report::{
    AnalysisReport, ArchitectureMetrics, CodeMetrics, ComplexityReport, DirectoryStat, FileEntry,
    Hotspot, SizeDistribution,
};

use std::fmt;

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

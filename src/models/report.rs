use serde::Serialize;
use serde::ser::{SerializeStruct, Serializer};

use super::file::FileReport;
use super::metrics::{ArchitectureMetrics, CodeMetrics};
use super::project::ProjectInfo;
use super::quality::QualityReport;

#[derive(Serialize)]
pub struct Hotspot {
    pub path: String,
    pub total_lines: usize,
    pub percentage: f64,
}

pub struct DependencyReport {
    pub list: Vec<String>,
}

impl Serialize for DependencyReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DependencyReport", 2)?;
        s.serialize_field("count", &self.list.len())?;
        s.serialize_field("items", &self.list)?;
        s.end()
    }
}

#[derive(Serialize)]
pub struct ArchitectureReport {
    pub metrics: ArchitectureMetrics,
    pub hotspots: Vec<Hotspot>,
    pub code_metrics: CodeMetrics,
}

#[derive(Serialize)]
pub struct AnalysisReport {
    pub project: ProjectInfo,
    pub files: FileReport,
    pub dependencies: DependencyReport,
    pub architecture: ArchitectureReport,
    pub quality: QualityReport,
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use crate::models::file::{DirectoryStat, FileEntry, FileReport, SizeDistribution};
    use crate::models::metrics::{ArchitectureMetrics, CodeMetrics};
    use crate::models::project::{ProjectInfo, ProjectType};
    use crate::models::quality::{ComplexityScore, HealthScore, QualityReport, ScoreFactor};
    use std::time::Duration;

    pub fn sample_report() -> AnalysisReport {
        AnalysisReport {
            project: ProjectInfo {
                project_type: ProjectType::Rust,
                entry_point: Some("src/main.rs".into()),
                project_root: "/test".into(),
                duration: Duration::from_secs_f64(0.5),
            },
            files: FileReport {
                entries: vec![FileEntry {
                    path: "src/main.rs".into(),
                    contents: "fn main() {}".into(),
                    extension: "rs".into(),
                    line_count: 1,
                    size_bytes: 12,
                }],
                directory_count: 2,
                total_lines: 1,
                language_map: [("rs".into(), 1)].into(),
                directory_stats: vec![DirectoryStat {
                    path: "src".into(),
                    file_count: 1,
                    total_lines: 1,
                    total_bytes: 12,
                }],
                depth_map: vec![(0, 1)],
                size_distribution: SizeDistribution {
                    noise: 1,
                    small: 0,
                    medium: 0,
                    large: 0,
                },
            },
            dependencies: DependencyReport {
                list: vec!["clap".into(), "serde".into()],
            },
            architecture: ArchitectureReport {
                metrics: ArchitectureMetrics {
                    max_depth: 2,
                    avg_loc_per_file: 1.0,
                    median_loc_per_file: 1.0,
                    avg_file_size: 12.0,
                },
                hotspots: vec![Hotspot {
                    path: "src/".into(),
                    total_lines: 1,
                    percentage: 100.0,
                }],
                code_metrics: CodeMetrics {
                    code_files: 1,
                    code_lines: 1,
                    config_files: 0,
                    config_lines: 0,
                    docs_files: 0,
                    docs_lines: 0,
                },
            },
            quality: QualityReport {
                complexity: ComplexityScore {
                    score: 10,
                    rating: "Low",
                    factors: vec![
                        ScoreFactor { name: "LOC", score: 5, max: 30 },
                        ScoreFactor { name: "Directory Depth", score: 2, max: 15 },
                        ScoreFactor { name: "Large Files", score: 0, max: 15 },
                        ScoreFactor { name: "Concentration", score: 3, max: 25 },
                        ScoreFactor { name: "Directories", score: 0, max: 15 },
                    ],
                },
                health: HealthScore {
                    score: 30,
                    rating: "Poor",
                    factors: vec![
                        ScoreFactor { name: "Tests", score: 0, max: 30 },
                        ScoreFactor { name: "Warnings", score: 15, max: 25 },
                        ScoreFactor { name: "Concentration", score: 0, max: 20 },
                        ScoreFactor { name: "Documentation", score: 0, max: 10 },
                        ScoreFactor { name: "Large Files", score: 15, max: 15 },
                    ],
                },
                warnings: vec!["No tests/ directory detected".into()],
            },
        }
    }
}

// Shared presentation layer for text-based output formats.
// Extracts all report data once into display-oriented structs so
// renderers only format — they never traverse the AnalysisReport.

use std::path::Path;

use crate::analysis::classification::language_name;
use crate::analysis::metrics::top_code_files;
use crate::models::{AnalysisReport, ScoreFactor};
use crate::utils::formatting::{format_duration, strip_root};

// ── Display helper types ─────────────────────────────────────────────────────

pub struct DirectoryRow {
    pub path: String,
    pub loc: usize,
    pub files: usize,
}

pub struct CodeMetricsRow {
    pub code_files: usize,
    pub code_lines: usize,
    pub config_files: usize,
    pub config_lines: usize,
    pub docs_files: usize,
    pub docs_lines: usize,
}

pub struct ArchitectureRow {
    pub max_depth: usize,
    pub avg_loc_per_file: f64,
    pub median_loc_per_file: f64,
    pub avg_file_size: u64,
}

pub struct HotspotRow {
    pub path: String,
    pub percentage: f64,
}

pub struct SizeDistributionRow {
    pub noise: usize,
    pub small: usize,
    pub medium: usize,
    pub large: usize,
}

pub struct CodeFileRow {
    pub name: String,
    pub lines: usize,
    pub bytes: u64,
}

/// A single factor in a score breakdown, ready for rendering.
#[derive(Clone)]
pub struct FactorRow {
    pub name: String,
    pub score: u8,
    pub max: u8,
}

/// Generic score display — shared by Complexity and Health sections.
pub struct ScoreDisplay {
    pub score: u8,
    pub rating: String,
    pub factors: Vec<FactorRow>,
}

impl ScoreDisplay {
    pub fn from_model(score: u8, rating: &str, factors: &[ScoreFactor]) -> Self {
        ScoreDisplay {
            score,
            rating: rating.to_string(),
            factors: factors
                .iter()
                .map(|f| FactorRow {
                    name: f.name.to_string(),
                    score: f.score,
                    max: f.max,
                })
                .collect(),
        }
    }

    pub fn factor_lines(&self) -> Vec<String> {
        self.factors
            .iter()
            .map(|f| format!("  {:<20} {:>2}/{}", f.name, f.score, f.max))
            .collect()
    }
}

// ── Section display names ────────────────────────────────────────────────────

pub mod section {
    pub const DEPENDENCIES: &str = "Dependencies";
    pub const LARGEST_DIRECTORIES: &str = "Largest Directories";
    pub const CODE_METRICS: &str = "Code Metrics";
    pub const ARCHITECTURE: &str = "Architecture";
    pub const HOTSPOTS: &str = "Hotspots";
    pub const COMPLEXITY: &str = "Complexity";
    pub const HEALTH: &str = "Health";
    pub const WARNINGS: &str = "Warnings";
    pub const LANGUAGES: &str = "Languages";
    pub const LARGEST_CODE_FILES: &str = "Largest Code Files";
}

// ── Presentation report ──────────────────────────────────────────────────────

pub struct PresentationReport {
    pub project_type_label: Option<String>,
    pub entry_point_label: Option<String>,
    pub file_count: usize,
    pub directory_count: usize,
    pub total_lines_display: String,
    pub duration_display: String,

    pub dependencies: Vec<String>,

    pub directory_rows: Vec<DirectoryRow>,

    pub code_metrics: CodeMetricsRow,
    pub architecture: ArchitectureRow,
    pub hotspot_rows: Vec<HotspotRow>,

    pub complexity_score: u8,
    pub complexity_rating: String,
    pub complexity_factors: Vec<FactorRow>,

    pub health_score: u8,
    pub health_rating: String,
    pub health_factors: Vec<FactorRow>,

    pub warnings: Vec<String>,

    pub depth_map: Vec<(usize, usize)>,
    pub size_distribution: SizeDistributionRow,

    pub sorted_languages: Vec<(String, usize)>,
    pub top_code_files: Vec<CodeFileRow>,
}

impl From<&AnalysisReport> for PresentationReport {
    fn from(report: &AnalysisReport) -> Self {
        let project_type_label = if report.project.project_type != crate::models::ProjectType::Unknown
        {
            Some(report.project.project_type.to_string())
        } else {
            None
        };

        let entry_point_label = report.project.entry_point.clone();

        let file_count = report.files.entries.len();
        let directory_count = report.files.directory_count;

        let total_lines_display = crate::utils::formatting::format_number(report.files.total_lines);
        let duration_display = format_duration(&report.project.duration);

        let dependencies = report.dependencies.list.clone();

        let directory_rows: Vec<DirectoryRow> = report
            .files
            .directory_stats
            .iter()
            .map(|s| DirectoryRow {
                path: strip_root(&s.path),
                loc: s.total_lines,
                files: s.file_count,
            })
            .collect();

        let cm = &report.architecture.code_metrics;
        let code_metrics = CodeMetricsRow {
            code_files: cm.code_files,
            code_lines: cm.code_lines,
            config_files: cm.config_files,
            config_lines: cm.config_lines,
            docs_files: cm.docs_files,
            docs_lines: cm.docs_lines,
        };

        let a = &report.architecture.metrics;
        let architecture = ArchitectureRow {
            max_depth: a.max_depth,
            avg_loc_per_file: a.avg_loc_per_file,
            median_loc_per_file: a.median_loc_per_file,
            avg_file_size: a.avg_file_size as u64,
        };

        let hotspot_rows: Vec<HotspotRow> = report
            .architecture
            .hotspots
            .iter()
            .map(|h| HotspotRow {
                path: h.path.clone(),
                percentage: h.percentage,
            })
            .collect();

        let complexity_score = report.quality.complexity.score;
        let complexity_rating = report.quality.complexity.rating.to_string();
        let complexity_factors = report
            .quality
            .complexity
            .factors
            .iter()
            .map(|f| FactorRow {
                name: f.name.to_string(),
                score: f.score,
                max: f.max,
            })
            .collect();

        let health_score = report.quality.health.score;
        let health_rating = report.quality.health.rating.to_string();
        let health_factors = report
            .quality
            .health
            .factors
            .iter()
            .map(|f| FactorRow {
                name: f.name.to_string(),
                score: f.score,
                max: f.max,
            })
            .collect();

        let warnings = report.quality.warnings.clone();

        let depth_map = report.files.depth_map.clone();
        let sd = &report.files.size_distribution;
        let size_distribution = SizeDistributionRow {
            noise: sd.noise,
            small: sd.small,
            medium: sd.medium,
            large: sd.large,
        };

        let mut sorted_languages: Vec<(String, usize)> = report
            .files
            .language_map
            .iter()
            .map(|(k, v)| (language_name(k.as_str()).to_string(), *v))
            .collect();
        sorted_languages.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        let top_code_files: Vec<CodeFileRow> = top_code_files(&report.files.entries, 3)
            .iter()
            .map(|f| {
                let name = Path::new(&f.path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| f.path.clone());
                CodeFileRow {
                    name,
                    lines: f.line_count,
                    bytes: f.size_bytes,
                }
            })
            .collect();

        PresentationReport {
            project_type_label,
            entry_point_label,
            file_count,
            directory_count,
            total_lines_display,
            duration_display,
            dependencies,
            directory_rows,
            code_metrics,
            architecture,
            hotspot_rows,
            complexity_score,
            complexity_rating,
            complexity_factors,
            health_score,
            health_rating,
            health_factors,
            warnings,
            depth_map,
            size_distribution,
            sorted_languages,
            top_code_files,
        }
    }
}

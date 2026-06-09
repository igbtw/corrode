use serde::Serialize;

#[derive(Serialize)]
pub struct ArchitectureMetrics {
    pub max_depth: usize,
    pub avg_loc_per_file: f64,
    pub median_loc_per_file: f64,
    pub avg_file_size: f64,
}

#[derive(Serialize)]
pub struct CodeMetrics {
    pub code_files: usize,
    pub code_lines: usize,
    pub config_files: usize,
    pub config_lines: usize,
    pub docs_files: usize,
    pub docs_lines: usize,
}

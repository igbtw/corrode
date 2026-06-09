use serde::Serialize;

#[derive(Serialize)]
pub struct FileEntry {
    pub path: String,
    #[serde(skip)]
    pub contents: String,
    pub extension: String,
    pub line_count: usize,
    pub size_bytes: u64,
}

#[derive(Serialize)]
pub struct DirectoryStat {
    pub path: String,
    pub file_count: usize,
    pub total_lines: usize,
    pub total_bytes: u64,
}

#[derive(Serialize)]
pub struct SizeDistribution {
    pub noise: usize,
    pub small: usize,
    pub medium: usize,
    pub large: usize,
}

#[derive(Serialize)]
pub struct FileReport {
    pub entries: Vec<FileEntry>,
    pub directory_count: usize,
    pub total_lines: usize,
    pub language_map: std::collections::HashMap<String, usize>,
    pub directory_stats: Vec<DirectoryStat>,
    pub depth_map: Vec<(usize, usize)>,
    pub size_distribution: SizeDistribution,
}

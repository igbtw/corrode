// Architecture metrics from code files only.

use crate::models::{ArchitectureMetrics, FileEntry};

// Caller pre-filters code files so each metric does not re-filter independently.
// File count could be zero when only non-code files exist.
pub fn compute_architecture(files: &[&FileEntry], max_depth: usize) -> ArchitectureMetrics {
    let file_count = files.len();
    if file_count == 0 {
        return ArchitectureMetrics {
            avg_file_size: 0.0,
            avg_loc_per_file: 0.0,
            max_depth: 0,
            median_loc_per_file: 0.0,
        };
    }

    let total_lines: usize = files.iter().map(|f| f.line_count).sum();
    let total_bytes: u64 = files.iter().map(|f| f.size_bytes).sum();

    let avg_loc = total_lines as f64 / file_count as f64;
    let avg_size = total_bytes as f64 / file_count as f64;

    let median_loc = {
        let mut lines: Vec<usize> = files.iter().map(|f| f.line_count).collect();
        lines.sort_unstable();
        let mid = lines.len() / 2;
        if lines.len() % 2 == 0 {
            (lines[mid - 1] + lines[mid]) as f64 / 2.0
        } else {
            lines[mid] as f64
        }
    };

    ArchitectureMetrics {
        avg_file_size: avg_size,
        avg_loc_per_file: avg_loc,
        max_depth,
        median_loc_per_file: median_loc,
    }
}

// Complexity score: measures intrinsic structural complexity (0–100).
// Factors are additive — each contributes 0..max, sum is capped at 100.

use crate::models::{ComplexityScore, Hotspot, ScoreFactor, SizeDistribution};

// ── Maximum possible values per factor (sum = 100) ──────────────────────────

/// Code lines in code files — weighted heaviest.
const MAX_LOC: u8 = 30;
/// Directory depth (max nesting).
const MAX_DEPTH: u8 = 15;
/// Number of files ≥500 LOC + largest file ratio.
const MAX_LARGE_FILES: u8 = 15;
/// Code share of the most-concentrated directory.
const MAX_CONCENTRATION: u8 = 25;
/// Number of directories.
const MAX_DIRECTORIES: u8 = 15;

// ── Saturation denominators (sqrt-scaled input reaches 1.0 at this value) ───

/// sqrt(40_000) / 200 = 1.0
const LOC_SAT: f64 = 200.0;
/// sqrt(9) / 3 = 1.0
const DEPTH_SAT: f64 = 3.0;
/// sqrt(100) / 10 = 1.0
const DIR_SAT: f64 = 10.0;
/// sqrt(16) / 4 = 1.0
const LARGE_FILE_SAT: f64 = 4.0;

/// Compute complexity score from pipeline data.
///
/// All size-related inputs use sqrt scaling so growth is sub-linear:
/// a 40K LOC project (~200 sqrt) does not dwarf a 1K LOC project (~32 sqrt).
pub fn compute_complexity(
    code_total_lines: usize,
    max_depth: usize,
    directory_count: usize,
    hotspots: &[Hotspot],
    size_distribution: &SizeDistribution,
    largest_code_ratio: f64,
) -> ComplexityScore {
    // LOC: sqrt-scaled, saturates at ~40K lines.
    let loc_score =
        (((code_total_lines as f64).sqrt() / LOC_SAT).min(1.0) * MAX_LOC as f64) as u8;

    // Directory depth: sqrt-scaled, saturates at depth 9.
    let depth_score =
        (((max_depth as f64).sqrt() / DEPTH_SAT).min(1.0) * MAX_DEPTH as f64) as u8;

    // Large files: count of files ≥500 LOC (sqrt-scaled, 10 pts max)
    //            + largest file ratio (linear, 5 pts max).
    let large_count =
        (((size_distribution.large as f64).sqrt() / LARGE_FILE_SAT).min(1.0) * 10.0) as u8;
    let large_ratio = (largest_code_ratio * 5.0).min(5.0) as u8;
    let large_files_score = (large_count + large_ratio).min(MAX_LARGE_FILES);

    // Concentration: linear with top directory's share of code.
    let top_pct = hotspots.first().map(|h| h.percentage).unwrap_or(0.0);
    let concentration_score = ((top_pct / 100.0) * MAX_CONCENTRATION as f64) as u8;

    // Directories: sqrt-scaled, saturates at ~100 dirs.
    let dir_score =
        (((directory_count as f64).sqrt() / DIR_SAT).min(1.0) * MAX_DIRECTORIES as f64) as u8;

    let factors = vec![
        ScoreFactor { name: "LOC", score: loc_score, max: MAX_LOC },
        ScoreFactor { name: "Directory Depth", score: depth_score, max: MAX_DEPTH },
        ScoreFactor { name: "Large Files", score: large_files_score, max: MAX_LARGE_FILES },
        ScoreFactor { name: "Concentration", score: concentration_score, max: MAX_CONCENTRATION },
        ScoreFactor { name: "Directories", score: dir_score, max: MAX_DIRECTORIES },
    ];

    let raw: u8 = factors.iter().map(|f| f.score).sum();
    let score = raw.min(100);

    let rating = match score {
        0..=20 => "Low",
        21..=40 => "Moderate",
        41..=60 => "High",
        61..=80 => "Very High",
        _ => "Extreme",
    };

    ComplexityScore {
        score,
        rating,
        factors,
    }
}

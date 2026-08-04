// Health score: measures maintainability and code hygiene (0–100).
// Accumulation-based: starts at 0 and earns points for each positive signal.
// A pristine project (tests, docs, no warnings, even distribution) scores 100.
//
// Factor weights recalibrated for robustness against pathological repos:
// - Tests (25):  Binary, but max lowered because tests alone don't fix monoliths.
// - Warnings (20):  sqrt-scaled penalty on total warnings.
// - Concentration (15):  Sum of top-3 hotspot percentages (not just top-1).
// - Documentation (10):  README presence.
// - Large Files (30):  Combined — count of ≥500-LOC files (log2, 12 pts),
//     largest-file ratio (linear, 10 pts), and absolute large-file sum (8 pts).

use crate::models::{HealthScore, Hotspot, ScoreFactor, SizeDistribution};

const MAX_TESTS: u8 = 25;
const MAX_WARNINGS: u8 = 20;
const MAX_CONCENTRATION: u8 = 15;
const MAX_DOCUMENTATION: u8 = 10;
const MAX_LARGE_FILES: u8 = 30;

pub fn compute_health(
    warnings: &[String],
    hotspots: &[Hotspot],
    has_tests: bool,
    has_readme: bool,
    largest_code_ratio: f64,
    size_distribution: &SizeDistribution,
    code_total_lines: usize,
    top_code_file_lines: &[usize],
) -> HealthScore {
    // Tests: binary.
    let tests_score = if has_tests { MAX_TESTS } else { 0 };

    // Warnings: sqrt-scaled penalty (~7 warnings saturate).
    let warning_penalty =
        (((warnings.len() as f64).sqrt() / 2.5).min(1.0) * MAX_WARNINGS as f64) as u8;
    let warnings_score = MAX_WARNINGS.saturating_sub(warning_penalty);

    // Concentration: sum of top-3 hotspot percentages (capped at 100%).
    let top_3_pct: f64 = hotspots.iter().take(3).map(|h| h.percentage).sum();
    let top_3_pct = top_3_pct.min(100.0);
    let concentration_penalty =
        ((top_3_pct / 100.0) * MAX_CONCENTRATION as f64) as u8;
    let concentration_score = MAX_CONCENTRATION.saturating_sub(concentration_penalty);

    // Documentation: binary.
    let docs_score = if has_readme { MAX_DOCUMENTATION } else { 0 };

    // Large files: three sub-penalties (combined max = MAX_LARGE_FILES).
    // 1. Count of files ≥500 LOC (log2-scaled, up to 12 pts).
    let large_count_penalty =
        (((size_distribution.large as f64 + 1.0).log2() / 8.0).min(1.0) * 12.0) as u8;
    // 2. Largest file ratio (linear, up to 10 pts).
    let ratio_penalty = (largest_code_ratio * 10.0).min(10.0) as u8;
    // 3. Absolute large-file sum — penalize when top-3 files dominate (up to 8 pts).
    let top_3_sum: usize = top_code_file_lines.iter().take(3).sum();
    let top_3_penalty = if code_total_lines > 0 {
        let top_ratio = top_3_sum as f64 / code_total_lines as f64;
        (top_ratio * 8.0).min(8.0) as u8
    } else {
        0
    };
    let total_large_penalty = (large_count_penalty + ratio_penalty + top_3_penalty).min(MAX_LARGE_FILES);
    let large_files_score = MAX_LARGE_FILES.saturating_sub(total_large_penalty);

    let factors = vec![
        ScoreFactor { name: "Tests", score: tests_score, max: MAX_TESTS },
        ScoreFactor { name: "Warnings", score: warnings_score, max: MAX_WARNINGS },
        ScoreFactor { name: "Concentration", score: concentration_score, max: MAX_CONCENTRATION },
        ScoreFactor { name: "Documentation", score: docs_score, max: MAX_DOCUMENTATION },
        ScoreFactor { name: "Large Files", score: large_files_score, max: MAX_LARGE_FILES },
    ];

    let raw: u8 = factors.iter().map(|f| f.score).sum();
    let score = raw.min(100);

    let rating = match score {
        80..=100 => "Excellent",
        60..=79 => "Good",
        40..=59 => "Fair",
        20..=39 => "Poor",
        _ => "Critical",
    };

    HealthScore {
        score,
        rating,
        factors,
    }
}

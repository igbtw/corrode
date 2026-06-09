// Health score: measures maintainability and code hygiene (0–100).
// Penalty-based: starts at 100, deducts for each deficiency.

use crate::models::{HealthScore, Hotspot, ScoreFactor};

// ── Maximum possible values per factor (sum = 100) ──────────────────────────

/// Tests directory present.
const MAX_TESTS: u8 = 30;
/// Number of warnings (fewer = better).
const MAX_WARNINGS: u8 = 25;
/// Code concentration in top directory (lower = better).
const MAX_CONCENTRATION: u8 = 20;
/// README present.
const MAX_DOCUMENTATION: u8 = 10;
/// Largest-file-to-total-code ratio (lower = better).
const MAX_LARGE_FILES: u8 = 15;

/// Compute health score from pipeline data.
///
/// Each factor scores from 0 to its max. The total is the sum.
/// A pristine project (tests, docs, no warnings, even distribution) scores 100.
pub fn compute_health(
    warnings: &[String],
    hotspots: &[Hotspot],
    has_tests: bool,
    has_readme: bool,
    largest_code_ratio: f64,
) -> HealthScore {
    // Tests: binary — either present (full score) or absent (zero).
    let tests_score = if has_tests { MAX_TESTS } else { 0 };

    // Warnings: sqrt-scaled penalty. ~7 warnings saturate at max penalty.
    let warning_penalty =
        (((warnings.len() as f64).sqrt() / 2.5).min(1.0) * MAX_WARNINGS as f64) as u8;
    let warnings_score = MAX_WARNINGS.saturating_sub(warning_penalty);

    // Concentration: linear with top directory's share of code.
    let top_pct = hotspots.first().map(|h| h.percentage).unwrap_or(0.0);
    let concentration_penalty =
        ((top_pct / 100.0) * MAX_CONCENTRATION as f64) as u8;
    let concentration_score = MAX_CONCENTRATION.saturating_sub(concentration_penalty);

    // Documentation: binary — README present or absent.
    let docs_score = if has_readme { MAX_DOCUMENTATION } else { 0 };

    // Large files: linear with largest-file-to-total ratio.
    let large_penalty =
        ((largest_code_ratio * MAX_LARGE_FILES as f64).min(MAX_LARGE_FILES as f64)) as u8;
    let large_files_score = MAX_LARGE_FILES.saturating_sub(large_penalty);

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

// Complexity score: measures intrinsic structural complexity (0–100).
// Factors are additive — each contributes 0..max, sum is capped at 100.
//
// Scaling: ALL size-related inputs use log2 so growth is sub-linear
//   but never fully saturates (unlike sqrt which flattens too early).
//   This keeps complexity meaningful from 1K to 1M+ LOC.
//
// Factor rationale (each max weight justified):
//
//   LOC (30):  Primary driver.  Log2-scaled so 4K LOC ≈ 12 pts,
//      40K ≈ 18 pts, 400K ≈ 24 pts, 4M ≈ 30 pts.  Never dwarfs other factors.
//
//   Directory Depth (15):  Nesting complexity.  Log2-scaled,
//      depth 4 → 7, depth 8 → 11, depth 16 → 15.
//
//   Large Files (20):  Combined count of files ≥500 LOC (log2, 12 pts max)
//      + largest-file-to-total ratio (linear, 8 pts max).  Penalizes
//      monolithic files even when LOC total is moderate.
//
//   Concentration (20):  Top directory's share of code (linear).
//      A 50%-concentrated project is inherently harder to navigate.
//
//   Directories (15):  Log2-scaled, 10 dirs → 4, 100 → 7, 1000 → 11,
//      4000 → 15.  Many directories = complex navigation.

use crate::models::{ComplexityScore, Hotspot, ScoreFactor, SizeDistribution};

const MAX_LOC: u8 = 30;
const MAX_DEPTH: u8 = 15;
const MAX_LARGE_FILES: u8 = 20;
const MAX_CONCENTRATION: u8 = 20;
const MAX_DIRECTORIES: u8 = 15;

/// Log2-based scale factor.  `value / log2(denom)` reaches 1.0 when value = denom.
fn log_scale(value: f64, denom: f64) -> f64 {
    (value + 1.0).log2() / (denom + 1.0).log2()
}

pub fn compute_complexity(
    code_total_lines: usize,
    max_depth: usize,
    directory_count: usize,
    hotspots: &[Hotspot],
    size_distribution: &SizeDistribution,
    largest_code_ratio: f64,
) -> ComplexityScore {
    // LOC: log2-scaled, denominator 4_000_000 → 1.0 at ~4M LOC.
    let loc_score =
        (log_scale(code_total_lines as f64, 4_000_000.0).min(1.0) * MAX_LOC as f64) as u8;

    // Directory depth: log2-scaled, denominator 16 → 1.0 at depth 15.
    let depth_score = (log_scale(max_depth as f64, 16.0).min(1.0) * MAX_DEPTH as f64) as u8;

    // Large files: count of files ≥500 LOC (log2, 12 pts max)
    //            + largest file ratio (linear, 8 pts max).
    let large_count = (log_scale(size_distribution.large as f64, 200.0).min(1.0) * 12.0) as u8;
    let large_ratio = (largest_code_ratio * 8.0).min(8.0) as u8;
    let large_files_score = (large_count + large_ratio).min(MAX_LARGE_FILES);

    // Concentration: linear with top directory's share of code.
    let top_pct = hotspots.first().map(|h| h.percentage).unwrap_or(0.0);
    let concentration_score = ((top_pct / 100.0) * MAX_CONCENTRATION as f64) as u8;

    // Directories: log2-scaled, denominator 4000 → 1.0 at ~4000 dirs.
    let dir_score =
        (log_scale(directory_count as f64, 4_000.0).min(1.0) * MAX_DIRECTORIES as f64) as u8;

    let factors = vec![
        ScoreFactor {
            name: "LOC",
            score: loc_score,
            max: MAX_LOC,
        },
        ScoreFactor {
            name: "Directory Depth",
            score: depth_score,
            max: MAX_DEPTH,
        },
        ScoreFactor {
            name: "Large Files",
            score: large_files_score,
            max: MAX_LARGE_FILES,
        },
        ScoreFactor {
            name: "Concentration",
            score: concentration_score,
            max: MAX_CONCENTRATION,
        },
        ScoreFactor {
            name: "Directories",
            score: dir_score,
            max: MAX_DIRECTORIES,
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Hotspot;

    fn make_hotspot(pct: f64) -> Hotspot {
        Hotspot {
            path: String::from("src/"),
            total_lines: 1000,
            percentage: pct,
        }
    }

    #[test]
    fn complexity_small_project() {
        // ~1K LOC, depth 2, 5 dirs, 1 large file, 20% concentration
        let sd = SizeDistribution {
            noise: 10,
            small: 20,
            medium: 5,
            large: 1,
        };
        let hs = vec![make_hotspot(20.0)];
        let score = compute_complexity(1_000, 2, 5, &hs, &sd, 0.05);
        assert!(
            score.score <= 30,
            "small project complexity too high: {}",
            score.score
        );
    }

    #[test]
    fn complexity_medium_project() {
        // ~50K LOC, depth 6, 50 dirs, 10 large files, 25% concentration
        let sd = SizeDistribution {
            noise: 100,
            small: 200,
            medium: 50,
            large: 10,
        };
        let hs = vec![make_hotspot(25.0)];
        let score = compute_complexity(50_000, 6, 50, &hs, &sd, 0.08);
        assert!(
            score.score >= 20 && score.score <= 60,
            "medium project complexity out of range: {}",
            score.score
        );
    }

    #[test]
    fn complexity_large_project() {
        // ~1.6M LOC, depth 12, 800 dirs, 200 large files, 26% concentration
        let sd = SizeDistribution {
            noise: 500,
            small: 1000,
            medium: 300,
            large: 200,
        };
        let hs = vec![make_hotspot(26.0)];
        let score = compute_complexity(1_600_000, 12, 800, &hs, &sd, 0.14);
        assert!(
            score.score >= 60,
            "large project complexity too low: {}",
            score.score
        );
        assert!(
            score.rating == "Very High" || score.rating == "Extreme",
            "large project should be Very High+, got {}",
            score.rating
        );
    }

    #[test]
    fn complexity_massive_project() {
        // ~4M LOC, depth 15, 2000 dirs, 500 large files, 30% concentration
        let sd = SizeDistribution {
            noise: 1000,
            small: 2000,
            medium: 1000,
            large: 500,
        };
        let hs = vec![make_hotspot(30.0)];
        let score = compute_complexity(4_000_000, 15, 2000, &hs, &sd, 0.10);
        assert!(
            score.score >= 70,
            "massive project complexity too low: {}",
            score.score
        );
    }

    #[test]
    fn complexity_very_small_project_is_low() {
        let sd = SizeDistribution {
            noise: 1,
            small: 1,
            medium: 0,
            large: 0,
        };
        let hs = vec![make_hotspot(10.0)];
        let score = compute_complexity(100, 1, 1, &hs, &sd, 0.0);
        assert_eq!(
            score.rating, "Low",
            "very small should be Low, got {}",
            score.rating
        );
    }
}

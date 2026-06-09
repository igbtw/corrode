use serde::Serialize;

/// A single named factor contributing to a score.
#[derive(Clone, Serialize)]
pub struct ScoreFactor {
    pub name: &'static str,
    pub score: u8,
    pub max: u8,
}

/// Complexity score: measures intrinsic structural complexity (0–100).
#[derive(Clone, Serialize)]
pub struct ComplexityScore {
    pub score: u8,
    pub rating: &'static str,
    pub factors: Vec<ScoreFactor>,
}

/// Health score: measures maintainability and code hygiene (0–100).
#[derive(Clone, Serialize)]
pub struct HealthScore {
    pub score: u8,
    pub rating: &'static str,
    pub factors: Vec<ScoreFactor>,
}

#[derive(Serialize)]
pub struct QualityReport {
    pub complexity: ComplexityScore,
    pub health: HealthScore,
    pub warnings: Vec<String>,
}

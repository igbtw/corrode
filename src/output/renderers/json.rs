// JSON output formatter.
// Serializes AnalysisReport directly — no presentation layer needed.

use crate::models::AnalysisReport;
use crate::output::reporter::ReportRenderer;

pub struct JsonReporter;

impl JsonReporter {
    pub fn render_to_string(&self, report: &AnalysisReport) -> String {
        serde_json::to_string_pretty(report).unwrap()
    }
}

impl ReportRenderer for JsonReporter {
    fn render(&self, report: &AnalysisReport) {
        println!("{}", self.render_to_string(report));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::report::test_helpers::sample_report;

    #[test]
    fn json_contains_project_type() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        assert!(json.contains("\"project_type\": \"Rust\""));
    }

    #[test]
    fn json_contains_entry_point() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        assert!(json.contains("\"entry_point\": \"src/main.rs\""));
    }

    #[test]
    fn json_contains_dependencies() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        assert!(json.contains("\"count\": 2"));
        assert!(json.contains("\"clap\""));
        assert!(json.contains("\"serde\""));
    }

    #[test]
    fn json_contains_architecture() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        assert!(json.contains("\"max_depth\": 2"));
    }

    #[test]
    fn json_contains_complexity() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        assert!(json.contains("\"score\": 10"));
    }

    #[test]
    fn json_contains_warnings() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        assert!(json.contains("No tests/ directory detected"));
    }

    #[test]
    fn json_valid_and_parseable() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["project"]["project_type"], "Rust");
        assert_eq!(parsed["quality"]["complexity"]["score"], 10);
    }

    #[test]
    fn json_does_not_include_contents() {
        let report = sample_report();
        let json = JsonReporter.render_to_string(&report);
        assert!(!json.contains("\"contents\""));
    }
}

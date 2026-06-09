// Trait for output formatters that render an AnalysisReport.

use crate::models::AnalysisReport;

pub trait ReportRenderer {
    fn render(&self, report: &AnalysisReport);
}

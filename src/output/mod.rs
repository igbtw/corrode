// Output module — format an AnalysisReport for different targets.
//
// Currently only terminal summary. JSON and Markdown formatters
// can be added here later as sibling modules.

pub mod summary;

pub use summary::print_summary;

// Output module — formats an AnalysisReport for different targets.
//
// Currently only the terminal summary is implemented.  Future
// formatters (JSON, Markdown, HTML) can be added as sibling
// modules and re-exported here.

pub mod summary;

pub use summary::print_summary;

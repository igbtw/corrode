// Analysis module: project detection, metrics, dependencies, report building.

pub mod dependencies;
pub mod metrics;
pub mod project;
pub mod report;

pub use report::analyse;

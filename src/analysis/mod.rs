// Analysis module: project detection, metrics, dependencies, report building.

pub mod architecture;
pub mod ast;
pub mod classification;
pub mod complexity;
pub mod dependencies;
pub mod health;
pub mod hotspots;
pub mod metrics;
pub mod project;
pub mod report;
pub mod warnings;

pub use report::analyse;

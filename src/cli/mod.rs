// CLI argument parsing and command dispatch.

pub mod commands;

mod flags;

pub use commands::Command;
pub use flags::parse_args;

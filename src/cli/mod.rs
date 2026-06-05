// Re-exports the CLI parsing types so they can be used
// from `main.rs` via `crate::cli::*`.
mod flags;

pub use flags::{Command, parse_args};

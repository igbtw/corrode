// CLI argument parsing — re-exports types from the flags submodule
// so that main.rs can import them via `crate::cli::{Command, parse_args}`.

mod flags;

pub use flags::{Command, parse_args};

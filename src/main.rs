// RSfactai — Codebase Analysis Engine (CLI entry point)
//
// Module declarations: each corresponds to a subdirectory
// under `src/`.
mod cli; // CLI argument parsing (clap)
mod filesystem; // File/directory scanning (stub)
mod utils; // Utility functions (analysis, formatting)

use std::process;

use crate::cli::Command;
use crate::cli::parse_args;
use crate::utils::analyse;

fn main() {
    // `parse_args()` calls `Cli::parse()` under the hood,
    // which reads `std::env::args()` automatically. No more
    // manual argument collection.
    //
    // If the user passes `--help`, `--version`, or invalid
    // arguments, clap prints the message and exits the
    // process on its own. That's why there's no error
    // handling here — parse either returns a valid `Cli`
    // or has already terminated.
    let cli = parse_args();

    // Dispatch to the appropriate handler based on the
    // parsed subcommand.
    match cli.command {
        Command::Analyse { path } => match analyse(&path) {
            Ok(contents) => println!("Content:\n{}", contents),
            Err(err) => {
                eprintln!("Failed to read '{}': {}", path, err);
                process::exit(1);
            }
        },
        Command::Version => {
            // `env!("CARGO_PKG_VERSION")` is replaced at compile
            // time with the `version` field from Cargo.toml —
            // they can never go out of sync.
            println!("RSfactai v{}", env!("CARGO_PKG_VERSION"));
        }
    }
}

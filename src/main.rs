// RSfactai — Codebase Analysis Engine (CLI entry point)
//
// Module declarations: each corresponds to a subdirectory
// under `src/`.
mod cli; // CLI argument parsing (clap)
mod filesystem; // File/directory scanning (stub)
mod models;
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

    // Handle global flags before dispatching to subcommands.
    // `--license` is a flag, not a subcommand — it's checked
    // directly on the `Cli` struct.
    if cli.license {
        println!(
            "MIT License

Copyright (c) 2026 igbtw

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the 'Software'), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE."
        );
        return;
    }

    // Dispatch to the appropriate handler based on the
    // parsed subcommand.
    match cli.command {
        None => {
            eprintln!("error: a subcommand is required\n");
            eprintln!("Usage: rsfactai [OPTIONS] <COMMAND>\n");
            eprintln!("For more information, try '--help'.");
            process::exit(1);
        }
        Some(Command::Analyse { path }) => match analyse(&path) {
            Ok(entries) => {
                for entry in &entries {
                    println!("── {} ──", entry.path);
                    println!("{}\n", entry.contents);
                }
            }
            Err(err) => {
                eprintln!("Failed to read '{}': {}", path, err);
                process::exit(1);
            }
        },
        Some(Command::Version) => {
            // `env!("CARGO_PKG_VERSION")` is replaced at compile
            // time with the `version` field from Cargo.toml —
            // they can never go out of sync.
            println!("RSfactai v{}", env!("CARGO_PKG_VERSION"));
        }
    }
}

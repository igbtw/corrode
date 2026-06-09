// Entry point. Parses CLI, dispatches to analyse or version.
// --license is checked before subcommand dispatch so it works without one.

use std::process;

use corrode::analysis::analyse;
use corrode::cli::Command;
use corrode::cli::parse_args;

use corrode::output::{print_summary, print_tree};
use corrode::output::renderers::json::JsonReporter;
use corrode::output::renderers::markdown::MarkdownReporter;
use corrode::output::reporter::ReportRenderer;

fn main() {
    let cli = parse_args();

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

    match cli.command {
        None => {
            eprintln!("error: a subcommand is required\n");
            eprintln!("Usage: corrode [OPTIONS] <COMMAND>\n");
            eprintln!("For more information, try '--help'.");
            process::exit(1);
        }
        Some(Command::Analyse {
            path,
            verbose,
            tree,
            json,
            markdown,
        }) => {
            // Tree mode is a quick filesystem dump — no analysis needed.
            if tree {
                print_tree(&path);
            } else {
                match analyse(&path) {
                    Ok(report) => {
                        if json {
                            JsonReporter.render(&report);
                        } else if markdown {
                            MarkdownReporter.render(&report);
                        } else {
                            print_summary(&report, verbose);
                        }
                    }
                    Err(err) => {
                        eprintln!("Failed to read '{}': {}", path, err);
                        process::exit(1);
                    }
                }
            }
        }
        Some(Command::Version) => {
            // env!("CARGO_PKG_VERSION") is resolved at compile time.
            println!("corrode v{}", env!("CARGO_PKG_VERSION"));
        }
    }
}

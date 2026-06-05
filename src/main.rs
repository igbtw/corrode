mod cli;
mod filesystem;
mod utils;

use std::env;
use std::process;

use crate::cli::{Command, parse_args};
use crate::utils::analyse;

fn main() {
    // Collect all command-line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Validate and parse the raw arguments into a structured configuration.
    // Exit the program if the provided arguments are invalid.
    let command = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match command {
        Command::Analyse { path } => {
            let contents: String = analyse(&path);
            println!("Content:\n{}", contents);
        }

        Command::Help => {
            println!(
                "RSfactAI - Command Line Analysis Tool (IN DEVELOPMENT)

USAGE:
    rsfactai <COMMAND> [ARGUMENTS]

COMMANDS:
    analyse, a <PATH>    Analyzes the specified file at the given path.
    version, v           Prints version information.
    help, h              Prints this help message.

EXAMPLES:
    rsfactai analyse poem.txt
    rsfactai a poem.txt
    rsfactai version
    rsfactai v"
            );
        }

        Command::Version => {
            println!("Version: RSfactai v0.0.1 (Beta)");
        }
    }
}

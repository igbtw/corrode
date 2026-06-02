mod cli;
mod utils;

use std::env;
use std::process;

use crate::cli::Flags;
use crate::utils::analysis::analyse_file;

fn main() {
    // Collect all command-line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Validate and parse the raw arguments into a structured configuration.
    // Exit the program if the provided arguments are invalid.
    let flags = Flags::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match flags.command.as_str() {
        "analyse" | "a" => {
            let contents = analyse_file(&flags.filename);
            println!("Content:\n{}", contents);
        }
        _ => {
            eprintln!("Unknown command: {}", flags.command);
            process::exit(1);
        }
    }
}

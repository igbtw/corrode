mod cli;

use std::env;
use std::fs;
use std::process;

use crate::cli::Flags;

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
            println!("Reading {}:", flags.filename);

            // Read the entire file into memory.
            // Exit the program if the file cannot be opened or read.
            let contents = fs::read_to_string(&flags.filename).unwrap_or_else(|err| {
                eprintln!("Failed to read '{}': {}", flags.filename, err);
                process::exit(1);
            });
            println!("The contents is:\n{}", contents);
        }
        _ => {
            eprintln!("Unknown command: {}", flags.command);
            process::exit(1);
        }
    }
}

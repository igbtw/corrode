use std::env;
use std::fs;
use std::process;

fn main() {
    // Collect all command-line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Validate and parse the raw arguments into a structured configuration.
    // Exit the program if the provided arguments are invalid.
    let flags = Flags::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Reading {}:", flags.filename);

    // Read the entire file into memory.
    // Exit the program if the file cannot be opened or read.
    let contents = fs::read_to_string(&flags.filename).unwrap_or_else(|err| {
        eprintln!("Failed to read '{}': {}", flags.filename, err);
        process::exit(1);
    });

    println!("The contents is:\n{}", contents);
}

// Stores the parsed command-line configuration.
struct Flags {
    filename: String,
}

impl Flags {
    // Convert raw CLI arguments into a Flags instance.
    // Returns an error when the required file argument is missing.
    fn new(args: &[String]) -> Result<Flags, &str> {
        // The first argument is always the program name,
        // so at least one additional argument is required.
        if args.len() < 2 {
            return Err("Not enough arguments/flags.");
        }

        // Treat the first user-provided argument as the target file.
        let filename = args[1].clone();

        Ok(Flags { filename })
    }
}


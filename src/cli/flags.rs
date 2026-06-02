// Stores the parsed command-line configuration.
pub struct Flags {
    pub command: String,
    pub filename: String,
    // TODO:
    // add more flags for the program
    // pub map: String,
    // pub document: String,
}

impl Flags {
    // Convert raw CLI arguments into a Flags instance.
    // Returns an error when the required file argument is missing.
    pub fn new(args: &[String]) -> Result<Flags, &str> {
        // The first argument is always the program name,
        // so at least one additional argument is required.
        if args.len() < 3 {
            return Err("Not enough arguments/flags.");
        }

        // Treat the first user-provided argument as the target file.
        let command = args[1].clone();
        let filename = args[2].clone();

        Ok(Flags { command, filename })
    }
}

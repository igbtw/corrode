// Stores the parsed command-line configuration.
pub enum Command {
    Analyse { path: String },
    Version,
    Help,
    // TODO:
    // add more flags for the program
    // pub map: String,
    // pub document: String,
}
// Convert raw CLI arguments into a Flags instance.
// Returns an error when the required file argument is missing.
pub fn parse_args(args: &[String]) -> Result<Command, &'static str> {
    if args.len() < 2 {
        return Err("No command provided.");
    }

    match args[1].as_str() {
        "analyse" | "a" => {
            if args.len() < 3 {
                return Err("Missing filename/path.");
            }
            Ok(Command::Analyse {
                path: args[2].clone(),
            })
        }
        "version" | "v" => Ok(Command::Version),
        "help" | "h" => Ok(Command::Help),
        _ => Err("Unknown command."),
    }
}

use indicatif::ProgressBar;
use std::fs;
use std::process;
use std::time::Duration;

pub fn analyse_file(path: &str) -> String {
    let pb = ProgressBar::new_spinner();

    pb.set_message("Reading file...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Read the entire file into memory.
    // Exit the program if the file cannot be opened or read.
    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Failed to read '{}': {}", path, err);
        process::exit(1);
    });

    pb.finish_with_message("File loaded");

    contents
}

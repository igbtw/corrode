use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::process;
use std::thread;
use std::time::Duration;

pub fn analyse(path: &str) -> String {
    let pb = ProgressBar::new_spinner();

    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());

    pb.set_message("[1/4] Reading file.");
    pb.enable_steady_tick(Duration::from_millis(100));

    thread::sleep(Duration::from_secs(3)); // ONLY FOR DEBUG!!!!!!!!!

    // Read the entire file into memory.
    // Exit the program if the file cannot be opened or read.
    let contents = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Failed to read '{}': {}", path, err);
        process::exit(1);
    });

    pb.finish_and_clear();
    println!("✓ [1/4] Reading file.");

    contents
}

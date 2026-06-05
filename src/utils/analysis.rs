// Core analysis logic

use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::time::Duration;

/// Reads the file at `path` into a String while displaying a
/// spinner progress bar to the user.
///
/// Returns `Ok(contents)` on success, or an error if the file
/// cannot be read. The caller (main.rs) is responsible for
/// deciding whether to exit the process.
pub fn analyse(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create a new spinner-style progress bar (indefinite spinner).
    let pb = ProgressBar::new_spinner();

    // Style the spinner: green spinner icon, then a message.
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());

    pb.set_message("[1/4] Reading file.");
    // Makes the spinner tick every 100ms for a smooth animation.
    pb.enable_steady_tick(Duration::from_millis(100));

    // `?` propagates the error to the caller instead of
    // calling process::exit() here — keeps this function
    // testable and reusable.
    let contents = fs::read_to_string(path)?;

    // Stop the spinner and print a completion checkmark.
    pb.finish_and_clear();
    println!("✓ [1/4] Reading file.");

    Ok(contents)
}

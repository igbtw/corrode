// ─────────────────────────────────────────────────────────────
// Terminal summary formatter
// ─────────────────────────────────────────────────────────────
//
// Takes an AnalysisReport and prints a human-readable project
// overview to stdout, including:
//
//   • Detected project type (if not Unknown)
//   • Project overview (files, dirs, LOC)
//   • Language breakdown (extension → count, sorted descending)
//   • Largest files (top 3 by line count)
//   • Elapsed time (auto-scaled: ms or s)

use std::path::Path;
use std::time::Duration;

use crate::models::{AnalysisReport, ProjectType};

/// Entry point for the terminal summary output.  Prints the
/// complete report to stdout.
pub fn print_summary(report: &AnalysisReport) {
    println!();

    // Show the detected project type (e.g. "Detected      Rust Project")
    // unless the type could not be determined.
    if report.project_type != ProjectType::Unknown {
        println!("Detected      {} Project", report.project_type);
        println!();
    }

    print_project_overview(report);
    print_languages(report);
    print_largest_files(report);

    println!();
    println!("Completed in {}", format_duration(&report.duration));
    println!();
}

/// Prints the one-line project overview:
///   Project      17 files · 7 dirs · 1,025 LOC
fn print_project_overview(report: &AnalysisReport) {
    println!(
        "Project      {} files · {} dirs · {} LOC",
        report.files.len(),
        report.directory_count,
        format_number(report.total_lines)
    );
    println!();
}

/// Prints the language breakdown sorted by file count (descending).
///
/// Each line shows the friendly language name, the count, and
/// pluralised "file" / "files".
fn print_languages(report: &AnalysisReport) {
    println!("Languages");

    let mut languages: Vec<_> = report.language_map.iter().collect();
    languages.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    for (ext, count) in languages {
        let label = if *count == 1 { "file" } else { "files" };
        println!("  {:<12} {:>2} {}", language_name(ext), count, label);
    }

    println!();
}

/// Prints the top 3 largest files by line count (descending).
///
/// Shows only the file name (not the full path) for readability.
fn print_largest_files(report: &AnalysisReport) {
    println!("Largest Files");

    let mut files: Vec<_> = report.files.iter().collect();
    files.sort_by(|a, b| b.line_count.cmp(&a.line_count));

    for file in files.into_iter().take(3) {
        let name = Path::new(&file.path)
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_else(|| file.path.as_str().into());

        println!("  {:<16} {}", name, file.line_count);
    }
}

/// Maps a lowercase file extension to its human-friendly name.
/// Unknown extensions are passed through as-is.
fn language_name(ext: &str) -> &str {
    match ext {
        "rs" => "Rust",
        "toml" => "TOML",
        "md" => "Markdown",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "txt" => "Text",
        "lock" => "Lock",
        other => other,
    }
}

/// Formats an integer with thousands separators (commas).
/// Example: 1025 → "1,025".
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

/// Formats a Duration for display, automatically choosing
/// milliseconds or seconds based on magnitude:
///
///   < 1 ms     → "0.410 ms"
///   < 1 s      → "23.4 ms"
///   otherwise  → "1.24 s"
fn format_duration(duration: &Duration) -> String {
    let ms = duration.as_secs_f64() * 1000.0;

    if ms < 1.0 {
        format!("{:.3} ms", ms)
    } else if ms < 1000.0 {
        format!("{:.1} ms", ms)
    } else {
        format!("{:.2} s", ms / 1000.0)
    }
}

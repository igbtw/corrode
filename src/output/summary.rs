// Terminal summary formatter.
// Takes an AnalysisReport and prints:
//   - Project type and entry point
//   - File/dir/LOC overview
//   - File categories (Code / Config / Documentation / Other)
//   - Directory structure (collapsed to top-level if >15 dirs)
//   - Language breakdown and raw extension counts
//   - Top 3 largest files by line count (with size)
//
// In verbose mode (--verbose) also prints:
//   - Depth map (file count per relative directory depth)
//   - Size distribution (noise/small/medium/large)

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use crate::models::{AnalysisReport, ProjectType};

// When structure has more than this many distinct directories,
// non-verbose output collapses entries under their top-level
// segment (e.g. "crates/foo" → "crates/").
const COLLAPSE_THRESHOLD: usize = 15;

pub fn print_summary(report: &AnalysisReport, verbose: bool) {
    println!();

    if report.project_type != ProjectType::Unknown {
        println!("Detected      {} Project", report.project_type);
        println!();
    }

    if let Some(ref ep) = report.entry_point {
        println!("Entry Point   {}", ep);
        println!();
    }

    print_project_overview(report);
    print_file_categories(report);
    print_structure(report, verbose);
    if verbose {
        print_depth_map(report);
        print_size_distribution(report);
    }
    print_languages(report);
    print_extensions(report);
    print_largest_files(report);

    println!();
    println!("Completed in {}", format_duration(&report.duration));
    println!();
}

fn print_file_categories(report: &AnalysisReport) {
    println!("File Categories");
    let mut cats: Vec<_> = report.file_categories.iter().collect();
    cats.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    for (cat, count) in cats {
        let label = if *count == 1 { "File" } else { "Files" };
        println!("  {:<22} {}", format!("{} {}", cat, label), format_number(*count));
    }
    println!();
}

fn print_project_overview(report: &AnalysisReport) {
    println!(
        "Project      {} files · {} dirs · {} LOC",
        report.files.len(),
        report.directory_count,
        format_number(report.total_lines)
    );
    println!();
}

// Groups files by parent directory and prints each dir with
// its file count. Sorted by count descending, then name ascending.
// If there are more dirs than COLLAPSE_THRESHOLD and verbose
// is off, dirs are collapsed to their top-level segment.
fn print_structure(report: &AnalysisReport, verbose: bool) {
    if report.files.len() <= 1 && report.project_type == ProjectType::Unknown {
        return;
    }

    println!("Structure");

    let mut dirs: HashMap<String, usize> = HashMap::new();
    for entry in &report.files {
        let parent = Path::new(&entry.path)
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        *dirs.entry(parent).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = dirs.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    if verbose || sorted.len() <= COLLAPSE_THRESHOLD {
        for (dir, count) in &sorted {
            let label = if *count == 1 { "file" } else { "files" };
            println!("  {:<20} {} {}", strip_root(dir), count, label);
        }
    } else {
        let mut collapsed: HashMap<String, usize> = HashMap::new();
        for (dir, count) in &sorted {
            let key = top_level(dir);
            *collapsed.entry(key).or_insert(0) += count;
        }
        let mut top_sorted: Vec<_> = collapsed.into_iter().collect();
        top_sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (dir, count) in &top_sorted {
            let label = if *count == 1 { "file" } else { "files" };
            println!("  {:<20} {} {}", dir, count, label);
        }
    }

    println!();
}

// Counts files at each relative depth level (0 = project root).
fn print_depth_map(report: &AnalysisReport) {
    println!("Depth Map");

    let root = Path::new(&report.project_root);

    let mut depths: HashMap<usize, usize> = HashMap::new();
    for entry in &report.files {
        let depth = Path::new(&entry.path)
            .strip_prefix(root)
            .map(|rel| rel.components().count().saturating_sub(1))
            .unwrap_or(0);
        *depths.entry(depth).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = depths.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    for (depth, count) in &sorted {
        let label = if *count == 1 { "file" } else { "files" };
        println!("  Level {}:  {} {}", depth, count, label);
    }

    println!();
}

// Groups files into 4 size buckets: noise (<10), small (10–100),
// medium (100–500), large (≥500).
fn print_size_distribution(report: &AnalysisReport) {
    println!("Size Distribution");

    let noise = report.files.iter().filter(|f| f.line_count < 10).count();
    let small = report
        .files
        .iter()
        .filter(|f| (10..100).contains(&f.line_count))
        .count();
    let medium = report
        .files
        .iter()
        .filter(|f| (100..500).contains(&f.line_count))
        .count();
    let large = report.files.iter().filter(|f| f.line_count >= 500).count();

    let noise_label = if noise == 1 { "file" } else { "files" };
    let small_label = if small == 1 { "file" } else { "files" };
    let medium_label = if medium == 1 { "file" } else { "files" };
    let large_label = if large == 1 { "file" } else { "files" };

    println!("  Noise  (<10 LOC):    {} {}", noise, noise_label);
    println!("  Small  (10–100):     {} {}", small, small_label);
    println!("  Medium (100–500):    {} {}", medium, medium_label);
    println!("  Large  (≥500):       {} {}", large, large_label);

    println!();
}

// Extension → display name mapping. Unknown extensions pass through.
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

// Extension counts (raw names, not display names).
fn print_extensions(report: &AnalysisReport) {
    println!("Extensions");
    let mut exts: Vec<_> = report.language_map.iter().collect();
    exts.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    for (ext, count) in exts {
        println!("  {:<12} {}", ext, count);
    }
    println!();
}

// Sorted by count descending, then name ascending. Each line
// shows the language name + file count.
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

// Sorted by line count descending, top 3 shown with size.
fn print_largest_files(report: &AnalysisReport) {
    println!("Largest Files");

    let mut files: Vec<_> = report.files.iter().collect();
    files.sort_by(|a, b| b.line_count.cmp(&a.line_count));

    for file in files.into_iter().take(3) {
        let name = Path::new(&file.path)
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_else(|| file.path.as_str().into());

        println!("  {:<16} {} lines · {}", name, format_number(file.line_count), format_bytes(file.size_bytes));
    }
}

// Format bytes into human-readable size (B, KB, MB).
fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    if bytes as f64 >= MB {
        format!("{:.2} MB", bytes as f64 / MB)
    } else if bytes as f64 >= KB {
        format!("{:.1} KB", bytes as f64 / KB)
    } else {
        format!("{} B", bytes)
    }
}

// Removes leading "./" from a path; returns "(root)" for empty/dot paths.
fn strip_root(dir: &str) -> String {
    let d = dir.strip_prefix("./").unwrap_or(dir);
    if d.is_empty() || d == "." {
        String::from("(root)")
    } else {
        format!("{}/", d)
    }
}

// Extracts the first path segment, e.g. "src/cli/" → "src/".
fn top_level(dir: &str) -> String {
    let d = dir.strip_prefix("./").unwrap_or(dir);
    if d.is_empty() || d == "." {
        return String::from("(root)");
    }
    if let Some(slash) = d.find('/') {
        format!("{}/", &d[..slash])
    } else {
        format!("{}/", d)
    }
}

// Adds thousands separator commas, e.g. 1234 → "1,234".
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

// Format a Duration for display: sub-ms shows 3 decimals,
// sub-second shows 1 decimal, otherwise 2 decimals in seconds.
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

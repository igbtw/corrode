// Terminal summary formatter.
// Takes an AnalysisReport and prints:
//   - Project type and entry point
//   - File/dir/LOC overview
//   - Dependencies
//   - Largest directories (top 5 by LOC)
//   - Code metrics, architecture, hotspots, complexity, warnings
//   - Language breakdown (top 10 + remainder)
//   - Top 3 largest code files by line count (with size)
//
// In verbose mode (--verbose) also prints:
//   - Depth map (file count per relative directory depth)
//   - Size distribution (noise/small/medium/large)

use std::path::Path;

use crate::analysis::metrics::is_code_extension;
use crate::models::{AnalysisReport, ProjectType};
use crate::utils::formatting::{format_bytes, format_duration, format_number};

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
    print_dependencies(report);
    print_largest_directories(report);
    print_code_metrics(report);
    print_architecture(report);
    print_hotspots(report);
    print_complexity(report);
    print_warnings(report);
    if verbose {
        print_depth_map(report);
        print_size_distribution(report);
    }
    print_languages(report);
    print_largest_code_files(report);

    println!();
    println!("Completed in {}", format_duration(&report.duration));
    println!();
}

fn print_dependencies(report: &AnalysisReport) {
    if report.dependencies.is_empty() {
        return;
    }

    let total = report.dependencies.len();
    let show = 4;
    println!("Dependencies");

    for dep in report.dependencies.iter().take(show) {
        println!("  {}", dep);
    }

    if total > show {
        println!("  +{} more", total - show);
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

fn print_largest_directories(report: &AnalysisReport) {
    println!("Largest Directories");
    for stat in &report.directory_stats {
        println!(
            "  {:<20} {} LOC · {} files",
            strip_root(&stat.path),
            format_number(stat.total_lines),
            stat.file_count,
        );
    }
    println!();
}

fn print_code_metrics(report: &AnalysisReport) {
    let cm = &report.code_metrics;
    println!("Code Metrics");
    println!(
        "  Code Files         {}  ·  {} LOC",
        format_number(cm.code_files),
        format_number(cm.code_lines),
    );
    println!(
        "  Config Files       {}  ·  {} LOC",
        format_number(cm.config_files),
        format_number(cm.config_lines),
    );
    println!(
        "  Docs Files         {}  ·  {} LOC",
        format_number(cm.docs_files),
        format_number(cm.docs_lines),
    );
    println!();
}

fn print_architecture(report: &AnalysisReport) {
    let a = &report.architecture;
    println!("Architecture");
    println!("  Max Depth          {}", a.max_depth);
    println!("  Avg LOC/File       {:.0}", a.avg_loc_per_file);
    println!("  Median LOC/File    {:.0}", a.median_loc_per_file);
    println!("  Avg File Size      {}", format_bytes(a.avg_file_size as u64));
    println!();
}

fn print_hotspots(report: &AnalysisReport) {
    println!("Hotspots");
    for spot in &report.hotspots {
        println!("  {:<20} {:>3.0}%", spot.path, spot.percentage);
    }
    println!();
}

fn print_complexity(report: &AnalysisReport) {
    println!("Complexity");
    println!("  Score             {}/100", report.complexity.score);
    println!("  Rating            {}", report.complexity.rating);
    println!();
}

fn print_warnings(report: &AnalysisReport) {
    if report.warnings.is_empty() {
        return;
    }

    println!("Warnings");
    for w in &report.warnings {
        println!("  \u{2022} {}", w);
    }
    println!();
}

fn print_depth_map(report: &AnalysisReport) {
    println!("Depth Map");
    for (depth, count) in &report.depth_map {
        let label = if *count == 1 { "file" } else { "files" };
        println!("  Level {}:  {} {}", depth, count, label);
    }
    println!();
}

fn print_size_distribution(report: &AnalysisReport) {
    let sd = &report.size_distribution;
    println!("Size Distribution");

    let noise_label = if sd.noise == 1 { "file" } else { "files" };
    let small_label = if sd.small == 1 { "file" } else { "files" };
    let medium_label = if sd.medium == 1 { "file" } else { "files" };
    let large_label = if sd.large == 1 { "file" } else { "files" };

    println!("  Noise  (<10 LOC):    {} {}", sd.noise, noise_label);
    println!("  Small  (10–100):     {} {}", sd.small, small_label);
    println!("  Medium (100–500):    {} {}", sd.medium, medium_label);
    println!("  Large  (≥500):       {} {}", sd.large, large_label);

    println!();
}

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

fn print_languages(report: &AnalysisReport) {
    println!("Languages");

    let mut languages: Vec<_> = report.language_map.iter().collect();
    languages.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));

    let total = languages.len();
    let show = 10;

    for (ext, count) in languages.iter().take(show) {
        let label = if **count == 1 { "file" } else { "files" };
        println!("  {:<12} {:>2} {}", language_name(ext), count, label);
    }

    if total > show {
        println!("  +{} more", total - show);
    }

    println!();
}

fn print_largest_code_files(report: &AnalysisReport) {
    println!("Largest Code Files");

    let mut files: Vec<_> = report.files.iter().collect();
    files.sort_by(|a, b| b.line_count.cmp(&a.line_count));

    let mut shown = 0;
    for file in files {
        if !is_code_extension(&file.extension) {
            continue;
        }
        if shown >= 3 {
            break;
        }
        shown += 1;

        let name = Path::new(&file.path)
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_else(|| file.path.as_str().into());

        println!(
            "  {:<16} {} lines · {}",
            name,
            format_number(file.line_count),
            format_bytes(file.size_bytes),
        );
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

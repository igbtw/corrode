use std::path::Path;

use crate::models::AnalysisReport;

pub fn print_summary(report: &AnalysisReport) {
    println!();

    print_header(report);
    print_languages(report);
    print_largest_files(report);

    println!();
    println!(
        "Completed in {:.3} ms",
        report.duration.as_secs_f64() * 1000.0
    );
    println!();
}

fn print_header(report: &AnalysisReport) {
    println!(
        "Project      {} files · {} dirs · {} LOC",
        report.files.len(),
        report.directory_count,
        format_number(report.total_lines)
    );
    println!();
}

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

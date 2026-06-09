// Terminal summary formatter.
// Consumes PresentationReport — no AnalysisReport traversal.

use crate::output::presentation::{section, PresentationReport};
use crate::output::reporter::ReportRenderer;
use crate::utils::formatting::{format_bytes, format_number};

pub struct SummaryReporter {
    pub verbose: bool,
}

impl ReportRenderer for SummaryReporter {
    fn render(&self, report: &crate::models::AnalysisReport) {
        let pres: PresentationReport = report.into();

        println!();

        if let Some(ref label) = pres.project_type_label {
            println!("Detected      {} Project", label);
            println!();
        }

        if let Some(ref ep) = pres.entry_point_label {
            println!("Entry Point   {}", ep);
            println!();
        }

        print_project_overview(&pres);
        print_dependencies(&pres);
        print_largest_directories(&pres);
        print_code_metrics(&pres);
        print_architecture(&pres);
        print_hotspots(&pres);
        print_score_section(section::COMPLEXITY, pres.complexity_score, &pres.complexity_rating, &pres.complexity_factors);
        print_score_section(section::HEALTH, pres.health_score, &pres.health_rating, &pres.health_factors);
        print_warnings(&pres);
        if self.verbose {
            print_depth_map(&pres);
            print_size_distribution(&pres);
        }
        print_languages(&pres);
        print_largest_code_files(&pres);

        println!();
        println!("Completed in {}", pres.duration_display);
        println!();
    }
}

pub fn print_summary(report: &crate::models::AnalysisReport, verbose: bool) {
    SummaryReporter { verbose }.render(report);
}

fn print_project_overview(pres: &PresentationReport) {
    println!(
        "Project      {} files · {} dirs · {} LOC",
        pres.file_count,
        pres.directory_count,
        pres.total_lines_display,
    );
    println!();
}

fn print_dependencies(pres: &PresentationReport) {
    if pres.dependencies.is_empty() {
        return;
    }

    let total = pres.dependencies.len();
    let show = 4;
    println!("{}", section::DEPENDENCIES);

    for dep in pres.dependencies.iter().take(show) {
        println!("  {}", dep);
    }

    if total > show {
        println!("  +{} more", total - show);
    }

    println!();
}

fn print_largest_directories(pres: &PresentationReport) {
    println!("{}", section::LARGEST_DIRECTORIES);
    for row in &pres.directory_rows {
        println!(
            "  {:<20} {} LOC · {} files",
            row.path,
            format_number(row.loc),
            row.files,
        );
    }
    println!();
}

fn print_code_metrics(pres: &PresentationReport) {
    let cm = &pres.code_metrics;
    println!("{}", section::CODE_METRICS);
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

fn print_architecture(pres: &PresentationReport) {
    let a = &pres.architecture;
    println!("{}", section::ARCHITECTURE);
    println!("  Max Depth          {}", a.max_depth);
    println!("  Avg LOC/File       {:.0}", a.avg_loc_per_file);
    println!("  Median LOC/File    {:.0}", a.median_loc_per_file);
    println!("  Avg File Size      {}", format_bytes(a.avg_file_size));
    println!();
}

fn print_hotspots(pres: &PresentationReport) {
    println!("{}", section::HOTSPOTS);
    for spot in &pres.hotspot_rows {
        println!("  {:<20} {:>3.0}%", spot.path, spot.percentage);
    }
    println!();
}

fn print_score_section(title: &str, score: u8, rating: &str, factors: &[crate::output::presentation::FactorRow]) {
    println!("{}", title);
    println!("  Score             {}/100", score);
    println!("  Rating            {}", rating);
    for f in factors {
        println!("  {:<20} {:>2}/{}", f.name, f.score, f.max);
    }
    println!();
}

fn print_warnings(pres: &PresentationReport) {
    if pres.warnings.is_empty() {
        return;
    }

    println!("{}", section::WARNINGS);
    for w in &pres.warnings {
        println!("  \u{2022} {}", w);
    }
    println!();
}

fn print_depth_map(pres: &PresentationReport) {
    println!("Depth Map");
    for (depth, count) in &pres.depth_map {
        let label = if *count == 1 { "file" } else { "files" };
        println!("  Level {}:  {} {}", depth, count, label);
    }
    println!();
}

fn print_size_distribution(pres: &PresentationReport) {
    let sd = &pres.size_distribution;
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

fn print_languages(pres: &PresentationReport) {
    println!("{}", section::LANGUAGES);

    let total = pres.sorted_languages.len();
    let show = 10;

    for (name, count) in pres.sorted_languages.iter().take(show) {
        let label = if *count == 1 { "file" } else { "files" };
        println!("  {:<12} {:>2} {}", name, count, label);
    }

    if total > show {
        println!("  +{} more", total - show);
    }

    println!();
}

fn print_largest_code_files(pres: &PresentationReport) {
    println!("{}", section::LARGEST_CODE_FILES);

    for file in &pres.top_code_files {
        println!(
            "  {:<16} {} lines · {}",
            file.name,
            format_number(file.lines),
            format_bytes(file.bytes),
        );
    }
}

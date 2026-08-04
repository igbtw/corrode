// Terminal summary formatter — report-oriented, Lynis-inspired.
// Sections flow top-to-bottom as a single continuous report.
// Every visual element communicates useful information.
// No decorative borders, no floating widgets.

use std::cmp;

use owo_colors::OwoColorize;
use unicode_width::UnicodeWidthChar;

use crate::output::presentation::PresentationReport;
use crate::output::presentation::shorten_path;
use crate::output::reporter::ReportRenderer;
use crate::utils::formatting::{format_bytes, format_number};

// ── Public entry point ──────────────────────────────────────────────────────

pub struct SummaryReporter {
    pub verbose: bool,
}

impl ReportRenderer for SummaryReporter {
    fn render(&self, report: &crate::models::AnalysisReport) {
        let pres: PresentationReport = report.into();
        let tw = term_width();

        println!();
        print_project_header(&pres, tw);

        println!();
        print_health_section(&pres, tw);

        println!();
        print_complexity_section(&pres, tw);

        if !pres.warnings.is_empty() {
            println!();
            print_key_findings_section(&pres, tw);
        }

        println!();
        print_architecture_section(&pres, tw);

        if !pres.hotspot_rows.is_empty() {
            println!();
            print_hotspots_section(&pres, tw);
        }

        println!();
        print_files_section(&pres, tw);

        println!();
        print_languages_section(&pres, tw);

        if !pres.dependencies.is_empty() {
            println!();
            print_dependencies_section(&pres, tw);
        }

        println!();
        print_code_section(&pres, tw);

        if self.verbose {
            println!();
            print_verbose_depth_map_section(&pres, tw);
            println!();
            print_verbose_size_dist_section(&pres, tw);
            println!();
            print_verbose_lang_pct_section(&pres, tw);
            println!();
            print_verbose_file_types_section(&pres, tw);
            println!();
            print_verbose_deep_dirs_section(&pres, tw);
            println!();
            print_verbose_top_files_section(report, tw);
        }

        println!();
        println!("  {}  {}", "Completed in".dimmed(), pres.duration_display.dimmed());
        println!();
    }
}

pub fn print_summary(report: &crate::models::AnalysisReport, verbose: bool) {
    SummaryReporter { verbose }.render(report);
}

// ── Terminal width ────────────────────────────────────────────────────────

fn term_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(80)
        .max(50)
}

// ── Gauge bar ─────────────────────────────────────────────────────────────

fn gauge(current: usize, max: usize, width: usize) -> String {
    let ratio = if max == 0 {
        0.0
    } else {
        (current as f64 / max as f64).min(1.0)
    };
    let filled = (ratio * width as f64).round() as usize;
    let filled = filled.min(width);
    let mut out = String::with_capacity(width);
    for _ in 0..filled {
        out.push('█');
    }
    for _ in 0..width.saturating_sub(filled) {
        out.push('░');
    }
    out
}

/// Visible character width, ignoring ANSI escape sequences.
fn visible_width(s: &str) -> usize {
    let mut w = 0usize;
    let mut esc = false;
    for c in s.chars() {
        if esc {
            if c == 'm' {
                esc = false;
            }
        } else if c == '\x1b' {
            esc = true;
        } else {
            w += UnicodeWidthChar::width(c).unwrap_or(0);
        }
    }
    w
}

// ── Color helpers ─────────────────────────────────────────────────────────

fn score_rating_line(score: u8, rating: &str) -> String {
    if score >= 80 {
        rating.bold().green().to_string()
    } else if score >= 40 {
        rating.bold().yellow().to_string()
    } else {
        rating.bold().red().to_string()
    }
}

fn color_gauge(score: u8, max: u8, width: usize) -> String {
    let g = gauge(score as usize, max as usize, width);
    if score >= 80 {
        g.green().to_string()
    } else if score >= 40 {
        g.yellow().to_string()
    } else {
        g.red().to_string()
    }
}

// ── Section rendering ─────────────────────────────────────────────────────

fn section_header(title: &str, tw: usize) -> String {
    let dashes = tw.saturating_sub(title.len() + 4);
    format!("── {} {}", title, "─".repeat(dashes))
}

// ── Project header ────────────────────────────────────────────────────────

fn print_project_header(pres: &PresentationReport, _tw: usize) {
    let proj = pres.project_type_label.as_deref().unwrap_or("Project");
    let ep = pres.entry_point_label.as_deref().unwrap_or("");

    let line1 = if ep.is_empty() {
        format!("  {}  Project", proj.bold())
    } else {
        format!("  {}  Project  •  {}", proj.bold(), ep)
    };

    let line2 = format!(
        "  {} files  •  {} dirs  •  {} LOC  •  {}",
        format_number(pres.file_count),
        pres.directory_count,
        pres.total_lines_display,
        pres.duration_display,
    );

    println!("{}", line1);
    println!("{}", line2);
}

// ── Health section (executive summary) ─────────────────────────────────────

fn print_health_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Health", tw));

    let gw = cmp::min(tw.saturating_sub(24), 20).max(4);
    let gauge_str = color_gauge(pres.health_score, 100, gw);
    println!(
        "  {}  {}/{}  {}",
        score_rating_line(pres.health_score, &pres.health_rating),
        pres.health_score,
        100,
        gauge_str,
    );

    let max_name = pres.health_factors.iter().map(|f| f.name.len()).max().unwrap_or(10) + 2;
    let max_score = pres
        .health_factors
        .iter()
        .map(|f| format!("{}/{}", f.score, f.max).len())
        .max()
        .unwrap_or(4)
        + 1;
    let threshold = 0.8;

    let strengths: Vec<&crate::output::presentation::FactorRow> = pres
        .health_factors
        .iter()
        .filter(|f| (f.score as f64) / (f.max as f64) >= threshold)
        .collect();

    let attention: Vec<&crate::output::presentation::FactorRow> = pres
        .health_factors
        .iter()
        .filter(|f| (f.score as f64) / (f.max as f64) < threshold)
        .collect();

    if !strengths.is_empty() {
        println!();
        for f in &strengths {
            println!(
                "  ✓ {:<nmw$} {:>nsw$}",
                f.name.green(),
                format!("{}/{}", f.score, f.max),
                nmw = max_name,
                nsw = max_score,
            );
        }
    }

    if !attention.is_empty() {
        println!();
        for f in &attention {
            println!(
                "  • {:<nmw$} {:>nsw$}",
                f.name.yellow(),
                format!("{}/{}", f.score, f.max),
                nmw = max_name,
                nsw = max_score,
            );
        }
    }
}

// ── Complexity section ─────────────────────────────────────────────────────

fn print_complexity_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Complexity", tw));

    let gw = cmp::min(tw.saturating_sub(24), 20).max(4);
    let gauge_str = color_gauge(pres.complexity_score, 100, gw);
    println!(
        "  {}  {}/{}  {}",
        score_rating_line(pres.complexity_score, &pres.complexity_rating),
        pres.complexity_score,
        100,
        gauge_str,
    );

    let max_name = pres.complexity_factors.iter().map(|f| f.name.len()).max().unwrap_or(14) + 2;
    let max_score = pres
        .complexity_factors
        .iter()
        .map(|f| format!("{}/{}", f.score, f.max).len())
        .max()
        .unwrap_or(4)
        + 1;

    println!();
    for f in &pres.complexity_factors {
        println!(
            "  {:<nmw$} {:>nsw$}",
            f.name,
            format!("{}/{}", f.score, f.max),
            nmw = max_name,
            nsw = max_score,
        );
    }
}

// ── Key Findings section ───────────────────────────────────────────────────

fn print_key_findings_section(pres: &PresentationReport, tw: usize) {
    let mut rows: Vec<(String, String, u8)> = Vec::new();

    for w in &pres.warnings {
        if let Some(rest) = w.strip_prefix("Largest code file represents ") {
            if let Some(pct_str) = rest.split('%').next() {
                if let Ok(pct) = pct_str.trim().parse::<f64>() {
                    rows.push(("Largest file share".into(), format!("{:.0}%", pct), 1));
                    continue;
                }
            }
        }

        if let Some(rest) = w.strip_prefix("Top 3 files represent ") {
            if let Some(pct_str) = rest.split('%').next() {
                if let Ok(pct) = pct_str.trim().parse::<f64>() {
                    let sev = if pct > 50.0 { 2 } else { 1 };
                    rows.push(("Top 3 file share".into(), format!("{:.0}%", pct), sev));
                    continue;
                }
            }
        }

        if w.starts_with("Markdown files represent") {
            rows.push(("Markdown ratio".into(), ">20%".into(), 0));
            continue;
        }

        if let Some(rest) = w.strip_prefix("More than ") {
            if let Some(cnt) = rest.split(' ').next() {
                rows.push(("JSON files".into(), cnt.into(), 0));
                continue;
            }
        }

        if w.starts_with("Project depth exceeds") {
            rows.push(("Depth".into(), ">8".into(), 1));
            continue;
        }

        if w.starts_with("No README.md") {
            rows.push(("README".into(), "absent".into(), 0));
            continue;
        }

        if w.starts_with("No tests/ directory") {
            rows.push(("Tests".into(), "absent".into(), 0));
            continue;
        }

        if let Some(rest) = w.strip_prefix("Skipped ") {
            if let Some(cnt) = rest.split(' ').next() {
                rows.push(("Binary skips".into(), cnt.into(), 0));
                continue;
            }
        }

        if w.contains("exceeds 1M LOC") {
            rows.push(("Repo size".into(), ">1M LOC".into(), 2));
            continue;
        }

        if w.starts_with("Repository exceeds 500K") {
            rows.push(("Repo size".into(), ">500K LOC".into(), 1));
            continue;
        }

        if w.starts_with("More than 500 directories") {
            rows.push(("Directories".into(), ">500".into(), 1));
            continue;
        }

        if w.starts_with("File exceeds 10,000") {
            rows.push(("File size".into(), ">10K LOC".into(), 1));
            continue;
        }

        rows.push(("Observation".into(), w.into(), 0));
    }

    println!("{}", section_header("Key Findings", tw));

    for (label, value, sev) in &rows {
        let display = match sev {
            0 => value.blue().to_string(),
            1 => format!("♦ {}", value).yellow().to_string(),
            2 => format!("♦ {}", value).red().to_string(),
            _ => value.clone(),
        };
        let display_vis = visible_width(&display);
        let dot_w = tw.saturating_sub(2 + label.len() + 1 + display_vis + 1);
        if dot_w > 0 && dot_w <= tw {
            let dots = ".".repeat(dot_w);
            println!("  {} {} {}", label, dots, display);
        } else {
            println!("  {} {}", label, display);
        }
    }
}

// ── Architecture section ───────────────────────────────────────────────────

fn print_architecture_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Architecture", tw));

    let a = &pres.architecture;
    let labels = ["Max Depth", "Avg LOC/File", "Median LOC", "Avg Size"];
    let vals = [
        a.max_depth.to_string(),
        (a.avg_loc_per_file as usize).to_string(),
        (a.median_loc_per_file as usize).to_string(),
        format_bytes(a.avg_file_size),
    ];
    let val_w = vals.iter().map(|s| s.len()).max().unwrap_or(6).max(6);
    let indent = 2usize;

    for (label, val) in labels.iter().zip(vals.iter()) {
        let dot_w = tw.saturating_sub(indent + label.len() + 1 + 1 + val_w);
        let dots = ".".repeat(dot_w);
        println!("  {} {} {:>vw$}", label, dots, val, vw = val_w);
    }
}

// ── Hotspots section ───────────────────────────────────────────────────────

fn print_hotspots_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Hotspots", tw));

    if pres.hotspot_rows.is_empty() {
        println!("  No hotspots detected");
        return;
    }

    let max_pct = pres
        .hotspot_rows
        .iter()
        .map(|s| s.percentage)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(100.0) as usize;

    let pct_w = 4usize;
    let gauge_w = cmp::min(tw.saturating_sub(pct_w + 26), 30).max(8);

    for spot in &pres.hotspot_rows {
        let pct = spot.percentage.round() as u8;
        if pct == 0 {
            continue;
        }
        let path = &spot.path;
        let bar = gauge(spot.percentage.round() as usize, max_pct, gauge_w);
        let used = 2 + pct_w + 1 + gauge_w + 1;
        let path_w = tw.saturating_sub(used);
        let path = shorten_path(path, path_w);
        println!("  {:>pct_w$}% {} {}", pct, bar, path);
    }
}

// ── Largest Files section ──────────────────────────────────────────────────

fn print_files_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Largest Files", tw));

    if pres.top_code_files.is_empty() {
        println!("  No code files detected");
        return;
    }

    let rw = 3usize;
    let loc_strs: Vec<String> = pres.top_code_files.iter()
        .map(|f| format!("{} LOC", format_number(f.lines)))
        .collect();
    let pct_strs: Vec<String> = pres.top_code_files.iter()
        .map(|f| format!("{:.1}%", f.percentage))
        .collect();
    let loc_w = loc_strs.iter().map(|s| s.len()).max().unwrap_or(9).max(9);
    let pct_w = pct_strs.iter().map(|s| s.len()).max().unwrap_or(5).max(5);
    let fixed = 2 + 1 + rw + 1 + 1 + 1; // indent + # + rw + space + space + space
    let name_area = cmp::min(tw.saturating_sub(fixed + loc_w + pct_w), 28).max(10);

    for (i, file) in pres.top_code_files.iter().enumerate() {
        let rank = i + 1;
        let name = if file.name.len() > name_area {
            format!("{}…", &file.name[..name_area.saturating_sub(1)])
        } else {
            file.name.clone()
        };
        println!(
            "  #{:<rw$} {:<na$} {:>loc_w$} {:>pct_w$}",
            rank, name, &loc_strs[i], &pct_strs[i],
            rw = rw,
            na = name_area,
            loc_w = loc_w,
            pct_w = pct_w,
        );
    }
}

// ── Languages section ──────────────────────────────────────────────────────

fn print_languages_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Languages", tw));

    if pres.sorted_languages.is_empty() {
        println!("  No languages detected");
        return;
    }

    let total = pres.file_count;
    let max_show = 8usize;
    let shown: Vec<&(String, usize)> = pres.sorted_languages.iter().take(max_show).collect();
    let remaining = pres.sorted_languages.len().saturating_sub(max_show);
    let other_label = if remaining > 0 {
        format!("Other ({})", remaining)
    } else {
        String::new()
    };

    let name_w = cmp::min(
        shown.iter()
            .map(|(n, _)| n.len())
            .chain(std::iter::once(other_label.len()))
            .max()
            .unwrap_or(0) + 2,
        20,
    ).max(8);

    let count_w = shown.iter()
        .map(|(_, c)| format_number(*c).len())
        .chain(
            pres.sorted_languages.iter()
                .skip(max_show)
                .map(|(_, c)| format_number(*c).len()),
        )
        .max()
        .unwrap_or(3)
        .max(3);

    for (name, count) in &shown {
        let pct = *count as f64 / total as f64 * 100.0;
        let noun = if *count == 1 { "file" } else { "files" };
        println!(
            "  {:<nw$} {:>cw$} {}  {:>5.1}%",
            name, format_number(*count), noun, pct,
            nw = name_w,
            cw = count_w,
        );
    }

    if remaining > 0 {
        let other_count: usize = pres.sorted_languages
            .iter()
            .skip(max_show)
            .map(|(_, c)| c)
            .sum();
        let other_pct = other_count as f64 / total as f64 * 100.0;
        let noun = if other_count == 1 { "file" } else { "files" };
        println!(
            "  {:<nw$} {:>cw$} {}  {:>5.1}%",
            other_label,
            format_number(other_count),
            noun,
            other_pct,
            nw = name_w,
            cw = count_w,
        );
    }
}

// ── Dependencies section ───────────────────────────────────────────────────

fn print_dependencies_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Dependencies", tw));

    let total = pres.dependencies.len();
    let noun = if total == 1 { "crate" } else { "crates" };
    let max_show = cmp::max(tw / 18, 3).min(6);

    let listed: Vec<&str> = pres.dependencies.iter().take(max_show).map(|s| s.as_str()).collect();
    let remaining = total.saturating_sub(max_show);

    let mut parts = String::from(listed[0]);
    for dep in &listed[1..] {
        parts.push_str(", ");
        parts.push_str(dep);
    }
    if remaining > 0 {
        parts.push_str(&format!(", +{} more", remaining));
    }

    println!("  {} {}: {}", total, noun, parts);
}

// ── Code section ───────────────────────────────────────────────────────────

fn print_code_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Code", tw));

    let cm = &pres.code_metrics;
    let labels = ["Code", "Config", "Docs"];
    let files = [
        format_number(cm.code_files),
        format_number(cm.config_files),
        format_number(cm.docs_files),
    ];
    let locs = [
        format_number(cm.code_lines),
        format_number(cm.config_lines),
        format_number(cm.docs_lines),
    ];
    let label_w = labels.iter().map(|s| s.len()).max().unwrap_or(6) + 1;
    let file_w = files.iter().map(|s| s.len()).max().unwrap_or(3).max(3);
    let loc_w = locs.iter().map(|s| s.len()).max().unwrap_or(5).max(5);

    for (i, label) in labels.iter().enumerate() {
        println!(
            "  {:<lw$} {:>fw$} files  •  {:>lw2$} LOC",
            label, &files[i], &locs[i],
            lw = label_w,
            fw = file_w,
            lw2 = loc_w,
        );
    }
}

// ── Verbose-only sections (appendix) ───────────────────────────────────────

fn print_verbose_depth_map_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Depth Map", tw));
    for (depth, count) in &pres.depth_map {
        let noun = if *count == 1 { "file" } else { "files" };
        println!("  Level {}:  {} {}", depth, count, noun);
    }
}

fn print_verbose_size_dist_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Size Distribution", tw));
    let sd = &pres.size_distribution;
    println!("  Noise  (<10 LOC):   {}", sd.noise);
    println!("  Small  (10–100):    {}", sd.small);
    println!("  Medium (100–500):   {}", sd.medium);
    println!("  Large  (≥500):      {}", sd.large);
}

fn print_verbose_lang_pct_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Languages (%)", tw));
    if pres.sorted_languages.is_empty() {
        println!("  No languages detected");
        return;
    }
    let total = pres.file_count;
    let name_w = cmp::min(
        pres.sorted_languages.iter().map(|(n, _)| n.len()).max().unwrap_or(0) + 2,
        20,
    ).max(8);

    for (name, count) in &pres.sorted_languages {
        let pct = *count as f64 / total as f64 * 100.0;
        let noun = if *count == 1 { "file" } else { "files" };
        println!("  {:<nw$} {:>4} {}  {:>5.1}%", name, count, noun, pct, nw = name_w);
    }
}

fn print_verbose_file_types_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("File Types", tw));
    let cm = &pres.code_metrics;
    let other = pres.file_count.saturating_sub(cm.code_files + cm.config_files + cm.docs_files);
    let labels = ["Code", "Config", "Docs", "Other"];
    let files = [
        format_number(cm.code_files),
        format_number(cm.config_files),
        format_number(cm.docs_files),
        format_number(other),
    ];
    let locs = [
        format_number(cm.code_lines),
        format_number(cm.config_lines),
        format_number(cm.docs_lines),
        String::from("—"),
    ];
    let label_w = labels.iter().map(|s| s.len()).max().unwrap_or(6) + 1;
    let file_w = files.iter().map(|s| s.len()).max().unwrap_or(3).max(3);
    let loc_w = locs.iter().map(|s| s.len()).max().unwrap_or(5).max(5);

    for (i, label) in labels.iter().enumerate() {
        println!(
            "  {:<lw$} {:>fw$} files  {:>lw2$} LOC",
            label, &files[i], &locs[i],
            lw = label_w,
            fw = file_w,
            lw2 = loc_w,
        );
    }
}

fn print_verbose_deep_dirs_section(pres: &PresentationReport, tw: usize) {
    println!("{}", section_header("Deep Directories", tw));
    if pres.directory_rows.is_empty() {
        println!("  No directories detected");
        return;
    }

    let mut with_depth: Vec<(usize, &crate::output::presentation::DirectoryRow)> = pres
        .directory_rows
        .iter()
        .map(|d| (d.path.matches('/').count(), d))
        .collect();
    with_depth.sort_by(|(da, a), (db, b)| db.cmp(da).then_with(|| b.loc.cmp(&a.loc)));

    let rows: Vec<&(usize, &crate::output::presentation::DirectoryRow)> = with_depth.iter().take(10).collect();
    let depth_strs: Vec<String> = rows.iter().map(|(d, _)| d.to_string()).collect();
    let loc_strs: Vec<String> = rows.iter().map(|(_, r)| format_number(r.loc)).collect();
    let file_strs: Vec<String> = rows.iter().map(|(_, r)| r.files.to_string()).collect();
    let depth_w = depth_strs.iter().map(|s| s.len()).max().unwrap_or(1);
    let loc_w = loc_strs.iter().map(|s| s.len()).max().unwrap_or(5);
    let file_w = file_strs.iter().map(|s| s.len()).max().unwrap_or(3);
    let suffix_w = 23 + depth_w + loc_w + file_w;
    let path_w = cmp::max(tw.saturating_sub(suffix_w + 4), 10);

    for (i, (_depth, row)) in rows.iter().enumerate() {
        let path = shorten_path(&row.path, path_w);
        println!("  {:<path_w$}  depth {}  {} LOC · {} files", path, depth_strs[i], loc_strs[i], file_strs[i]);
    }
}

fn print_verbose_top_files_section(report: &crate::models::AnalysisReport, tw: usize) {
    use crate::analysis::classification::classify_extension;

    println!("{}", section_header("Top Files", tw));

    let mut code: Vec<&crate::models::FileEntry> = report
        .files
        .entries
        .iter()
        .filter(|f| classify_extension(&f.extension) == "code")
        .collect();
    if code.is_empty() {
        println!("  No code files detected");
        return;
    }
    code.sort_by(|a, b| b.line_count.cmp(&a.line_count));

    let total_code: usize = code.iter().map(|f| f.line_count).sum();
    let rw = 3usize;
    let top10: Vec<&&crate::models::FileEntry> = code.iter().take(10).collect();
    let loc_strs: Vec<String> = top10.iter().map(|f| format!("{} LOC", format_number(f.line_count))).collect();
    let pct_strs: Vec<String> = top10.iter().map(|f| {
        let pct = if total_code > 0 { f.line_count as f64 / total_code as f64 * 100.0 } else { 0.0 };
        format!("{:.1}%", pct)
    }).collect();
    let loc_w = loc_strs.iter().map(|s| s.len()).max().unwrap_or(9).max(9);
    let pct_w = pct_strs.iter().map(|s| s.len()).max().unwrap_or(5).max(5);
    let fixed = 2 + 1 + rw + 1 + 1 + 1;
    let name_w = cmp::min(tw.saturating_sub(fixed + loc_w + pct_w), 30).max(10);

    for (i, file) in top10.iter().enumerate() {
        let rank = i + 1;
        let name = std::path::Path::new(&file.path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| file.path.clone());
        let name = if name.len() > name_w {
            format!("{}…", &name[..name_w.saturating_sub(1)])
        } else {
            name
        };
        println!(
            "  #{:<rw$} {:<name_w$} {:>loc_w$} {:>pct_w$}",
            rank, name, &loc_strs[i], &pct_strs[i],
        );
    }
}

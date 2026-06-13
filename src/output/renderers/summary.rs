// Terminal summary formatter — compact, dense, btop-inspired card layout.
// All content lines go through card_content_line() for consistent width.

use std::cmp;

use owo_colors::OwoColorize;
use unicode_width::UnicodeWidthChar;

use crate::output::presentation::PresentationReport;
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
        render_card(&project_card(&pres, tw));

        let hw = 30usize;
        let cw = 30usize;
        if hw + cw + 4 + 4 + 3 <= tw {
            render_side_by_side(&[health_card(&pres, hw), complexity_card(&pres, cw)], 3);
        } else {
            render_card(&health_card(&pres, tw.saturating_sub(4)));
            render_card(&complexity_card(&pres, tw.saturating_sub(4)));
        }

        render_card(&hotspots_card(&pres, tw.saturating_sub(4)));

        let aw = 22usize;
        let dw = 36usize;
        if aw + dw + 4 + 4 + 3 <= tw {
            render_side_by_side(&[architecture_card(&pres, aw), directories_card(&pres, dw)], 3);
        } else {
            render_card(&architecture_card(&pres, tw.saturating_sub(4)));
            render_card(&directories_card(&pres, tw.saturating_sub(4)));
        }

        render_card(&languages_card(&pres, tw.saturating_sub(4)));

        let depw = 30usize;
        let codew = 30usize;
        if depw + codew + 4 + 4 + 3 <= tw {
            render_side_by_side(&[dependencies_card(&pres, depw), code_card(&pres, codew)], 3);
        } else {
            if !pres.dependencies.is_empty() {
                render_card(&dependencies_card(&pres, tw.saturating_sub(4)));
            }
            render_card(&code_card(&pres, tw.saturating_sub(4)));
        }

        render_card(&files_card(&pres, tw.saturating_sub(4)));

        if !pres.warnings.is_empty() {
            render_card(&warnings_card(&pres, tw.saturating_sub(4)));
        }

        if self.verbose {
            render_card(&depth_map_card(&pres, tw.saturating_sub(4)));
            render_card(&size_distribution_card(&pres, tw.saturating_sub(4)));
        }

        println!(" {} {}", "Completed in".dimmed(), pres.duration_display.dimmed());
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

// ── Card rendering ────────────────────────────────────────────────────────

struct Card {
    title: String,
    inner: usize,
    lines: Vec<String>,
}

fn card_border_top(title: &str, inner: usize) -> String {
    // Total visual width must equal inner + 4 (matching card_line's │  │)
    // ╭─ (3) + title + space + dashes + ╮ (1) = inner + 4
    // dashes = inner + 4 - 5 - title.len() = inner - 1 - title.len()
    let dashes = inner.saturating_sub(title.len() + 1);
    format!(
        "╭─ {} {}╮",
        title.bold().cyan(),
        "─".repeat(dashes)
    )
}

fn card_border_bottom(inner: usize) -> String {
    format!("╰{}╯", "─".repeat(inner + 2))
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

fn card_line(content: &str, inner: usize) -> String {
    let pad = inner.saturating_sub(visible_width(content));
    format!("│ {}{:pad$} │", content, "", pad = pad)
}

fn card_empty(inner: usize) -> String {
    format!("│ {:inner$} │", "", inner = inner)
}

fn render_card(card: &Card) {
    println!("{}", card_border_top(&card.title, card.inner));
    for line in &card.lines {
        println!("{}", card_line(line, card.inner));
    }
    println!("{}", card_border_bottom(card.inner));
}

fn render_side_by_side(cards: &[Card], gap: usize) {
    let card_inner: Vec<usize> = cards.iter().map(|c| c.inner).collect();

    let mut frames: Vec<Vec<String>> = cards
        .iter()
        .map(|c| {
            let mut lines = vec![card_border_top(&c.title, c.inner)];
            for line in &c.lines {
                lines.push(card_line(line, c.inner));
            }
            lines
        })
        .collect();

    let max_content = frames.iter().map(|f| f.len()).max().unwrap_or(0);
    for (ci, frame) in frames.iter_mut().enumerate() {
        while frame.len() < max_content {
            frame.push(card_empty(card_inner[ci]));
        }
    }

    for (ci, frame) in frames.iter_mut().enumerate() {
        frame.push(card_border_bottom(card_inner[ci]));
    }

    for i in 0..frames[0].len() {
        let mut row = String::new();
        for (ci, frame) in frames.iter().enumerate() {
            if ci > 0 {
                for _ in 0..gap {
                    row.push(' ');
                }
            }
            row.push_str(&frame[i]);
        }
        println!("{}", row);
    }
}

// ── Color helpers ─────────────────────────────────────────────────────────

fn colored_score_line(score: u8, rating: &str, inner: usize) -> String {
    let raw = format!("{}/100  {}", score, rating);
    let padded = format!("{:<width$}", raw, width = inner);
    if score >= 80 {
        padded.green().to_string()
    } else if score >= 40 {
        padded.yellow().to_string()
    } else {
        padded.red().to_string()
    }
}

fn colored_gauge_line(score: u8, max: u8, inner: usize) -> String {
    let gw = cmp::min(inner, 20);
    let g = gauge(score as usize, max as usize, gw);
    format!("{:<inner$}", g, inner = inner)
}

// ── Card factories ────────────────────────────────────────────────────────

fn project_card(pres: &PresentationReport, term_width: usize) -> Card {
    let inner = term_width.saturating_sub(4).min(76);
    let proj = pres.project_type_label.as_deref().unwrap_or("Project");
    let ep = pres.entry_point_label.as_deref().unwrap_or("");

    let line1 = if ep.is_empty() {
        format!("{}  Project", proj.bold())
    } else {
        format!("{}  Project  •  {}", proj.bold(), ep)
    };

    let line2 = format!(
        "{} files  •  {} dirs  •  {} LOC  •  {}",
        format_number(pres.file_count),
        pres.directory_count,
        pres.total_lines_display,
        pres.duration_display,
    );

    Card {
        title: String::from("corrode"),
        inner,
        lines: vec![truncate_to(line1, inner), truncate_to(line2, inner)],
    }
}

fn truncate_to(s: String, max: usize) -> String {
    if s.len() > max {
        let mut t = s;
        t.truncate(max.saturating_sub(1));
        t.push('…');
        t
    } else {
        s
    }
}

fn health_card(pres: &PresentationReport, inner: usize) -> Card {
    let factor_gw = cmp::min(inner.saturating_sub(24), 20).max(4);
    let mut lines = vec![
        colored_score_line(pres.health_score, &pres.health_rating, inner),
        colored_gauge_line(pres.health_score, 100, inner),
    ];
    for f in &pres.health_factors {
        lines.push(format!(
            "{:<14} {} {:>2}/{}",
            f.name,
            gauge(f.score as usize, f.max as usize, factor_gw),
            f.score,
            f.max,
        ));
    }
    Card { title: String::from("Health"), inner, lines }
}

fn complexity_card(pres: &PresentationReport, inner: usize) -> Card {
    let factor_gw = cmp::min(inner.saturating_sub(24), 20).max(4);
    let mut lines = vec![
        colored_score_line(pres.complexity_score, &pres.complexity_rating, inner),
        colored_gauge_line(pres.complexity_score, 100, inner),
    ];
    for f in &pres.complexity_factors {
        lines.push(format!(
            "{:<14} {} {:>2}/{}",
            f.name,
            gauge(f.score as usize, f.max as usize, factor_gw),
            f.score,
            f.max,
        ));
    }
    Card { title: String::from("Complexity"), inner, lines }
}

fn hotspots_card(pres: &PresentationReport, inner: usize) -> Card {
    let label_w = 18usize;
    let pct_w = 5usize;
    let gw = cmp::min(inner.saturating_sub(label_w + pct_w + 2), 60).max(4);
    let mut lines = Vec::new();
    for spot in &pres.hotspot_rows {
        let label = if spot.path.len() > label_w {
            format!("{}…", &spot.path[..label_w.saturating_sub(1)])
        } else {
            format!("{:<label_w$}", spot.path, label_w = label_w)
        };
        let pct = spot.percentage.round() as u8;
        lines.push(format!(
            "{}{}{:>pct_w$}",
            label,
            gauge(pct as usize, 100, gw),
            format!("{}%", pct),
            pct_w = pct_w,
        ));
    }
    Card { title: String::from("Hotspots"), inner, lines }
}

fn architecture_card(pres: &PresentationReport, inner: usize) -> Card {
    let a = &pres.architecture;
    Card {
        title: String::from("Architecture"),
        inner,
        lines: vec![
            format!("{:<14} {}", "Max Depth", a.max_depth),
            format!("{:<14} {}", "Avg LOC/File", (a.avg_loc_per_file as usize).to_string()),
            format!("{:<14} {}", "Median LOC", (a.median_loc_per_file as usize).to_string()),
            format!("{:<14} {}", "Avg Size", format_bytes(a.avg_file_size)),
        ],
    }
}

fn directories_card(pres: &PresentationReport, inner: usize) -> Card {
    let mut lines = Vec::new();
    for row in &pres.directory_rows {
        let raw = format!(
            "{} {} LOC · {}",
            row.path,
            format_number(row.loc),
            row.files,
        );
        lines.push(truncate_to(raw, inner));
    }
    Card { title: String::from("Directories"), inner, lines }
}

fn languages_card(pres: &PresentationReport, inner: usize) -> Card {
    let max_count = pres
        .sorted_languages
        .iter()
        .map(|(_, c)| *c)
        .max()
        .unwrap_or(0);

    let name_w = 10usize;
    let count_w = 4usize;
    let gw = cmp::min(inner.saturating_sub(name_w + count_w + 3), 60).max(4);
    let mut lines = Vec::new();
    for (name, count) in &pres.sorted_languages {
        lines.push(format!(
            "{:<name_w$} {} {:>count_w$}",
            name,
            gauge(*count, max_count, gw),
            count,
            name_w = name_w,
            count_w = count_w,
        ));
    }
    Card { title: String::from("Languages"), inner, lines }
}

fn dependencies_card(pres: &PresentationReport, inner: usize) -> Card {
    let deps = &pres.dependencies;
    let total = deps.len();
    let per_dep = 14usize;
    let max_show = cmp::max(inner / per_dep, 1);
    let show = cmp::min(total, max_show);

    let mut line = String::new();
    for dep in deps.iter().take(show) {
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(dep);
    }

    let remaining = total.saturating_sub(show);
    if remaining > 0 {
        line.push_str(&format!(" +{} more", remaining));
    }

    Card { title: String::from("Dependencies"), inner, lines: vec![line] }
}

fn code_card(pres: &PresentationReport, inner: usize) -> Card {
    let cm = &pres.code_metrics;
    Card {
        title: String::from("Code"),
        inner,
        lines: vec![
            format!("{:<6}{:>3} files  ·  {:>5} LOC", "Code", format_number(cm.code_files), format_number(cm.code_lines)),
            format!("{:<6}{:>3} files  ·  {:>5} LOC", "Config", format_number(cm.config_files), format_number(cm.config_lines)),
            format!("{:<6}{:>3} files  ·  {:>5} LOC", "Docs", format_number(cm.docs_files), format_number(cm.docs_lines)),
        ],
    }
}

fn files_card(pres: &PresentationReport, inner: usize) -> Card {
    // Table: File | LOC | Size
    let loc_w = 7usize;
    let size_w = 10usize;
    let sep_w = 2usize;
    let file_w = inner.saturating_sub(loc_w + size_w + sep_w);

    let mut lines = Vec::new();

    // Header
    let hdr = format!("{:<file_w$}{:>loc_w$} {:>size_w$}", "File", "LOC", "Size");
    lines.push(hdr);
    let ruler_w = cmp::min(file_w + loc_w + size_w + 1, inner);
    lines.push("─".repeat(ruler_w));

    for file in &pres.top_code_files {
        let name = if file.name.len() > file_w {
            format!("{}…", &file.name[..file_w.saturating_sub(1)])
        } else {
            file.name.clone()
        };
        let loc_str = format_number(file.lines);
        let size_str = format_bytes(file.bytes);
        lines.push(format!(
            "{:<file_w$}{:>loc_w$} {:>size_w$}",
            name, loc_str, size_str,
        ));
    }

    Card { title: String::from("Largest Files"), inner, lines }
}

fn warnings_card(pres: &PresentationReport, inner: usize) -> Card {
    let mut lines = Vec::new();
    for w in &pres.warnings {
        let raw = format!("⚠ {}", w);
        let line = if raw.len() > inner {
            format!("{}…", &raw[..inner.saturating_sub(1)])
        } else {
            raw
        };
        lines.push(line.yellow().to_string());
    }
    Card { title: String::from("Warnings"), inner, lines }
}

fn depth_map_card(pres: &PresentationReport, inner: usize) -> Card {
    let mut lines = Vec::new();
    for (depth, count) in &pres.depth_map {
        let label = if *count == 1 { "file" } else { "files" };
        lines.push(format!("Level {}:  {} {}", depth, count, label));
    }
    Card { title: String::from("Depth Map"), inner, lines }
}

fn size_distribution_card(pres: &PresentationReport, inner: usize) -> Card {
    let sd = &pres.size_distribution;
    Card {
        title: String::from("Size Distribution"),
        inner,
        lines: vec![
            format!("Noise  (<10 LOC):   {}", sd.noise),
            format!("Small  (10–100):    {}", sd.small),
            format!("Medium (100–500):   {}", sd.medium),
            format!("Large  (≥500):      {}", sd.large),
        ],
    }
}

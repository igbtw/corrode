// Markdown output formatter.
// Consumes PresentationReport — no AnalysisReport traversal.

use std::fmt::Write;

use crate::models::AnalysisReport;
use crate::output::presentation::{section, PresentationReport};
use crate::output::reporter::ReportRenderer;
use crate::utils::formatting::{format_bytes, format_number};

pub struct MarkdownReporter;

impl MarkdownReporter {
    pub fn render_to_string(&self, report: &AnalysisReport) -> String {
        let pres: PresentationReport = report.into();

        let mut md = String::new();

        writeln!(md, "# Codebase Analysis Report\n").unwrap();

        // ── Project ──────────────────────────────────────────────────
        writeln!(md, "## Project\n").unwrap();
        if let Some(ref label) = pres.project_type_label {
            writeln!(md, "- **Type:** {}", label).unwrap();
        }
        if let Some(ref ep) = pres.entry_point_label {
            writeln!(md, "- **Entry Point:** {}", ep).unwrap();
        }
        writeln!(md, "- **Files:** {}", pres.file_count).unwrap();
        writeln!(md, "- **Directories:** {}", pres.directory_count).unwrap();
        writeln!(md, "- **Total LOC:** {}", pres.total_lines_display).unwrap();
        writeln!(md, "- **Completed in:** {}", pres.duration_display).unwrap();
        writeln!(md).unwrap();

        // ── Dependencies ─────────────────────────────────────────────
        if !pres.dependencies.is_empty() {
            writeln!(md, "## {}", section::DEPENDENCIES).unwrap();
            for dep in &pres.dependencies {
                writeln!(md, "- {}", dep).unwrap();
            }
            writeln!(
                md,
                "\n*({} dependencies)*\n",
                pres.dependencies.len()
            )
            .unwrap();
        }

        // ── Largest Directories ──────────────────────────────────────
        if !pres.directory_rows.is_empty() {
            writeln!(md, "## {}", section::LARGEST_DIRECTORIES).unwrap();
            writeln!(md, "| Directory | LOC | Files |").unwrap();
            writeln!(md, "|-----------|-----|-------|").unwrap();
            for row in &pres.directory_rows {
                writeln!(
                    md,
                    "| {} | {} | {} |",
                    row.path,
                    format_number(row.loc),
                    row.files,
                )
                .unwrap();
            }
            writeln!(md).unwrap();
        }

        // ── Code Metrics ─────────────────────────────────────────────
        let cm = &pres.code_metrics;
        writeln!(md, "## {}", section::CODE_METRICS).unwrap();
        writeln!(md, "| Category | Files | LOC |").unwrap();
        writeln!(md, "|----------|-------|-----|").unwrap();
        writeln!(
            md,
            "| Code | {} | {} |",
            format_number(cm.code_files),
            format_number(cm.code_lines),
        )
        .unwrap();
        writeln!(
            md,
            "| Config | {} | {} |",
            format_number(cm.config_files),
            format_number(cm.config_lines),
        )
        .unwrap();
        writeln!(
            md,
            "| Docs | {} | {} |\n",
            format_number(cm.docs_files),
            format_number(cm.docs_lines),
        )
        .unwrap();

        // ── Architecture ─────────────────────────────────────────────
        let a = &pres.architecture;
        writeln!(md, "## {}", section::ARCHITECTURE).unwrap();
        writeln!(md, "| Metric | Value |").unwrap();
        writeln!(md, "|--------|-------|").unwrap();
        writeln!(md, "| Max Depth | {} |", a.max_depth).unwrap();
        writeln!(md, "| Avg LOC/File | {:.0} |", a.avg_loc_per_file).unwrap();
        writeln!(md, "| Median LOC/File | {:.0} |", a.median_loc_per_file).unwrap();
        writeln!(
            md,
            "| Avg File Size | {} |\n",
            format_bytes(a.avg_file_size)
        )
        .unwrap();

        // ── Hotspots ─────────────────────────────────────────────────
        if !pres.hotspot_rows.is_empty() {
            writeln!(md, "## {}", section::HOTSPOTS).unwrap();
            writeln!(md, "| Directory | Share |").unwrap();
            writeln!(md, "|-----------|-------|").unwrap();
            for spot in &pres.hotspot_rows {
                writeln!(md, "| {} | {:.0}% |", spot.path, spot.percentage).unwrap();
            }
            writeln!(md).unwrap();
        }

        // ── Complexity ───────────────────────────────────────────────
        writeln!(md, "## {}", section::COMPLEXITY).unwrap();
        writeln!(
            md,
            "**Score:** {}/100 — **{}**\n",
            pres.complexity_score, pres.complexity_rating,
        )
        .unwrap();
        writeln!(md, "| Factor | Score |").unwrap();
        writeln!(md, "|--------|-------|").unwrap();
        for f in &pres.complexity_factors {
            writeln!(md, "| {} | {}/{} |", f.name, f.score, f.max).unwrap();
        }
        writeln!(md).unwrap();

        // ── Health ───────────────────────────────────────────────────
        writeln!(md, "## {}", section::HEALTH).unwrap();
        writeln!(
            md,
            "**Score:** {}/100 — **{}**\n",
            pres.health_score, pres.health_rating,
        )
        .unwrap();
        writeln!(md, "| Factor | Score |").unwrap();
        writeln!(md, "|--------|-------|").unwrap();
        for f in &pres.health_factors {
            writeln!(md, "| {} | {}/{} |", f.name, f.score, f.max).unwrap();
        }
        writeln!(md).unwrap();

        // ── Warnings ─────────────────────────────────────────────────
        if !pres.warnings.is_empty() {
            writeln!(md, "## {}", section::WARNINGS).unwrap();
            for w in &pres.warnings {
                writeln!(md, "- {}", w).unwrap();
            }
            writeln!(md).unwrap();
        }

        // ── Languages ────────────────────────────────────────────────
        if !pres.sorted_languages.is_empty() {
            writeln!(md, "## {}", section::LANGUAGES).unwrap();
            writeln!(md, "| Language | Files |").unwrap();
            writeln!(md, "|----------|-------|").unwrap();
            for (name, count) in &pres.sorted_languages {
                writeln!(md, "| {} | {} |", name, count).unwrap();
            }
            writeln!(md).unwrap();
        }

        // ── Largest Code Files ───────────────────────────────────────
        writeln!(md, "## {}", section::LARGEST_CODE_FILES).unwrap();
        writeln!(md, "| File | Lines | Size |").unwrap();
        writeln!(md, "|------|-------|------|").unwrap();
        for file in &pres.top_code_files {
            writeln!(
                md,
                "| {} | {} | {} |",
                file.name,
                format_number(file.lines),
                format_bytes(file.bytes),
            )
            .unwrap();
        }
        writeln!(md).unwrap();

        md
    }
}

impl ReportRenderer for MarkdownReporter {
    fn render(&self, report: &AnalysisReport) {
        print!("{}", self.render_to_string(report));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::report::test_helpers::sample_report;

    #[test]
    fn markdown_contains_main_header() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.starts_with("# Codebase Analysis Report"));
    }

    #[test]
    fn markdown_contains_project_section() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("## Project"));
        assert!(md.contains("Rust"));
        assert!(md.contains("src/main.rs"));
    }

    #[test]
    fn markdown_contains_dependencies_section() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("## Dependencies"));
        assert!(md.contains("clap"));
        assert!(md.contains("serde"));
    }

    #[test]
    fn markdown_contains_architecture_table() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("## Architecture"));
        assert!(md.contains("| Max Depth | 2 |"));
    }

    #[test]
    fn markdown_contains_complexity() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("## Complexity"));
        assert!(md.contains("10/100"));
        assert!(md.contains("Low"));
    }

    #[test]
    fn markdown_contains_warnings() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("## Warnings"));
        assert!(md.contains("No tests/ directory detected"));
    }

    #[test]
    fn markdown_contains_languages() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("## Languages"));
        assert!(md.contains("Rust"));
    }

    #[test]
    fn markdown_contains_largest_code_files() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("## Largest Code Files"));
    }

    #[test]
    fn markdown_keys_match_json_data() {
        let report = sample_report();
        let md = MarkdownReporter.render_to_string(&report);
        assert!(md.contains("Rust"));
        assert!(md.contains("src/main.rs"));
        assert!(md.contains("Low"));
    }
}

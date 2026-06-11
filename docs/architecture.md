# Architecture

corrode uses a linear pipeline: **scan → analyse → render**.

## Pipeline

```
                ┌──────────────┐
                │  CLI (clap)  │
                │  flags.rs    │
                │  commands.rs │
                └──────┬───────┘
                       │ ParseArgs
                       ▼
                ┌──────────────────────────┐
                │        main.rs           │
                │        dispatch          │
                ├──────────────────────────┤
                │ --tree → print_tree()    │
                │ default → analyse() +    │
                │    + render(report)      │
                └──────┬───────────────────┘
                       │ Analyse { path, verbose, json, markdown }
                       ▼
          ┌────────────────────────┐
          │  filesystem/           │
          │  scanner.rs → scan()   │
          │  filters.rs → skip     │
          │  predicates            │
          └───────────┬────────────┘
                      │ Vec<FileEntry>
                      ▼
          ┌────────────────────────┐
          │  analysis/report.rs    │
          │  analyse()             │
          ├────────────────────────┤
          │  calls each module     │
          │  and assembles         │
          │  AnalysisReport        │
          └───────────┬────────────┘
                      │ AnalysisReport
                      ▼
          ┌────────────────────────┐
          │  output/               │
          │  *Reporter::render()   │
          ├────────────────────────┤
          │  summary.rs (terminal) │
          │  json.rs               │
          │  markdown.rs           │
          └───────────┬────────────┘
                      │ stdout
                      ▼
                  Terminal / Pipe
```

## Module tree

```
src/
├── main.rs                  Entry point, dispatch
├── cli/                     Argument parsing
│   ├── mod.rs
│   ├── flags.rs             Cli struct + parse_args
│   └── commands.rs          Command enum (Analyse)
├── filesystem/              File-tree scanning
│   ├── mod.rs
│   ├── scanner.rs           walkdir-based recursive scan
│   └── filters.rs           SKIP_DIRS, SKIP_EXTENSIONS, is_minified
├── analysis/                Metrics + pipeline
│   ├── mod.rs               Module declarations, re-exports analyse()
│   ├── report.rs            Orchestrator: calls all sub-modules
│   ├── project.rs           Project-type + entry-point detection
│   ├── dependencies.rs      Cargo.toml dependency extraction
│   ├── classification.rs    Extension classification (code/config/docs)
│   ├── metrics.rs           Aggregation: language map, depth map,
│   │                        size distribution, code metrics, top files
│   ├── architecture.rs      Max depth, avg/median LOC/file, avg size
│   ├── complexity.rs        Score (0–100) + rating
│   ├── hotspots.rs          LOC share by directory
│   ├── warnings.rs          Health checks
│   └── ast/                 Placeholder for future AST parsing
│       ├── mod.rs
│       └── rust.rs
├── models/                  Data types
│   ├── mod.rs               ProjectType enum + re-exports
│   └── report.rs            AnalysisReport + 5 sub-reports + Serialize
├── output/                  Renderers
│   ├── mod.rs               Module declarations + re-exports
│   ├── reporter.rs          ReportRenderer trait
│   ├── summary.rs           Terminal output (default)
│   ├── tree.rs              Directory-tree visualisation
│   ├── json.rs              JSON output [--json]
│   └── markdown.rs          Markdown output [--markdown]
└── utils/                   Shared helpers
    ├── mod.rs
    └── formatting.rs        format_bytes, format_number, format_duration, strip_root
```

## Key design decisions

### ReportRenderer trait

All output formatters implement `ReportRenderer`:

```rust
pub trait ReportRenderer {
    fn render(&self, report: &AnalysisReport);
}
```

This allows adding new output formats (HTML, SARIF, etc.) by creating a new module in `output/` with a struct that implements the trait, plus a CLI flag in `commands.rs`.

### AnalysisReport sub-reports

The monolithic report was split into five sub-reports for clearer ownership:

| Sub-report           | Fields                                                                                             | Owner                                                                     |
| -------------------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `ProjectInfo`        | project_type, entry_point, duration                                                                | `analysis/project.rs`                                                     |
| `FileReport`         | entries, directory_count, total_lines, directory_stats, depth_map, size_distribution, language_map | `filesystem/scanner.rs` + `analysis/metrics.rs`                           |
| `DependencyReport`   | list                                                                                               | `analysis/dependencies.rs`                                                |
| `ArchitectureReport` | code_metrics, metrics, hotspots                                                                    | `analysis/metrics.rs`, `analysis/architecture.rs`, `analysis/hotspots.rs` |
| `QualityReport`      | complexity, warnings                                                                               | `analysis/complexity.rs`, `analysis/warnings.rs`                          |

### Data flow

1. `scan()` returns `Vec<FileEntry>` with path, extension, line_count, size_bytes, contents
2. `analyse()` passes entries through each sub-module
3. Each sub-module returns its portion of the report
4. `analyse()` assembles the full `AnalysisReport`
5. The chosen renderer serialises the report to the desired output format

### Adding a new metric

1. Create a new module under `analysis/` (e.g. `analysis/coupling.rs`)
2. Define the function signature: `pub fn compute_*(files: &[FileEntry], ...) -> ...`
3. Add a new struct or field to the appropriate sub-report in `models/report.rs`
4. Call the function from `analysis/report.rs::analyse()`
5. Update `output/summary.rs` and `output/markdown.rs` to display the metric

### Adding a new output format

1. Create a new module in `output/` (e.g. `output/html.rs`)
2. Define a struct that implements `ReportRenderer`
3. Add a CLI flag in `cli/commands.rs`
4. Add the flag to the mutually-exclusive set in `cli/flags.rs`
5. Wire the flag in `main.rs`

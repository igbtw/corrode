# corrode

> Understand unfamiliar codebases before they rot.

corrode scans a project directory and produces a structured report in milliseconds: project type, entry point, dependencies, architecture metrics, hotspots, complexity score, and warning flags. It answers the questions every developer asks when opening a repository for the first time.

## Why corrode?

Codebases corrode as they age:

* **Layers accumulate.** Quick fixes become permanent. The original design blurs.
* **Knowledge leaks.** The author moves on. Documentation goes stale.
* **Structure degrades.** Nesting deepens. Nobody knows where the entry point is.

corrode gives you a clear, structured picture of a repository before you start making changes. It is the first tool you run when you clone a project, the report you attach to a PR to show architectural changes, and the CI step that catches complexity creep.

## Quick example

```text
$ corrode analyse .

✓ Scanning project...

Detected      Rust Project
Entry Point   src/main.rs
Project      35 files · 8 dirs · 2,591 LOC

Dependencies
  clap          indicatif         walkdir         serde          +1 more

Largest Directories    Code Metrics
  src/output/    679 LOC · 6 files    Code Files     29  ·  1,913 LOC
  (root)         678 LOC · 6 files    Config Files    2  ·  473 LOC
  src/analysis/  677 LOC · 10 files   Docs Files      2  ·  183 LOC

Architecture                  Hotspots
  Max Depth          3          src/output/      35%
  Avg LOC/File      61          src/analysis/    35%
  Median LOC/File   37          src/models/      14%
  Avg File Size   1.9 KB

Complexity                    Warnings
  Score  29/100                 • Largest code file represents 14% of code LOC
  Rating Moderate               • No tests/ directory detected

Languages                     Largest Code Files
  Rust         29 files          markdown.rs   270 lines · 10.6 KB
  Markdown      1 file           summary.rs    241 lines ·  6.9 KB
  TOML          1 file           report.rs     232 lines ·  6.7 KB

Completed in 0.653 ms
```

## Features

* Project type and entry point detection (Rust, Node, Go, Python, Ruby)
* Dependency extraction from Cargo.toml
* Top-5 directories by LOC with file count
* Code / config / docs classification with per-category LOC
* Architecture metrics: max depth, average and median LOC/file, average file size
* Hotspot analysis: top-level directory LOC share
* Structural complexity score (0–100) with human-readable rating — measures
  repository size and nesting, not cyclomatic complexity
* Health warnings (large-file concentration, missing tests, deep nesting, etc.)
* Language breakdown by extension (top 10)
* Top-3 largest code files
* Verbose mode: depth map + size distribution
* Tree visualisation, Markdown export, JSON export

## Installation

```bash
cargo install --git https://gitlab.com/igbtw/corrode.git
```

### Build from source

```bash
git clone https://gitlab.com/igbtw/corrode.git
cd corrode
cargo install --path .
```

Requires Rust 1.85+ (edition 2024).

## Usage

```bash
corrode analyse .                         # default analysis
corrode analyse src/main.rs               # single file
corrode analyse . --verbose               # depth map + size buckets
corrode analyse . --tree                  # directory tree view
corrode analyse . --json > report.json    # machine-readable export
corrode analyse . --markdown > report.md  # markdown report
```

| Flag | Description |
|------|-------------|
| `-v`, `--verbose` | Show depth map and size distribution |
| `--tree` | Print directory tree and exit |
| `--json` | Export report as JSON |
| `--markdown` | Export report as Markdown |
| `-L`, `--license` | Print license information |

`--tree`, `--json`, and `--markdown` are mutually exclusive.

## Why not cloc / scc?

cloc and scc are excellent LOC counters. corrode is not a LOC counter. It is a repository reconnaissance tool. The line counts exist only as context for the metrics that matter: architecture depth, code concentration, complexity trends, and health warnings. If you need a LOC count, use cloc. If you need to understand a project's structure before contributing to it, use corrode.

## Current status

corrode is alpha software under active development. Output formats and CLI interfaces may change. The current analyser focuses on Rust and Node projects; other language backends are planned. The tool already provides useful output for any directory.

## Roadmap

* **Deep Analysis** — cyclomatic complexity, dependency graphs, dead-code heuristics, churn analysis.
* **AST Support** — Rust `syn` integration, tree-sitter backend for multi-language symbol extraction.
* **Output Formats** — HTML reports, SARIF output for IDE integration.

## Contributing

Bug reports and feature requests are welcome at the [GitLab repository](https://gitlab.com/igbtw/corrode). See [`CONTRIBUTING.md`](CONTRIBUTING.md) for the full contribution guide, and [`docs/`](docs/) for architecture and metrics documentation. Before submitting a PR, run `cargo test` and ensure zero warnings.

## License

MIT

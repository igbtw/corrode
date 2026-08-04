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

  Rust  Project  •  src/lib.rs
  54 files  •  11 dirs  •  5,279 LOC  •  1.2 ms

── Health ──────────────────────────────────────────────────────────────────────
  Good  75/100  ███████████████░░░░░

  ✓ Tests            25/25
  ✓ Documentation    10/10
  ✓ Large Files      26/30

  • Warnings          9/20
  • Concentration     5/15

── Complexity ──────────────────────────────────────────────────────────────────
  Moderate  34/100  ███████░░░░░░░░░░░░░

  LOC                16/30
  Directory Depth     7/15
  Large Files         2/20
  Concentration       5/20
  Directories         4/15

── Key Findings ────────────────────────────────────────────────────────────────
  Largest file share ..................................................... ♦ 20%
  Top 3 file share ....................................................... ♦ 35%

── Architecture ────────────────────────────────────────────────────────────────
  Max Depth .............................................................      3
  Avg LOC/File ..........................................................     85
  Median LOC ............................................................     45
  Avg Size .............................................................. 2.7 KB

── Hotspots ────────────────────────────────────────────────────────────────────
    29% ██████████████████████████████ src/output/renderers/
    29% ██████████████████████████████ src/analysis/
    15% ████████████████░░░░░░░░░░░░░░ tests/
    11% ███████████░░░░░░░░░░░░░░░░░░░ src/output/
     8% ████████░░░░░░░░░░░░░░░░░░░░░░ src/models/
     2% ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░ src/
     2% ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░ src/filesystem/
     2% ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░ src/cli/
     1% █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ src/utils/
     1% █░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ (root)

── Largest Files ───────────────────────────────────────────────────────────────
  #1   summary.rs                     762 LOC 20.0%
  #2   presentation.rs                323 LOC  8.5%
  #3   markdown.rs                    274 LOC  7.2%

── Languages ───────────────────────────────────────────────────────────────────
  Rust        42 files   77.8%
  Markdown     7 files   13.0%
  Lock         1 file    1.9%
  TOML         1 file    1.9%
  Text         1 file    1.9%

── Dependencies ────────────────────────────────────────────────────────────────
  7 crates: clap, indicatif, owo-colors, serde, +3 more

── Code ────────────────────────────────────────────────────────────────────────
  Code     42 files  •  3,801 LOC
  Config    2 files  •    483 LOC
  Docs      8 files  •    973 LOC

  Completed in  1.2 ms
```

## Features

* Project type and entry point detection (Rust, Node, Go, Python, Ruby)
* Dependency extraction from Cargo.toml (aggregated across workspace manifests)
* Code / config / docs classification with per-category LOC
* Architecture metrics: max depth, average and median LOC/file, average file size
* Hotspot analysis: directory LOC share with gauge bars, sorted by importance
* Health score (0–100) with strengths / needs-attention breakdown
* Complexity score (0–100) with human-readable rating — measures
  repository size and nesting, not cyclomatic complexity
* Key findings with severity flags (♦ = caution, critical)
* Language breakdown by extension with percentages
* Top code files with LOC and contribution %
* Verbose mode: depth map, size distribution, language %, file types,
  deep directories, top files
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
| `-v`, `--verbose` | Append detail sections (depth map, size distribution, file types, top files) |
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

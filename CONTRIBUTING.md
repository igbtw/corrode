# Contributing

## Project philosophy

- **Scan fast, report fast.** The full pipeline must complete in < 10ms for a 1,000-file project.
- **No new crate dependencies for features.** Prefer string-based parsing, simple math, and standard library. The only exceptions are `walkdir`, `indicatif`, `clap`, `serde`, and `serde_json`.
- **No AST parsing (yet).** Design for it, but don't implement it.
- **Remove duplication, not abstractions.** Extract a pattern when it appears three times, not before.

## Before you start

1. Open an issue on GitLab describing the change
2. Run `cargo test` and note the current state
3. Keep changes focused — one feature or fix per PR

## Code conventions

- Indentation: 4 spaces
- Line length: 100
- Naming: `snake_case` for functions/variables, `PascalCase` for types/traits
- Imports: `std` → `crate::` → external crates, blank-line separated
- Comments: only for heuristics, algorithms, metrics, non-obvious decisions
- `unwrap()` is acceptable in CLI tools when the failure path is truly unrecoverable

## How to add a metric

1. Create a module under `src/analysis/` (e.g. `src/analysis/coupling.rs`)
2. The module should export a single public function (e.g. `compute_coupling`)
3. If the metric produces new data, add a struct or field to the appropriate sub-report in `src/models/report.rs`
4. Call the function from `src/analysis/report.rs::analyse()`
5. Add display logic to `src/output/summary.rs` and optionally `src/output/markdown.rs`
6. Add tests using `sample_report()` from `models::report::test_helpers`

## How to add a warning

1. Open `src/analysis/warnings.rs`
2. Add the check using the existing `Vec<String>` pattern
3. Make the message actionable: what is wrong, why it matters, what the user should do
4. Set the threshold high enough to avoid noise in small projects

## How to add a reporter

1. Create a module under `src/output/` (e.g. `src/output/html.rs`)
2. Define a struct and implement the `ReportRenderer` trait from `src/output/reporter.rs`
3. Add a CLI flag in `src/cli/commands.rs`
4. Add the flag to the mutually-exclusive set in `src/cli/flags.rs`
5. Wire the flag in `src/main.rs`

## Testing

- Run `cargo test` before every commit
- Avoid adding test fixtures — use `sample_report()` which produces a complete `AnalysisReport` with representative data
- New JSON/Markdown reporters should include tests that verify structural output (headers, tables, presence of key data points)
- All tests must pass with zero compiler warnings

## Releasing

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if one exists)
3. Tag the release: `git tag v0.x.x`
4. Push: `git push origin main --tags`

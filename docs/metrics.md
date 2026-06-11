# Metrics Reference

## Project type detection

Heuristic string-match on file names in the project root:

| Pattern | Type |
|---|---|
| `Cargo.toml` | Rust |
| `package.json` | Node |
| `go.mod` | Go |
| `setup.py` / `pyproject.toml` | Python |
| `Gemfile` | Ruby |

Entry point is the canonical main file for the detected type (e.g. `src/main.rs`, `index.js`, `main.go`).

> **Extração de dependências:** atualmente implementada apenas para projetos
> Rust (leitura de `Cargo.toml`). Para Node, Go, Python e Ruby, o campo
> `dependencies` retorna lista vazia. Suporte a `package.json`, `go.mod`,
> `requirements.txt` e `Gemfile` está planejado para versões futuras.

## File classification

Every file is classified into one of three categories:

| Category | Extensions (example) |
|---|---|
| `code` | rs, py, js, ts, go, rb, java, c, h, cpp, hpp |
| `config` | json, toml, yaml, yml, ini, cfg |
| `docs` | md, rst, txt, html |
| `other` | Everything else |

Classification source of truth: `analysis/classification.rs`.

## Depth map

Relative depth of each file from `project_root`, grouped by level. File count per level.

### How it works
```rust
let depth = Path::new(&entry.path)
    .strip_prefix(root_path)
    .map(|rel| rel.components().count().saturating_sub(1))
    .unwrap_or(0);
```

`saturating_sub(1)` handles files at the root (component count 1 → depth 0).

## Size distribution

Files grouped by line count:

| Bucket | Range |
|---|---|
| Noise | < 10 lines |
| Small | 10–99 |
| Medium | 100–499 |
| Large | ≥ 500 |

## Code metrics (code / config / docs split)

`analysis/metrics.rs::compute_code_metrics()` uses `classify_extension()` to tally files and LOC per category.

### Why this matters
- High config-LOC ratio → project may have complex build tooling
- High docs-LOC ratio → project has documentation (usually good)
- Low code-files-to-config-files ratio → may indicate yak-shaving

## Architecture metrics

### Max depth
Deepest relative directory depth across all files. Filesystem-level, not AST-level.

### Average LOC/file
`total_lines / total_files` across **all** files (not just code).

### Median LOC/file
Sorted line counts, midpoint value. More robust than average for skewed distributions.

### Average file size
`total_bytes / total_files` across all files.

## Complexity score

A heuristic formula (not cyclomatic complexity). Five sqrt-scaled factors
sum to 100:

| Factor | Max pts | Formula | Saturation |
|---|---|---|---|
| LOC | 30 | `(sqrt(code_lines) / 200).min(1.0) × 30` | ~40 000 lines |
| Directory Depth | 15 | `(sqrt(max_depth) / 3).min(1.0) × 15` | depth 9 |
| Large Files | 15 | `min((sqrt(large_count) / 4).min(1.0) × 10 + min(ratio × 5, 5), 15)` | 16 files ≥500 LOC |
| Concentration | 25 | `(top_dir_share / 100) × 25` | 100% in one dir |
| Directories | 15 | `(sqrt(dir_count) / 10).min(1.0) × 15` | ~100 directories |

Score = sum of the five factors, capped at 100.

### Ratings

| Score | Rating |
|---|---|
| 0–20 | Low |
| 21–40 | Moderate |
| 41–60 | High |
| 61–80 | Very High |
| 81–100 | Extreme |

> **Nota:** este score mede complexidade *estrutural* do repositório
> (tamanho, profundidade de diretórios, concentração de código) — não é
> complexidade ciclomática de McCabe, que mede branches lógicos dentro de
> funções. Para análise ciclomática real, consulte o roadmap (AST support).

## Hotspots

LOC share by top-level directory, computed from code files only (filtered via `is_code_extension`).

### Algorithm
1. Filter to code files only
2. Group by parent directory (relative to project root)
3. Sum LOC per group
4. Sort descending, show top 5, lump rest as "other"

### Why pure LOC share?
File count weighting would dilute the signal: a directory with 100 10-line files has the same cognitive weight as one with 10 100-line files. Pure LOC share already encodes both file count and file size, making it a single-dimensional proxy for code concentration.

### Interpreting hotspots
- A single directory > 40% → strong concentration, possibly a monolith
- Even distribution across directories → well-modularised
- "other" > 30% → many small directories, may benefit from regrouping

## Warnings

| Threshold | Warning | Action |
|---|---|---|
| Single code file > 5% of code LOC | Large file warning | Consider splitting into smaller modules |
| Markdown files > 20% of total files | Docs bloat | Audit which docs are stale |
| JSON files > 100 | Data dependency | Consider a database or config consolidation |
| Depth > 8 levels | Deep nesting | Consider flattening the directory structure |
| No README.md | Missing documentation | Add a project overview |
| No tests/ directory | Missing tests | Add test coverage |

Thresholds are conservative (intentionally high) to avoid noise in small or typical projects.

## Top-N code files

`analysis/metrics.rs::top_code_files()` is the shared utility used by both `summary.rs` and `markdown.rs` to avoid duplicating the `filter(code) → sort(LOC) → take(N)` pattern.

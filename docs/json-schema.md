# JSON Schema

## Stability

The JSON output is considered a **public API**. Breaking changes to the
schema will be reflected in the crate's major version.

The following stability guarantees apply:

| Stability Level | Meaning |
|---|---|
| **Stable** | Will not change within the same major version |
| **Draft** | May change without notice — used for pre-release features |

All fields documented here are **Stable** unless marked otherwise.

---

## Top-level structure

```json
{
  "project": { ... },
  "files": { ... },
  "dependencies": { ... },
  "architecture": { ... },
  "quality": { ... }
}
```

---

## `project`

Project-level metadata.

| Field | Type | Description |
|---|---|---|
| `project_type` | `string` | One of: `"Rust"`, `"Node"`, `"Go"`, `"Python"`, `"Ruby"`, `"Unknown"`. Detected from manifest files in the project root. |
| `entry_point` | `string \| null` | The canonical main file for the detected project type (e.g. `"src/main.rs"` for Rust). `null` when no known entry point is found. |
| `project_root` | `string` | The path passed to the `analyse` command. |
| `duration` | `float` | Wall-clock time in seconds (e.g. `0.653`). Measured from pipeline start to report assembly. |

---

## `files`

File-level data: inventory, grouping, distribution.

### `files.entries[]`

One entry per scanned file. Does **not** include skipped files
(binary, minified, ignored directories).

| Field | Type | Description |
|---|---|---|
| `path` | `string` | Absolute or relative path as returned by the scanner. |
| `extension` | `string` | Lowercased file extension (e.g. `"rs"`, `"toml"`, `"md"`). Empty string for files without extension. |
| `line_count` | `integer` | Number of lines in the file (via `contents.lines().count()`). |
| `size_bytes` | `integer` | File size in bytes (via `std::fs::metadata`). |

### `files.directory_count`

`integer` — Number of non-root directories scanned (ignoring skipped dirs).

### `files.total_lines`

`integer` — Sum of `line_count` across all entries.

### `files.language_map`

`object` — Mapping of extension → file count (e.g. `{"rs": 29, "md": 3}`).
Only includes extensions that appeared in at least one file.

### `files.directory_stats[]`

Top 5 directories by total lines.

| Field | Type | Description |
|---|---|---|
| `path` | `string` | Full directory path. |
| `file_count` | `integer` | Number of files in the directory. |
| `total_lines` | `integer` | Sum of line counts in the directory. |
| `total_bytes` | `integer` | Sum of file sizes in the directory. |

### `files.depth_map`

`array<[integer, integer]>` — Array of `[depth, file_count]` pairs.
Depth is the relative directory depth from the project root (root files
have depth 0). Sorted by depth ascending.

### `files.size_distribution`

File count by size bucket based on `line_count`.

| Field | Type | Bucket | Range |
|---|---|---|---|
| `noise` | `integer` | < 10 lines | `line_count < 10` |
| `small` | `integer` | 10–99 lines | `10 <= line_count < 100` |
| `medium` | `integer` | 100–499 lines | `100 <= line_count < 500` |
| `large` | `integer` | ≥ 500 lines | `line_count >= 500` |

---

## `dependencies`

> **Suporte atual:** a extração de itens em `dependencies.items` está
> implementada somente para projetos Rust. Para outras linguagens detectadas,
> `count` será `0` e `items` será `[]`.

```json
{ "count": 5, "items": ["clap", "serde"] }
```

| Field | Type | Description |
|---|---|---|
| `count` | `integer` | Number of dependencies extracted. |
| `items` | `array<string>` | Dependency names. May be empty. |

Currently only supports Cargo.toml (Rust). Other project types return
`{"count": 0, "items": []}`.

---

## `architecture`

Structural and organisational metrics.

### `architecture.metrics`

| Field | Type | Description |
|---|---|---|
| `max_depth` | `integer` | Deepest relative directory depth across all files. |
| `avg_loc_per_file` | `float` | Mean LOC/file across **code** files only. |
| `median_loc_per_file` | `float` | Median LOC/file across code files. More robust than mean for skewed distributions. |
| `avg_file_size` | `float` | Mean file size in bytes across code files. |

### `architecture.hotspots[]`

Top 5 directories by code LOC share, plus an `"other"` bucket.

| Field | Type | Description |
|---|---|---|
| `path` | `string` | Directory path relative to project root, or `"(root)"` for files in the project root, or `"other"` for the remaining aggregated directories. |
| `total_lines` | `integer` | Total code lines in this directory. |
| `percentage` | `float` | Percentage of total code LOC (0.0–100.0). |

### `architecture.code_metrics`

Counts of files and LOC by category.

| Field | Type | Description |
|---|---|---|
| `code_files` | `integer` | Files classified as code (e.g. `.rs`, `.py`, `.js`). |
| `code_lines` | `integer` | Total lines in code files. |
| `config_files` | `integer` | Files classified as config (e.g. `.toml`, `.json`, `.yaml`). |
| `config_lines` | `integer` | Total lines in config files. |
| `docs_files` | `integer` | Files classified as docs (e.g. `.md`, `.txt`). |
| `docs_lines` | `integer` | Total lines in docs files. |

---

## `quality`

Health and complexity assessment.

### `quality.complexity`

| Field | Type | Description |
|---|---|---|
| `score` | `integer` | Heuristic complexity score 0–100. See [metrics.md](metrics.md) for the formula. |
| `rating` | `string` | Human-readable rating: `"Tiny"`, `"Small"`, `"Medium"`, `"Large"`, or `"Massive"`. |

### `quality.warnings`

`array<string>` — Human-readable warning strings. May be empty.

Current checks:

| Trigger | Example warning |
|---|---|
| Single code file > 5% of code LOC | `"Largest code file represents 45% of code LOC — consider splitting into smaller modules"` |
| Markdown files > 20% of total | `"Markdown files represent 45% of project files"` |
| JSON file count > 100 | `"More than 150 JSON files detected"` |
| Max depth > 8 | `"Project depth exceeds 8 levels — consider flattening the directory structure"` |
| No `README.md` | `"No README.md found — add a project overview for new contributors"` |
| No `tests/` directory | `"No tests/ directory detected — add test coverage to improve maintainability"` |

---

## Example

```json
{
  "project": {
    "project_type": "Rust",
    "entry_point": "src/main.rs",
    "project_root": "/home/user/project",
    "duration": 0.653
  },
  "files": {
    "entries": [
      {
        "path": "/home/user/project/src/main.rs",
        "extension": "rs",
        "line_count": 185,
        "size_bytes": 4284
      }
    ],
    "directory_count": 8,
    "total_lines": 2591,
    "language_map": {
      "rs": 29,
      "toml": 3,
      "md": 2,
      "json": 1,
      "yaml": 1
    },
    "directory_stats": [
      {
        "path": "/home/user/project/src",
        "file_count": 12,
        "total_lines": 980,
        "total_bytes": 34200
      }
    ],
    "depth_map": [[0, 5], [1, 15], [2, 8], [3, 1]],
    "size_distribution": {
      "noise": 2,
      "small": 15,
      "medium": 10,
      "large": 2
    }
  },
  "dependencies": {
    "count": 5,
    "items": ["clap", "serde", "indicatif", "walkdir", "serde_json"]
  },
  "architecture": {
    "metrics": {
      "max_depth": 3,
      "avg_loc_per_file": 66.6,
      "median_loc_per_file": 55.0,
      "avg_file_size": 2240.2
    },
    "hotspots": [
      { "path": "src/output/", "total_lines": 850, "percentage": 35.1 },
      { "path": "src/analysis/", "total_lines": 620, "percentage": 25.6 },
      { "path": "other", "total_lines": 150, "percentage": 6.2 }
    ],
    "code_metrics": {
      "code_files": 29,
      "code_lines": 1913,
      "config_files": 5,
      "config_lines": 120,
      "docs_files": 3,
      "docs_lines": 45
    }
  },
  "quality": {
    "complexity": { "score": 29, "rating": "Small" },
    "warnings": [
      "No tests/ directory detected — add test coverage to improve maintainability"
    ]
  }
}
```

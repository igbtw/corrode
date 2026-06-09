# Output Formats

## Default (terminal summary)

Produced by `SummaryReporter` in `output/summary.rs`.

Sections:
- Project type + entry point
- File/dir/LOC overview
- Dependencies (top 4 + remainder count)
- Largest directories (top 5 by LOC)
- Code metrics (code/config/docs split)
- Architecture (depth, avg/median LOC/file, avg size)
- Hotspots (top 5 directories by LOC share + other)
- Complexity score + rating
- Warnings
- Languages (top 10 + remainder)
- Largest code files (top 3)

Verbose mode (`--verbose`) adds:
- Depth map (file count per relative depth level)
- Size distribution (noise/small/medium/large buckets)

### Example
```text
Detected      Rust Project
Entry Point   src/main.rs
...
```

## JSON (`--json`)

Produced by `JsonReporter` in `output/json.rs`.

- All model types derive `serde::Serialize`
- `FileEntry.contents` is skipped via `#[serde(skip)]` (multi-KB contents irrelevant for metadata)
- `DependencyReport` custom-serialises as `{"count": N, "items": [...]}`
- `ProjectInfo.duration` serialises as `f64` seconds via private `duration_serde` module
- Output uses `serde_json::to_string_pretty` for readability

### Schema
```json
{
  "project": {
    "project_type": "Rust",
    "entry_point": "src/main.rs",
    "duration": 0.653
  },
  "files": {
    "entries": [{ "path": "...", "extension": "rs",
                  "line_count": 100, "size_bytes": 2048 }],
    "directory_count": 8,
    "total_lines": 2591,
    "directory_stats": [...],
    "depth_map": [[0, 5], [1, 20], ...],
    "size_distribution": { "noise": 2, "small": 15, ... },
    "language_map": { "rs": 29, "md": 1, ... }
  },
  "dependencies": { "count": 5, "items": ["clap", ...] },
  "architecture": {
    "code_metrics": { "code_files": 29, "code_lines": 1913, ... },
    "metrics": { "max_depth": 3, "avg_loc_per_file": 61.0, ... },
    "hotspots": [{ "path": "src/output/", "percentage": 35.0, ... }]
  },
  "quality": {
    "complexity": { "score": 29, "rating": "Small" },
    "warnings": ["Large code file...", "No tests/..."]
  }
}
```

## Markdown (`--markdown`)

Produced by `MarkdownReporter` in `output/markdown.rs`.

GitHub-Flavoured Markdown with tables. Same data as terminal summary but formatted as a readable `.md` document. Ideal for CI artifacts or PR attachments.

### Sections
- Project (bullet list)
- Dependencies (bullet list)
- Largest Directories (table)
- Code Metrics (table)
- Architecture (table)
- Hotspots (table)
- Complexity (inline)
- Warnings (bullet list)
- Languages (table)
- Largest Code Files (table)

### Example output
```markdown
# Codebase Analysis Report

## Project
- **Type:** Rust
- **Entry Point:** src/main.rs
- **Files:** 35
- **Directories:** 8
- **Total LOC:** 2,591
...
```

## Directory tree (`--tree`)

Produced by `print_tree()` in `output/tree.rs`.

Simple tree visualisation using `├──` / `└──` unicode box drawing. Not affected by `--verbose`, `--json`, or `--markdown` flags. Mutually exclusive with other output flags.

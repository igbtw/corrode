# corrode

> Alpha software. APIs and output formats may change without notice.

corrode is a codebase analysis engine written in Rust that helps developers understand unfamiliar software systems faster.

It scans a project directory and produces a clean terminal summary with file counts, language breakdown, largest files, and directory structure — all in milliseconds.

## Example

```text
$ corrode analyse .

✓ Scanning project...

Detected      Rust Project

Entry Point   src/main.rs

Project      18 files · 7 dirs · 1,350 LOC

Structure
  (root)               6 files
  src/output/          3 files
  src/cli/             2 files
  src/filesystem/      2 files
  src/utils/           2 files
  src/                 1 file
  src/engine/          1 file
  src/models/          1 file

Languages
  Rust         12 files
  Lock          1 file
  Markdown      1 file
  TOML          1 file
  Text          1 file

Largest Files
  Cargo.lock       398
  summary.rs       257
  README.md        143

Completed in 0.461 ms
```

## Installation

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (edition 2024)

### Install from GitLab

```bash
cargo install --git https://gitlab.com/igbtw/corrode.git
```

### Build from source

```bash
git clone https://gitlab.com/igbtw/corrode.git
cd corrode
cargo install --path .
```

## Usage

```bash
# Analyze current directory
corrode analyse .

# Analyze a single file
corrode analyse POEM.txt

# Verbose mode (depth map + size distribution)
corrode analyse . --verbose

# Directory tree visualization
corrode analyse . --tree
```

### Commands

| Command          | Description                 |
| ---------------- | --------------------------- |
| `analyse <PATH>` | Analyze a file or directory |
| `version`        | Show version information    |

### Flags (analyse)

| Flag              | Description                            |
| ----------------- | -------------------------------------- |
| `-v, --verbose`   | Show depth map and size distribution   |
| `--tree`          | Print a directory tree and exit        |

### Global flags

| Flag            | Description               |
| --------------- | ------------------------- |
| `-h, --help`    | Print help information    |
| `-V, --version` | Print version information |
| `-L, --license` | Print license information |

## How it works

```
Source code → Scanner → Reader → Report
```

1. **Project detection** — checks for `Cargo.toml`, `package.json`, `go.mod`, etc.
2. **Entry point detection** — finds `src/main.rs`, `index.js`, etc. based on project type
3. **Scanner** — walks the directory tree, skipping `target/`, `.git/`, and binary extensions
4. **Reader** — reads each file, counts lines, records extension and size
5. **Report** — aggregates everything into a summary with language stats, directory structure, and largest files

## Current capabilities

- [x] CLI with clap (subcommands, flags, auto-generated `--help`)
- [x] Single-file and directory analysis
- [x] Recursive directory scanning with `walkdir`
- [x] Smart exclusion of build artifacts (`target/`, `.git/`, etc.) and binary files
- [x] Project type detection (Rust, Node, Go, Python, Ruby)
- [x] Entry point detection (`src/main.rs`, `src/lib.rs`, `index.js`, etc.)
- [x] Directory structure with collapse threshold
- [x] Language breakdown by extension
- [x] Largest files by line count (top 3)
- [x] Progress spinner during analysis
- [x] Auto-scaled timing (ms / s)
- [x] Verbose mode (depth map + size distribution)
- [x] Directory tree visualization (`--tree`)

## Roadmap

### Deep code analysis

- [ ] AST parsing with `syn` (Rust)
- [ ] Module dependency graph
- [ ] Function / struct / enum extraction
- [ ] Dead code detection
- [ ] Complexity metrics

### Output formats

- [ ] JSON output (`--format json`)
- [ ] Markdown reports (`--format markdown`)

### Multi-language

- [ ] `tree-sitter` integration
- [ ] Python, JavaScript, Go support

## Contributing

Contributions, bug reports, and suggestions are very welcome.

Open an issue or merge request on [GitLab](https://gitlab.com/igbtw/corrode).

## License

MIT

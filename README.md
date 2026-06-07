# RSfactai

> ⚠️ Alpha software. APIs and output formats may change without notice.

RSfactai is a codebase analysis engine written in Rust that helps developers understand unfamiliar software systems faster.

It scans a project directory and produces a clean terminal summary with file counts, language breakdown, and the largest files — all in milliseconds.

## Example

```text
$ rsfactai analyse .

✓ Scanning project...

Detected      Rust Project

Project      17 files · 7 dirs · 1,226 LOC

Languages
  Rust         11 files
  Lock          1 file
  Markdown      1 file
  TOML          1 file
  Text          1 file

Largest Files
  Cargo.lock       398
  summary.rs       134
  README.md        132

Completed in 0.451 ms
```

## Installation

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (edition 2024)

### Install from GitLab

```bash
cargo install --git https://gitlab.com/igbtw/RSfactai.git
```

### Build from source

```bash
git clone https://gitlab.com/igbtw/RSfactai.git
cd RSfactai
cargo install --path .
```

## Usage

```bash
# Analyze a file or directory
rsfactai analyse POEM.txt
rsfactai analyse .

# Show help
rsfactai --help

# Show version
rsfactai --version

# Show license
rsfactai --license
```

### Commands

| Command          | Description                 |
| ---------------- | --------------------------- |
| `analyse <PATH>` | Analyze a file or directory |
| `version`        | Show version information    |

### Flags

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
2. **Scanner** — walks the directory tree, skipping `target/`, `.git/`, and binary extensions
3. **Reader** — reads each file, counts lines, records extension and size
4. **Report** — aggregates everything into a summary with language stats and largest files

## Current capabilities

- [x] CLI with clap (subcommands, flags, auto-generated `--help`)
- [x] Single-file and directory analysis
- [x] Recursive directory scanning with `walkdir`
- [x] Smart exclusion of build artifacts (`target/`, `.git/`, etc.) and binary files
- [x] Project type detection (Rust, Node, Go, Python, Ruby)
- [x] Language breakdown by extension
- [x] Largest files by line count (top 3)
- [x] Progress spinner during analysis
- [x] Auto-scaled timing (ms / s)

## Roadmap

### Project structure & entry points

- [ ] Directory tree overview (`Structure` section in report)
- [ ] Entry point detection (`src/main.rs`, `src/lib.rs`)

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

Contributions, bug reports, and suggestions are so welcome.

Open an issue or merge request on [GitLab](https://gitlab.com/igbtw/RSfactai).

## License

MIT

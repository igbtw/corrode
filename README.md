# RSfactai

**RSfactai** is a codebase analysis engine (built with Rust) that helps developers understand complex software systems in a fraction of the time.

It analyzes codebases by:

- Mapping the architecture (modules, dependencies, entry points)
- Detecting issues automatically (dead code, complexity, anti-patterns)
- Generating documentation from source analysis
- Planning modernization strategies

The goal is to reduce the time required to understand large and unfamiliar projects from **weeks to hours**.

## Status

This project is in early development. Currently it reads and analyzes individual files. Directory-wide scanning and deep code parsing are coming next.

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2024, Rust 1.85+)

### Install directly from GitLab

```bash
cargo install --git https://gitlab.com/igbtw/RSfactai.git
```

### Build from source

```bash
git clone https://gitlab.com/igbtw/RSfactai.git
cd RSfactai
cargo install --path .
```

Either way, the `rsfactai` binary will be placed in `~/.cargo/bin/` (make sure it's in your `PATH`).

## Usage

```bash
# Analyze a file
rsfactai analyse POEM.txt

# Show help
rsfactai --help

# Show version
rsfactai --version
```

### Commands

| Command          | Description                 |
| ---------------- | --------------------------- |
| `analyse <PATH>` | Analyze a file or directory |
| `help`           | Print help information      |
| `version`        | Print version information   |

## How it works

```
Source code → Scanner → Parser → Analysis → Report
```

1. **Scanner** walks the directory tree and discovers source files
2. **Parser** builds an AST (abstract syntax tree) for each file
3. **Analysis** applies rules to detect patterns, issues, and architecture
4. **Report** formats the results as terminal output, JSON, or Markdown

## Roadmap

- [x] CLI with clap
- [x] File reading with progress indicator
- [ ] Directory-wide scanning (`walkdir`)
- [ ] Rust source parsing (`syn`)
- [ ] Multi-language support (`tree-sitter`)
- [ ] Issue detection (dead code, complexity metrics)
- [ ] Markdown documentation generation
- [ ] JSON output for tooling integration

## Contributing

Contributions are welcome! Feel free to open issues or merge requests on [GitLab](https://gitlab.com/igbtw/RSfactai).

# RSfactai

> ⚠️ Alpha software. APIs and output formats may change without notice.

RSfactai is a codebase analysis engine written in Rust that aims to help developers understand unfamiliar software systems faster.

The long-term goal is to automatically map project architecture, analyze source code, detect issues, and generate useful documentation.

## Development Status

RSfactai is currently in early development.

The project already provides:

- Command-line interface built with Clap
- File loading and processing pipeline
- Progress indicators for analysis tasks
- Modular architecture for future scanners and parsers

Planned features include project-wide scanning, AST analysis, architecture mapping, documentation generation, and issue detection.

---

## Why RSfactai?

Understanding an unfamiliar codebase can take days or even weeks.

RSfactai aims to reduce that effort by automatically answering questions such as:

- Where does the application start?
- How are modules connected?
- Which files are most important?
- What parts of the code are unused?
- Which areas need refactoring?
- How can the project be modernized?

---

## Installation

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.85+ (Cargo included)
- [Cargo](https://doc.rust-lang.org/cargo/)

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

---

## Usage

```bash
# Analyze a file
rsfactai analyse POEM.txt

# Show help
rsfactai --help

# Show version
rsfactai --version
```

---

### Commands

| Command          | Description                 |
| ---------------- | --------------------------- |
| `analyse <PATH>` | Analyze a file or directory |
| `help`           | Print help information      |
| `version`        | Print version information   |

---

## Roadmap

### Core Engine

- [x] CLI interface
- [x] File loading
- [x] Progress indicators
- [ ] Directory scanning
- [ ] Project discovery
- [ ] Multi-file analysis

### Rust Analysis

- [ ] AST parsing with syn
- [ ] Module dependency graph
- [ ] Entry-point detection
- [ ] Dead code detection
- [ ] Complexity metrics

### Reporting

- [ ] Terminal reports
- [ ] Markdown reports
- [ ] JSON output
- [ ] HTML reports

### Future

- [ ] tree-sitter integration
- [ ] Multi-language support
- [ ] Architecture diagrams
- [ ] AI-assisted explanations

---

## Contributing

Contributions, bug reports, and suggestions are welcome.

Open an issue or merge request on [GitLab](https://gitlab.com/igbtw/RSfactai).

---

## License

This project is licensed under the MIT License.

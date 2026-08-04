# Corrode Agent Guide

## Project Goal

Corrode is a Rust codebase analysis tool focused on:

- project structure
- architecture insights
- maintainability metrics
- developer-facing reports

It is NOT intended to be:

- a linter
- a formatter
- a static security scanner

## Design Principles

1. Fast execution
2. Zero external services
3. Human-readable output
4. Useful on large repositories
5. Architecture over micro-metrics

## Output Rules

- Prefer concise metrics over long sentences.
- Avoid truncating important information.
- Use consistent alignment.
- Verbose mode may contain additional cards.
- Normal mode must remain compact.

## Health Score

Purpose:
Evaluate maintainability.

Do not remove score components without justification.

Current factors:

- Tests
- Warnings
- Concentration
- Documentation
- Large Files

## Complexity Score

Purpose:
Measure structural complexity.

Current factors:

- LOC
- Directory Depth
- Large Files
- Concentration
- Directories

## Hotspots

Hotspots represent code concentration.

Goals:

- Reveal dominant directories
- Minimize "other"
- Preserve readability

## Performance Requirements

Analysis should remain usable on repositories with:

- > 1M LOC
- > 500 directories

Avoid algorithms worse than O(n log n) when possible.

## UI Philosophy

Inspired by:

- btop
- htop
- lazygit

Prefer:
metrics > prose

Prefer:
information density > decorative output

Avoid:
excessive icons

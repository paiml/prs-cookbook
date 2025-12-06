# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PRS Cookbook is a collection of examples demonstrating the **Presentar Scene Format** (`.prs`) - a declarative YAML-based manifest for sharing visualization dashboards, ML model interfaces, and interactive data applications. The project follows **IIUR Principles**: Isolated, Idempotent, Useful, and Reproducible.

## Project Status

The core library is implemented with 48 unit tests + 7 property-based tests. Categories A-G examples are complete (27 examples total). The specification is at `docs/specifications/cookbook-spec.md`.

## Build Commands
```bash
cargo build                           # Build the library
cargo test                            # Run all tests
cargo run --example <name>            # Run a specific example
cargo clippy -- -D warnings           # Lint (zero warnings policy)
cargo fmt --check                     # Check formatting
cargo llvm-cov --fail-under 95        # Coverage (95% minimum)
cargo mutants                         # Mutation testing (80% minimum)
```

## Architecture

### Recipe Structure
Each recipe is a self-contained example:
```
examples/
└── category/
    └── recipe_name.rs

scenes/
└── category/
    └── recipe_name.prs
```

### Recipe Categories
- **A**: Scene Creation (layouts: minimal, grid, flex, absolute, nested)
- **B**: Widget Configuration (textbox, slider, dropdown, charts, tables)
- **C**: Model & Dataset Resources (local/remote models, datasets)
- **D**: Bindings & Interactions (triggers, debounce, inference, chains)
- **E**: Theming & Styling (presets, custom colors/fonts)
- **F**: Permissions & Security (network, filesystem, minimal)
- **G**: Expression Language (select, filter, sort, aggregation, format)
- **H**: WASM & Browser (requires `browser` feature)
- **I**: CLI Tools (requires `cli` feature)
- **J**: Complete Applications (full runtime integration)

### Feature Flags
- `default`: Core parsing and validation
- `browser`: WASM target support
- `cli`: Command-line tools
- `presentar`: Full runtime integration
- `full`: All features

## Key Design Principles

### IIUR Compliance
Every recipe must be:
1. **Isolated**: Use `tempfile::tempdir()` for file I/O, no global state
2. **Idempotent**: Same input always produces same output
3. **Useful**: Solve real problems with copy-paste ready code
4. **Reproducible**: Pinned dependencies, cross-platform (Linux, macOS, WASM)

### Code Standards
- No `unwrap()` in logic - use proper error handling with `thiserror`
- All scenes must validate at parse time
- Remote resources require BLAKE3 hash verification
- 95% line coverage, 90% branch coverage, 80% mutation score

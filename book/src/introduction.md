# PRS Cookbook

Welcome to the **PRS Cookbook** — a collection of production-ready recipes for creating visualization dashboards using the Presentar Scene Format (`.prs`).

## What is PRS?

The Presentar Scene Format (`.prs`) is a YAML-based declarative format for defining visualization dashboards. It provides:

- **Declarative Scenes**: Define UI layouts, widgets, and bindings in YAML
- **Type-Safe Parsing**: Full validation with descriptive error messages
- **Zero Runtime**: Pure Rust + WASM — no Python interpreter required
- **Resource Management**: Models and datasets with BLAKE3 hash verification
- **Expression Language**: Data transforms with `{{ data | filter | sort }}`

## IIUR Principles

Every recipe in this cookbook adheres to Toyota Production System quality standards:

| Principle | Implementation |
|-----------|----------------|
| **Isolated** | Each recipe uses `tempfile::tempdir()` — no shared state |
| **Idempotent** | Same input always produces same output — verified by proptest |
| **Useful** | Solves real visualization problems — copy-paste ready |
| **Reproducible** | Pinned dependencies, cross-platform (Linux, macOS, WASM) |

## Recipe Categories

This cookbook contains **27 recipes** across 7 categories:

| Category | Recipes | Description |
|----------|---------|-------------|
| **A: Scene Creation** | 5 | Layouts: minimal, grid, flex, absolute, nested |
| **B: Widget Configuration** | 5 | Inputs: textbox, slider, dropdown, charts, tables |
| **C: Model & Dataset Resources** | 4 | Local/remote models, datasets, fallback sources |
| **D: Bindings & Interactions** | 5 | Triggers, debounce, inference, chains, conditionals |
| **E: Theming & Styling** | 4 | Dark/light presets, custom colors, typography |
| **F: Permissions & Security** | 3 | Network, filesystem, minimal privilege |
| **G: Expression Language** | 5 | Select, filter, sort, aggregation, format |

## Sovereign Stack Integration

PRS Cookbook is part of the Sovereign AI Stack:

```
trueno (SIMD) → aprender (ML) → presentar (Runtime) → prs-cookbook (Recipes)
```

| Component | Purpose |
|-----------|---------|
| [trueno](https://github.com/paiml/trueno) | SIMD-accelerated tensor operations |
| [aprender](https://github.com/paiml/aprender) | ML algorithms and APR format |
| [presentar](https://github.com/paiml/presentar) | Scene rendering runtime |
| **prs-cookbook** | Production-ready visualization recipes |

## Getting Started

```bash
# Add to your Cargo.toml
[dependencies]
prs-cookbook = "0.1"

# Run an example
cargo run --example create_minimal_scene
```

Ready to dive in? Start with [Installation](./getting-started/installation.md) or jump to the [Quick Start](./getting-started/quick-start.md).

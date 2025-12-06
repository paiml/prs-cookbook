# Installation

## Requirements

- Rust 1.75 or later
- Cargo (comes with Rust)

## Add to Your Project

Add `prs-cookbook` to your `Cargo.toml`:

```toml
[dependencies]
prs-cookbook = "0.1"
```

With optional features:

```toml
[dependencies]
prs-cookbook = { version = "0.1", features = ["browser", "cli"] }
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `default` | Core parsing and validation |
| `browser` | WASM target support (wasm-bindgen) |
| `cli` | Command-line tools (clap) |
| `presentar` | Full runtime integration |
| `full` | All features enabled |

## Verify Installation

```bash
# Create a new project
cargo new my-dashboard
cd my-dashboard

# Add prs-cookbook
cargo add prs-cookbook

# Build to verify
cargo build
```

## Next Steps

Continue to [Quick Start](./quick-start.md) to create your first scene.

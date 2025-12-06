# Feature Flags

## Available Features

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `default` | Core parsing and validation | serde, serde_yaml |
| `browser` | WASM target support | wasm-bindgen |
| `cli` | Command-line tools | clap |
| `presentar` | Full runtime integration | presentar |
| `full` | All features enabled | All above |

## Usage

```toml
# Default (core only)
[dependencies]
prs-cookbook = "0.1"

# With browser support
[dependencies]
prs-cookbook = { version = "0.1", features = ["browser"] }

# All features
[dependencies]
prs-cookbook = { version = "0.1", features = ["full"] }
```

## Conditional Compilation

```rust
#[cfg(feature = "browser")]
pub fn init_browser() {
    // Browser-specific initialization
}

#[cfg(feature = "cli")]
pub fn run_cli() {
    // CLI-specific code
}
```

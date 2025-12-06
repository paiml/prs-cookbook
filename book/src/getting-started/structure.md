# Project Structure

## Repository Layout

```
prs-cookbook/
├── Cargo.toml           # Package configuration
├── Cargo.lock           # Pinned dependencies
├── Makefile             # Build automation
├── src/
│   ├── lib.rs           # Main library module
│   ├── scene.rs         # Scene types and parsing
│   ├── context.rs       # RecipeContext for isolation
│   └── error.rs         # Error types
├── examples/
│   ├── a_scene_creation/    # Category A recipes
│   ├── b_widget_config/     # Category B recipes
│   ├── c_resources/         # Category C recipes
│   ├── d_bindings/          # Category D recipes
│   ├── e_theming/           # Category E recipes
│   ├── f_permissions/       # Category F recipes
│   └── g_expressions/       # Category G recipes
├── benches/
│   └── parsing.rs       # Performance benchmarks
├── book/                # This documentation
└── .pmat/               # Quality tracking
```

## Example Naming Convention

Examples follow the pattern: `{action}_{subject}.rs`

- `create_minimal_scene.rs`
- `widget_text_input.rs`
- `binding_debounced.rs`

## Running Examples

```bash
# Run a specific example
cargo run --example create_minimal_scene

# Run all examples
make examples
```

## Quality Gates

```bash
# Quick validation (< 5 min)
make quick-validate

# Full validation
make validate

# Coverage report
make coverage
```

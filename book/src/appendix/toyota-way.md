# Toyota Way Principles

This cookbook follows Toyota Production System principles for software quality.

## Core Principles

### Jidoka (Built-in Quality)

Quality is built in, not inspected in.

- Rust's type system catches errors at compile time
- Validation rules in `Scene::validate()`
- Property-based testing with proptest

### Muda (Waste Elimination)

Eliminate unnecessary work and resources.

- Zero runtime dependencies (no Python)
- Automatic temp directory cleanup
- Minimal binary size

### Heijunka (Leveling)

Consistent, predictable patterns.

- All recipes follow the same structure
- `RecipeContext` standardizes execution
- Uniform error handling

### Genchi Genbutsu (Go and See)

Understand by doing.

- Every recipe is runnable
- Examples demonstrate real use cases
- Copy-paste ready code

## Quality Gates

```bash
# Pre-commit
make quick-validate

# Pre-release
make validate
make coverage
```

## Continuous Improvement (Kaizen)

- Property-based tests discover edge cases
- Coverage tracking identifies gaps
- PMAT metrics track quality over time

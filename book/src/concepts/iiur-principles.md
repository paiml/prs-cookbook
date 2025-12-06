# IIUR Principles

Every recipe follows the **IIUR Principles** from the Toyota Production System.

## Isolated

Each recipe uses `tempfile::tempdir()` — no shared state between recipes.

```rust
let ctx = RecipeContext::new("my-recipe")?;
// All files created in isolated temp directory
let path = ctx.path("scene.prs");
// Automatic cleanup on drop
```

## Idempotent

Same input always produces same output. Deterministic RNG seeded by recipe name.

```rust
let mut ctx = RecipeContext::new("deterministic")?;

// Same recipe name = same RNG sequence
let val1: u64 = ctx.rng().gen();
let val2: u64 = ctx.rng().gen();
// Always produces the same sequence
```

## Useful

Solves real visualization problems. Code is copy-paste ready.

- Each recipe demonstrates a specific concept
- Examples compile and run standalone
- Real-world patterns for production use

## Reproducible

Pinned dependencies, cross-platform (Linux, macOS, WASM).

- `Cargo.lock` committed for determinism
- CI verified on multiple platforms
- Property-based testing with proptest

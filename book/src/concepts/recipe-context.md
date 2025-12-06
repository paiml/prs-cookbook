# Recipe Context

The `RecipeContext` provides isolated, reproducible execution for recipes.

## Creating a Context

```rust
use prs_cookbook::prelude::*;

let ctx = RecipeContext::new("my-recipe")?;
```

## Deterministic RNG

The RNG is seeded from the recipe name using BLAKE3:

```rust
use rand::Rng;

let mut ctx = RecipeContext::new("deterministic")?;
let value: u64 = ctx.rng().gen();
// Same name = same sequence every time
```

## Metrics Tracking

Record metrics for reporting:

```rust
ctx.record_metric("widget_count", 10);
ctx.record_float_metric("parse_time", 1.234);
ctx.record_duration("validation", Duration::from_millis(42));
ctx.record_string_metric("scene_name", "dashboard");

ctx.report_metrics()?;
```

## File I/O

Work with files in the isolated temp directory:

```rust
// Get path in temp directory
let path = ctx.path("scene.prs");

// Write and read scenes
ctx.write_scene("scene.prs", &scene)?;
let loaded = ctx.read_scene("scene.prs")?;
```

## Helper Functions

```rust
// Hash name to deterministic seed
let seed = hash_name_to_seed("recipe-name");

// Generate deterministic test data
let data = generate_test_data(seed, 100);

// Generate test scene YAML
let yaml = generate_test_scene(seed, 5);
```

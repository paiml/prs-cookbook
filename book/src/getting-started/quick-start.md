# Quick Start

Create your first PRS scene in under 5 minutes.

## Minimal Scene

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    // Parse a .prs scene
    let yaml = r#"
prs_version: "1.0"
metadata:
  name: "hello-world"
layout:
  type: flex
  direction: column
widgets:
  - id: greeting
    type: markdown
    config:
      content: "# Hello, Presentar!"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Scene: {} ({} widgets)",
        scene.metadata.name,
        scene.widgets.len());
    Ok(())
}
```

## Using RecipeContext

For isolated, reproducible recipes, use `RecipeContext`:

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    // Create isolated context with deterministic RNG
    let ctx = RecipeContext::new("my-recipe")?;

    // Files are created in temp directory
    let scene_path = ctx.path("scene.prs");

    // ... work with scene

    // Automatic cleanup on drop
    Ok(())
}
```

## Run an Example

```bash
cargo run --example create_minimal_scene
```

Output:
```
Created minimal scene:
  Name: hello-world
  Widgets: 1
=== Recipe: create_minimal_scene ===
Duration: 0.25ms
Scene: hello-world
Version: 1.0
Layout: Flex
Widgets: 1
===
```

## Next Steps

- Learn about [Project Structure](./structure.md)
- Explore [Scene Creation Recipes](../recipes/a-scene-creation/index.md)

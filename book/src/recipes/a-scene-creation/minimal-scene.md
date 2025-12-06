# Minimal Scene

The simplest possible PRS scene.

## Code

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_minimal_scene")?;

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

    ctx.report(&scene)?;
    Ok(())
}
```

## Run

```bash
cargo run --example create_minimal_scene
```

## Key Points

- Every scene needs `prs_version`, `metadata`, `layout`, and `widgets`
- `metadata.name` must be kebab-case
- Widgets need unique `id` fields
- `RecipeContext` provides isolated execution

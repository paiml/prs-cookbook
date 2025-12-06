# Absolute Layout

Create scenes with absolute positioning for pixel-perfect control.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - uses `tempfile::tempdir()` |
| Idempotency | Guaranteed - same input → same output |
| Usefulness | Pixel-perfect dashboards and fixed layouts |
| Reproducibility | Deterministic RNG seeded by recipe name |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_absolute_layout")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "absolute-demo"
  title: "Absolute Layout Demo"

layout:
  type: absolute
  width: 800
  height: 600

widgets:
  - id: title
    type: markdown
    position: { x: 20, y: 20, width: 760, height: 60 }
    config:
      content: "Absolute Positioning Demo"

  - id: sidebar
    type: markdown
    position: { x: 20, y: 100, width: 200, height: 480 }
    config:
      content: "Sidebar"

  - id: main
    type: markdown
    position: { x: 240, y: 100, width: 540, height: 480 }
    config:
      content: "Main Content Area"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Absolute layout:");
    println!(
        "  Canvas: {}x{}",
        scene.layout.width.unwrap_or(0),
        scene.layout.height.unwrap_or(0)
    );

    ctx.report(&scene)?;
    Ok(())
}
```

## Key Concepts

### Canvas Properties

| Property | Type | Description |
|----------|------|-------------|
| `width` | u32 | Canvas width in pixels |
| `height` | u32 | Canvas height in pixels |

### Widget Positioning

```yaml
position:
  x: 20       # Left offset in pixels
  y: 100      # Top offset in pixels
  width: 200  # Widget width
  height: 480 # Widget height
```

## Run

```bash
cargo run --example create_absolute_layout
```

## When to Use

- **Fixed dashboards**: Known screen dimensions
- **Presentations**: Slide-like layouts
- **Precise control**: When responsive layout isn't needed
- **Canvas-based UIs**: Drawing or diagram tools

## Considerations

- Not responsive to screen size changes
- Requires manual positioning calculations
- Best for fixed-size displays or embedded contexts

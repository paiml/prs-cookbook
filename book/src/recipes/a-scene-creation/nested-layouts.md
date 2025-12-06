# Nested Layouts

Compose multiple layout types for complex UI structures.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - uses `tempfile::tempdir()` |
| Idempotency | Guaranteed - same input → same output |
| Usefulness | Complex multi-panel interfaces |
| Reproducibility | Deterministic RNG seeded by recipe name |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_nested_layouts")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "nested-layouts-demo"
  title: "Nested Layouts Demo"

layout:
  type: grid
  columns: 2
  rows: 1
  gap: 16

widgets:
  - id: left-panel
    type: container
    position: { row: 0, col: 0 }
    config:
      layout:
        type: flex
        direction: column
        gap: 8
      children:
        - type: markdown
          content: "Left Panel Header"
        - type: textbox
          label: "Search"

  - id: right-panel
    type: container
    position: { row: 0, col: 1 }
    config:
      layout:
        type: flex
        direction: column
        gap: 8
      children:
        - type: markdown
          content: "Right Panel"
        - type: data_table
          columns: ["Name", "Value"]
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Nested layout structure:");
    println!("  Outer: {:?}", scene.layout.layout_type);
    println!(
        "  Grid: {}x{}",
        scene.layout.columns.unwrap_or(0),
        scene.layout.rows.unwrap_or(0)
    );

    ctx.report(&scene)?;
    Ok(())
}
```

## Key Concepts

### Container Widget

The `container` widget type allows nesting layouts:

```yaml
- id: panel
  type: container
  config:
    layout:
      type: flex        # Nested layout type
      direction: column
    children:           # Child widgets
      - type: markdown
        content: "..."
```

### Common Patterns

#### Sidebar + Main Content

```yaml
layout:
  type: grid
  columns: 2

widgets:
  - id: sidebar
    type: container
    position: { col: 0 }
    config:
      layout: { type: flex, direction: column }

  - id: main
    type: container
    position: { col: 1 }
    config:
      layout: { type: flex, direction: column }
```

#### Header + Body + Footer

```yaml
layout:
  type: flex
  direction: column

widgets:
  - id: header
    type: container
    config:
      layout: { type: flex, direction: row }

  - id: body
    type: container
    config:
      layout: { type: grid, columns: 3 }

  - id: footer
    type: container
    config:
      layout: { type: flex, direction: row }
```

## Run

```bash
cargo run --example create_nested_layouts
```

## Best Practices

- Use grid for the outer structure
- Use flex for inner content flow
- Keep nesting depth shallow (max 2-3 levels)
- Name containers descriptively (`sidebar`, `main-content`)

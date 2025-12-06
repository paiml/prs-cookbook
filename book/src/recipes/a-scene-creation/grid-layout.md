# Grid Layout

Create scenes with CSS Grid layout for dashboard-style interfaces.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - uses `tempfile::tempdir()` |
| Idempotency | Guaranteed - same input → same output |
| Usefulness | Dashboard layouts with positioned widgets |
| Reproducibility | Deterministic RNG seeded by recipe name |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_grid_layout")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "grid-demo"
  title: "Grid Layout Demo"

layout:
  type: grid
  columns: 3
  rows: 2
  gap: 16

widgets:
  - id: header
    type: markdown
    position: { row: 0, col: 0, colspan: 3 }
    config:
      content: "# Dashboard Header"

  - id: chart1
    type: bar_chart
    position: { row: 1, col: 0 }
    config:
      title: "Sales"

  - id: chart2
    type: line_chart
    position: { row: 1, col: 1 }
    config:
      title: "Trends"

  - id: gauge
    type: gauge
    position: { row: 1, col: 2 }
    config:
      value: 75
      min: 0
      max: 100
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!(
        "Grid layout: {}x{}",
        scene.layout.columns.unwrap_or(0),
        scene.layout.rows.unwrap_or(0)
    );

    ctx.report(&scene)?;
    Ok(())
}
```

## Key Concepts

### Grid Properties

| Property | Type | Description |
|----------|------|-------------|
| `columns` | u32 | Number of columns (required for validation) |
| `rows` | u32 | Number of rows |
| `gap` | u32 | Gap between cells in pixels |

### Widget Positioning

```yaml
position:
  row: 0      # Row index (0-based)
  col: 0      # Column index (0-based)
  colspan: 3  # Span multiple columns
  rowspan: 2  # Span multiple rows
```

## Run

```bash
cargo run --example create_grid_layout
```

## Validation Rules

- Grid layout **requires** `columns` to be specified
- Widget positions must be within grid bounds
- `colspan`/`rowspan` must not exceed grid dimensions

# Flex Layout

Create scenes with flexbox layout for responsive, flowing interfaces.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - uses `tempfile::tempdir()` |
| Idempotency | Guaranteed - same input → same output |
| Usefulness | Forms, lists, and flowing content |
| Reproducibility | Deterministic RNG seeded by recipe name |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_flex_layout")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "flex-demo"
  title: "Flexbox Layout Demo"

layout:
  type: flex
  direction: column
  gap: 24

widgets:
  - id: title
    type: markdown
    config:
      content: "# Flex Layout Demo"

  - id: description
    type: markdown
    config:
      content: "Widgets flow vertically with 24px gap"

  - id: input
    type: textbox
    config:
      label: "Enter text"
      placeholder: "Type here..."

  - id: button
    type: button
    config:
      label: "Submit"
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Flex layout:");
    println!("  Direction: {:?}", scene.layout.direction);
    println!("  Gap: {:?}px", scene.layout.gap);

    ctx.report(&scene)?;
    Ok(())
}
```

## Key Concepts

### Flex Properties

| Property | Type | Values | Description |
|----------|------|--------|-------------|
| `direction` | enum | `row`, `column` | Main axis direction |
| `gap` | u32 | pixels | Space between items |
| `justify` | enum | `start`, `center`, `end`, `between` | Main axis alignment |
| `align` | enum | `start`, `center`, `end`, `stretch` | Cross axis alignment |

### Direction Examples

```yaml
# Horizontal (default)
layout:
  type: flex
  direction: row

# Vertical
layout:
  type: flex
  direction: column
```

## Run

```bash
cargo run --example create_flex_layout
```

## When to Use

- **Forms**: Vertical stack of input fields
- **Toolbars**: Horizontal row of buttons
- **Lists**: Dynamic content that flows naturally
- **Simple layouts**: When you don't need precise positioning

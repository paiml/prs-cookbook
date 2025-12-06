# Custom Colors

Override theme colors with custom values.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - color overrides are independent |
| Idempotency | Guaranteed - same colors → same appearance |
| Usefulness | Brand identity, custom design |
| Reproducibility | Consistent color rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_custom_colors")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "custom-colors-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "# Custom Colors Demo"

theme:
  preset: "light"
  colors:
    primary: "#3498db"
    secondary: "#2ecc71"
    background: "#f8f9fa"
    surface: "#ffffff"
    text: "#2c3e50"
    error: "#e74c3c"
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

```yaml
theme:
  preset: "light"   # Base preset
  colors:           # Overrides
    primary: "#3498db"
    secondary: "#2ecc71"
    background: "#f8f9fa"
    surface: "#ffffff"
    text: "#2c3e50"
    error: "#e74c3c"
```

## Color Tokens

| Token | Usage |
|-------|-------|
| `primary` | Primary buttons, links, accents |
| `secondary` | Secondary buttons, badges |
| `background` | Page background |
| `surface` | Card, panel backgrounds |
| `text` | Body text, labels |
| `error` | Error messages, validation |
| `warning` | Warning alerts |
| `success` | Success messages |

## Run

```bash
cargo run --example theme_custom_colors
```

## Color Format

Colors support hex notation:

```yaml
colors:
  primary: "#3498db"      # Full hex
  secondary: "#2ec"       # Short hex
```

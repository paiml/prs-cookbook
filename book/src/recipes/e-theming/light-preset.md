# Light Preset

Apply light theme preset to scenes.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - theme is independent |
| Idempotency | Guaranteed - same config → same appearance |
| Usefulness | Standard light mode UI |
| Reproducibility | Consistent light theme rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_preset_light")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "light-theme-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "# Light Theme Demo"

theme:
  preset: "light"
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
  preset: "light"
```

## Light Theme Colors

| Token | Default Value |
|-------|---------------|
| `background` | `#ffffff` |
| `surface` | `#f8f9fa` |
| `primary` | `#3498db` |
| `text` | `#2c3e50` |
| `error` | `#e74c3c` |

## Run

```bash
cargo run --example theme_preset_light
```

## When to Use

- Well-lit environments
- Print-friendly layouts
- Default/standard appearance
- High contrast readability

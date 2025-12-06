# Dark Preset

Apply dark theme preset to scenes.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - theme is independent |
| Idempotency | Guaranteed - same config → same appearance |
| Usefulness | Dark mode UI, reduced eye strain |
| Reproducibility | Consistent dark theme rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_preset_dark")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "dark-theme-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "# Dark Theme Demo"

  - id: description
    type: markdown
    config:
      content: "This scene uses the dark theme preset"

theme:
  preset: "dark"
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
  preset: "dark"
```

## Dark Theme Colors

| Token | Default Value |
|-------|---------------|
| `background` | `#1a1a2e` |
| `surface` | `#16213e` |
| `primary` | `#0f3460` |
| `text` | `#e8e8e8` |
| `error` | `#e74c3c` |

## Run

```bash
cargo run --example theme_preset_dark
```

## When to Use

- Low-light environments
- Reducing eye strain
- OLED screen optimization
- User preference

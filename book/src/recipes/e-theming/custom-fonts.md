# Custom Fonts

Configure custom typography for headings and body text.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - font config is independent |
| Idempotency | Guaranteed - same fonts → same typography |
| Usefulness | Brand typography, readability |
| Reproducibility | Consistent font rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_custom_fonts")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "custom-fonts-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "Custom Fonts Demo"

theme:
  preset: "light"
  fonts:
    heading: "Inter, system-ui, sans-serif"
    body: "Source Sans Pro, system-ui, sans-serif"
    mono: "JetBrains Mono, Fira Code, monospace"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

```yaml
theme:
  fonts:
    heading: "Inter, system-ui, sans-serif"
    body: "Source Sans Pro, system-ui, sans-serif"
    mono: "JetBrains Mono, Fira Code, monospace"
```

## Font Tokens

| Token | Usage |
|-------|-------|
| `heading` | H1-H6 headings |
| `body` | Paragraphs, labels, inputs |
| `mono` | Code blocks, tables |

## Run

```bash
cargo run --example theme_custom_fonts
```

## Font Stack Best Practices

Always include fallbacks:

```yaml
# Good: Multiple fallbacks
heading: "Inter, system-ui, sans-serif"

# Bad: No fallbacks
heading: "CustomFont"
```

## System Font Stacks

```yaml
# Modern system fonts
fonts:
  heading: "system-ui, -apple-system, BlinkMacSystemFont, sans-serif"
  body: "system-ui, sans-serif"
  mono: "ui-monospace, SFMono-Regular, monospace"
```

# Simple Update

Connect widget change events to actions.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - bindings are independent |
| Idempotency | Guaranteed - same trigger → same action |
| Usefulness | Real-time widget updates |
| Reproducibility | Deterministic event handling |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_simple_update")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "binding-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: slider
    type: slider
    config:
      label: "Value"
      min: 0
      max: 100
      default: 50

  - id: display
    type: markdown
    config:
      content: "Current value: {{ state.slider_value }}"

bindings:
  - trigger: "slider.change"
    actions:
      - target: display
        action: refresh
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `trigger` | string | Yes | Event in format `widget.event` |
| `actions` | array | Yes | List of actions to execute |

## Action Options

| Option | Type | Description |
|--------|------|-------------|
| `target` | string | Widget ID to act on |
| `action` | string | Action type: `refresh`, `update`, `show`, `hide` |
| `input` | string | Template expression for input |
| `params` | object | Additional parameters |

## Run

```bash
cargo run --example binding_simple_update
```

## Common Patterns

### Slider → Display

```yaml
bindings:
  - trigger: "slider.change"
    actions:
      - target: display
        action: refresh
```

### Input → Validation

```yaml
bindings:
  - trigger: "email.change"
    actions:
      - target: email-error
        action: refresh
```

# Text Input

Configure text input widgets with validation and placeholder text.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - independent widget configuration |
| Idempotency | Guaranteed - same config → same widget |
| Usefulness | Forms, search boxes, user input |
| Reproducibility | Deterministic rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_text_input")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "text-input-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: username
    type: textbox
    config:
      label: "Username"
      placeholder: "Enter your username..."
      max_length: 50

  - id: email
    type: textbox
    config:
      label: "Email"
      placeholder: "user@example.com"
      max_length: 100

  - id: bio
    type: textbox
    config:
      label: "Bio"
      placeholder: "Tell us about yourself..."
      max_length: 500
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `label` | string | No | Label displayed above input |
| `placeholder` | string | No | Hint text when empty |
| `max_length` | u32 | No | Maximum character count |
| `default` | string | No | Initial value |
| `disabled` | bool | No | Disable user interaction |
| `required` | bool | No | Mark as required field |

## Run

```bash
cargo run --example widget_text_input
```

## Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `change` | `{ value: string }` | Text changed |
| `focus` | `{}` | Input focused |
| `blur` | `{ value: string }` | Input unfocused |

## Binding Example

```yaml
bindings:
  - trigger: "username.change"
    debounce_ms: 300
    actions:
      - target: validation
        action: refresh
```

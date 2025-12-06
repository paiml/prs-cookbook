# Dropdown

Configure dropdown widgets for option selection.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - independent widget configuration |
| Idempotency | Guaranteed - same config → same widget |
| Usefulness | Category selection, settings, filters |
| Reproducibility | Deterministic rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_dropdown")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "dropdown-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: country
    type: dropdown
    config:
      label: "Country"
      placeholder: "Select a country..."
      options:
        - value: "us"
          label: "United States"
        - value: "uk"
          label: "United Kingdom"
        - value: "de"
          label: "Germany"
        - value: "fr"
          label: "France"

  - id: language
    type: dropdown
    config:
      label: "Language"
      default: "en"
      options:
        - value: "en"
          label: "English"
        - value: "es"
          label: "Spanish"
        - value: "fr"
          label: "French"
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
| `options` | array | Yes | Available options |
| `label` | string | No | Label displayed above dropdown |
| `placeholder` | string | No | Hint text when no selection |
| `default` | string | No | Initial selected value |
| `disabled` | bool | No | Disable user interaction |
| `searchable` | bool | No | Enable option filtering |

## Option Structure

```yaml
options:
  - value: "key"      # Unique identifier
    label: "Display"  # User-visible text
    icon: "icon-name" # Optional icon
    disabled: false   # Disable this option
```

## Run

```bash
cargo run --example widget_dropdown
```

## Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `change` | `{ value: string, label: string }` | Selection changed |
| `open` | `{}` | Dropdown opened |
| `close` | `{}` | Dropdown closed |

## Binding Example

```yaml
bindings:
  - trigger: "country.change"
    actions:
      - target: cities-dropdown
        action: refresh
        input: "{{ country.value }}"
```

# Slider

Configure slider widgets for numeric range selection.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - independent widget configuration |
| Idempotency | Guaranteed - same config → same widget |
| Usefulness | Volume, brightness, numeric parameters |
| Reproducibility | Deterministic rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_slider")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "slider-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: volume
    type: slider
    config:
      label: "Volume"
      min: 0
      max: 100
      default: 50
      step: 1

  - id: brightness
    type: slider
    config:
      label: "Brightness"
      min: 0
      max: 100
      default: 75
      step: 5

  - id: temperature
    type: slider
    config:
      label: "Temperature (°C)"
      min: -20
      max: 40
      default: 22
      step: 0.5
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
| `min` | number | Yes | Minimum value |
| `max` | number | Yes | Maximum value |
| `step` | number | No | Step increment (default: 1) |
| `default` | number | No | Initial value |
| `label` | string | No | Label displayed above slider |
| `show_value` | bool | No | Display current value |
| `disabled` | bool | No | Disable user interaction |

## Run

```bash
cargo run --example widget_slider
```

## Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `change` | `{ value: number }` | Slider moved |
| `release` | `{ value: number }` | Drag ended |

## Binding Example

```yaml
bindings:
  - trigger: "volume.change"
    actions:
      - target: audio
        action: set_volume
        input: "{{ volume.value }}"
```

## Step Values

```yaml
# Integer steps
step: 1     # 0, 1, 2, 3...

# Fine control
step: 0.1   # 0.0, 0.1, 0.2...

# Coarse steps
step: 10    # 0, 10, 20, 30...
```

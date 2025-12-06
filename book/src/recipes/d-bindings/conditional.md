# Conditional

Execute actions conditionally based on state.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - conditions are per-binding |
| Idempotency | Guaranteed - same state → same result |
| Usefulness | Dynamic UI, visibility toggling |
| Reproducibility | Deterministic condition evaluation |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_conditional")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "conditional-binding-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: age-input
    type: slider
    config:
      label: "Age"
      min: 0
      max: 100
      default: 18

  - id: adult-content
    type: markdown
    config:
      content: "Adult content section"
      visible: false

  - id: minor-content
    type: markdown
    config:
      content: "Minor content section"
      visible: true

bindings:
  - trigger: "age-input.change"
    condition: "{{ age-input.value >= 18 }}"
    actions:
      - target: adult-content
        action: show
      - target: minor-content
        action: hide

  - trigger: "age-input.change"
    condition: "{{ age-input.value < 18 }}"
    actions:
      - target: adult-content
        action: hide
      - target: minor-content
        action: show
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

| Option | Type | Description |
|--------|------|-------------|
| `condition` | string | Template expression evaluating to boolean |

## Condition Syntax

```yaml
condition: "{{ expression }}"
```

### Comparison Operators

```yaml
condition: "{{ value >= 18 }}"      # Greater or equal
condition: "{{ value < 100 }}"      # Less than
condition: "{{ value == 'active' }}" # Equality
condition: "{{ value != null }}"    # Inequality
```

### Logical Operators

```yaml
condition: "{{ age >= 18 && verified }}"  # AND
condition: "{{ admin || moderator }}"     # OR
condition: "{{ !disabled }}"              # NOT
```

## Run

```bash
cargo run --example binding_conditional
```

## Common Patterns

### Show/Hide Based on Value

```yaml
bindings:
  - trigger: "toggle.change"
    condition: "{{ toggle.checked }}"
    actions:
      - target: panel
        action: show

  - trigger: "toggle.change"
    condition: "{{ !toggle.checked }}"
    actions:
      - target: panel
        action: hide
```

### Validate Input

```yaml
bindings:
  - trigger: "email.change"
    condition: "{{ email.value | is_email }}"
    actions:
      - target: submit-btn
        action: enable

  - trigger: "email.change"
    condition: "{{ !(email.value | is_email) }}"
    actions:
      - target: submit-btn
        action: disable
```

### Role-Based Display

```yaml
bindings:
  - trigger: "scene.load"
    condition: "{{ user.role == 'admin' }}"
    actions:
      - target: admin-panel
        action: show
```

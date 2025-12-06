# Minimal

Demonstrate principle of least privilege with minimal permissions.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - no external dependencies |
| Idempotency | Guaranteed - pure client-side |
| Usefulness | Offline tools, calculators |
| Reproducibility | No external state |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("permission_minimal")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "minimal-permission-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: calculator
    type: markdown
    config:
      content: "Offline Calculator - No permissions required"

  - id: input-a
    type: textbox
    config:
      label: "Number A"

  - id: input-b
    type: textbox
    config:
      label: "Number B"

  - id: result
    type: markdown
    config:
      content: "Result: {{ state.result }}"

permissions:
  network: []
  filesystem: []
  clipboard: false
  camera: false
  microphone: false
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

```yaml
permissions:
  network: []       # No network access
  filesystem: []    # No file access
  clipboard: false  # No clipboard
  camera: false     # No camera
  microphone: false # No microphone
```

## Run

```bash
cargo run --example permission_minimal
```

## Use Cases

- **Calculators**: Pure computation
- **Converters**: Unit/format conversion
- **Validators**: Input validation
- **Editors**: Text/data editing
- **Games**: Simple games

## Benefits

| Benefit | Description |
|---------|-------------|
| Security | No attack surface |
| Privacy | No data leakage |
| Offline | Works without network |
| Trust | Users can verify safety |

## Principle of Least Privilege

Only request permissions that are **actually needed**:

```yaml
# Good: No permissions for offline tool
permissions:
  network: []
  filesystem: []

# Bad: Unnecessary permissions
permissions:
  network:
    - "https://*"  # Why does a calculator need network?
```

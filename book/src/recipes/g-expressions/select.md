# Select

Use `select()` transform to extract fields from data.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - doesn't mutate source |
| Idempotency | Guaranteed - same field → same value |
| Usefulness | Field extraction, data access |
| Reproducibility | Deterministic selection |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_select")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-select-demo"

resources:
  models:
    classifier:
      type: apr
      source: "./models/classifier.apr"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: confidence
    type: gauge
    config:
      title: "Confidence"
      value: "{{ inference.classifier | select('confidence') | percentage }}"
      min: 0
      max: 100

  - id: label
    type: markdown
    config:
      content: "Predicted: {{ inference.classifier | select('label') }}"

  - id: scores
    type: data_table
    config:
      title: "All Scores"
      data: "{{ inference.classifier | select('scores') }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Syntax

```yaml
"{{ source | select('field_name') }}"
```

## Nested Selection

```yaml
# Deep field access
"{{ data | select('user.profile.name') }}"

# Array index
"{{ data | select('items[0]') }}"
```

## Run

```bash
cargo run --example expression_select
```

## Use Cases

### Model Output

```yaml
value: "{{ inference.sentiment | select('score') }}"
```

### Dataset Column

```yaml
data: "{{ datasets.users | select('name, email') }}"
```

### State Access

```yaml
content: "{{ state | select('user.name') }}"
```

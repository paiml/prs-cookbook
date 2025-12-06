# Debounced

Debounce rapid input changes to reduce action frequency.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - debounce is per-binding |
| Idempotency | Guaranteed - last value wins |
| Usefulness | Search, validation, API calls |
| Reproducibility | Deterministic debounce timing |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_debounced")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "debounce-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: search
    type: textbox
    config:
      label: "Search"
      placeholder: "Type to search..."

  - id: results
    type: data_table
    config:
      title: "Search Results"

bindings:
  - trigger: "search.change"
    debounce_ms: 300
    actions:
      - target: results
        action: refresh
        input: "{{ search.value }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `debounce_ms` | u32 | 0 | Milliseconds to wait before executing |

## How Debounce Works

```
User types: H-e-l-l-o
Events:     [H][e][l][l][o]
            |  |  |  |  |
            ↓  ↓  ↓  ↓  ↓
Debounce:   [cancel][cancel][cancel][cancel][execute after 300ms]
```

Only the final value triggers the action after the debounce period.

## Run

```bash
cargo run --example binding_debounced
```

## Recommended Debounce Times

| Use Case | Debounce (ms) |
|----------|---------------|
| Search autocomplete | 200-300 |
| Form validation | 150-250 |
| API calls | 300-500 |
| Model inference | 500-1000 |

## Without Debounce

Without debounce, every keystroke triggers an action:

```yaml
# DON'T: Causes excessive API calls
bindings:
  - trigger: "search.change"
    actions:
      - target: api.search
```

## With Debounce

```yaml
# DO: Waits for user to stop typing
bindings:
  - trigger: "search.change"
    debounce_ms: 300
    actions:
      - target: api.search
```

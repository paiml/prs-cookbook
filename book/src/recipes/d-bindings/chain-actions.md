# Chain Actions

Chain multiple actions from a single trigger event.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - action chains are independent |
| Idempotency | Guaranteed - same sequence every time |
| Usefulness | Complex workflows, multi-widget updates |
| Reproducibility | Deterministic action ordering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_chain_actions")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "chain-actions-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: submit-btn
    type: button
    config:
      label: "Submit"

  - id: status
    type: markdown
    config:
      content: "Ready"

  - id: results
    type: data_table
    config:
      title: "Results"

  - id: chart
    type: bar_chart
    config:
      title: "Visualization"

bindings:
  - trigger: "submit-btn.click"
    actions:
      - target: status
        action: update
        params:
          content: "Processing..."
      - target: results
        action: refresh
      - target: chart
        action: refresh
      - target: status
        action: update
        params:
          content: "Complete"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

Actions execute in sequence:

```yaml
actions:
  - target: widget-1   # Step 1
    action: action-1
  - target: widget-2   # Step 2
    action: action-2
  - target: widget-3   # Step 3
    action: action-3
```

## Run

```bash
cargo run --example binding_chain_actions
```

## Execution Order

Actions execute sequentially, not in parallel:

```
Trigger → Action 1 → Action 2 → Action 3 → Done
```

## Common Patterns

### Status → Process → Status

```yaml
actions:
  - target: status
    action: update
    params: { content: "Loading..." }
  - target: data
    action: refresh
  - target: status
    action: update
    params: { content: "Done" }
```

### Refresh Multiple Widgets

```yaml
actions:
  - target: table
    action: refresh
  - target: chart
    action: refresh
  - target: summary
    action: refresh
```

### Cascade Updates

```yaml
# Dropdown changes filter → table → chart
actions:
  - target: table
    action: refresh
    input: "{{ filter.value }}"
  - target: chart
    action: refresh
```

## Action Types

| Action | Description |
|--------|-------------|
| `refresh` | Re-render with current data |
| `update` | Update config params |
| `show` | Make visible |
| `hide` | Make invisible |
| `reset` | Reset to default state |

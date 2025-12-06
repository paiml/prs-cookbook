# Model Inference

Trigger model inference on user input.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - inference is per-model |
| Idempotency | Guaranteed - same input → same output |
| Usefulness | Real-time ML predictions |
| Reproducibility | Deterministic model execution |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_model_inference")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "inference-binding-demo"

resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: text-input
    type: textbox
    config:
      label: "Enter text for analysis"

  - id: sentiment-gauge
    type: gauge
    config:
      title: "Sentiment"
      min: -1
      max: 1
      value: "{{ inference.sentiment.score }}"

  - id: confidence
    type: markdown
    config:
      content: "Confidence: {{ inference.sentiment.confidence | percentage }}"

bindings:
  - trigger: "text-input.change"
    debounce_ms: 500
    actions:
      - target: inference.sentiment
        input: "{{ text-input.value }}"
      - target: sentiment-gauge
        action: refresh
      - target: confidence
        action: refresh
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

### Inference Action

```yaml
actions:
  - target: inference.model-name  # models.<name>
    input: "{{ widget.value }}"   # Input data
```

### Accessing Results

```yaml
# In widget config
value: "{{ inference.sentiment.score }}"
content: "{{ inference.sentiment.confidence }}"
```

## Run

```bash
cargo run --example binding_model_inference
```

## Inference Flow

```
1. User types text
2. Debounce waits 500ms
3. Inference action runs model
4. Results stored in inference.sentiment.*
5. Dependent widgets refresh
```

## Best Practices

### Always Debounce

```yaml
bindings:
  - trigger: "input.change"
    debounce_ms: 500  # Prevent excessive inference
    actions:
      - target: inference.model
```

### Chain with Refresh

```yaml
actions:
  - target: inference.sentiment   # Run inference
  - target: result-gauge          # Then refresh displays
    action: refresh
```

### Display Loading State

```yaml
actions:
  - target: status
    action: update
    params: { content: "Analyzing..." }
  - target: inference.sentiment
  - target: status
    action: update
    params: { content: "Done" }
```

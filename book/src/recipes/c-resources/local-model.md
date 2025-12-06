# Local Model

Reference local `.apr` model files in scene resources.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - model loaded independently |
| Idempotency | Guaranteed - same file → same model |
| Usefulness | Local ML inference without network |
| Reproducibility | Deterministic model loading |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("resource_local_model")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "local-model-demo"

resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"

layout:
  type: flex
  direction: column

widgets:
  - id: input
    type: textbox
    config:
      label: "Text to analyze"

  - id: result
    type: gauge
    config:
      title: "Sentiment Score"
      min: 0
      max: 100

bindings:
  - trigger: "input.change"
    debounce_ms: 300
    actions:
      - target: inference.sentiment
        input: "{{ input.value }}"
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
| `type` | enum | Yes | Model format: `apr`, `onnx` |
| `source` | string | Yes | Path to model file |
| `hash` | string | No | Optional BLAKE3 hash |

## Model Types

| Type | Description |
|------|-------------|
| `apr` | APR format (Presentar native) |
| `onnx` | ONNX format for interop |

## Run

```bash
cargo run --example resource_local_model
```

## Directory Structure

```
my-scene/
├── scene.prs
└── models/
    └── sentiment.apr
```

## Inference Binding

```yaml
bindings:
  - trigger: "input.change"
    actions:
      - target: inference.sentiment  # models.<name>
        input: "{{ input.value }}"
```

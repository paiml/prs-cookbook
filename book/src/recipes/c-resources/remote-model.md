# Remote Model

Reference remote models with BLAKE3 hash verification.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - model downloaded independently |
| Idempotency | Guaranteed - hash ensures same model |
| Usefulness | CDN-hosted models, shared assets |
| Reproducibility | BLAKE3 hash verification |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("resource_remote_model")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "remote-model-demo"

resources:
  models:
    classifier:
      type: apr
      source: "https://registry.paiml.com/models/classifier.apr"
      hash: "blake3:a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
      size_bytes: 45000000

permissions:
  network:
    - "https://registry.paiml.com/*"

layout:
  type: flex

widgets:
  - id: status
    type: markdown
    config:
      content: "Model loaded with verified hash"
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
| `source` | string | Yes | HTTPS URL to model |
| `hash` | string | **Yes** | BLAKE3 hash (required for remote) |
| `size_bytes` | u64 | No | Expected file size |

## Hash Format

```yaml
hash: "blake3:a1b2c3d4e5f6..."
```

The hash is a 64-character hex string prefixed with `blake3:`.

## Generate Hash

```bash
b3sum model.apr
# Output: a1b2c3d4e5f6... model.apr
```

## Run

```bash
cargo run --example resource_remote_model
```

## Network Permission

Remote models require network permissions:

```yaml
permissions:
  network:
    - "https://registry.paiml.com/*"
```

## Validation

Remote models without hash will fail validation:

```
Error: Remote resource 'classifier' missing required hash
```

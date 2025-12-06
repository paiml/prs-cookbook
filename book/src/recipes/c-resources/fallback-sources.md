# Fallback Sources

Configure source fallback chains for resilient resource loading.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - fallback chain is independent |
| Idempotency | Guaranteed - same hash regardless of source |
| Usefulness | Resilient loading, offline support |
| Reproducibility | Hash verification ensures same content |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("resource_fallback_sources")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "fallback-sources-demo"

resources:
  models:
    classifier:
      type: apr
      sources:
        - "https://primary.cdn.example.com/model.apr"
        - "https://backup.cdn.example.com/model.apr"
        - "./local-cache/model.apr"
      hash: "blake3:a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"

permissions:
  network:
    - "https://primary.cdn.example.com/*"
    - "https://backup.cdn.example.com/*"

layout:
  type: flex

widgets:
  - id: status
    type: markdown
    config:
      content: "Model with fallback sources configured"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration

Use `sources` (plural) instead of `source`:

```yaml
models:
  classifier:
    type: apr
    sources:                    # Array of sources
      - "https://primary.cdn.example.com/model.apr"
      - "https://backup.cdn.example.com/model.apr"
      - "./local-cache/model.apr"
    hash: "blake3:..."          # Same hash for all
```

## Fallback Order

1. **Primary**: First source in array
2. **Secondary**: Second source (on primary failure)
3. **Tertiary**: Third source (on secondary failure)
4. **Local**: Local cache as final fallback

## Run

```bash
cargo run --example resource_fallback_sources
```

## Use Cases

### CDN Failover

```yaml
sources:
  - "https://us-east.cdn.example.com/model.apr"
  - "https://eu-west.cdn.example.com/model.apr"
```

### Offline Support

```yaml
sources:
  - "https://cdn.example.com/model.apr"
  - "./cache/model.apr"  # Local fallback
```

### Development Override

```yaml
sources:
  - "./dev-models/model.apr"      # Dev version first
  - "https://cdn.example.com/model.apr"
```

## Hash Verification

The same hash is used regardless of which source succeeds:

```yaml
hash: "blake3:a1b2c3..."  # Verified for any source
```

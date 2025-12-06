# Network

Grant network access to specific domains.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - network permissions are per-scene |
| Idempotency | Guaranteed - same permissions → same access |
| Usefulness | API calls, CDN resources |
| Reproducibility | Predictable network access |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("permission_network")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "network-permission-demo"

resources:
  models:
    api_model:
      type: apr
      source: "https://api.example.com/model.apr"
      hash: "blake3:abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"

permissions:
  network:
    - "https://api.example.com/*"
    - "https://cdn.example.com/*"
  clipboard: false
  camera: false

layout:
  type: flex

widgets:
  - id: status
    type: markdown
    config:
      content: "Network access granted to api.example.com"
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
  network:
    - "https://api.example.com/*"
    - "https://cdn.example.com/*"
```

## Pattern Syntax

| Pattern | Matches |
|---------|---------|
| `https://example.com/*` | All paths on domain |
| `https://*.example.com/*` | All subdomains |
| `https://api.example.com/v1/*` | Specific path prefix |

## Run

```bash
cargo run --example permission_network
```

## Security Best Practices

```yaml
# Good: Specific domain
network:
  - "https://api.example.com/*"

# Bad: Too broad
network:
  - "https://*"
```

## Remote Resources

Network permissions are **required** for remote resources:

```yaml
resources:
  models:
    classifier:
      source: "https://cdn.example.com/model.apr"
      hash: "blake3:..."  # Required

permissions:
  network:
    - "https://cdn.example.com/*"  # Must match source
```

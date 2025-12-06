# Filesystem

Grant filesystem access to specific paths.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - filesystem permissions are per-scene |
| Idempotency | Guaranteed - same permissions → same access |
| Usefulness | Local data files, output directories |
| Reproducibility | Predictable file access |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("permission_filesystem")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "filesystem-permission-demo"

resources:
  datasets:
    data:
      type: csv
      source: "./data/input.csv"

permissions:
  filesystem:
    - "./data/*"
    - "./output/*"
  network: []
  clipboard: false

layout:
  type: flex

widgets:
  - id: file-browser
    type: file_browser
    config:
      root: "./data"
      allowed_extensions:
        - ".csv"
        - ".json"
        - ".parquet"
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
  filesystem:
    - "./data/*"    # Read access
    - "./output/*"  # Write access
```

## Pattern Syntax

| Pattern | Matches |
|---------|---------|
| `./data/*` | All files in ./data |
| `./data/**/*` | Recursive subdirectories |
| `./data/*.csv` | Only CSV files |

## Run

```bash
cargo run --example permission_filesystem
```

## Security Best Practices

```yaml
# Good: Limited to specific directories
filesystem:
  - "./data/*"
  - "./output/*"

# Bad: Too broad
filesystem:
  - "./*"
  - "/*"
```

## Read vs Write

Pattern permissions allow both read and write. Use separate directories for input and output:

```yaml
filesystem:
  - "./input/*"   # Source data (read)
  - "./output/*"  # Results (write)
```

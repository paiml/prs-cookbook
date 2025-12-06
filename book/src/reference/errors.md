# Error Handling

## Error Types

```rust
pub enum Error {
    SceneNotFound(String),
    InvalidYaml(serde_yaml::Error),
    ValidationFailed(String),
    DuplicateWidgetId(String),
    InvalidNameFormat(String),
    RemoteMissingHash(String),
    Io(std::io::Error),
}
```

## Result Type

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## Example Usage

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;
    Ok(())
}
```

## Validation Errors

| Error | Cause |
|-------|-------|
| `InvalidNameFormat` | Name not kebab-case |
| `DuplicateWidgetId` | Two widgets with same ID |
| `ValidationFailed` | Grid missing columns |
| `RemoteMissingHash` | Remote resource without hash |

# Scene Structure

## Scene Type

The `Scene` struct represents a complete PRS document:

```rust
pub struct Scene {
    pub prs_version: String,
    pub metadata: Metadata,
    pub resources: Resources,
    pub layout: Layout,
    pub widgets: Vec<Widget>,
    pub bindings: Vec<Binding>,
    pub theme: Option<Theme>,
    pub permissions: Permissions,
}
```

## Parsing and Validation

```rust
use prs_cookbook::prelude::*;

// Parse from YAML
let scene = Scene::from_yaml(yaml_str)?;

// Validate constraints
scene.validate()?;

// Serialize back to YAML
let yaml = scene.to_yaml()?;
```

## Validation Rules

| Rule | Description |
|------|-------------|
| Version format | Must match `X.Y` pattern |
| Metadata name | Must be kebab-case |
| Widget IDs | Must be unique |
| Grid layout | Requires `columns` field |
| Absolute layout | Requires `width` and `height` |
| Remote resources | Require BLAKE3 hash |

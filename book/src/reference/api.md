# API Documentation

## Core Types

### Scene

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

impl Scene {
    pub fn from_yaml(yaml: &str) -> Result<Self>;
    pub fn to_yaml(&self) -> Result<String>;
    pub fn validate(&self) -> Result<()>;
}
```

### RecipeContext

```rust
pub struct RecipeContext {
    // ... internal fields
}

impl RecipeContext {
    pub fn new(name: &str) -> Result<Self>;
    pub fn name(&self) -> &str;
    pub fn temp_dir(&self) -> &Path;
    pub fn path(&self, filename: &str) -> PathBuf;
    pub fn rng(&mut self) -> &mut StdRng;
    pub fn elapsed(&self) -> Duration;
    pub fn record_metric(&mut self, name: &str, value: i64);
    pub fn report(&self, scene: &Scene) -> Result<()>;
}
```

## Prelude

```rust
use prs_cookbook::prelude::*;

// Includes:
// - Scene, Metadata, Layout, Widget, etc.
// - RecipeContext, RecipeMetadata, MetricValue
// - hash_name_to_seed, generate_test_data, generate_test_scene
// - Error, Result
```

## Full Documentation

```bash
cargo doc --open
```

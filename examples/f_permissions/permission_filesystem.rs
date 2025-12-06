//! # Recipe: Filesystem Permissions
//!
//! **Category**: Permissions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Grant filesystem access to specific paths.
//!
//! ## Run Command
//! ```bash
//! cargo run --example permission_filesystem
//! ```

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

    println!("Filesystem permissions:");
    for pattern in &scene.permissions.filesystem {
        println!("  Allow: {}", pattern);
    }

    ctx.report(&scene)?;
    Ok(())
}

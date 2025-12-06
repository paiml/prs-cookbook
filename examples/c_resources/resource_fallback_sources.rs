//! # Recipe: Fallback Sources
//!
//! **Category**: Resources
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Configure source fallback chain for resilient resource loading.
//!
//! ## Run Command
//! ```bash
//! cargo run --example resource_fallback_sources
//! ```

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

    println!("Fallback source chain:");
    for (name, model) in &scene.resources.models {
        println!("  {}:", name);
        for (i, source) in model.source.all().iter().enumerate() {
            let priority = if i == 0 { "primary" } else { "fallback" };
            println!("    [{}] {}", priority, source);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

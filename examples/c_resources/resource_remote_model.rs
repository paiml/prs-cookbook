//! # Recipe: Remote Model Resource
//!
//! **Category**: Resources
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Reference remote model with BLAKE3 hash verification.
//!
//! ## Run Command
//! ```bash
//! cargo run --example resource_remote_model
//! ```

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

    println!("Remote model resources:");
    for (name, model) in &scene.resources.models {
        println!("  {}: {:?}", name, model.model_type);
        println!("    Source: {}", model.source.primary());
        println!("    Hash: {:?}", model.hash);
        println!("    Size: {:?} bytes", model.size_bytes);
    }

    ctx.report(&scene)?;
    Ok(())
}

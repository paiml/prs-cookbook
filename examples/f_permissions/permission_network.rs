//! # Recipe: Network Permissions
//!
//! **Category**: Permissions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Grant network access to specific domains.
//!
//! ## Run Command
//! ```bash
//! cargo run --example permission_network
//! ```

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

    println!("Network permissions:");
    for pattern in &scene.permissions.network {
        println!("  Allow: {}", pattern);
    }
    println!("Clipboard: {}", scene.permissions.clipboard);
    println!("Camera: {}", scene.permissions.camera);

    ctx.report(&scene)?;
    Ok(())
}

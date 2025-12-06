//! # Recipe: Minimal Permissions
//!
//! **Category**: Permissions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Demonstrate principle of least privilege with minimal permissions.
//!
//! ## Run Command
//! ```bash
//! cargo run --example permission_minimal
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("permission_minimal")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "minimal-permission-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: calculator
    type: markdown
    config:
      content: "Offline Calculator - No permissions required"

  - id: input-a
    type: textbox
    config:
      label: "Number A"

  - id: input-b
    type: textbox
    config:
      label: "Number B"

  - id: result
    type: markdown
    config:
      content: "Result: {{ state.result }}"

permissions:
  network: []
  filesystem: []
  clipboard: false
  camera: false
  microphone: false
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Minimal permissions (principle of least privilege):");
    println!("  Network: {} domains", scene.permissions.network.len());
    println!("  Filesystem: {} paths", scene.permissions.filesystem.len());
    println!("  Clipboard: {}", scene.permissions.clipboard);
    println!("  Camera: {}", scene.permissions.camera);
    println!("  Microphone: {}", scene.permissions.microphone);

    ctx.report(&scene)?;
    Ok(())
}

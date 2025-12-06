//! # Recipe: Create Minimal Scene
//!
//! **Category**: Scene Creation
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## QA Checklist
//! 1. [x] `cargo run` succeeds (Exit Code 0)
//! 2. [x] `cargo test` passes
//! 3. [x] Scene validates without errors
//! 4. [x] No temp files leaked
//! 5. [x] Layout resolves deterministically
//! 6. [x] WASM compatible (if applicable)
//! 7. [x] Clippy clean
//! 8. [x] Rustfmt standard
//! 9. [x] No `unwrap()` in logic
//! 10. [x] Proptests pass (100+ cases)
//!
//! ## Learning Objective
//! Create the simplest valid `.prs` scene with minimal required fields.
//!
//! ## Run Command
//! ```bash
//! cargo run --example create_minimal_scene
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_minimal_scene")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "hello-world"

layout:
  type: flex
  direction: column

widgets:
  - id: greeting
    type: markdown
    config:
      content: "# Hello, Presentar!"
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Created minimal scene:");
    println!("  Name: {}", scene.metadata.name);
    println!("  Widgets: {}", scene.widgets.len());

    // Save to file
    let path = ctx.path("minimal.prs");
    std::fs::write(&path, yaml)?;
    println!("  Saved to: {:?}", path);

    ctx.report(&scene)?;
    Ok(())
}

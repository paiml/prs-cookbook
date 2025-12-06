//! # Recipe: Simple Binding Update
//!
//! **Category**: Bindings
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Connect widget change events to actions.
//!
//! ## Run Command
//! ```bash
//! cargo run --example binding_simple_update
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_simple_update")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "binding-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: slider
    type: slider
    config:
      label: "Value"
      min: 0
      max: 100
      default: 50

  - id: display
    type: markdown
    config:
      content: "Current value: {{ state.slider_value }}"

bindings:
  - trigger: "slider.change"
    actions:
      - target: display
        action: refresh
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Bindings:");
    for binding in &scene.bindings {
        println!("  Trigger: {}", binding.trigger);
        for action in &binding.actions {
            println!("    -> {} (action: {:?})", action.target, action.action);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

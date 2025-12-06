//! # Recipe: Conditional Binding
//!
//! **Category**: Bindings
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Execute actions conditionally based on state.
//!
//! ## Run Command
//! ```bash
//! cargo run --example binding_conditional
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_conditional")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "conditional-binding-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: age-input
    type: slider
    config:
      label: "Age"
      min: 0
      max: 100
      default: 18

  - id: adult-content
    type: markdown
    config:
      content: "Adult content section"
      visible: false

  - id: minor-content
    type: markdown
    config:
      content: "Minor content section"
      visible: true

bindings:
  - trigger: "age-input.change"
    condition: "{{ age-input.value >= 18 }}"
    actions:
      - target: adult-content
        action: show
      - target: minor-content
        action: hide

  - trigger: "age-input.change"
    condition: "{{ age-input.value < 18 }}"
    actions:
      - target: adult-content
        action: hide
      - target: minor-content
        action: show
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Conditional bindings:");
    for binding in &scene.bindings {
        println!("  Trigger: {}", binding.trigger);
        if let Some(cond) = &binding.condition {
            println!("  Condition: {}", cond);
        }
        println!("  Actions: {}", binding.actions.len());
    }

    ctx.report(&scene)?;
    Ok(())
}

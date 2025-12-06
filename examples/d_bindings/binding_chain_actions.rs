//! # Recipe: Chain Actions Binding
//!
//! **Category**: Bindings
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Chain multiple actions from a single trigger event.
//!
//! ## Run Command
//! ```bash
//! cargo run --example binding_chain_actions
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_chain_actions")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "chain-actions-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: submit-btn
    type: button
    config:
      label: "Submit"

  - id: status
    type: markdown
    config:
      content: "Ready"

  - id: results
    type: data_table
    config:
      title: "Results"

  - id: chart
    type: bar_chart
    config:
      title: "Visualization"

bindings:
  - trigger: "submit-btn.click"
    actions:
      - target: status
        action: update
        params:
          content: "Processing..."
      - target: results
        action: refresh
      - target: chart
        action: refresh
      - target: status
        action: update
        params:
          content: "Complete"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Chained actions:");
    for binding in &scene.bindings {
        println!("  Trigger: {}", binding.trigger);
        println!("  Action chain ({} steps):", binding.actions.len());
        for (i, action) in binding.actions.iter().enumerate() {
            println!("    {}. {} -> {:?}", i + 1, action.target, action.action);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

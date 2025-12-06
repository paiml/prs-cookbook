//! # Recipe: Debounced Binding
//!
//! **Category**: Bindings
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Debounce rapid input changes to reduce action frequency.
//!
//! ## Run Command
//! ```bash
//! cargo run --example binding_debounced
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_debounced")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "debounce-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: search
    type: textbox
    config:
      label: "Search"
      placeholder: "Type to search..."

  - id: results
    type: data_table
    config:
      title: "Search Results"

bindings:
  - trigger: "search.change"
    debounce_ms: 300
    actions:
      - target: results
        action: refresh
        input: "{{ search.value }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Debounced bindings:");
    for binding in &scene.bindings {
        println!("  Trigger: {}", binding.trigger);
        println!("  Debounce: {:?}ms", binding.debounce_ms);
    }

    ctx.report(&scene)?;
    Ok(())
}

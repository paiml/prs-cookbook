//! # Recipe: Local Model Resource
//!
//! **Category**: Resources
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Reference local `.apr` model file in scene resources.
//!
//! ## Run Command
//! ```bash
//! cargo run --example resource_local_model
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("resource_local_model")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "local-model-demo"

resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"

layout:
  type: flex
  direction: column

widgets:
  - id: input
    type: textbox
    config:
      label: "Text to analyze"

  - id: result
    type: gauge
    config:
      title: "Sentiment Score"
      min: 0
      max: 100

bindings:
  - trigger: "input.change"
    debounce_ms: 300
    actions:
      - target: inference.sentiment
        input: "{{ input.value }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Local model resources:");
    for (name, model) in &scene.resources.models {
        println!(
            "  {}: {:?} from {}",
            name,
            model.model_type,
            model.source.primary()
        );
    }

    ctx.report(&scene)?;
    Ok(())
}

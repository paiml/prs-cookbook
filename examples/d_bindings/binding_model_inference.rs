//! # Recipe: Model Inference Binding
//!
//! **Category**: Bindings
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Trigger model inference on user input.
//!
//! ## Run Command
//! ```bash
//! cargo run --example binding_model_inference
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_model_inference")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "inference-binding-demo"

resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: text-input
    type: textbox
    config:
      label: "Enter text for analysis"

  - id: sentiment-gauge
    type: gauge
    config:
      title: "Sentiment"
      min: -1
      max: 1
      value: "{{ inference.sentiment.score }}"

  - id: confidence
    type: markdown
    config:
      content: "Confidence: {{ inference.sentiment.confidence | percentage }}"

bindings:
  - trigger: "text-input.change"
    debounce_ms: 500
    actions:
      - target: inference.sentiment
        input: "{{ text-input.value }}"
      - target: sentiment-gauge
        action: refresh
      - target: confidence
        action: refresh
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Model inference binding:");
    for binding in &scene.bindings {
        println!("  Trigger: {}", binding.trigger);
        println!("  Actions: {}", binding.actions.len());
        for action in &binding.actions {
            println!("    -> {}", action.target);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

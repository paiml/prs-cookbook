//! # Recipe: Select Expression
//!
//! **Category**: Expressions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Use `select()` transform to extract fields from data.
//!
//! ## Run Command
//! ```bash
//! cargo run --example expression_select
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_select")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-select-demo"

resources:
  models:
    classifier:
      type: apr
      source: "./models/classifier.apr"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: confidence
    type: gauge
    config:
      title: "Confidence"
      value: "{{ inference.classifier | select('confidence') | percentage }}"
      min: 0
      max: 100

  - id: label
    type: markdown
    config:
      content: "Predicted: {{ inference.classifier | select('label') }}"

  - id: scores
    type: data_table
    config:
      title: "All Scores"
      data: "{{ inference.classifier | select('scores') }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Select expressions in widgets:");
    for widget in &scene.widgets {
        if let Some(value) = widget.config.get("value") {
            println!("  {}: {:?}", widget.id, value);
        }
        if let Some(content) = widget.config.get("content") {
            if content
                .as_str()
                .map(|s| s.contains("select"))
                .unwrap_or(false)
            {
                println!("  {}: {:?}", widget.id, content);
            }
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

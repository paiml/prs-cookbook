//! # Recipe: Format Expressions
//!
//! **Category**: Expressions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Use `format()` and `join()` transforms for string formatting.
//!
//! ## Run Command
//! ```bash
//! cargo run --example expression_format
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_format")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-format-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: currency
    type: markdown
    config:
      content: "Price: {{ state.price | format('$,.2f') }}"

  - id: percentage
    type: markdown
    config:
      content: "Discount: {{ state.discount | percentage }}%"

  - id: date-format
    type: markdown
    config:
      content: "Date: {{ state.timestamp | date('MMMM D, YYYY') }}"

  - id: list-join
    type: markdown
    config:
      content: "Tags: {{ state.tags | join(', ') }}"

  - id: truncate
    type: markdown
    config:
      content: "{{ state.description | truncate(100) }}..."

  - id: uppercase
    type: markdown
    config:
      content: "{{ state.title | uppercase }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Format expressions:");
    for widget in &scene.widgets {
        if let Some(content) = widget.config.get("content") {
            println!("  {}: {:?}", widget.id, content);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

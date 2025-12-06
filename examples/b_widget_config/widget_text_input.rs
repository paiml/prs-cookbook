//! # Recipe: Text Input Widget
//!
//! **Category**: Widget Configuration
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Configure text input widget with validation and placeholder.
//!
//! ## Run Command
//! ```bash
//! cargo run --example widget_text_input
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_text_input")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "text-input-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: username
    type: textbox
    config:
      label: "Username"
      placeholder: "Enter your username..."
      max_length: 50

  - id: email
    type: textbox
    config:
      label: "Email"
      placeholder: "user@example.com"
      max_length: 100

  - id: bio
    type: textbox
    config:
      label: "Bio"
      placeholder: "Tell us about yourself..."
      max_length: 500
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Text input widgets:");
    for widget in &scene.widgets {
        if let Some(label) = widget.config.get("label") {
            println!("  {}: {:?}", widget.id, label);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

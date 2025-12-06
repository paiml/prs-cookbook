//! # Recipe: Create Flex Layout
//!
//! **Category**: Scene Creation
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Create scene with flexbox layout for responsive UIs.
//!
//! ## Run Command
//! ```bash
//! cargo run --example create_flex_layout
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_flex_layout")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "flex-demo"
  title: "Flexbox Layout Demo"

layout:
  type: flex
  direction: column
  gap: 24

widgets:
  - id: title
    type: markdown
    config:
      content: "# Flex Layout Demo"

  - id: description
    type: markdown
    config:
      content: "Widgets flow vertically with 24px gap"

  - id: input
    type: textbox
    config:
      label: "Enter text"
      placeholder: "Type here..."

  - id: button
    type: button
    config:
      label: "Submit"
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Flex layout:");
    println!("  Direction: {:?}", scene.layout.direction);
    println!("  Gap: {:?}px", scene.layout.gap);
    println!("  Widgets: {}", scene.widgets.len());

    ctx.report(&scene)?;
    Ok(())
}

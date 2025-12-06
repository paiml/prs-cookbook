//! # Recipe: Create Nested Layouts
//!
//! **Category**: Scene Creation
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Compose multiple layout types for complex UI structures.
//!
//! ## Run Command
//! ```bash
//! cargo run --example create_nested_layouts
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_nested_layouts")?;

    // Nested layouts use a grid as the outer container
    // with flex containers inside grid cells
    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "nested-layouts-demo"
  title: "Nested Layouts Demo"

layout:
  type: grid
  columns: 2
  rows: 1
  gap: 16

widgets:
  - id: left-panel
    type: container
    position: { row: 0, col: 0 }
    config:
      layout:
        type: flex
        direction: column
        gap: 8
      children:
        - type: markdown
          content: "Left Panel Header"
        - type: textbox
          label: "Search"

  - id: right-panel
    type: container
    position: { row: 0, col: 1 }
    config:
      layout:
        type: flex
        direction: column
        gap: 8
      children:
        - type: markdown
          content: "Right Panel"
        - type: data_table
          columns: ["Name", "Value"]
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Nested layout structure:");
    println!("  Outer: {:?}", scene.layout.layout_type);
    println!(
        "  Grid: {}x{}",
        scene.layout.columns.unwrap_or(0),
        scene.layout.rows.unwrap_or(0)
    );
    println!("  Widgets (containers): {}", scene.widgets.len());

    ctx.report(&scene)?;
    Ok(())
}

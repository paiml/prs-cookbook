//! # Recipe: Create Grid Layout
//!
//! **Category**: Scene Creation
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Create scene with CSS Grid layout and positioned widgets.
//!
//! ## Run Command
//! ```bash
//! cargo run --example create_grid_layout
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_grid_layout")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "grid-demo"
  title: "Grid Layout Demo"

layout:
  type: grid
  columns: 3
  rows: 2
  gap: 16

widgets:
  - id: header
    type: markdown
    position: { row: 0, col: 0, colspan: 3 }
    config:
      content: "# Dashboard Header"

  - id: chart1
    type: bar_chart
    position: { row: 1, col: 0 }
    config:
      title: "Sales"

  - id: chart2
    type: line_chart
    position: { row: 1, col: 1 }
    config:
      title: "Trends"

  - id: gauge
    type: gauge
    position: { row: 1, col: 2 }
    config:
      value: 75
      min: 0
      max: 100
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!(
        "Grid layout: {}x{}",
        scene.layout.columns.unwrap_or(0),
        scene.layout.rows.unwrap_or(0)
    );
    println!("Widgets: {}", scene.widgets.len());

    for widget in &scene.widgets {
        if let Some(pos) = &widget.position {
            println!("  {} at row={:?}, col={:?}", widget.id, pos.row, pos.col);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

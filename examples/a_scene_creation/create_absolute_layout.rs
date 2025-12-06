//! # Recipe: Create Absolute Layout
//!
//! **Category**: Scene Creation
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Create scene with absolute positioning for pixel-perfect layouts.
//!
//! ## Run Command
//! ```bash
//! cargo run --example create_absolute_layout
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_absolute_layout")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "absolute-demo"
  title: "Absolute Layout Demo"

layout:
  type: absolute
  width: 800
  height: 600

widgets:
  - id: title
    type: markdown
    position: { x: 20, y: 20, width: 760, height: 60 }
    config:
      content: "Absolute Positioning Demo"

  - id: sidebar
    type: markdown
    position: { x: 20, y: 100, width: 200, height: 480 }
    config:
      content: "Sidebar"

  - id: main
    type: markdown
    position: { x: 240, y: 100, width: 540, height: 480 }
    config:
      content: "Main Content Area"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Absolute layout:");
    println!(
        "  Canvas: {}x{}",
        scene.layout.width.unwrap_or(0),
        scene.layout.height.unwrap_or(0)
    );
    println!("  Widgets: {}", scene.widgets.len());

    for widget in &scene.widgets {
        if let Some(pos) = &widget.position {
            println!(
                "  {} at ({:?}, {:?}) size {:?}x{:?}",
                widget.id, pos.x, pos.y, pos.width, pos.height
            );
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

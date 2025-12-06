//! # Recipe: Custom Theme Colors
//!
//! **Category**: Theming
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Override theme colors with custom values.
//!
//! ## Run Command
//! ```bash
//! cargo run --example theme_custom_colors
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_custom_colors")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "custom-colors-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "# Custom Colors Demo"

theme:
  preset: "light"
  colors:
    primary: "#3498db"
    secondary: "#2ecc71"
    background: "#f8f9fa"
    surface: "#ffffff"
    text: "#2c3e50"
    error: "#e74c3c"
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    if let Some(theme) = &scene.theme {
        println!("Theme: {:?}", theme.preset);
        if let Some(colors) = &theme.colors {
            println!("Custom colors:");
            println!("  Primary: {:?}", colors.primary);
            println!("  Secondary: {:?}", colors.secondary);
            println!("  Background: {:?}", colors.background);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

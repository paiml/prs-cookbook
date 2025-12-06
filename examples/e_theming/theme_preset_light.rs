//! # Recipe: Light Theme Preset
//!
//! **Category**: Theming
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Apply light theme preset to scene.
//!
//! ## Run Command
//! ```bash
//! cargo run --example theme_preset_light
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_preset_light")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "light-theme-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "# Light Theme Demo"

theme:
  preset: "light"
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    if let Some(theme) = &scene.theme {
        println!("Theme preset: {:?}", theme.preset);
    }

    ctx.report(&scene)?;
    Ok(())
}

//! # Recipe: Dark Theme Preset
//!
//! **Category**: Theming
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Apply dark theme preset to scene.
//!
//! ## Run Command
//! ```bash
//! cargo run --example theme_preset_dark
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_preset_dark")?;

    let yaml = r##"
prs_version: "1.0"

metadata:
  name: "dark-theme-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "# Dark Theme Demo"

  - id: description
    type: markdown
    config:
      content: "This scene uses the dark theme preset"

theme:
  preset: "dark"
"##;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    if let Some(theme) = &scene.theme {
        println!("Theme preset: {:?}", theme.preset);
    }

    ctx.report(&scene)?;
    Ok(())
}

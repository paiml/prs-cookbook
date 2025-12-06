//! # Recipe: Custom Theme Fonts
//!
//! **Category**: Theming
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Configure custom typography for headings and body text.
//!
//! ## Run Command
//! ```bash
//! cargo run --example theme_custom_fonts
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_custom_fonts")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "custom-fonts-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "Custom Fonts Demo"

theme:
  preset: "light"
  fonts:
    heading: "Inter, system-ui, sans-serif"
    body: "Source Sans Pro, system-ui, sans-serif"
    mono: "JetBrains Mono, Fira Code, monospace"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    if let Some(theme) = &scene.theme {
        if let Some(fonts) = &theme.fonts {
            println!("Custom fonts:");
            println!("  Heading: {:?}", fonts.heading);
            println!("  Body: {:?}", fonts.body);
            println!("  Mono: {:?}", fonts.mono);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

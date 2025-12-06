//! # Recipe: Dropdown Widget
//!
//! **Category**: Widget Configuration
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Configure dropdown widget with static options.
//!
//! ## Run Command
//! ```bash
//! cargo run --example widget_dropdown
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_dropdown")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "dropdown-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: country
    type: dropdown
    config:
      label: "Country"
      placeholder: "Select a country..."
      options:
        - value: "us"
          label: "United States"
        - value: "uk"
          label: "United Kingdom"
        - value: "de"
          label: "Germany"
        - value: "fr"
          label: "France"

  - id: language
    type: dropdown
    config:
      label: "Language"
      default: "en"
      options:
        - value: "en"
          label: "English"
        - value: "es"
          label: "Spanish"
        - value: "fr"
          label: "French"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Dropdown widgets:");
    for widget in &scene.widgets {
        let label = widget.config.get("label").map(|v| v.as_str().unwrap_or(""));
        let options = widget.config.get("options");
        let count = options.and_then(|o| o.as_sequence()).map_or(0, Vec::len);
        println!("  {}: {:?} ({} options)", widget.id, label, count);
    }

    ctx.report(&scene)?;
    Ok(())
}

//! # Recipe: Slider Widget
//!
//! **Category**: Widget Configuration
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Configure slider widget with min/max/step values.
//!
//! ## Run Command
//! ```bash
//! cargo run --example widget_slider
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_slider")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "slider-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: volume
    type: slider
    config:
      label: "Volume"
      min: 0
      max: 100
      default: 50
      step: 1

  - id: brightness
    type: slider
    config:
      label: "Brightness"
      min: 0
      max: 100
      default: 75
      step: 5

  - id: temperature
    type: slider
    config:
      label: "Temperature (°C)"
      min: -20
      max: 40
      default: 22
      step: 0.5
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Slider widgets:");
    for widget in &scene.widgets {
        let label = widget.config.get("label").map(|v| v.as_str().unwrap_or(""));
        let min = widget.config.get("min");
        let max = widget.config.get("max");
        println!(
            "  {}: {:?} (range {:?} to {:?})",
            widget.id, label, min, max
        );
    }

    ctx.report(&scene)?;
    Ok(())
}

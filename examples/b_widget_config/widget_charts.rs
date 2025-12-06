//! # Recipe: Chart Widgets
//!
//! **Category**: Widget Configuration
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Configure bar, line, and gauge chart widgets.
//!
//! ## Run Command
//! ```bash
//! cargo run --example widget_charts
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_charts")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "charts-demo"

layout:
  type: grid
  columns: 2
  rows: 2
  gap: 16

widgets:
  - id: bar-chart
    type: bar_chart
    position: { row: 0, col: 0 }
    config:
      title: "Monthly Sales"
      x_axis: "Month"
      y_axis: "Revenue ($)"
      data:
        - label: "Jan"
          value: 1200
        - label: "Feb"
          value: 1800
        - label: "Mar"
          value: 1500

  - id: line-chart
    type: line_chart
    position: { row: 0, col: 1 }
    config:
      title: "User Growth"
      x_axis: "Week"
      y_axis: "Users"
      series:
        - name: "Active"
          color: "blue"
        - name: "New"
          color: "green"

  - id: gauge
    type: gauge
    position: { row: 1, col: 0 }
    config:
      title: "CPU Usage"
      value: 67
      min: 0
      max: 100
      unit: "%"
      thresholds:
        - value: 50
          color: "green"
        - value: 80
          color: "yellow"
        - value: 100
          color: "red"

  - id: pie-chart
    type: pie_chart
    position: { row: 1, col: 1 }
    config:
      title: "Traffic Sources"
      data:
        - label: "Organic"
          value: 45
        - label: "Direct"
          value: 30
        - label: "Referral"
          value: 25
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Chart widgets:");
    for widget in &scene.widgets {
        let title = widget.config.get("title").map(|v| v.as_str().unwrap_or(""));
        println!("  {}: {} - {:?}", widget.id, widget.widget_type, title);
    }

    ctx.report(&scene)?;
    Ok(())
}

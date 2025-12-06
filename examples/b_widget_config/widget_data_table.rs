//! # Recipe: Data Table Widget
//!
//! **Category**: Widget Configuration
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Configure data table widget with columns and sorting.
//!
//! ## Run Command
//! ```bash
//! cargo run --example widget_data_table
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_data_table")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "data-table-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: users-table
    type: data_table
    config:
      title: "User List"
      columns:
        - field: "id"
          header: "ID"
          width: 60
          sortable: true
        - field: "name"
          header: "Name"
          width: 200
          sortable: true
        - field: "email"
          header: "Email"
          width: 250
        - field: "role"
          header: "Role"
          width: 100
          sortable: true
        - field: "status"
          header: "Status"
          width: 80
      pagination:
        enabled: true
        page_size: 10
      search:
        enabled: true
        placeholder: "Search users..."
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Data table widget:");
    for widget in &scene.widgets {
        let columns = widget.config.get("columns");
        let count = columns.and_then(|c| c.as_sequence()).map_or(0, Vec::len);
        println!("  {}: {} columns", widget.id, count);
    }

    ctx.report(&scene)?;
    Ok(())
}

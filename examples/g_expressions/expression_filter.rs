//! # Recipe: Filter Expression
//!
//! **Category**: Expressions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Use `filter()` transform to filter data based on conditions.
//!
//! ## Run Command
//! ```bash
//! cargo run --example expression_filter
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_filter")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-filter-demo"

resources:
  datasets:
    sales:
      type: parquet
      source: "./data/sales.parquet"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: high-value
    type: data_table
    config:
      title: "High Value Sales (> $1000)"
      data: "{{ datasets.sales | filter('amount > 1000') }}"

  - id: recent
    type: data_table
    config:
      title: "Recent Sales (last 7 days)"
      data: "{{ datasets.sales | filter('date >= now() - 7d') }}"

  - id: category-filter
    type: data_table
    config:
      title: "Electronics Only"
      data: "{{ datasets.sales | filter('category == \"electronics\"') }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Filter expressions:");
    for widget in &scene.widgets {
        if let Some(data) = widget.config.get("data") {
            if data.as_str().is_some_and(|s| s.contains("filter")) {
                println!("  {}: {:?}", widget.id, data);
            }
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

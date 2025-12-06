//! # Recipe: Aggregation Expressions
//!
//! **Category**: Expressions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Use `sum()`, `mean()`, `count()` transforms for data aggregation.
//!
//! ## Run Command
//! ```bash
//! cargo run --example expression_aggregation
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_aggregation")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-aggregation-demo"

resources:
  datasets:
    orders:
      type: parquet
      source: "./data/orders.parquet"

layout:
  type: grid
  columns: 2
  rows: 2
  gap: 16

widgets:
  - id: total-revenue
    type: markdown
    position: { row: 0, col: 0 }
    config:
      content: "Total Revenue: ${{ datasets.orders | sum('amount') | format(',.2f') }}"

  - id: avg-order
    type: markdown
    position: { row: 0, col: 1 }
    config:
      content: "Avg Order: ${{ datasets.orders | mean('amount') | format(',.2f') }}"

  - id: order-count
    type: markdown
    position: { row: 1, col: 0 }
    config:
      content: "Total Orders: {{ datasets.orders | count() }}"

  - id: unique-customers
    type: markdown
    position: { row: 1, col: 1 }
    config:
      content: "Unique Customers: {{ datasets.orders | count_distinct('customer_id') }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Aggregation expressions:");
    for widget in &scene.widgets {
        if let Some(content) = widget.config.get("content") {
            println!("  {}: {:?}", widget.id, content);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

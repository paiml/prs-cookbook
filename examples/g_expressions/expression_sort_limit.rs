//! # Recipe: Sort and Limit Expressions
//!
//! **Category**: Expressions
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Use `sort()` and `limit()` transforms for data ordering and pagination.
//!
//! ## Run Command
//! ```bash
//! cargo run --example expression_sort_limit
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_sort_limit")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-sort-limit-demo"

resources:
  datasets:
    products:
      type: parquet
      source: "./data/products.parquet"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: top-sellers
    type: data_table
    config:
      title: "Top 10 Best Sellers"
      data: "{{ datasets.products | sort('sales_count desc') | limit(10) }}"

  - id: cheapest
    type: data_table
    config:
      title: "5 Cheapest Products"
      data: "{{ datasets.products | sort('price asc') | limit(5) }}"

  - id: newest
    type: data_table
    config:
      title: "Recently Added"
      data: "{{ datasets.products | sort('created_at desc') | limit(20) }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Sort and limit expressions:");
    for widget in &scene.widgets {
        if let Some(data) = widget.config.get("data") {
            println!("  {}: {:?}", widget.id, data);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

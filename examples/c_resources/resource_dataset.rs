//! # Recipe: Dataset Resource
//!
//! **Category**: Resources
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: None
//!
//! ## Learning Objective
//! Reference `.ald` or parquet dataset with transforms.
//!
//! ## Run Command
//! ```bash
//! cargo run --example resource_dataset
//! ```

use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("resource_dataset")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "dataset-demo"

resources:
  datasets:
    sales:
      type: parquet
      source: "./data/sales.parquet"
      transforms:
        - "filter(year >= 2023)"
        - "sort(date desc)"
        - "limit(1000)"

    users:
      type: csv
      source: "./data/users.csv"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: sales-table
    type: data_table
    config:
      title: "Sales Data"
      source: "datasets.sales"

  - id: sales-chart
    type: bar_chart
    config:
      title: "Sales by Month"
      source: "datasets.sales"
      x: "month"
      y: "revenue"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Dataset resources:");
    for (name, dataset) in &scene.resources.datasets {
        println!("  {}: {:?}", name, dataset.dataset_type);
        println!("    Source: {}", dataset.source.primary());
        if !dataset.transforms.is_empty() {
            println!("    Transforms: {:?}", dataset.transforms);
        }
    }

    ctx.report(&scene)?;
    Ok(())
}

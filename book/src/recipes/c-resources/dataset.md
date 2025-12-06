# Dataset

Reference data files with optional transforms.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - dataset loaded independently |
| Idempotency | Guaranteed - same transforms → same data |
| Usefulness | Charts, tables, data visualization |
| Reproducibility | Deterministic transform chain |

## Example

```rust
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

    ctx.report(&scene)?;
    Ok(())
}
```

## Configuration Options

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `type` | enum | Yes | Format: `parquet`, `csv`, `ald` |
| `source` | string | Yes | Path to data file |
| `transforms` | array | No | Transform chain |
| `hash` | string | No | BLAKE3 hash (required for remote) |

## Dataset Types

| Type | Description |
|------|-------------|
| `parquet` | Apache Parquet columnar format |
| `csv` | Comma-separated values |
| `ald` | APR Layer Data format |

## Transforms

```yaml
transforms:
  - "filter(year >= 2023)"    # Row filtering
  - "select(name, revenue)"   # Column selection
  - "sort(date desc)"         # Ordering
  - "limit(1000)"             # Row limit
  - "aggregate(sum(revenue) by month)"  # Aggregation
```

## Run

```bash
cargo run --example resource_dataset
```

## Widget Binding

```yaml
widgets:
  - id: chart
    type: bar_chart
    config:
      source: "datasets.sales"  # Reference by name
      x: "month"
      y: "revenue"
```

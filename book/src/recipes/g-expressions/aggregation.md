# Aggregation

Use `sum()`, `mean()`, `count()` transforms for data aggregation.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - doesn't mutate source |
| Idempotency | Guaranteed - same data → same result |
| Usefulness | Statistics, summaries |
| Reproducibility | Deterministic aggregation |

## Example

```rust
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

    ctx.report(&scene)?;
    Ok(())
}
```

## Aggregation Functions

| Function | Description |
|----------|-------------|
| `sum('field')` | Sum of values |
| `mean('field')` | Average |
| `count()` | Row count |
| `count_distinct('field')` | Unique values |
| `min('field')` | Minimum value |
| `max('field')` | Maximum value |

## Run

```bash
cargo run --example expression_aggregation
```

## Use Cases

### Total

```yaml
content: "Total: {{ orders | sum('amount') }}"
```

### Average

```yaml
content: "Average: {{ scores | mean('value') }}"
```

### Count

```yaml
content: "Items: {{ items | count() }}"
```

### Group By

```yaml
data: "{{ sales | group_by('category') | sum('revenue') }}"
```

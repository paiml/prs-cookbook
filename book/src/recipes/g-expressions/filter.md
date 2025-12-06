# Filter

Use `filter()` transform to filter data based on conditions.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - doesn't mutate source |
| Idempotency | Guaranteed - same condition → same rows |
| Usefulness | Data filtering, search |
| Reproducibility | Deterministic filtering |

## Example

```rust
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

    ctx.report(&scene)?;
    Ok(())
}
```

## Syntax

```yaml
"{{ source | filter('condition') }}"
```

## Operators

| Operator | Description |
|----------|-------------|
| `==` | Equal |
| `!=` | Not equal |
| `>`, `>=` | Greater than |
| `<`, `<=` | Less than |
| `&&` | AND |
| `||` | OR |

## Run

```bash
cargo run --example expression_filter
```

## Use Cases

### Numeric Comparison

```yaml
data: "{{ sales | filter('amount > 1000') }}"
```

### String Match

```yaml
data: "{{ users | filter('status == \"active\"') }}"
```

### Date Filter

```yaml
data: "{{ orders | filter('date >= now() - 7d') }}"
```

### Combined Conditions

```yaml
data: "{{ products | filter('price < 100 && in_stock == true') }}"
```

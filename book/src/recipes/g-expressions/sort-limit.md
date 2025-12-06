# Sort & Limit

Use `sort()` and `limit()` transforms for data ordering and pagination.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - doesn't mutate source |
| Idempotency | Guaranteed - same params → same order |
| Usefulness | Top-N queries, pagination |
| Reproducibility | Deterministic ordering |

## Example

```rust
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

    ctx.report(&scene)?;
    Ok(())
}
```

## Syntax

```yaml
# Sort ascending
"{{ data | sort('field asc') }}"

# Sort descending
"{{ data | sort('field desc') }}"

# Limit rows
"{{ data | limit(10) }}"

# Combined
"{{ data | sort('field desc') | limit(10) }}"
```

## Run

```bash
cargo run --example expression_sort_limit
```

## Use Cases

### Top N

```yaml
data: "{{ sales | sort('revenue desc') | limit(10) }}"
```

### Most Recent

```yaml
data: "{{ orders | sort('date desc') | limit(5) }}"
```

### Pagination

```yaml
# Page 1
data: "{{ items | limit(20) }}"

# Page 2 (with offset)
data: "{{ items | offset(20) | limit(20) }}"
```

### Multi-field Sort

```yaml
data: "{{ products | sort('category asc, price desc') }}"
```

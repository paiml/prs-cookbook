# Data Table

Configure data table widgets for tabular data display.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - independent widget configuration |
| Idempotency | Guaranteed - same config → same widget |
| Usefulness | Lists, records, database views |
| Reproducibility | Deterministic rendering |

## Example

```rust
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

    ctx.report(&scene)?;
    Ok(())
}
```

## Column Configuration

| Option | Type | Description |
|--------|------|-------------|
| `field` | string | Data field name |
| `header` | string | Column header text |
| `width` | number | Column width in pixels |
| `sortable` | bool | Enable sorting |
| `filterable` | bool | Enable column filtering |
| `format` | string | Display format |

## Pagination Options

```yaml
pagination:
  enabled: true
  page_size: 10
  page_sizes: [10, 25, 50, 100]
```

## Search Options

```yaml
search:
  enabled: true
  placeholder: "Search..."
  fields: ["name", "email"]  # Limit search to fields
```

## Run

```bash
cargo run --example widget_data_table
```

## Events

| Event | Payload | Trigger |
|-------|---------|---------|
| `select` | `{ row: object }` | Row selected |
| `sort` | `{ field: string, direction: string }` | Column sorted |
| `page` | `{ page: number }` | Page changed |
| `search` | `{ query: string }` | Search performed |

## Data Binding

```yaml
resources:
  datasets:
    users:
      type: parquet
      source: "./data/users.parquet"

widgets:
  - id: users-table
    type: data_table
    config:
      data_source: "datasets.users"
      columns: [...]
```

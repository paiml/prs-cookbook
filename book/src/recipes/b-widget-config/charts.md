# Charts

Configure chart widgets for data visualization.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - independent widget configuration |
| Idempotency | Guaranteed - same config → same widget |
| Usefulness | Dashboards, analytics, metrics |
| Reproducibility | Deterministic rendering |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_charts")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "charts-demo"

layout:
  type: grid
  columns: 2
  rows: 2
  gap: 16

widgets:
  - id: bar-chart
    type: bar_chart
    position: { row: 0, col: 0 }
    config:
      title: "Monthly Sales"
      x_axis: "Month"
      y_axis: "Revenue ($)"
      data:
        - label: "Jan"
          value: 1200
        - label: "Feb"
          value: 1800
        - label: "Mar"
          value: 1500

  - id: line-chart
    type: line_chart
    position: { row: 0, col: 1 }
    config:
      title: "User Growth"
      x_axis: "Week"
      y_axis: "Users"
      series:
        - name: "Active"
          color: "blue"
        - name: "New"
          color: "green"

  - id: gauge
    type: gauge
    position: { row: 1, col: 0 }
    config:
      title: "CPU Usage"
      value: 67
      min: 0
      max: 100
      unit: "%"
      thresholds:
        - value: 50
          color: "green"
        - value: 80
          color: "yellow"
        - value: 100
          color: "red"

  - id: pie-chart
    type: pie_chart
    position: { row: 1, col: 1 }
    config:
      title: "Traffic Sources"
      data:
        - label: "Organic"
          value: 45
        - label: "Direct"
          value: 30
        - label: "Referral"
          value: 25
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Chart Types

### Bar Chart

```yaml
type: bar_chart
config:
  title: "Sales"
  x_axis: "Month"
  y_axis: "Revenue"
  data:
    - label: "Jan"
      value: 1200
```

### Line Chart

```yaml
type: line_chart
config:
  title: "Growth"
  series:
    - name: "Users"
      color: "blue"
```

### Gauge

```yaml
type: gauge
config:
  value: 75
  min: 0
  max: 100
  thresholds:
    - value: 50
      color: "green"
```

### Pie Chart

```yaml
type: pie_chart
config:
  data:
    - label: "A"
      value: 40
    - label: "B"
      value: 60
```

## Run

```bash
cargo run --example widget_charts
```

## Data Binding

Charts can bind to dataset resources:

```yaml
widgets:
  - id: sales-chart
    type: bar_chart
    config:
      data_source: "datasets.sales"
      x_field: "month"
      y_field: "revenue"
```

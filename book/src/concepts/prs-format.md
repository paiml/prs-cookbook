# The PRS Format

The Presentar Scene Format (`.prs`) is a YAML-based declarative format for visualization dashboards.

## Basic Structure

```yaml
prs_version: "1.0"

metadata:
  name: "dashboard-app"      # kebab-case identifier
  title: "Sales Dashboard"   # Human-readable title
  author: "Team Name"

resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"
  datasets:
    sales:
      type: parquet
      source: "./data/sales.parquet"

layout:
  type: grid
  columns: 3
  rows: 2
  gap: 16

widgets:
  - id: header
    type: markdown
    position: { row: 0, col: 0, colspan: 3 }
    config:
      content: "# Dashboard"

bindings:
  - trigger: "filter.change"
    debounce_ms: 300
    actions:
      - target: chart
        action: refresh

theme:
  preset: "dark"

permissions:
  network: []
  filesystem: ["./data/*"]
  clipboard: false
```

## Key Sections

| Section | Purpose |
|---------|---------|
| `prs_version` | Format version (currently "1.0") |
| `metadata` | Scene identification and attribution |
| `resources` | Models and datasets with sources |
| `layout` | Container layout (flex, grid, absolute) |
| `widgets` | UI components with configuration |
| `bindings` | Event triggers and actions |
| `theme` | Visual styling |
| `permissions` | Security constraints |

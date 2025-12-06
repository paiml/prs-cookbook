# Overview

Widget configuration recipes for interactive scene components.

## Available Widgets

| Widget | Purpose | Key Config |
|--------|---------|------------|
| [Text Input](./text-input.md) | User text entry | `label`, `placeholder`, `max_length` |
| [Slider](./slider.md) | Numeric range selection | `min`, `max`, `step`, `default` |
| [Dropdown](./dropdown.md) | Option selection | `options`, `default`, `placeholder` |
| [Charts](./charts.md) | Data visualization | `title`, `data`, `series` |
| [Data Table](./data-table.md) | Tabular data display | `columns`, `pagination`, `search` |

## IIUR Properties

All widget recipes follow IIUR principles:

- **Isolated**: Each widget is independently configurable
- **Idempotent**: Same config produces same widget
- **Useful**: Real-world interactive components
- **Reproducible**: Deterministic rendering

## Common Widget Structure

```yaml
widgets:
  - id: my-widget        # Unique identifier
    type: textbox        # Widget type
    position: { ... }    # Optional positioning
    config:              # Widget-specific config
      label: "..."
      # ... more options
```

## Widget Events

Widgets emit events that can trigger bindings:

| Event | Widgets | Description |
|-------|---------|-------------|
| `change` | All inputs | Value changed |
| `focus` | Text inputs | Widget focused |
| `blur` | Text inputs | Widget unfocused |
| `select` | Dropdown, Table | Item selected |
| `sort` | Data Table | Column sorted |

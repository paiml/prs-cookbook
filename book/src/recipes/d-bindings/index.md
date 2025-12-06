# Overview

Binding recipes for connecting widgets and triggering actions.

## Binding Types

| Binding | Purpose | Key Config |
|---------|---------|------------|
| [Simple Update](./simple-update.md) | Basic widget refresh | `trigger`, `actions` |
| [Debounced](./debounced.md) | Rate-limited updates | `debounce_ms` |
| [Model Inference](./model-inference.md) | ML predictions | `inference.*` target |
| [Chain Actions](./chain-actions.md) | Sequential actions | Multiple actions |
| [Conditional](./conditional.md) | State-based logic | `condition` |

## IIUR Properties

All binding recipes follow IIUR principles:

- **Isolated**: Bindings are independently testable
- **Idempotent**: Same trigger → same actions
- **Useful**: Real-world reactive patterns
- **Reproducible**: Deterministic event handling

## Binding Structure

```yaml
bindings:
  - trigger: "widget-id.event"   # Event source
    debounce_ms: 300             # Optional debounce
    condition: "{{ expr }}"      # Optional condition
    actions:                     # Action list
      - target: target-widget
        action: refresh
        input: "{{ value }}"
```

## Events

| Event | Widgets | Description |
|-------|---------|-------------|
| `change` | All inputs | Value changed |
| `click` | Button | Button clicked |
| `submit` | Form | Form submitted |
| `select` | Dropdown, Table | Item selected |

## Actions

| Action | Description |
|--------|-------------|
| `refresh` | Re-render widget |
| `update` | Update widget config |
| `show` | Make widget visible |
| `hide` | Hide widget |

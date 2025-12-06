# Category A: Scene Creation

This category covers the fundamentals of creating PRS scenes with different layout types.

## Recipes

| Recipe | Example | Description |
|--------|---------|-------------|
| [Minimal Scene](./minimal-scene.md) | `create_minimal_scene` | Simplest possible scene |
| [Grid Layout](./grid-layout.md) | `create_grid_layout` | Row/column based layout |
| [Flex Layout](./flex-layout.md) | `create_flex_layout` | Flexbox-style layout |
| [Absolute Layout](./absolute-layout.md) | `create_absolute_layout` | Pixel-positioned widgets |
| [Nested Layouts](./nested-layouts.md) | `create_nested_layouts` | Layouts within layouts |

## Quick Reference

### Layout Types

```yaml
# Flex layout
layout:
  type: flex
  direction: column  # or row
  gap: 16

# Grid layout
layout:
  type: grid
  columns: 3
  rows: 2
  gap: 8

# Absolute layout
layout:
  type: absolute
  width: 800
  height: 600
```

## Run Examples

```bash
cargo run --example create_minimal_scene
cargo run --example create_grid_layout
cargo run --example create_flex_layout
cargo run --example create_absolute_layout
cargo run --example create_nested_layouts
```

# Overview

Theming recipes for customizing scene appearance.

## Theme Options

| Theme | Purpose | Key Config |
|-------|---------|------------|
| [Dark Preset](./dark-preset.md) | Dark mode | `preset: "dark"` |
| [Light Preset](./light-preset.md) | Light mode | `preset: "light"` |
| [Custom Colors](./custom-colors.md) | Brand colors | `colors` |
| [Custom Fonts](./custom-fonts.md) | Typography | `fonts` |

## IIUR Properties

All theme recipes follow IIUR principles:

- **Isolated**: Theme changes don't affect logic
- **Idempotent**: Same theme config → same appearance
- **Useful**: Real-world UI customization
- **Reproducible**: Consistent rendering

## Theme Structure

```yaml
theme:
  preset: "dark"       # Base theme
  colors:              # Color overrides
    primary: "#3498db"
    background: "#1a1a2e"
  fonts:               # Typography
    heading: "Inter, sans-serif"
    body: "Source Sans Pro, sans-serif"
```

## Presets

| Preset | Description |
|--------|-------------|
| `light` | Light background, dark text |
| `dark` | Dark background, light text |
| `system` | Follow OS preference |

## Color Tokens

| Token | Usage |
|-------|-------|
| `primary` | Buttons, links, accents |
| `secondary` | Secondary actions |
| `background` | Page background |
| `surface` | Card, panel backgrounds |
| `text` | Body text |
| `error` | Error messages |

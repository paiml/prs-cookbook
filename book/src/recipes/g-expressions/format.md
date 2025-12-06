# Format

Use `format()` and `join()` transforms for string formatting.

## IIUR Properties

| Property | Value |
|----------|-------|
| Isolation | Full - doesn't mutate source |
| Idempotency | Guaranteed - same format → same output |
| Usefulness | Display formatting, localization |
| Reproducibility | Deterministic formatting |

## Example

```rust
use prs_cookbook::prelude::*;

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_format")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-format-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: currency
    type: markdown
    config:
      content: "Price: {{ state.price | format('$,.2f') }}"

  - id: percentage
    type: markdown
    config:
      content: "Discount: {{ state.discount | percentage }}%"

  - id: date-format
    type: markdown
    config:
      content: "Date: {{ state.timestamp | date('MMMM D, YYYY') }}"

  - id: list-join
    type: markdown
    config:
      content: "Tags: {{ state.tags | join(', ') }}"

  - id: truncate
    type: markdown
    config:
      content: "{{ state.description | truncate(100) }}..."

  - id: uppercase
    type: markdown
    config:
      content: "{{ state.title | uppercase }}"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    ctx.report(&scene)?;
    Ok(())
}
```

## Format Functions

| Function | Description |
|----------|-------------|
| `format('spec')` | Number formatting |
| `percentage` | As percentage (0-100) |
| `date('pattern')` | Date formatting |
| `join('sep')` | Join array |
| `truncate(n)` | Limit characters |
| `uppercase` | Uppercase |
| `lowercase` | Lowercase |

## Run

```bash
cargo run --example expression_format
```

## Number Formatting

```yaml
# Currency
"{{ price | format('$,.2f') }}"  # $1,234.56

# Percentage
"{{ ratio | percentage }}"  # 75%

# Fixed decimals
"{{ value | format('.3f') }}"  # 1.234
```

## Date Formatting

```yaml
# Full date
"{{ date | date('MMMM D, YYYY') }}"  # January 15, 2024

# Short date
"{{ date | date('MM/DD/YY') }}"  # 01/15/24

# ISO
"{{ date | date('YYYY-MM-DD') }}"  # 2024-01-15
```

## String Transforms

```yaml
# Join array
"{{ tags | join(', ') }}"  # "tag1, tag2, tag3"

# Truncate
"{{ text | truncate(50) }}"  # "First 50 characters..."

# Case
"{{ title | uppercase }}"
"{{ title | lowercase }}"
```

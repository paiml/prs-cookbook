# Overview

Expression recipes for data transformation and formatting.

## Expression Types

| Expression | Purpose | Key Transforms |
|------------|---------|----------------|
| [Select](./select.md) | Field extraction | `select('field')` |
| [Filter](./filter.md) | Row filtering | `filter('condition')` |
| [Sort & Limit](./sort-limit.md) | Ordering | `sort()`, `limit()` |
| [Aggregation](./aggregation.md) | Statistics | `sum()`, `mean()`, `count()` |
| [Format](./format.md) | Display formatting | `format()`, `date()` |

## IIUR Properties

All expression recipes follow IIUR principles:

- **Isolated**: Expressions don't mutate source data
- **Idempotent**: Same input → same output
- **Useful**: Real-world data transformations
- **Reproducible**: Deterministic evaluation

## Expression Syntax

```yaml
"{{ source | transform1 | transform2 }}"
```

## Pipe Chaining

Transforms are applied left-to-right:

```yaml
"{{ data | filter('active') | sort('name') | limit(10) }}"
```

## Data Sources

| Source | Description |
|--------|-------------|
| `datasets.name` | Dataset resource |
| `inference.model` | Model output |
| `state.field` | Widget state |
| `widget.value` | Widget value |

## Common Transforms

| Transform | Description |
|-----------|-------------|
| `select('field')` | Extract field |
| `filter('cond')` | Filter rows |
| `sort('field dir')` | Sort rows |
| `limit(n)` | Limit rows |
| `format('spec')` | Format value |

# Overview

Resource configuration recipes for models and datasets.

## Resource Types

| Resource | Purpose | Key Config |
|----------|---------|------------|
| [Local Model](./local-model.md) | Local `.apr` files | `type`, `source` |
| [Remote Model](./remote-model.md) | CDN-hosted models | `hash`, `size_bytes` |
| [Dataset](./dataset.md) | Data files | `transforms` |
| [Fallback Sources](./fallback-sources.md) | Resilient loading | `sources` array |

## IIUR Properties

All resource recipes follow IIUR principles:

- **Isolated**: Resources are independently loadable
- **Idempotent**: Same resource config loads same data
- **Useful**: Real-world ML and data scenarios
- **Reproducible**: Hash verification ensures integrity

## Resource Structure

```yaml
resources:
  models:
    model-name:
      type: apr          # Model format
      source: "./path"   # File path or URL
      hash: "blake3:..." # Required for remote

  datasets:
    dataset-name:
      type: parquet      # csv, parquet, ald
      source: "./path"
      transforms: [...]  # Optional transforms
```

## Security

Remote resources **require** BLAKE3 hash verification:

```yaml
models:
  classifier:
    source: "https://cdn.example.com/model.apr"
    hash: "blake3:a1b2c3..."  # REQUIRED
```

This prevents supply-chain attacks and ensures reproducibility.

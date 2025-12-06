# Overview

Permission recipes for controlling scene capabilities.

## Permission Types

| Permission | Purpose | Key Config |
|------------|---------|------------|
| [Network](./network.md) | HTTP access | `network` patterns |
| [Filesystem](./filesystem.md) | File access | `filesystem` patterns |
| [Minimal](./minimal.md) | No permissions | Empty arrays |

## IIUR Properties

All permission recipes follow IIUR principles:

- **Isolated**: Permissions are per-scene
- **Idempotent**: Same permissions → same capabilities
- **Useful**: Security through least privilege
- **Reproducible**: Predictable security model

## Permission Structure

```yaml
permissions:
  network:
    - "https://api.example.com/*"
  filesystem:
    - "./data/*"
  clipboard: false
  camera: false
  microphone: false
```

## Security Model

Presentar follows **principle of least privilege**:

1. All permissions denied by default
2. Explicitly grant only what's needed
3. Use patterns to limit scope
4. Review permissions before running

## Network Patterns

```yaml
network:
  - "https://api.example.com/*"    # Specific domain
  - "https://*.cdn.example.com/*"  # Wildcard subdomain
```

## Filesystem Patterns

```yaml
filesystem:
  - "./data/*"      # Read from data directory
  - "./output/*"    # Write to output directory
```

## Boolean Permissions

| Permission | Default | Description |
|------------|---------|-------------|
| `clipboard` | false | Read/write clipboard |
| `camera` | false | Access camera |
| `microphone` | false | Access microphone |

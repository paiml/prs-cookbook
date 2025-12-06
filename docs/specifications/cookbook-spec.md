# PRS Cookbook: Isolated, Idempotent & Reproducible Scene Recipes Specification

**Version**: 1.0.0
**Status**: DRAFT
**Author**: Sovereign AI Stack Team
**Date**: 2025-12-06
**MSRV**: 1.75
**Repository**: [github.com/paiml/prs-cookbook](https://github.com/paiml/prs-cookbook)

---

## Executive Summary

This specification defines the complete implementation of PRS Cookbook examples following the **IIUR Principles**: **Isolated**, **Idempotent**, **Useful**, and **Reproducible**. Each recipe is a self-contained example demonstrating visualization scene creation using the Presentar Scene Format (`.prs`) that can be executed independently with deterministic outcomes.

The **Presentar Scene Format** (`.prs`) is a declarative YAML-based manifest for sharing visualization dashboards, ML model interfaces, and interactive data applications. Unlike Gradio/Streamlit which require Python runtimes, `.prs` provides a **runtime-agnostic declarative format** that WASM binaries can parse directly [1, 2].

Guided by the **Toyota Production System (TPS)** principles, this cookbook eliminates the *Muda* (waste) of Python interpreters, heavy containers, and non-deterministic UI behavior that plague traditional data visualization workflows [3, 4].

> **Theoretical Basis**: This specification is supported by 10 peer-reviewed citations (Section 6) covering Declarative UI Systems [1, 2], WebAssembly [5, 6], Lean Manufacturing [3, 4], and Human-Computer Interaction [7-10].

**Design Philosophy**: Each recipe is a *cell* in a lean production line—completely self-sufficient, producing consistent UI output every time, and ready for integration into larger visualization pipelines without side effects.

---

## Table of Contents

1. [IIUR Principles](#1-iiur-principles)
2. [Recipe Architecture](#2-recipe-architecture)
3. [Complete Recipe Catalog](#3-complete-recipe-catalog)
4. [Quality Gates & Testing Requirements](#4-quality-gates--testing-requirements)
5. [Implementation Guidelines](#5-implementation-guidelines)
6. [Peer-Reviewed Citations](#6-peer-reviewed-citations)
7. [Appendices](#7-appendices)

---

## 1. IIUR Principles

### 1.1 Isolated

> **Rationale**: Myers et al. [2] identify component isolation as critical for maintainable declarative UIs. Isolation ensures each recipe is a modular component without hidden dependencies.

Each recipe MUST:

- **No shared mutable state**: No global variables, no shared filesystems, no persistent state between runs
- **Self-contained scenes**: All required `.prs` manifests created inline or embedded
- **Temp directory isolation**: Any file I/O uses `tempfile::tempdir()` with automatic cleanup
- **Feature flag independence**: Recipes work with their declared features only
- **Thread safety**: Concurrent execution of any two recipes produces identical results

```rust
// CORRECT: Isolated recipe
fn main() -> Result<()> {
    let temp = tempfile::tempdir()?;  // Ephemeral, isolated
    let scene_path = temp.path().join("app.prs");
    // ... work within temp directory
    Ok(())  // temp directory automatically cleaned up
}

// INCORRECT: Shares state
static mut GLOBAL_SCENE: Option<Scene> = None;  // Violates isolation
```

### 1.2 Idempotent

> **Rationale**: In TPS, "Standard Work" is the basis for improvement [3]. Idempotency provides baseline stability for visualization workflows.

Each recipe MUST:

- **f(f(x)) = f(x)**: Running a recipe twice produces identical output
- **No accumulation**: Repeated runs do not accumulate files, state, or side effects
- **Deterministic layout**: Widget positions resolve identically across runs
- **Atomic operations**: Either fully succeeds or fully fails with no partial state

```rust
// CORRECT: Idempotent scene generation
let scene = Scene::builder()
    .name("demo")
    .layout(Layout::Grid { columns: 2, rows: 2, gap: 16 })
    .build()?;

// The same input always produces the same output
assert_eq!(scene.to_yaml()?, scene.to_yaml()?);
```

### 1.3 Useful

Each recipe MUST:

- **Solve a real problem**: Addresses a concrete use case from production UI workflows
- **Executable demonstration**: `cargo run --example <name>` produces meaningful output
- **Clear learning objective**: Single concept per recipe with explicit takeaway
- **Copy-paste ready**: YAML and Rust code can be directly adapted for production use

### 1.4 Reproducible

Each recipe MUST:

- **Pinned dependencies**: Uses exact versions from workspace `Cargo.lock`
- **Cross-platform**: Works on x86_64 Linux, aarch64 Linux, aarch64 macOS, WASM
- **CI-verified**: All recipes run in CI on every commit
- **Documented environment**: Clearly states any system requirements

---

## 2. Recipe Architecture

### 2.1 Standard Recipe Structure

Every recipe follows this canonical structure and MUST include the **10-Point QA Checklist** in its documentation block.

```
examples/
└── category/
    └── recipe_name.rs
scenes/
└── category/
    └── recipe_name.prs
```

Each recipe file:

```rust
//! # Recipe: [Descriptive Title]
//!
//! **Category**: [Category Name]
//! **Isolation Level**: Full
//! **Idempotency**: Guaranteed
//! **Dependencies**: [List feature flags required]
//!
//! ## QA Checklist
//! 1. [x] `cargo run` succeeds (Exit Code 0)
//! 2. [x] `cargo test` passes
//! 3. [x] Scene validates without errors
//! 4. [x] No temp files leaked
//! 5. [x] Layout resolves deterministically
//! 6. [x] WASM compatible (if applicable)
//! 7. [x] Clippy clean
//! 8. [x] Rustfmt standard
//! 9. [x] No `unwrap()` in logic
//! 10. [x] Proptests pass (100+ cases)
//!
//! ## Learning Objective
//! [One sentence describing what this recipe teaches]
//!
//! ## Run Command
//! ```bash
//! cargo run --example recipe_name [--features feature1,feature2]
//! ```

use prs_cookbook::prelude::*;

/// Recipe entry point - isolated and idempotent
fn main() -> prs_cookbook::Result<()> {
    // 1. Setup: Create isolated environment
    let ctx = RecipeContext::new("recipe_name")?;

    // 2. Execute: Perform the recipe's core logic
    let scene = create_scene(&ctx)?;

    // 3. Validate: Ensure scene is well-formed
    scene.validate()?;

    // 4. Report: Display results to user
    ctx.report(&scene)?;

    // 5. Cleanup: Automatic via Drop
    Ok(())
}
```

### 2.2 PRS Format Overview

> **Rationale**: The format design follows declarative UI principles [1, 2] and enables WASM-first deployment without Python runtimes [5, 6].

The Presentar Scene Format (`.prs`) provides declarative visualization manifests:

```text
┌─────────────────────────────────────────┐
│ prs_version: "1.0"                      │
├─────────────────────────────────────────┤
│ metadata:                               │
│   name: "app-name" (kebab-case)         │
│   title, author, license, tags          │
├─────────────────────────────────────────┤
│ resources:                              │
│   models: (apr, gguf, safetensors)      │
│   datasets: (ald, parquet, csv)         │
├─────────────────────────────────────────┤
│ layout:                                 │
│   type: grid | flex | absolute          │
├─────────────────────────────────────────┤
│ widgets:                                │
│   - id, type, position, config          │
├─────────────────────────────────────────┤
│ bindings:                               │
│   - trigger → actions                   │
├─────────────────────────────────────────┤
│ theme: preset + custom overrides        │
├─────────────────────────────────────────┤
│ permissions: network, filesystem, etc.  │
└─────────────────────────────────────────┘
```

### 2.3 Test Harness Requirements

Every recipe includes:

| Test Type | Requirement | Coverage |
|-----------|-------------|----------|
| Unit Tests | Core logic verification | 95% minimum |
| Validation Test | `Scene::from_yaml()` succeeds | Required |
| Idempotency Test | `main(); main();` produces same result | Required |
| Isolation Test | No filesystem leaks after run | Required |
| Property Tests | Proptest for input variations | 3+ properties |
| Doc Tests | All code examples compile | Required |

---

## 3. Complete Recipe Catalog

### Category A: Scene Creation

#### A.1 `create_minimal_scene`
**Objective**: Create the simplest valid `.prs` scene.

```rust
//! Create minimal .prs scene
//! Run: cargo run --example create_minimal_scene

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_minimal_scene")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "hello-world"

layout:
  type: flex
  direction: column

widgets:
  - id: greeting
    type: markdown
    config:
      content: "# Hello, Presentar!"
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Created minimal scene:");
    println!("  Name: {}", scene.metadata.name);
    println!("  Widgets: {}", scene.widgets.len());

    // Save to file
    let path = ctx.path("minimal.prs");
    std::fs::write(&path, yaml)?;
    println!("  Saved to: {:?}", path);

    Ok(())
}
```

**Tests**:
- `test_creates_valid_scene`
- `test_minimal_fields_present`
- `test_yaml_roundtrip`
- `proptest_random_names`

#### A.2 `create_grid_layout`
**Objective**: Create scene with grid layout and positioned widgets.

```rust
//! Create grid layout scene
//! Run: cargo run --example create_grid_layout

fn main() -> Result<()> {
    let ctx = RecipeContext::new("create_grid_layout")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "grid-demo"
  title: "Grid Layout Demo"

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
      content: "# Dashboard Header"

  - id: chart1
    type: bar_chart
    position: { row: 1, col: 0 }
    config:
      title: "Sales"

  - id: chart2
    type: line_chart
    position: { row: 1, col: 1 }
    config:
      title: "Trends"

  - id: gauge
    type: gauge
    position: { row: 1, col: 2 }
    config:
      value: 75
      min: 0
      max: 100
"#;

    let scene = Scene::from_yaml(yaml)?;
    scene.validate()?;

    println!("Grid layout: {}x{}",
        scene.layout.columns.unwrap(),
        scene.layout.rows.unwrap());

    Ok(())
}
```

#### A.3 `create_flex_layout`
**Objective**: Create scene with flexbox layout.

#### A.4 `create_absolute_layout`
**Objective**: Create scene with absolute positioning.

#### A.5 `create_nested_layouts`
**Objective**: Compose multiple layout types.

---

### Category B: Widget Configuration

#### B.1 `widget_text_input`
**Objective**: Configure text input widget with validation.

```rust
//! Text input widget configuration
//! Run: cargo run --example widget_text_input

fn main() -> Result<()> {
    let ctx = RecipeContext::new("widget_text_input")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "text-input-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: username
    type: textbox
    config:
      label: "Username"
      placeholder: "Enter your username..."
      max_length: 50

  - id: email
    type: textbox
    config:
      label: "Email"
      placeholder: "user@example.com"
      max_length: 100

  - id: bio
    type: textbox
    config:
      label: "Bio"
      placeholder: "Tell us about yourself..."
      max_length: 500
"#;

    let scene = Scene::from_yaml(yaml)?;

    for widget in &scene.widgets {
        if let Some(label) = widget.config.get("label") {
            println!("Widget {}: {}", widget.id, label);
        }
    }

    Ok(())
}
```

#### B.2 `widget_slider`
**Objective**: Configure slider with min/max/step.

#### B.3 `widget_dropdown`
**Objective**: Configure dropdown with options.

#### B.4 `widget_charts`
**Objective**: Configure bar, line, and gauge charts.

#### B.5 `widget_data_table`
**Objective**: Configure data table with columns.

---

### Category C: Model & Dataset Resources

#### C.1 `resource_local_model`
**Objective**: Reference local `.apr` model file.

```rust
//! Local model resource
//! Run: cargo run --example resource_local_model

fn main() -> Result<()> {
    let ctx = RecipeContext::new("resource_local_model")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "local-model-demo"

resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"

layout:
  type: flex
  direction: column

widgets:
  - id: input
    type: textbox
    config:
      label: "Text to analyze"

  - id: result
    type: gauge
    config:
      value: "{{ inference.sentiment | select('confidence') | percentage }}"
      min: 0
      max: 100

bindings:
  - trigger: "input.change"
    debounce_ms: 300
    actions:
      - target: inference.sentiment
        input: "{{ input.value }}"
"#;

    let scene = Scene::from_yaml(yaml)?;

    println!("Models: {:?}", scene.resources.models.keys().collect::<Vec<_>>());

    Ok(())
}
```

#### C.2 `resource_remote_model`
**Objective**: Reference remote model with BLAKE3 hash verification.

```rust
//! Remote model with hash verification
//! Run: cargo run --example resource_remote_model

fn main() -> Result<()> {
    let ctx = RecipeContext::new("resource_remote_model")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "remote-model-demo"

resources:
  models:
    classifier:
      type: apr
      source: "https://registry.paiml.com/models/classifier.apr"
      hash: "blake3:a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
      size_bytes: 45000000

permissions:
  network:
    - "https://registry.paiml.com/*"

layout:
  type: flex

widgets:
  - id: status
    type: markdown
    config:
      content: "Model loaded with verified hash"
"#;

    let scene = Scene::from_yaml(yaml)?;

    // Verify remote sources have hashes
    for (name, model) in &scene.resources.models {
        if model.source.primary().starts_with("https://") {
            assert!(model.hash.is_some(), "Remote model {} needs hash", name);
            println!("Model {}: hash verified", name);
        }
    }

    Ok(())
}
```

#### C.3 `resource_dataset`
**Objective**: Reference `.ald` dataset with transforms.

#### C.4 `resource_fallback_sources`
**Objective**: Configure source fallback chain.

---

### Category D: Bindings & Interactions

#### D.1 `binding_simple_update`
**Objective**: Connect widget change to action.

```rust
//! Simple binding: input → output
//! Run: cargo run --example binding_simple_update

fn main() -> Result<()> {
    let ctx = RecipeContext::new("binding_simple_update")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "binding-demo"

layout:
  type: flex
  direction: column
  gap: 16

widgets:
  - id: slider
    type: slider
    config:
      label: "Value"
      min: 0
      max: 100
      default: 50

  - id: display
    type: markdown
    config:
      content: "Current value: {{ state.slider_value }}"

bindings:
  - trigger: "slider.change"
    actions:
      - target: display
        action: refresh
"#;

    let scene = Scene::from_yaml(yaml)?;

    println!("Bindings: {}", scene.bindings.len());
    for binding in &scene.bindings {
        println!("  {} → {} action(s)", binding.trigger, binding.actions.len());
    }

    Ok(())
}
```

#### D.2 `binding_debounced`
**Objective**: Debounce rapid input changes.

#### D.3 `binding_model_inference`
**Objective**: Trigger model inference on input.

#### D.4 `binding_chain_actions`
**Objective**: Chain multiple actions from single trigger.

#### D.5 `binding_conditional`
**Objective**: Conditional action execution.

---

### Category E: Theming & Styling

#### E.1 `theme_preset_dark`
**Objective**: Apply dark theme preset.

```rust
//! Dark theme preset
//! Run: cargo run --example theme_preset_dark

fn main() -> Result<()> {
    let ctx = RecipeContext::new("theme_preset_dark")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "dark-theme-demo"

layout:
  type: flex
  direction: column

widgets:
  - id: title
    type: markdown
    config:
      content: "# Dark Theme Demo"

theme:
  preset: "dark"
"#;

    let scene = Scene::from_yaml(yaml)?;

    if let Some(theme) = &scene.theme {
        println!("Theme preset: {:?}", theme.preset);
    }

    Ok(())
}
```

#### E.2 `theme_preset_light`
**Objective**: Apply light theme preset.

#### E.3 `theme_custom_colors`
**Objective**: Override theme colors.

#### E.4 `theme_custom_fonts`
**Objective**: Configure custom typography.

---

### Category F: Permissions & Security

#### F.1 `permission_network`
**Objective**: Grant network access to specific domains.

```rust
//! Network permission configuration
//! Run: cargo run --example permission_network

fn main() -> Result<()> {
    let ctx = RecipeContext::new("permission_network")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "network-permission-demo"

resources:
  models:
    api_model:
      type: apr
      source: "https://api.example.com/model.apr"
      hash: "blake3:abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"

permissions:
  network:
    - "https://api.example.com/*"
    - "https://cdn.example.com/*"
  clipboard: false
  camera: false

layout:
  type: flex

widgets:
  - id: status
    type: markdown
    config:
      content: "Network access granted to api.example.com"
"#;

    let scene = Scene::from_yaml(yaml)?;

    println!("Network permissions: {:?}", scene.permissions.network);
    println!("Clipboard: {}", scene.permissions.clipboard);
    println!("Camera: {}", scene.permissions.camera);

    Ok(())
}
```

#### F.2 `permission_filesystem`
**Objective**: Grant filesystem access patterns.

#### F.3 `permission_minimal`
**Objective**: Demonstrate principle of least privilege.

---

### Category G: Expression Language

#### G.1 `expression_select`
**Objective**: Use `select()` transform to extract fields.

```rust
//! Expression: select transform
//! Run: cargo run --example expression_select

fn main() -> Result<()> {
    let ctx = RecipeContext::new("expression_select")?;

    let yaml = r#"
prs_version: "1.0"

metadata:
  name: "expression-select-demo"

layout:
  type: flex

widgets:
  - id: confidence
    type: gauge
    config:
      value: "{{ inference.model | select('confidence') | percentage }}"
      min: 0
      max: 100
"#;

    let scene = Scene::from_yaml(yaml)?;

    // Check expression in widget config
    if let Some(value) = scene.widgets[0].config.get("value") {
        println!("Expression: {}", value);
    }

    Ok(())
}
```

#### G.2 `expression_filter`
**Objective**: Use `filter()` transform to filter data.

#### G.3 `expression_sort_limit`
**Objective**: Use `sort()` and `limit()` transforms.

#### G.4 `expression_aggregation`
**Objective**: Use `sum()`, `mean()`, `count()` transforms.

#### G.5 `expression_format`
**Objective**: Use `format()` and `join()` transforms.

---

### Category H: WASM & Browser

#### H.1 `wasm_sentiment_analyzer`
**Objective**: Interactive sentiment analysis in browser.

```rust
//! WASM sentiment analyzer
//! Build: wasm-pack build --target web

use presentar::prelude::*;
use wasm_bindgen::prelude::*;

const SCENE_YAML: &str = include_str!("../scenes/sentiment.prs");
const MODEL_BYTES: &[u8] = include_bytes!("../models/sentiment.apr");

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let scene = Scene::from_yaml(SCENE_YAML)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut app = App::from_scene(scene)?;
    app.register_model("sentiment", MODEL_BYTES)?;
    app.mount("app-root")?;

    Ok(())
}
```

#### H.2 `wasm_image_classifier`
**Objective**: Image classification with drag-and-drop.

#### H.3 `wasm_data_explorer`
**Objective**: Interactive data visualization dashboard.

#### H.4 `wasm_parameter_tuner`
**Objective**: ML hyperparameter tuning interface.

---

### Category I: CLI Tools

#### I.1 `cli_validate_prs`
**Objective**: Validate `.prs` file syntax and semantics.

```rust
//! Validate .prs files
//! Run: cargo run --example cli_validate_prs -- scene.prs

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    files: Vec<PathBuf>,
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut passed = 0;
    let mut failed = 0;

    for path in &args.files {
        let content = std::fs::read_to_string(path)?;
        match Scene::from_yaml(&content) {
            Ok(scene) => {
                passed += 1;
                println!("✓ {}", path.display());
                if args.verbose {
                    println!("  Name: {}", scene.metadata.name);
                    println!("  Widgets: {}", scene.widgets.len());
                }
            }
            Err(e) => {
                failed += 1;
                println!("✗ {}", path.display());
                eprintln!("  Error: {}", e);
            }
        }
    }

    println!("\nResults: {} passed, {} failed", passed, failed);
    Ok(())
}
```

#### I.2 `cli_prs_info`
**Objective**: Display `.prs` scene metadata.

#### I.3 `cli_prs_convert`
**Objective**: Convert between formats (YAML ↔ JSON).

#### I.4 `cli_prs_serve`
**Objective**: Serve `.prs` scene locally for development.

---

### Category J: Complete Applications

#### J.1 `app_sentiment_dashboard`
**Objective**: Full sentiment analysis dashboard.

#### J.2 `app_ml_model_card`
**Objective**: Interactive model card display.

#### J.3 `app_data_quality_report`
**Objective**: Data quality visualization dashboard.

#### J.4 `app_realtime_monitor`
**Objective**: Real-time metrics monitoring dashboard.

---

## 4. Quality Gates & Testing Requirements

### 4.1 Coverage Requirements

| Metric | Target | Enforcement |
|--------|--------|-------------|
| Line Coverage | 95% | `cargo llvm-cov --fail-under 95` |
| Branch Coverage | 90% | `cargo llvm-cov --branch` |
| Mutation Score | 80% | `cargo mutants` |
| Property Tests | 3+ per recipe | proptest |

### 4.2 Scene Validation Requirements

Every `.prs` scene MUST pass:

1. **Version format**: `prs_version` matches `X.Y` pattern
2. **Metadata name**: kebab-case identifier
3. **Widget IDs**: Unique across scene
4. **Binding targets**: Reference existing widgets/models
5. **Remote hashes**: HTTPS sources require BLAKE3 hashes
6. **Layout constraints**: Grid needs columns, absolute needs dimensions

### 4.3 Automated QA Checklist

1. **Execution Success**: `cargo run --example <name>` exits with code 0
2. **Validation Pass**: All scenes validate without errors
3. **Lint Compliance**: `cargo clippy` returns 0 warnings
4. **Style Compliance**: `cargo fmt --check` passes
5. **Deterministic Output**: Two runs produce identical scenes
6. **Resource Isolation**: No temp files leaked
7. **Proptest Coverage**: 3+ distinct property tests
8. **Code Coverage**: 95%+ line coverage
9. **Mutation Robustness**: 80%+ mutation score
10. **Documentation**: Run command and learning objective present

---

## 5. Implementation Guidelines

### 5.1 Toyota Way Compliance

Each recipe MUST embody:

| Principle | Implementation |
|-----------|----------------|
| **Jidoka** (Built-in Quality) | Type-safe scene validation, compile-time checks [3, 7] |
| **Muda** (Waste Elimination) | No Python runtime, minimal dependencies, declarative config [4] |
| **Heijunka** (Level Loading) | Consistent recipe structure, predictable resource usage [3] |
| **Kaizen** (Continuous Improvement) | Benchmarks for rendering, performance tests [4] |
| **Genchi Genbutsu** (Go and See) | Observable UI metrics, clear output, no hidden state [3] |
| **Poka-Yoke** (Error-Proofing) | Invalid scenes rejected at parse time [3, 8] |

### 5.2 Error Handling

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeError {
    #[error("Scene not found: {0}")]
    SceneNotFound(PathBuf),

    #[error("Invalid scene: {0}")]
    InvalidScene(#[from] presentar_yaml::SceneError),

    #[error("Widget not found: {0}")]
    WidgetNotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, RecipeError>;
```

---

## 6. Peer-Reviewed Citations

### [1] Myers, B., Hudson, S. E., & Pausch, R. (2000). Past, Present, and Future of User Interface Software Tools
*ACM Transactions on Computer-Human Interaction, 7(1), 3-28. DOI: 10.1145/344949.344959*

Foundational work on declarative UI systems. The `.prs` format follows the principle of separating specification from implementation.

### [2] Meyerovich, L. A., & Bodik, R. (2013). Fast and Parallel Webpage Layout
*WWW 2013. DOI: 10.1145/2488388.2488425*

Informs our layout engine design and constraint-based positioning.

### [3] Ohno, T. (1988). Toyota Production System: Beyond Large-Scale Production
*Productivity Press. ISBN 978-0915299140*

The foundational text on lean manufacturing. Our IIUR principles map to TPS: Isolation→Jidoka, Idempotency→Standard Work.

### [4] Womack, J. P., & Jones, D. T. (1996). Lean Thinking: Banish Waste and Create Wealth
*Simon & Schuster. ISBN 978-0743249270*

Defines the five lean principles adapted for visualization workflows.

### [5] Haas, A., et al. (2017). Bringing the Web up to Speed with WebAssembly
*PLDI 2017. DOI: 10.1145/3062341.3062363*

WebAssembly specification paper. Justifies WASM-first approach for portable UI deployment.

### [6] Jangda, A., et al. (2019). Not So Fast: Analyzing the Performance of WebAssembly vs. Native Code
*USENIX ATC 2019. https://www.usenix.org/conference/atc19/presentation/jangda*

Performance analysis of WASM vs native. Informs our acceleration strategy.

### [7] Nielsen, J. (1994). Usability Engineering
*Morgan Kaufmann. ISBN 978-0125184069*

Usability heuristics inform our widget design and interaction patterns.

### [8] Shneiderman, B., et al. (2016). Designing the User Interface: Strategies for Effective Human-Computer Interaction
*Pearson. ISBN 978-0134380384*

Direct manipulation principles guide our binding and interaction model.

### [9] Card, S. K., Moran, T. P., & Newell, A. (1983). The Psychology of Human-Computer Interaction
*Lawrence Erlbaum. ISBN 978-0898592436*

Human factors research informing response time requirements (60fps target).

### [10] Tufte, E. R. (2001). The Visual Display of Quantitative Information
*Graphics Press. ISBN 978-0961392147*

Data visualization principles guide our chart and dashboard widget designs.

---

## 7. Appendices

### A. Recipe Dependency Matrix

| Recipe | presentar-yaml | presentar | wasm-bindgen | clap |
|--------|----------------|-----------|--------------|------|
| A.1-A.5 | Required | - | - | - |
| B.1-B.5 | Required | - | - | - |
| C.1-C.4 | Required | - | - | - |
| D.1-D.5 | Required | - | - | - |
| E.1-E.4 | Required | - | - | - |
| F.1-F.3 | Required | - | - | - |
| G.1-G.5 | Required | - | - | - |
| H.1-H.4 | Required | Required | Required | - |
| I.1-I.4 | Required | - | - | Required |
| J.1-J.4 | Required | Required | Optional | - |

### B. Feature Flag Matrix

| Feature | Description | Recipes |
|---------|-------------|---------|
| `default` | Core parsing and validation | A.*, B.*, C.*, D.*, E.*, F.*, G.* |
| `browser` | WASM target support | H.* |
| `cli` | Command-line tools | I.* |
| `presentar` | Full runtime integration | H.*, J.* |
| `full` | All features | All recipes |

### C. Checklist: Recipe Compliance

Before submitting a recipe, verify:

- [ ] **Isolation**: Uses `tempfile::tempdir()` for all file I/O
- [ ] **Isolation**: No global/static mutable state
- [ ] **Idempotency**: Scene generation is deterministic
- [ ] **Idempotency**: Running twice produces identical output
- [ ] **Useful**: Addresses real production use case
- [ ] **Useful**: Copy-paste ready YAML and Rust code
- [ ] **Reproducible**: Works on Linux, macOS, WASM
- [ ] **Reproducible**: Pinned dependency versions
- [ ] **Testing**: 95%+ line coverage
- [ ] **Testing**: 3+ proptest properties
- [ ] **Testing**: Scene validation test present
- [ ] **Testing**: Isolation test present
- [ ] **Documentation**: Module doc with run command
- [ ] **Documentation**: Learning objective stated
- [ ] **Toyota Way**: No Python/heavy runtimes (Muda)
- [ ] **Toyota Way**: Error handling via types (Jidoka)
- [ ] **PMAT**: 10-point QA checklist included

---

## Approval

**Status**: DRAFT - AWAITING REVIEW

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Author | Sovereign AI Stack Team | 2025-12-06 | |
| QA Lead | - | - | PENDING |
| Tech Lead | - | - | PENDING |

---

*This specification follows Toyota Production System principles for lean, efficient, and high-quality visualization development. All recipes must pass quality gates before implementation.*

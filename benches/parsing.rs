//! Benchmarks for PRS scene parsing and validation
//!
//! Run with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use prs_cookbook::prelude::*;

/// Minimal scene YAML for baseline parsing
const MINIMAL_SCENE: &str = r#"
prs_version: "1.0"
metadata:
  name: "bench-minimal"
layout:
  type: flex
widgets: []
"#;

/// Scene with multiple widgets
fn scene_with_widgets(count: usize) -> String {
    let widgets: Vec<String> = (0..count)
        .map(|i| {
            format!(
                r#"  - id: widget-{}
    type: markdown
    config:
      content: "Widget {}""#,
                i, i
            )
        })
        .collect();

    format!(
        r#"prs_version: "1.0"
metadata:
  name: "bench-widgets-{}"
layout:
  type: flex
  direction: column
widgets:
{}"#,
        count,
        widgets.join("\n")
    )
}

/// Scene with resources
const SCENE_WITH_RESOURCES: &str = r##"
prs_version: "1.0"
metadata:
  name: "bench-resources"
resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"
    classifier:
      type: apr
      source: "./models/classifier.apr"
  datasets:
    sales:
      type: parquet
      source: "./data/sales.parquet"
    users:
      type: csv
      source: "./data/users.csv"
layout:
  type: grid
  columns: 3
  rows: 2
widgets:
  - id: header
    type: markdown
    position:
      row: 0
      col: 0
      colspan: 3
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
"##;

fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");

    // Minimal scene parsing
    group.bench_function("minimal_scene", |b| {
        b.iter(|| {
            let scene = Scene::from_yaml(black_box(MINIMAL_SCENE)).unwrap();
            black_box(scene)
        });
    });

    // Scene with resources
    group.bench_function("scene_with_resources", |b| {
        b.iter(|| {
            let scene = Scene::from_yaml(black_box(SCENE_WITH_RESOURCES)).unwrap();
            black_box(scene)
        });
    });

    // Varying widget counts
    for count in [10, 50, 100] {
        let yaml = scene_with_widgets(count);
        group.bench_with_input(BenchmarkId::new("widgets", count), &yaml, |b, yaml| {
            b.iter(|| {
                let scene = Scene::from_yaml(black_box(yaml)).unwrap();
                black_box(scene)
            });
        });
    }

    group.finish();
}

fn bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");

    // Pre-parse scenes for validation benchmarks
    let minimal = Scene::from_yaml(MINIMAL_SCENE).unwrap();
    let with_resources = Scene::from_yaml(SCENE_WITH_RESOURCES).unwrap();
    let with_100_widgets = Scene::from_yaml(&scene_with_widgets(100)).unwrap();

    group.bench_function("minimal_scene", |b| {
        b.iter(|| {
            let result = minimal.validate();
            black_box(result)
        });
    });

    group.bench_function("scene_with_resources", |b| {
        b.iter(|| {
            let result = with_resources.validate();
            black_box(result)
        });
    });

    group.bench_function("100_widgets", |b| {
        b.iter(|| {
            let result = with_100_widgets.validate();
            black_box(result)
        });
    });

    group.finish();
}

fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization");

    let minimal = Scene::from_yaml(MINIMAL_SCENE).unwrap();
    let with_resources = Scene::from_yaml(SCENE_WITH_RESOURCES).unwrap();

    group.bench_function("minimal_to_yaml", |b| {
        b.iter(|| {
            let yaml = minimal.to_yaml().unwrap();
            black_box(yaml)
        });
    });

    group.bench_function("resources_to_yaml", |b| {
        b.iter(|| {
            let yaml = with_resources.to_yaml().unwrap();
            black_box(yaml)
        });
    });

    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    group.bench_function("parse_validate_serialize", |b| {
        b.iter(|| {
            let scene = Scene::from_yaml(black_box(SCENE_WITH_RESOURCES)).unwrap();
            scene.validate().unwrap();
            let yaml = scene.to_yaml().unwrap();
            black_box(yaml)
        });
    });

    group.finish();
}

fn bench_context(c: &mut Criterion) {
    let mut group = c.benchmark_group("context");

    group.bench_function("create_context", |b| {
        b.iter(|| {
            let ctx = RecipeContext::new(black_box("bench_context")).unwrap();
            black_box(ctx)
        });
    });

    group.bench_function("hash_name_to_seed", |b| {
        b.iter(|| {
            let seed = hash_name_to_seed(black_box("bench_recipe_name"));
            black_box(seed)
        });
    });

    group.bench_function("generate_test_data_100", |b| {
        b.iter(|| {
            let data = generate_test_data(black_box(42), black_box(100));
            black_box(data)
        });
    });

    group.bench_function("generate_test_scene_10_widgets", |b| {
        b.iter(|| {
            let scene = generate_test_scene(black_box(42), black_box(10));
            black_box(scene)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parsing,
    bench_validation,
    bench_serialization,
    bench_roundtrip,
    bench_context,
);
criterion_main!(benches);

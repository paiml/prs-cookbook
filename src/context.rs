//! Recipe infrastructure for isolated, idempotent, and reproducible examples.
//!
//! This module provides the `RecipeContext` utility that ensures all recipes
//! follow the IIUR principles:
//!
//! - **Isolated**: Uses temp directories, no shared state
//! - **Idempotent**: Deterministic RNG seeded by recipe name
//! - **Useful**: Standardized reporting and metrics
//! - **Reproducible**: Cross-platform, CI-verified
//!
//! # Philosophy (Toyota Way)
//!
//! - **Jidoka**: Built-in quality via type-safe context
//! - **Muda**: Automatic cleanup eliminates resource waste
//! - **Heijunka**: Consistent recipe structure

use crate::error::{Error, Result};
use crate::scene::Scene;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Context for recipe execution providing isolation and reproducibility.
///
/// Each recipe creates a `RecipeContext` which provides:
/// - Isolated temporary directory (auto-cleanup on drop)
/// - Deterministic RNG seeded by recipe name
/// - Timing and reporting utilities
///
/// # Example
///
/// ```
/// use prs_cookbook::RecipeContext;
///
/// fn main() -> prs_cookbook::Result<()> {
///     let mut ctx = RecipeContext::new("my_recipe")?;
///     let scene_path = ctx.path("scene.prs");
///     // ... do work in isolated temp directory
///     ctx.record_metric("widgets", 5);
///     ctx.report_metrics()?;
///     Ok(())  // temp directory automatically cleaned up
/// }
/// ```
#[derive(Debug)]
pub struct RecipeContext {
    /// Recipe name for identification and seed generation
    name: String,
    /// Isolated temporary directory (auto-cleanup on drop)
    temp_dir: TempDir,
    /// Deterministic RNG seeded by recipe name hash
    rng: StdRng,
    /// Start time for duration tracking
    start_time: Instant,
    /// Collected metrics for reporting
    metrics: HashMap<String, MetricValue>,
    /// Recipe metadata
    metadata: RecipeMetadata,
}

/// Metadata about a recipe.
#[derive(Debug, Clone, Default)]
pub struct RecipeMetadata {
    /// Recipe name
    pub name: String,
    /// Category (e.g., "scene_creation", "widget_config")
    pub category: Option<String>,
    /// Learning objective
    pub objective: Option<String>,
    /// Required features
    pub features: Vec<String>,
}

/// A metric value that can be recorded.
#[derive(Debug, Clone)]
pub enum MetricValue {
    /// Integer metric (e.g., widget count)
    Int(i64),
    /// Float metric (e.g., throughput)
    Float(f64),
    /// Duration metric
    Duration(Duration),
    /// String metric
    String(String),
}

impl RecipeContext {
    /// Create a new recipe context with isolated environment.
    ///
    /// The RNG is seeded deterministically from the recipe name,
    /// ensuring reproducible results across runs.
    ///
    /// # Errors
    ///
    /// Returns an error if the temporary directory cannot be created.
    pub fn new(name: &str) -> Result<Self> {
        let seed = hash_name_to_seed(name);
        let temp_dir = tempfile::Builder::new()
            .prefix(&format!("prs-{}-", name))
            .tempdir()
            .map_err(Error::from)?;

        Ok(Self {
            name: name.to_string(),
            temp_dir,
            rng: StdRng::seed_from_u64(seed),
            start_time: Instant::now(),
            metrics: HashMap::new(),
            metadata: RecipeMetadata {
                name: name.to_string(),
                ..Default::default()
            },
        })
    }

    /// Create a context with custom metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the temporary directory cannot be created.
    pub fn with_metadata(name: &str, metadata: RecipeMetadata) -> Result<Self> {
        let mut ctx = Self::new(name)?;
        ctx.metadata = metadata;
        Ok(ctx)
    }

    /// Get the recipe name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the temporary directory path.
    pub fn temp_dir(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Get a path within the isolated temp directory.
    ///
    /// All file operations should use paths from this method to ensure
    /// isolation and automatic cleanup.
    pub fn path(&self, filename: &str) -> PathBuf {
        self.temp_dir.path().join(filename)
    }

    /// Get mutable access to the deterministic RNG.
    ///
    /// This RNG is seeded from the recipe name, so the same recipe
    /// will always produce the same sequence of random numbers.
    pub fn rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }

    /// Get the recipe metadata.
    pub fn metadata(&self) -> &RecipeMetadata {
        &self.metadata
    }

    /// Get elapsed time since context creation.
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Record an integer metric.
    pub fn record_metric(&mut self, name: &str, value: i64) {
        self.metrics
            .insert(name.to_string(), MetricValue::Int(value));
    }

    /// Record a float metric.
    pub fn record_float_metric(&mut self, name: &str, value: f64) {
        self.metrics
            .insert(name.to_string(), MetricValue::Float(value));
    }

    /// Record a duration metric.
    pub fn record_duration(&mut self, name: &str, duration: Duration) {
        self.metrics
            .insert(name.to_string(), MetricValue::Duration(duration));
    }

    /// Record a string metric.
    pub fn record_string_metric(&mut self, name: &str, value: impl Into<String>) {
        self.metrics
            .insert(name.to_string(), MetricValue::String(value.into()));
    }

    /// Get a recorded metric.
    pub fn get_metric(&self, name: &str) -> Option<&MetricValue> {
        self.metrics.get(name)
    }

    /// Write a scene to a file in the temp directory
    ///
    /// # Arguments
    /// * `filename` - Name of the file
    /// * `scene` - Scene to write
    ///
    /// # Returns
    /// Path to the written file
    pub fn write_scene(&self, filename: &str, scene: &Scene) -> Result<PathBuf> {
        let path = self.path(filename);
        let yaml = scene.to_yaml()?;
        std::fs::write(&path, yaml)?;
        Ok(path)
    }

    /// Read a scene from a file in the temp directory
    ///
    /// # Arguments
    /// * `filename` - Name of the file
    ///
    /// # Returns
    /// Parsed scene
    pub fn read_scene(&self, filename: &str) -> Result<Scene> {
        let path = self.path(filename);
        let yaml = std::fs::read_to_string(&path)?;
        Scene::from_yaml(&yaml)
    }

    /// Generate and print a report for a scene (legacy compatibility)
    pub fn report(&self, scene: &Scene) -> Result<()> {
        println!("=== Recipe: {} ===", self.name);
        println!("Duration: {:.2}ms", self.elapsed().as_secs_f64() * 1000.0);
        println!("Scene: {}", scene.metadata.name);
        if let Some(title) = &scene.metadata.title {
            println!("Title: {}", title);
        }
        println!("Version: {}", scene.prs_version);
        println!("Layout: {:?}", scene.layout.layout_type);
        println!("Widgets: {}", scene.widgets.len());
        println!("Bindings: {}", scene.bindings.len());
        println!("Models: {}", scene.resources.models.len());
        println!("Datasets: {}", scene.resources.datasets.len());
        if scene.theme.is_some() {
            println!("Theme: configured");
        }

        if !self.metrics.is_empty() {
            println!("Metrics:");
            for (name, value) in &self.metrics {
                match value {
                    MetricValue::Int(v) => println!("  {}: {}", name, v),
                    MetricValue::Float(v) => println!("  {}: {:.4}", name, v),
                    MetricValue::Duration(d) => {
                        println!("  {}: {:.2}ms", name, d.as_secs_f64() * 1000.0);
                    }
                    MetricValue::String(s) => println!("  {}: {}", name, s),
                }
            }
        }

        println!("===");
        Ok(())
    }

    /// Print a standardized report of recipe execution (metrics only).
    pub fn report_metrics(&self) -> Result<()> {
        println!("=== Recipe: {} ===", self.name);
        println!("Duration: {:.2}ms", self.elapsed().as_secs_f64() * 1000.0);

        if !self.metrics.is_empty() {
            println!("Metrics:");
            for (name, value) in &self.metrics {
                match value {
                    MetricValue::Int(v) => println!("  {}: {}", name, v),
                    MetricValue::Float(v) => println!("  {}: {:.4}", name, v),
                    MetricValue::Duration(d) => {
                        println!("  {}: {:.2}ms", name, d.as_secs_f64() * 1000.0);
                    }
                    MetricValue::String(s) => println!("  {}: {}", name, s),
                }
            }
        }

        println!("===");
        Ok(())
    }

    /// Verify that running the recipe twice produces the same output.
    ///
    /// This is a test helper for verifying idempotency.
    pub fn verify_idempotency<F, T>(&mut self, f: F) -> bool
    where
        F: Fn(&mut Self) -> T,
        T: PartialEq,
    {
        // Reset RNG to initial state
        let seed = hash_name_to_seed(&self.name);
        self.rng = StdRng::seed_from_u64(seed);
        let result1 = f(self);

        // Reset again and run
        self.rng = StdRng::seed_from_u64(seed);
        let result2 = f(self);

        result1 == result2
    }
}

impl RecipeMetadata {
    /// Create metadata from just a name.
    pub fn from_name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Set the category.
    #[must_use]
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set the learning objective.
    #[must_use]
    pub fn with_objective(mut self, objective: impl Into<String>) -> Self {
        self.objective = Some(objective.into());
        self
    }

    /// Add a required feature.
    #[must_use]
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }
}

/// Hash a recipe name to a deterministic u64 seed.
///
/// Uses BLAKE3 for consistent cross-platform hashing.
pub fn hash_name_to_seed(name: &str) -> u64 {
    let hash = blake3::hash(name.as_bytes());
    let bytes = hash.as_bytes();
    u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

/// Generate deterministic test data for a given seed.
///
/// Useful for creating reproducible test fixtures.
pub fn generate_test_data(seed: u64, size: usize) -> Vec<f32> {
    use rand::Rng;
    let mut rng = StdRng::seed_from_u64(seed);
    (0..size).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

/// Generate a deterministic scene payload for testing.
///
/// Creates reproducible scene YAML content.
pub fn generate_test_scene(seed: u64, widget_count: usize) -> String {
    use rand::Rng;
    let mut rng = StdRng::seed_from_u64(seed);

    let widgets: Vec<String> = (0..widget_count)
        .map(|i| {
            let widget_type = match rng.gen_range(0..3) {
                0 => "markdown",
                1 => "textbox",
                _ => "slider",
            };
            format!(
                r#"  - id: widget-{}
    type: {}
    config:
      label: "Widget {}""#,
                i, widget_type, i
            )
        })
        .collect();

    format!(
        r#"prs_version: "1.0"
metadata:
  name: "test-scene-{}"
layout:
  type: flex
  direction: column
widgets:
{}"#,
        seed,
        widgets.join("\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_context_creation() {
        let ctx = RecipeContext::new("test_recipe").unwrap();
        assert_eq!(ctx.name(), "test_recipe");
        assert!(ctx.temp_dir().exists());
    }

    #[test]
    fn test_recipe_context_path() {
        let ctx = RecipeContext::new("test_recipe").unwrap();
        let path = ctx.path("scene.prs");
        assert!(path.starts_with(ctx.temp_dir()));
        assert!(path.ends_with("scene.prs"));
    }

    #[test]
    fn test_deterministic_rng() {
        // Same recipe name should produce same RNG sequence
        let mut ctx1 = RecipeContext::new("deterministic_test").unwrap();
        let mut ctx2 = RecipeContext::new("deterministic_test").unwrap();

        use rand::Rng;
        let seq1: Vec<u64> = (0..10).map(|_| ctx1.rng().gen()).collect();
        let seq2: Vec<u64> = (0..10).map(|_| ctx2.rng().gen()).collect();

        assert_eq!(
            seq1, seq2,
            "Same recipe name should produce same RNG sequence"
        );
    }

    #[test]
    fn test_different_recipes_different_rng() {
        let mut ctx1 = RecipeContext::new("recipe_a").unwrap();
        let mut ctx2 = RecipeContext::new("recipe_b").unwrap();

        use rand::Rng;
        let val1: u64 = ctx1.rng().gen();
        let val2: u64 = ctx2.rng().gen();

        assert_ne!(
            val1, val2,
            "Different recipe names should produce different RNG"
        );
    }

    #[test]
    fn test_temp_dir_isolation() {
        let ctx1 = RecipeContext::new("isolation_test_1").unwrap();
        let ctx2 = RecipeContext::new("isolation_test_2").unwrap();

        assert_ne!(
            ctx1.temp_dir(),
            ctx2.temp_dir(),
            "Each context should have its own temp directory"
        );
    }

    #[test]
    fn test_metrics_recording() {
        let mut ctx = RecipeContext::new("metrics_test").unwrap();

        ctx.record_metric("widget_count", 10);
        ctx.record_float_metric("parse_time", 1.234);
        ctx.record_duration("validation_time", Duration::from_millis(42));
        ctx.record_string_metric("scene_name", "test-scene");

        match ctx.get_metric("widget_count") {
            Some(MetricValue::Int(v)) => assert_eq!(*v, 10),
            _ => panic!("Expected Int metric"),
        }

        match ctx.get_metric("parse_time") {
            Some(MetricValue::Float(v)) => assert!((v - 1.234).abs() < 0.001),
            _ => panic!("Expected Float metric"),
        }
    }

    #[test]
    fn test_hash_name_to_seed_deterministic() {
        let seed1 = hash_name_to_seed("my_recipe");
        let seed2 = hash_name_to_seed("my_recipe");
        assert_eq!(seed1, seed2);
    }

    #[test]
    fn test_hash_name_to_seed_different_names() {
        let seed1 = hash_name_to_seed("recipe_a");
        let seed2 = hash_name_to_seed("recipe_b");
        assert_ne!(seed1, seed2);
    }

    #[test]
    fn test_generate_test_data_deterministic() {
        let data1 = generate_test_data(42, 100);
        let data2 = generate_test_data(42, 100);
        assert_eq!(data1, data2);
    }

    #[test]
    fn test_generate_test_data_different_seeds() {
        let data1 = generate_test_data(42, 100);
        let data2 = generate_test_data(43, 100);
        assert_ne!(data1, data2);
    }

    #[test]
    fn test_generate_test_scene_deterministic() {
        let scene1 = generate_test_scene(42, 5);
        let scene2 = generate_test_scene(42, 5);
        assert_eq!(scene1, scene2);
    }

    #[test]
    fn test_recipe_metadata_builder() {
        let metadata = RecipeMetadata::from_name("test")
            .with_category("scene_creation")
            .with_objective("Learn scene parsing")
            .with_feature("browser");

        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.category, Some("scene_creation".to_string()));
        assert_eq!(metadata.objective, Some("Learn scene parsing".to_string()));
        assert_eq!(metadata.features, vec!["browser"]);
    }

    #[test]
    fn test_verify_idempotency() {
        let mut ctx = RecipeContext::new("idempotency_test").unwrap();

        let is_idempotent = ctx.verify_idempotency(|ctx| {
            use rand::Rng;
            ctx.rng().gen::<u64>()
        });

        assert!(is_idempotent, "Same RNG operations should be idempotent");
    }

    #[test]
    fn test_temp_dir_cleanup() {
        let path = {
            let ctx = RecipeContext::new("cleanup_test").unwrap();
            ctx.temp_dir().to_path_buf()
        };
        // After ctx is dropped, temp dir should be cleaned up
        assert!(
            !path.exists(),
            "Temp directory should be cleaned up on drop"
        );
    }

    #[test]
    fn test_elapsed_time() {
        let ctx = RecipeContext::new("elapsed_test").unwrap();
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = ctx.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_write_and_read_scene() {
        let ctx = RecipeContext::new("io_test").unwrap();

        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "test-scene"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();

        let path = ctx.write_scene("test.prs", &scene).unwrap();
        assert!(path.exists());

        let loaded = ctx.read_scene("test.prs").unwrap();
        assert_eq!(loaded.metadata.name, "test-scene");
    }
}

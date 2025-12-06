//! Recipe execution context for isolation and reproducibility
//!
//! The `RecipeContext` provides an isolated environment for recipe execution,
//! ensuring that all file I/O is contained within a temporary directory that
//! is automatically cleaned up when the context is dropped.

use crate::error::Result;
use crate::scene::Scene;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Isolated execution context for recipes
///
/// Provides:
/// - Automatic temporary directory creation and cleanup
/// - Path helpers for creating files within the isolated environment
/// - Report generation for recipe output
///
/// # Example
///
/// ```rust
/// use prs_cookbook::RecipeContext;
///
/// let ctx = RecipeContext::new("my-recipe").unwrap();
/// let scene_path = ctx.path("scene.prs");
/// // scene_path is within an isolated temp directory
/// // Directory is automatically cleaned up when ctx is dropped
/// ```
pub struct RecipeContext {
    /// Recipe name for identification
    name: String,
    /// Temporary directory (cleaned up on drop)
    temp: TempDir,
}

impl RecipeContext {
    /// Create a new recipe context with isolated temporary directory
    ///
    /// # Arguments
    /// * `name` - Recipe name used for identification and directory naming
    ///
    /// # Returns
    /// A new `RecipeContext` with an isolated temporary directory
    ///
    /// # Errors
    /// Returns an error if the temporary directory cannot be created
    pub fn new(name: &str) -> Result<Self> {
        let temp = tempfile::Builder::new()
            .prefix(&format!("prs-{}-", name))
            .tempdir()?;

        Ok(Self {
            name: name.to_string(),
            temp,
        })
    }

    /// Get the recipe name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the temporary directory path
    pub fn temp_dir(&self) -> &Path {
        self.temp.path()
    }

    /// Create a path within the temporary directory
    ///
    /// # Arguments
    /// * `filename` - Name of the file to create a path for
    ///
    /// # Returns
    /// Full path to the file within the temporary directory
    pub fn path(&self, filename: &str) -> PathBuf {
        self.temp.path().join(filename)
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

    /// Generate and print a report for a scene
    ///
    /// # Arguments
    /// * `scene` - Scene to report on
    pub fn report(&self, scene: &Scene) -> Result<()> {
        println!("=== Recipe: {} ===", self.name);
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
        println!("Temp dir: {:?}", self.temp_dir());
        println!("===");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_name() {
        let ctx = RecipeContext::new("test").unwrap();
        assert_eq!(ctx.name(), "test");
    }

    #[test]
    fn test_context_temp_dir_exists() {
        let ctx = RecipeContext::new("test").unwrap();
        assert!(ctx.temp_dir().exists());
        assert!(ctx.temp_dir().is_dir());
    }

    #[test]
    fn test_context_path_within_temp() {
        let ctx = RecipeContext::new("test").unwrap();
        let path = ctx.path("file.txt");
        assert!(path.starts_with(ctx.temp_dir()));
        assert!(path.ends_with("file.txt"));
    }

    #[test]
    fn test_context_write_and_read_scene() {
        let ctx = RecipeContext::new("test").unwrap();

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

    #[test]
    fn test_context_cleanup_on_drop() {
        let temp_path: PathBuf;
        {
            let ctx = RecipeContext::new("cleanup").unwrap();
            temp_path = ctx.temp_dir().to_path_buf();

            // Write a file to ensure directory has contents
            std::fs::write(ctx.path("test.txt"), "content").unwrap();
            assert!(temp_path.exists());
        }
        // After drop, directory should be removed
        assert!(!temp_path.exists());
    }

    #[test]
    fn test_context_isolation() {
        let ctx1 = RecipeContext::new("recipe1").unwrap();
        let ctx2 = RecipeContext::new("recipe2").unwrap();

        // Each context should have its own directory
        assert_ne!(ctx1.temp_dir(), ctx2.temp_dir());

        // Files in one context don't affect the other
        std::fs::write(ctx1.path("shared.txt"), "context1").unwrap();
        assert!(!ctx2.path("shared.txt").exists());
    }
}

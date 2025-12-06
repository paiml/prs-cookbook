//! Scene types for the Presentar Scene Format (.prs)
//!
//! This module contains all types needed to represent and manipulate `.prs` scenes.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Root scene structure representing a complete `.prs` manifest
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Scene {
    /// PRS format version (e.g., "1.0")
    pub prs_version: String,

    /// Scene metadata
    pub metadata: Metadata,

    /// Resource definitions (models, datasets)
    #[serde(default)]
    pub resources: Resources,

    /// Layout configuration
    pub layout: Layout,

    /// Widget definitions
    #[serde(default)]
    pub widgets: Vec<Widget>,

    /// Event bindings
    #[serde(default)]
    pub bindings: Vec<Binding>,

    /// Theme configuration
    #[serde(default)]
    pub theme: Option<Theme>,

    /// Permission configuration
    #[serde(default)]
    pub permissions: Permissions,
}

impl Scene {
    /// Parse a scene from YAML string
    ///
    /// # Example
    /// ```
    /// use prs_cookbook::Scene;
    ///
    /// let yaml = r#"
    /// prs_version: "1.0"
    /// metadata:
    ///   name: "my-scene"
    /// layout:
    ///   type: flex
    /// widgets: []
    /// "#;
    /// let scene = Scene::from_yaml(yaml).unwrap();
    /// assert_eq!(scene.metadata.name, "my-scene");
    /// ```
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let scene: Scene = serde_yaml::from_str(yaml)?;
        Ok(scene)
    }

    /// Serialize the scene to YAML string
    pub fn to_yaml(&self) -> Result<String> {
        let yaml = serde_yaml::to_string(self)?;
        Ok(yaml)
    }

    /// Serialize the scene to JSON string
    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(self)?;
        Ok(json)
    }

    /// Validate the scene structure and semantics
    ///
    /// Checks:
    /// - Version format is valid (X.Y pattern)
    /// - Metadata name is kebab-case
    /// - Widget IDs are unique
    /// - Binding targets reference existing widgets/models
    /// - Remote resources have hash verification
    /// - Layout constraints are satisfied
    pub fn validate(&self) -> Result<()> {
        self.validate_version()?;
        self.validate_metadata()?;
        self.validate_widget_ids()?;
        self.validate_layout()?;
        self.validate_resources()?;
        Ok(())
    }

    fn validate_version(&self) -> Result<()> {
        let parts: Vec<&str> = self.prs_version.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidVersionFormat(self.prs_version.clone()));
        }
        for part in parts {
            if part.parse::<u32>().is_err() {
                return Err(Error::InvalidVersionFormat(self.prs_version.clone()));
            }
        }
        Ok(())
    }

    fn validate_metadata(&self) -> Result<()> {
        if !is_kebab_case(&self.metadata.name) {
            return Err(Error::InvalidNameFormat(self.metadata.name.clone()));
        }
        Ok(())
    }

    fn validate_widget_ids(&self) -> Result<()> {
        let mut seen = HashSet::new();
        for widget in &self.widgets {
            if !seen.insert(&widget.id) {
                return Err(Error::DuplicateWidgetId(widget.id.clone()));
            }
        }
        Ok(())
    }

    fn validate_layout(&self) -> Result<()> {
        match self.layout.layout_type {
            LayoutType::Grid => {
                if self.layout.columns.is_none() {
                    return Err(Error::GridMissingColumns);
                }
            }
            LayoutType::Absolute => {
                if self.layout.width.is_none() || self.layout.height.is_none() {
                    return Err(Error::AbsoluteMissingDimensions);
                }
            }
            LayoutType::Flex => {}
        }
        Ok(())
    }

    fn validate_resources(&self) -> Result<()> {
        for (name, model) in &self.resources.models {
            let source = model.source.primary();
            let is_remote = source.starts_with("https://") || source.starts_with("http://");
            if is_remote && model.hash.is_none() {
                return Err(Error::RemoteMissingHash(name.clone()));
            }
        }
        for (name, dataset) in &self.resources.datasets {
            let source = dataset.source.primary();
            let is_remote = source.starts_with("https://") || source.starts_with("http://");
            if is_remote && dataset.hash.is_none() {
                return Err(Error::RemoteMissingHash(name.clone()));
            }
        }
        Ok(())
    }
}

/// Check if a string is valid kebab-case
fn is_kebab_case(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
}

/// Scene metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    /// Scene name (kebab-case identifier)
    pub name: String,

    /// Human-readable title
    #[serde(default)]
    pub title: Option<String>,

    /// Author name
    #[serde(default)]
    pub author: Option<String>,

    /// License identifier
    #[serde(default)]
    pub license: Option<String>,

    /// Descriptive tags
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Resources section containing models and datasets
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Resources {
    /// Model resources
    #[serde(default)]
    pub models: HashMap<String, Model>,

    /// Dataset resources
    #[serde(default)]
    pub datasets: HashMap<String, Dataset>,
}

/// Model resource definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Model {
    /// Model file type
    #[serde(rename = "type")]
    pub model_type: ModelType,

    /// Model source location
    #[serde(flatten)]
    pub source: Source,

    /// BLAKE3 hash for verification (required for remote sources)
    #[serde(default)]
    pub hash: Option<String>,

    /// File size in bytes
    #[serde(default)]
    pub size_bytes: Option<u64>,
}

/// Supported model types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ModelType {
    /// Aprender model format
    Apr,
    /// GGUF format (llama.cpp compatible)
    Gguf,
    /// SafeTensors format
    Safetensors,
}

/// Dataset resource definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dataset {
    /// Dataset file type
    #[serde(rename = "type")]
    pub dataset_type: DatasetType,

    /// Dataset source location
    #[serde(flatten)]
    pub source: Source,

    /// BLAKE3 hash for verification
    #[serde(default)]
    pub hash: Option<String>,

    /// File size in bytes
    #[serde(default)]
    pub size_bytes: Option<u64>,

    /// Data transforms to apply
    #[serde(default)]
    pub transforms: Vec<String>,
}

/// Supported dataset types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DatasetType {
    /// Aprender dataset format
    Ald,
    /// Apache Parquet
    Parquet,
    /// CSV format
    Csv,
}

/// Source location with optional fallbacks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Source {
    /// Single source
    Single { source: String },
    /// Multiple sources with fallback
    Multiple { sources: Vec<String> },
}

impl Source {
    /// Get the primary (first) source
    pub fn primary(&self) -> &str {
        match self {
            Source::Single { source } => source,
            Source::Multiple { sources } => sources.first().map_or("", String::as_str),
        }
    }

    /// Get all sources
    pub fn all(&self) -> Vec<&str> {
        match self {
            Source::Single { source } => vec![source],
            Source::Multiple { sources } => sources.iter().map(String::as_str).collect(),
        }
    }
}

/// Layout configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Layout {
    /// Layout type
    #[serde(rename = "type")]
    pub layout_type: LayoutType,

    /// Flex direction (for flex layout)
    #[serde(default)]
    pub direction: Option<Direction>,

    /// Number of columns (for grid layout)
    #[serde(default)]
    pub columns: Option<u32>,

    /// Number of rows (for grid layout)
    #[serde(default)]
    pub rows: Option<u32>,

    /// Gap between items in pixels
    #[serde(default)]
    pub gap: Option<u32>,

    /// Width in pixels (for absolute layout)
    #[serde(default)]
    pub width: Option<u32>,

    /// Height in pixels (for absolute layout)
    #[serde(default)]
    pub height: Option<u32>,

    /// Child layouts for nesting
    #[serde(default)]
    pub children: Vec<Layout>,
}

/// Layout type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LayoutType {
    /// Flexbox layout
    Flex,
    /// CSS Grid layout
    Grid,
    /// Absolute positioning
    Absolute,
}

/// Flex direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Row,
    Column,
}

/// Widget definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Widget {
    /// Unique widget identifier
    pub id: String,

    /// Widget type (e.g., "markdown", "textbox", "slider")
    #[serde(rename = "type")]
    pub widget_type: String,

    /// Position in layout (for grid/absolute)
    #[serde(default)]
    pub position: Option<Position>,

    /// Widget-specific configuration
    #[serde(default)]
    pub config: HashMap<String, serde_yaml::Value>,
}

/// Widget position in layout
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Position {
    /// Row index (for grid)
    #[serde(default)]
    pub row: Option<u32>,

    /// Column index (for grid)
    #[serde(default)]
    pub col: Option<u32>,

    /// Column span (for grid)
    #[serde(default)]
    pub colspan: Option<u32>,

    /// Row span (for grid)
    #[serde(default)]
    pub rowspan: Option<u32>,

    /// X coordinate (for absolute)
    #[serde(default)]
    pub x: Option<i32>,

    /// Y coordinate (for absolute)
    #[serde(default)]
    pub y: Option<i32>,

    /// Width (for absolute)
    #[serde(default)]
    pub width: Option<u32>,

    /// Height (for absolute)
    #[serde(default)]
    pub height: Option<u32>,
}

/// Event binding
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Binding {
    /// Trigger event (e.g., "slider.change")
    pub trigger: String,

    /// Debounce delay in milliseconds
    #[serde(default)]
    pub debounce_ms: Option<u32>,

    /// Condition for conditional execution
    #[serde(default)]
    pub condition: Option<String>,

    /// Actions to perform
    pub actions: Vec<Action>,
}

/// Binding action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    /// Target widget or model
    pub target: String,

    /// Action type (e.g., "refresh", "update")
    #[serde(default)]
    pub action: Option<String>,

    /// Input expression
    #[serde(default)]
    pub input: Option<String>,

    /// Additional parameters
    #[serde(flatten)]
    pub params: HashMap<String, serde_yaml::Value>,
}

/// Theme configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    /// Theme preset
    #[serde(default)]
    pub preset: Option<ThemePreset>,

    /// Custom color overrides
    #[serde(default)]
    pub colors: Option<ThemeColors>,

    /// Custom font configuration
    #[serde(default)]
    pub fonts: Option<ThemeFonts>,
}

/// Theme preset options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreset {
    Light,
    Dark,
    System,
}

/// Custom theme colors
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ThemeColors {
    #[serde(default)]
    pub primary: Option<String>,
    #[serde(default)]
    pub secondary: Option<String>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub surface: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

/// Custom theme fonts
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ThemeFonts {
    #[serde(default)]
    pub heading: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub mono: Option<String>,
}

/// Permission configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Permissions {
    /// Allowed network URLs (glob patterns)
    #[serde(default)]
    pub network: Vec<String>,

    /// Allowed filesystem paths (glob patterns)
    #[serde(default)]
    pub filesystem: Vec<String>,

    /// Clipboard access
    #[serde(default)]
    pub clipboard: bool,

    /// Camera access
    #[serde(default)]
    pub camera: bool,

    /// Microphone access
    #[serde(default)]
    pub microphone: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_kebab_case_valid() {
        assert!(is_kebab_case("hello"));
        assert!(is_kebab_case("hello-world"));
        assert!(is_kebab_case("my-app-123"));
        assert!(is_kebab_case("a1-b2-c3"));
    }

    #[test]
    fn test_is_kebab_case_invalid() {
        assert!(!is_kebab_case(""));
        assert!(!is_kebab_case("Hello"));
        assert!(!is_kebab_case("hello_world"));
        assert!(!is_kebab_case("hello world"));
        assert!(!is_kebab_case("-hello"));
        assert!(!is_kebab_case("hello-"));
        assert!(!is_kebab_case("hello--world"));
    }

    #[test]
    fn test_source_primary_single() {
        let source = Source::Single {
            source: "path/to/file".to_string(),
        };
        assert_eq!(source.primary(), "path/to/file");
    }

    #[test]
    fn test_source_primary_multiple() {
        let source = Source::Multiple {
            sources: vec!["first".to_string(), "second".to_string()],
        };
        assert_eq!(source.primary(), "first");
    }

    #[test]
    fn test_source_all() {
        let source = Source::Multiple {
            sources: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        };
        assert_eq!(source.all(), vec!["a", "b", "c"]);
    }
}

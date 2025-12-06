//! Error types for PRS Cookbook
//!
//! Provides structured error handling following the cookbook's reliability requirements.

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for PRS Cookbook operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during PRS Cookbook operations
#[derive(Debug, Error)]
pub enum Error {
    /// Scene file not found at specified path
    #[error("Scene not found: {0}")]
    SceneNotFound(PathBuf),

    /// Invalid scene YAML syntax or structure
    #[error("Invalid scene YAML: {0}")]
    InvalidYaml(#[from] serde_yaml::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Scene validation failed
    #[error("Scene validation failed: {0}")]
    ValidationFailed(String),

    /// Widget not found in scene
    #[error("Widget not found: {0}")]
    WidgetNotFound(String),

    /// Model not found in resources
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Dataset not found in resources
    #[error("Dataset not found: {0}")]
    DatasetNotFound(String),

    /// Binding references invalid target
    #[error("Invalid binding target: {0}")]
    InvalidBindingTarget(String),

    /// Duplicate widget ID detected
    #[error("Duplicate widget ID: {0}")]
    DuplicateWidgetId(String),

    /// Invalid metadata name format
    #[error(
        "Invalid metadata name '{0}': must be kebab-case (lowercase letters, numbers, and hyphens)"
    )]
    InvalidNameFormat(String),

    /// Grid layout missing required columns
    #[error("Grid layout requires 'columns' to be specified")]
    GridMissingColumns,

    /// Grid layout missing required rows
    #[error("Grid layout requires 'rows' to be specified")]
    GridMissingRows,

    /// Absolute layout missing dimensions
    #[error("Absolute layout requires 'width' and 'height' to be specified")]
    AbsoluteMissingDimensions,

    /// Remote resource missing hash verification
    #[error("Remote resource '{0}' requires hash verification")]
    RemoteMissingHash(String),

    /// Invalid PRS version format
    #[error("Invalid PRS version format: {0}")]
    InvalidVersionFormat(String),

    /// IO error during file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_scene_not_found() {
        let err = Error::SceneNotFound(PathBuf::from("/path/to/scene.prs"));
        assert!(err.to_string().contains("/path/to/scene.prs"));
    }

    #[test]
    fn test_error_display_validation_failed() {
        let err = Error::ValidationFailed("test error".to_string());
        assert!(err.to_string().contains("test error"));
    }

    #[test]
    fn test_error_display_duplicate_widget_id() {
        let err = Error::DuplicateWidgetId("my-widget".to_string());
        assert!(err.to_string().contains("my-widget"));
    }

    #[test]
    fn test_error_display_invalid_name_format() {
        let err = Error::InvalidNameFormat("Bad Name".to_string());
        assert!(err.to_string().contains("kebab-case"));
    }

    #[test]
    fn test_error_display_remote_missing_hash() {
        let err = Error::RemoteMissingHash("model.apr".to_string());
        assert!(err.to_string().contains("hash"));
    }
}

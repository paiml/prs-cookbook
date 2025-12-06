//! # PRS Cookbook
//!
//! Cookbook examples for the Presentar Scene Format (`.prs`) following
//! **IIUR Principles**: Isolated, Idempotent, Useful, and Reproducible.
//!
//! ## Quick Start
//!
//! ```rust
//! use prs_cookbook::prelude::*;
//!
//! let yaml = r##"
//! prs_version: "1.0"
//! metadata:
//!   name: "hello-world"
//! layout:
//!   type: flex
//!   direction: column
//! widgets:
//!   - id: greeting
//!     type: markdown
//!     config:
//!       content: "# Hello, Presentar!"
//! "##;
//!
//! let scene = Scene::from_yaml(yaml).unwrap();
//! assert_eq!(scene.metadata.name, "hello-world");
//! ```

pub mod context;
pub mod error;
pub mod scene;

pub use context::RecipeContext;
pub use error::{Error, Result};
pub use scene::*;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::context::RecipeContext;
    pub use crate::error::{Error, Result};
    pub use crate::scene::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use pretty_assertions::assert_eq;

    // ========================================
    // SCENE PARSING TESTS
    // ========================================

    #[test]
    fn test_parse_minimal_scene() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "test-scene"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert_eq!(scene.prs_version, "1.0");
        assert_eq!(scene.metadata.name, "test-scene");
    }

    #[test]
    fn test_parse_scene_with_title() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "my-app"
  title: "My Application"
  author: "Test Author"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert_eq!(scene.metadata.title, Some("My Application".to_string()));
        assert_eq!(scene.metadata.author, Some("Test Author".to_string()));
    }

    #[test]
    fn test_parse_flex_layout() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "flex-test"
layout:
  type: flex
  direction: column
  gap: 16
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert!(matches!(scene.layout.layout_type, LayoutType::Flex));
        assert_eq!(scene.layout.direction, Some(Direction::Column));
        assert_eq!(scene.layout.gap, Some(16));
    }

    #[test]
    fn test_parse_grid_layout() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "grid-test"
layout:
  type: grid
  columns: 3
  rows: 2
  gap: 8
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert!(matches!(scene.layout.layout_type, LayoutType::Grid));
        assert_eq!(scene.layout.columns, Some(3));
        assert_eq!(scene.layout.rows, Some(2));
    }

    #[test]
    fn test_parse_absolute_layout() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "absolute-test"
layout:
  type: absolute
  width: 800
  height: 600
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert!(matches!(scene.layout.layout_type, LayoutType::Absolute));
        assert_eq!(scene.layout.width, Some(800));
        assert_eq!(scene.layout.height, Some(600));
    }

    #[test]
    fn test_parse_widget() {
        let yaml = r##"
prs_version: "1.0"
metadata:
  name: "widget-test"
layout:
  type: flex
widgets:
  - id: greeting
    type: markdown
    config:
      content: "# Hello"
"##;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert_eq!(scene.widgets.len(), 1);
        assert_eq!(scene.widgets[0].id, "greeting");
        assert_eq!(scene.widgets[0].widget_type, "markdown");
    }

    #[test]
    fn test_parse_widget_with_position() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "position-test"
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
      content: "Header"
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let pos = scene.widgets[0].position.as_ref().unwrap();
        assert_eq!(pos.row, Some(0));
        assert_eq!(pos.col, Some(0));
        assert_eq!(pos.colspan, Some(3));
    }

    // ========================================
    // RESOURCES TESTS
    // ========================================

    #[test]
    fn test_parse_local_model_resource() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "model-test"
resources:
  models:
    sentiment:
      type: apr
      source: "./models/sentiment.apr"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let model = scene.resources.models.get("sentiment").unwrap();
        assert_eq!(model.model_type, ModelType::Apr);
        assert_eq!(model.source.primary(), "./models/sentiment.apr");
    }

    #[test]
    fn test_parse_remote_model_with_hash() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "remote-model-test"
resources:
  models:
    classifier:
      type: apr
      source: "https://example.com/model.apr"
      hash: "blake3:a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2"
      size_bytes: 45000000
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let model = scene.resources.models.get("classifier").unwrap();
        assert!(model.hash.is_some());
        assert_eq!(model.size_bytes, Some(45000000));
    }

    #[test]
    fn test_parse_dataset_resource() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "dataset-test"
resources:
  datasets:
    sales:
      type: parquet
      source: "./data/sales.parquet"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let dataset = scene.resources.datasets.get("sales").unwrap();
        assert_eq!(dataset.dataset_type, DatasetType::Parquet);
    }

    // ========================================
    // BINDINGS TESTS
    // ========================================

    #[test]
    fn test_parse_simple_binding() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "binding-test"
layout:
  type: flex
widgets:
  - id: slider
    type: slider
    config:
      min: 0
      max: 100
bindings:
  - trigger: "slider.change"
    actions:
      - target: display
        action: refresh
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert_eq!(scene.bindings.len(), 1);
        assert_eq!(scene.bindings[0].trigger, "slider.change");
        assert_eq!(scene.bindings[0].actions.len(), 1);
    }

    #[test]
    fn test_parse_debounced_binding() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "debounce-test"
layout:
  type: flex
widgets: []
bindings:
  - trigger: "input.change"
    debounce_ms: 300
    actions:
      - target: inference.model
        input: "{{ input.value }}"
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert_eq!(scene.bindings[0].debounce_ms, Some(300));
    }

    // ========================================
    // THEME TESTS
    // ========================================

    #[test]
    fn test_parse_theme_preset() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "theme-test"
layout:
  type: flex
widgets: []
theme:
  preset: "dark"
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert_eq!(
            scene.theme.as_ref().unwrap().preset,
            Some(ThemePreset::Dark)
        );
    }

    #[test]
    fn test_parse_custom_theme_colors() {
        let yaml = r##"
prs_version: "1.0"
metadata:
  name: "custom-theme-test"
layout:
  type: flex
widgets: []
theme:
  preset: "light"
  colors:
    primary: "#3498db"
    background: "#ffffff"
"##;
        let scene = Scene::from_yaml(yaml).unwrap();
        let theme = scene.theme.as_ref().unwrap();
        let colors = theme.colors.as_ref().unwrap();
        assert_eq!(colors.primary, Some("#3498db".to_string()));
    }

    // ========================================
    // PERMISSIONS TESTS
    // ========================================

    #[test]
    fn test_parse_network_permissions() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "permission-test"
layout:
  type: flex
widgets: []
permissions:
  network:
    - "https://api.example.com/*"
  clipboard: false
  camera: false
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert_eq!(scene.permissions.network.len(), 1);
        assert!(!scene.permissions.clipboard);
        assert!(!scene.permissions.camera);
    }

    // ========================================
    // VALIDATION TESTS
    // ========================================

    #[test]
    fn test_validate_valid_scene() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "valid-scene"
layout:
  type: flex
widgets:
  - id: widget1
    type: markdown
    config:
      content: "Hello"
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        assert!(scene.validate().is_ok());
    }

    #[test]
    fn test_validate_duplicate_widget_ids() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "duplicate-ids"
layout:
  type: flex
widgets:
  - id: same_id
    type: markdown
    config: {}
  - id: same_id
    type: textbox
    config: {}
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let result = scene.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Duplicate widget ID"));
    }

    #[test]
    fn test_validate_invalid_name_format() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "Invalid Name With Spaces"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let result = scene.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("kebab-case"));
    }

    #[test]
    fn test_validate_grid_requires_columns() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "grid-no-columns"
layout:
  type: grid
  rows: 2
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let result = scene.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("columns"));
    }

    #[test]
    fn test_validate_remote_model_requires_hash() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "remote-no-hash"
resources:
  models:
    test:
      type: apr
      source: "https://example.com/model.apr"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let result = scene.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("hash"));
    }

    // ========================================
    // SERIALIZATION TESTS
    // ========================================

    #[test]
    fn test_yaml_roundtrip() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "roundtrip-test"
layout:
  type: flex
  direction: column
widgets:
  - id: test
    type: markdown
    config:
      content: "Hello"
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let serialized = scene.to_yaml().unwrap();
        let reparsed = Scene::from_yaml(&serialized).unwrap();
        assert_eq!(scene.metadata.name, reparsed.metadata.name);
        assert_eq!(scene.widgets.len(), reparsed.widgets.len());
    }

    #[test]
    fn test_idempotent_serialization() {
        let yaml = r#"
prs_version: "1.0"
metadata:
  name: "idempotent-test"
layout:
  type: flex
widgets: []
"#;
        let scene = Scene::from_yaml(yaml).unwrap();
        let first = scene.to_yaml().unwrap();
        let second = scene.to_yaml().unwrap();
        assert_eq!(first, second, "Serialization must be idempotent");
    }

    // ========================================
    // RECIPE CONTEXT TESTS
    // ========================================

    #[test]
    fn test_recipe_context_creates_temp_dir() {
        let ctx = RecipeContext::new("test-recipe").unwrap();
        assert!(ctx.temp_dir().exists());
    }

    #[test]
    fn test_recipe_context_path_helper() {
        let ctx = RecipeContext::new("test-recipe").unwrap();
        let path = ctx.path("test.prs");
        assert!(path.ends_with("test.prs"));
        assert!(path.starts_with(ctx.temp_dir()));
    }

    #[test]
    fn test_recipe_context_cleanup_on_drop() {
        let temp_path;
        {
            let ctx = RecipeContext::new("cleanup-test").unwrap();
            temp_path = ctx.temp_dir().to_path_buf();
            assert!(temp_path.exists());
        }
        // After ctx is dropped, temp directory should be cleaned up
        assert!(!temp_path.exists());
    }
}

#[cfg(test)]
mod proptests {
    use super::prelude::*;
    use proptest::prelude::*;

    /// Strategy for generating valid kebab-case names
    fn kebab_case_name() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9]*(-[a-z0-9]+)*".prop_filter("name too long", |s| s.len() <= 50)
    }

    /// Strategy for generating valid widget IDs
    fn widget_id() -> impl Strategy<Value = String> {
        "[a-z][a-z0-9_-]*".prop_filter("id too long", |s| s.len() <= 30)
    }

    proptest! {
        /// Property: All valid kebab-case names should parse and validate
        #[test]
        fn prop_valid_names_parse_and_validate(name in kebab_case_name()) {
            let yaml = format!(r#"
prs_version: "1.0"
metadata:
  name: "{}"
layout:
  type: flex
widgets: []
"#, name);
            let scene = Scene::from_yaml(&yaml).expect("should parse");
            prop_assert!(scene.validate().is_ok(), "valid name should validate: {}", name);
        }

        /// Property: Parsing and re-serializing should preserve the scene name
        #[test]
        fn prop_yaml_roundtrip_preserves_name(name in kebab_case_name()) {
            let yaml = format!(r#"
prs_version: "1.0"
metadata:
  name: "{}"
layout:
  type: flex
widgets: []
"#, name);
            let scene = Scene::from_yaml(&yaml).expect("should parse");
            let reserialized = scene.to_yaml().expect("should serialize");
            let reparsed = Scene::from_yaml(&reserialized).expect("should reparse");
            prop_assert_eq!(scene.metadata.name, reparsed.metadata.name);
        }

        /// Property: Serialization is idempotent (f(x) == f(f(x)))
        #[test]
        fn prop_serialization_idempotent(name in kebab_case_name()) {
            let yaml = format!(r#"
prs_version: "1.0"
metadata:
  name: "{}"
layout:
  type: flex
widgets: []
"#, name);
            let scene = Scene::from_yaml(&yaml).expect("should parse");
            let first = scene.to_yaml().expect("first serialize");
            let second = scene.to_yaml().expect("second serialize");
            prop_assert_eq!(first, second, "serialization must be idempotent");
        }

        /// Property: Unique widget IDs should validate, duplicates should fail
        #[test]
        fn prop_unique_widget_ids_validate(
            id1 in widget_id(),
            id2 in widget_id()
        ) {
            let yaml = format!(r#"
prs_version: "1.0"
metadata:
  name: "test-scene"
layout:
  type: flex
widgets:
  - id: "{}"
    type: markdown
    config: {{}}
  - id: "{}"
    type: textbox
    config: {{}}
"#, id1, id2);
            let scene = Scene::from_yaml(&yaml).expect("should parse");
            let result = scene.validate();
            if id1 == id2 {
                prop_assert!(result.is_err(), "duplicate IDs should fail validation");
            } else {
                prop_assert!(result.is_ok(), "unique IDs should validate");
            }
        }

        /// Property: Grid columns/rows affect validation
        #[test]
        fn prop_grid_requires_columns(cols in proptest::option::of(1u32..20), rows in proptest::option::of(1u32..20)) {
            let cols_yaml = cols.map(|c| format!("columns: {}", c)).unwrap_or_default();
            let rows_yaml = rows.map(|r| format!("rows: {}", r)).unwrap_or_default();
            let yaml = format!(r#"
prs_version: "1.0"
metadata:
  name: "grid-test"
layout:
  type: grid
  {}
  {}
widgets: []
"#, cols_yaml, rows_yaml);
            let scene = Scene::from_yaml(&yaml).expect("should parse");
            let result = scene.validate();
            if cols.is_none() {
                prop_assert!(result.is_err(), "grid without columns should fail");
            } else {
                prop_assert!(result.is_ok(), "grid with columns should validate");
            }
        }

        /// Property: Remote models always require hash
        #[test]
        fn prop_remote_models_require_hash(
            has_hash in any::<bool>(),
        ) {
            let hash_yaml = if has_hash {
                "hash: \"blake3:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\""
            } else {
                ""
            };
            let yaml = format!(r#"
prs_version: "1.0"
metadata:
  name: "remote-model-test"
resources:
  models:
    test:
      type: apr
      source: "https://example.com/model.apr"
      {}
layout:
  type: flex
widgets: []
"#, hash_yaml);
            let scene = Scene::from_yaml(&yaml).expect("should parse");
            let result = scene.validate();
            if has_hash {
                prop_assert!(result.is_ok(), "remote with hash should validate");
            } else {
                prop_assert!(result.is_err(), "remote without hash should fail");
            }
        }

        /// Property: RecipeContext provides isolated temp directories
        #[test]
        fn prop_context_isolation(
            name1 in "[a-z]{3,10}",
            name2 in "[a-z]{3,10}"
        ) {
            let ctx1 = RecipeContext::new(&name1).expect("context1");
            let ctx2 = RecipeContext::new(&name2).expect("context2");
            prop_assert_ne!(ctx1.temp_dir(), ctx2.temp_dir(), "contexts must be isolated");
            prop_assert!(ctx1.temp_dir().exists());
            prop_assert!(ctx2.temp_dir().exists());
        }
    }
}

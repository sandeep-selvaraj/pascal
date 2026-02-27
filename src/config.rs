use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Top-level pascal.toml structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PascalConfig {
    pub workspace: WorkspaceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub name: String,
    pub python: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub packages: Option<Vec<String>>,
    #[serde(default)]
    pub apps: Option<Vec<String>>,
}

/// Minimal pyproject.toml representation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PyProject {
    pub project: Option<ProjectMeta>,
    #[serde(default)]
    pub tool: Option<ToolConfig>,
    #[serde(default, rename = "build-system")]
    pub build_system: Option<BuildSystem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMeta {
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "requires-python")]
    pub requires_python: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub scripts: Option<IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToolConfig {
    #[serde(default)]
    pub uv: Option<UvToolConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UvToolConfig {
    #[serde(default)]
    pub workspace: Option<UvWorkspaceConfig>,
    #[serde(default)]
    pub sources: Option<IndexMap<String, UvSource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UvWorkspaceConfig {
    #[serde(default)]
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UvSource {
    Workspace { workspace: bool },
    Path { path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildSystem {
    pub requires: Vec<String>,
    #[serde(rename = "build-backend")]
    pub build_backend: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── PascalConfig ─────────────────────────────────────────────────────────

    #[test]
    fn parse_minimal_pascal_toml() {
        let toml = r#"
[workspace]
name = "my-ws"
python = "3.12"
"#;
        let cfg: PascalConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.workspace.name, "my-ws");
        assert_eq!(cfg.workspace.python, "3.12");
        assert!(cfg.workspace.description.is_none());
        assert!(cfg.workspace.packages.is_none());
        assert!(cfg.workspace.apps.is_none());
    }

    #[test]
    fn parse_pascal_toml_with_explicit_lists() {
        let toml = r#"
[workspace]
name = "ws"
python = "3.11"
description = "a workspace"
packages = ["packages/cart", "packages/auth"]
apps     = ["apps/api"]
"#;
        let cfg: PascalConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.workspace.description.as_deref(), Some("a workspace"));
        assert_eq!(
            cfg.workspace.packages.as_deref(),
            Some(&["packages/cart".to_string(), "packages/auth".to_string()][..])
        );
        assert_eq!(
            cfg.workspace.apps.as_deref(),
            Some(&["apps/api".to_string()][..])
        );
    }

    // ── PyProject ────────────────────────────────────────────────────────────

    #[test]
    fn parse_pyproject_with_dependencies() {
        let toml = r#"
[project]
name = "api"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["cart", "httpx>=0.27"]
"#;
        let pp: PyProject = toml::from_str(toml).unwrap();
        let proj = pp.project.unwrap();
        assert_eq!(proj.name, "api");
        assert_eq!(proj.dependencies, vec!["cart", "httpx>=0.27"]);
        assert_eq!(proj.requires_python.as_deref(), Some(">=3.12"));
    }

    #[test]
    fn pyproject_defaults_to_empty_dependencies() {
        let toml = r#"
[project]
name = "cart"
"#;
        let pp: PyProject = toml::from_str(toml).unwrap();
        assert!(pp.project.unwrap().dependencies.is_empty());
    }

    // ── UvSource ─────────────────────────────────────────────────────────────

    #[test]
    fn parse_uv_source_workspace_variant() {
        let toml = r#"
[project]
name = "api"

[tool.uv.sources]
cart = { workspace = true }
"#;
        let pp: PyProject = toml::from_str(toml).unwrap();
        let sources = pp.tool.unwrap().uv.unwrap().sources.unwrap();
        assert!(matches!(
            sources["cart"],
            UvSource::Workspace { workspace: true }
        ));
    }

    #[test]
    fn parse_uv_source_path_variant() {
        let toml = r#"
[project]
name = "api"

[tool.uv.sources]
cart = { path = "../cart" }
"#;
        let pp: PyProject = toml::from_str(toml).unwrap();
        let sources = pp.tool.unwrap().uv.unwrap().sources.unwrap();
        assert!(matches!(&sources["cart"], UvSource::Path { path } if path == "../cart"));
    }
}

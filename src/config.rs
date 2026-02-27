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

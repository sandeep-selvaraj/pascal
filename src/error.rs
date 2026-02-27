use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum PascalError {
    #[error("No pascal.toml found in current directory or any parent")]
    WorkspaceNotFound,

    #[error("Workspace already exists at {0}")]
    WorkspaceExists(String),

    #[error("Package or app '{0}' not found in workspace")]
    BrickNotFound(String),

    #[error("Package or app '{0}' already exists")]
    BrickExists(String),

    #[error("Circular dependency detected: {0}")]
    CircularDependency(String),

    #[error("Invalid name '{0}': must be lowercase alphanumeric with underscores/hyphens")]
    InvalidName(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("{0}")]
    Other(String),
}

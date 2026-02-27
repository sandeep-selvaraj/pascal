use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::config::{PascalConfig, PyProject};
use crate::error::PascalError;

/// A single package or app in the workspace
#[derive(Debug, Clone)]
pub struct Brick {
    pub name: String,
    #[allow(dead_code)]
    pub kind: BrickKind,
    pub path: PathBuf,         // absolute path to the brick directory
    pub pyproject: PyProject,  // parsed pyproject.toml
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrickKind {
    Package,
    App,
}

impl BrickKind {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            BrickKind::Package => "package",
            BrickKind::App => "app",
        }
    }
}

/// The resolved workspace with all discovered bricks
#[derive(Debug)]
pub struct Workspace {
    pub root: PathBuf,
    pub config: PascalConfig,
    pub packages: Vec<Brick>,
    pub apps: Vec<Brick>,
}

impl Workspace {
    /// Walk up from CWD to find pascal.toml and load the workspace
    pub fn discover() -> Result<Self> {
        let root = find_workspace_root()?;
        Self::load_from(&root)
    }

    /// Load workspace from a specific root directory
    pub fn load_from(root: &Path) -> Result<Self> {
        let config_path = root.join("pascal.toml");
        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;
        let config: PascalConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", config_path.display()))?;

        let packages = discover_bricks(root, BrickKind::Package, &config)?;
        let apps = discover_bricks(root, BrickKind::App, &config)?;

        Ok(Workspace { root: root.to_path_buf(), config, packages, apps })
    }

    /// Find a brick by name (searches both packages and apps)
    pub fn find_brick(&self, name: &str) -> Option<&Brick> {
        self.packages.iter().chain(self.apps.iter()).find(|b| b.name == name)
    }

    /// All workspace member names
    pub fn member_names(&self) -> Vec<String> {
        self.packages.iter().chain(self.apps.iter()).map(|b| b.name.clone()).collect()
    }
}

/// Walk up from CWD until we find pascal.toml
pub fn find_workspace_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("pascal.toml").exists() {
            return Ok(dir);
        }
        match dir.parent() {
            Some(p) => dir = p.to_path_buf(),
            None => return Err(PascalError::WorkspaceNotFound.into()),
        }
    }
}

fn discover_bricks(root: &Path, kind: BrickKind, config: &PascalConfig) -> Result<Vec<Brick>> {
    let subdir = match kind {
        BrickKind::Package => "packages",
        BrickKind::App => "apps",
    };

    // Use explicit list if provided, otherwise auto-discover
    let explicit_paths: Option<Vec<PathBuf>> = match &kind {
        BrickKind::Package => config.workspace.packages.as_ref().map(|ps| {
            ps.iter().map(|p| root.join(p)).collect()
        }),
        BrickKind::App => config.workspace.apps.as_ref().map(|ps| {
            ps.iter().map(|p| root.join(p)).collect()
        }),
    };

    let dirs: Vec<PathBuf> = if let Some(paths) = explicit_paths {
        paths
    } else {
        // Auto-discover: scan <root>/<subdir>/*/pyproject.toml
        let base = root.join(subdir);
        if !base.exists() {
            return Ok(vec![]);
        }
        WalkDir::new(&base)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .filter(|e| e.path().join("pyproject.toml").exists())
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    let mut bricks = Vec::new();
    for dir in dirs {
        let pyproject_path = dir.join("pyproject.toml");
        if !pyproject_path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(&pyproject_path)
            .with_context(|| format!("Failed to read {}", pyproject_path.display()))?;
        let pyproject: PyProject = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", pyproject_path.display()))?;

        let name = pyproject
            .project
            .as_ref()
            .map(|p| p.name.replace('-', "_"))
            .unwrap_or_else(|| {
                dir.file_name().unwrap_or_default().to_string_lossy().into_owned()
            });

        bricks.push(Brick { name, kind: kind.clone(), path: dir, pyproject });
    }

    Ok(bricks)
}

/// Read and parse a pyproject.toml
pub fn read_pyproject(path: &Path) -> Result<PyProject> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))
}

/// Write a pyproject.toml (serialized from struct)
pub fn write_pyproject(path: &Path, pyproject: &PyProject) -> Result<()> {
    let content = toml::to_string_pretty(pyproject)?;
    std::fs::write(path, content)?;
    Ok(())
}

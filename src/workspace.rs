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
    pub path: PathBuf,        // absolute path to the brick directory
    pub pyproject: PyProject, // parsed pyproject.toml
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

        Ok(Workspace {
            root: root.to_path_buf(),
            config,
            packages,
            apps,
        })
    }

    /// Find a brick by name (searches both packages and apps)
    pub fn find_brick(&self, name: &str) -> Option<&Brick> {
        self.packages
            .iter()
            .chain(self.apps.iter())
            .find(|b| b.name == name)
    }

    /// All workspace member names
    pub fn member_names(&self) -> Vec<String> {
        self.packages
            .iter()
            .chain(self.apps.iter())
            .map(|b| b.name.clone())
            .collect()
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
        BrickKind::Package => config
            .workspace
            .packages
            .as_ref()
            .map(|ps| ps.iter().map(|p| root.join(p)).collect()),
        BrickKind::App => config
            .workspace
            .apps
            .as_ref()
            .map(|ps| ps.iter().map(|p| root.join(p)).collect()),
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
                dir.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            });

        bricks.push(Brick {
            name,
            kind: kind.clone(),
            path: dir,
            pyproject,
        });
    }

    Ok(bricks)
}

/// Read and parse a pyproject.toml
pub fn read_pyproject(path: &Path) -> Result<PyProject> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))
}

/// Write a pyproject.toml (serialized from struct)
pub fn write_pyproject(path: &Path, pyproject: &PyProject) -> Result<()> {
    let content = toml::to_string_pretty(pyproject)?;
    std::fs::write(path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ProjectMeta, WorkspaceConfig};

    fn make_brick(name: &str, kind: BrickKind) -> Brick {
        Brick {
            name: name.to_string(),
            kind,
            path: PathBuf::from(format!("/fake/{name}")),
            pyproject: PyProject::default(),
        }
    }

    fn make_workspace(packages: Vec<Brick>, apps: Vec<Brick>) -> Workspace {
        Workspace {
            root: PathBuf::from("/fake/root"),
            config: PascalConfig {
                workspace: WorkspaceConfig {
                    name: "test-ws".to_string(),
                    python: "3.12".to_string(),
                    description: None,
                    packages: None,
                    apps: None,
                },
            },
            packages,
            apps,
        }
    }

    // ── find_brick ───────────────────────────────────────────────────────────

    #[test]
    fn find_brick_finds_package_by_name() {
        let ws = make_workspace(vec![make_brick("cart", BrickKind::Package)], vec![]);
        assert!(ws.find_brick("cart").is_some());
    }

    #[test]
    fn find_brick_finds_app_by_name() {
        let ws = make_workspace(vec![], vec![make_brick("api", BrickKind::App)]);
        assert!(ws.find_brick("api").is_some());
    }

    #[test]
    fn find_brick_returns_none_for_unknown() {
        let ws = make_workspace(vec![make_brick("cart", BrickKind::Package)], vec![]);
        assert!(ws.find_brick("nope").is_none());
    }

    // ── member_names ─────────────────────────────────────────────────────────

    #[test]
    fn member_names_includes_packages_and_apps() {
        let ws = make_workspace(
            vec![
                make_brick("cart", BrickKind::Package),
                make_brick("auth", BrickKind::Package),
            ],
            vec![make_brick("api", BrickKind::App)],
        );
        let names = ws.member_names();
        assert_eq!(names, vec!["cart", "auth", "api"]);
    }

    #[test]
    fn member_names_empty_when_no_bricks() {
        let ws = make_workspace(vec![], vec![]);
        assert!(ws.member_names().is_empty());
    }

    // ── read_pyproject / write_pyproject ─────────────────────────────────────

    #[test]
    fn read_write_pyproject_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("pyproject.toml");

        let mut pp = PyProject::default();
        pp.project = Some(ProjectMeta {
            name: "cart".to_string(),
            version: Some("0.1.0".to_string()),
            dependencies: vec!["httpx".to_string()],
            ..Default::default()
        });

        write_pyproject(&path, &pp).unwrap();
        let loaded = read_pyproject(&path).unwrap();

        let proj = loaded.project.unwrap();
        assert_eq!(proj.name, "cart");
        assert_eq!(proj.version.as_deref(), Some("0.1.0"));
        assert_eq!(proj.dependencies, vec!["httpx"]);
    }

    #[test]
    fn read_pyproject_error_on_missing_file() {
        let result = read_pyproject(Path::new("/nonexistent/pyproject.toml"));
        assert!(result.is_err());
    }

    // ── load_from ────────────────────────────────────────────────────────────

    #[test]
    fn load_from_auto_discovers_packages_and_apps() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();

        // pascal.toml
        std::fs::write(
            root.join("pascal.toml"),
            "[workspace]\nname = \"ws\"\npython = \"3.12\"\n",
        )
        .unwrap();

        // packages/cart/pyproject.toml
        let cart = root.join("packages").join("cart");
        std::fs::create_dir_all(&cart).unwrap();
        std::fs::write(cart.join("pyproject.toml"), "[project]\nname = \"cart\"\n").unwrap();

        // apps/api/pyproject.toml
        let api = root.join("apps").join("api");
        std::fs::create_dir_all(&api).unwrap();
        std::fs::write(api.join("pyproject.toml"), "[project]\nname = \"api\"\n").unwrap();

        let ws = Workspace::load_from(root).unwrap();
        assert_eq!(ws.packages.len(), 1);
        assert_eq!(ws.packages[0].name, "cart");
        assert_eq!(ws.apps.len(), 1);
        assert_eq!(ws.apps[0].name, "api");
    }

    #[test]
    fn load_from_returns_empty_vecs_when_no_subdirs() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("pascal.toml"),
            "[workspace]\nname = \"ws\"\npython = \"3.12\"\n",
        )
        .unwrap();

        let ws = Workspace::load_from(dir.path()).unwrap();
        assert!(ws.packages.is_empty());
        assert!(ws.apps.is_empty());
    }
}

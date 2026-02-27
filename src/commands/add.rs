use anyhow::{bail, Result};
use indexmap::IndexMap;

use crate::config::{ToolConfig, UvSource, UvToolConfig};
use crate::display;
use crate::workspace::{read_pyproject, write_pyproject, Workspace};

pub fn run(package: &str, to: &str) -> Result<()> {
    let ws = Workspace::discover()?;

    // Verify the package exists in the workspace
    let pkg_brick = ws
        .packages
        .iter()
        .find(|b| b.name == package || b.name == package.replace('-', "_"))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Package '{}' not found in workspace. Available: {}",
                package,
                ws.packages
                    .iter()
                    .map(|b| b.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    // Find the target brick (app or package)
    let target_brick = ws
        .find_brick(to)
        .or_else(|| ws.find_brick(&to.replace('-', "_")))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Target '{}' not found in workspace. Available: {}",
                to,
                ws.member_names().join(", ")
            )
        })?;

    let pyproject_path = target_brick.path.join("pyproject.toml");
    let mut pyproject = read_pyproject(&pyproject_path)?;

    // Normalize the package name for the dependency string
    let dep_name = pkg_brick.name.replace('_', "-");

    // Check if dependency already present
    if let Some(project) = &pyproject.project {
        if project
            .dependencies
            .iter()
            .any(|d| d.starts_with(&dep_name))
        {
            display::warning(&format!(
                "'{}' is already a dependency of '{}'",
                dep_name, to
            ));
            return Ok(());
        }
    }

    // Add to [project.dependencies]
    if let Some(project) = &mut pyproject.project {
        project.dependencies.push(dep_name.clone());
    } else {
        bail!("Target '{}' has no [project] section in pyproject.toml", to);
    }

    // Add to [tool.uv.sources]
    let source_name = pkg_brick.name.replace('-', "_").replace('_', "-");
    let tool = pyproject.tool.get_or_insert_with(ToolConfig::default);
    let uv = tool.uv.get_or_insert_with(UvToolConfig::default);
    let sources = uv.sources.get_or_insert_with(IndexMap::new);
    sources.insert(source_name.clone(), UvSource::Workspace { workspace: true });

    write_pyproject(&pyproject_path, &pyproject)?;

    display::modified(&pyproject_path.to_string_lossy());
    println!();
    display::success(&format!("Added '{}' as a dependency of '{}'", dep_name, to));
    println!("  Run `pascal sync` to update the UV lockfile.");

    Ok(())
}

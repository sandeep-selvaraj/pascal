use anyhow::{bail, Result};

use crate::display;
use crate::uv::uv_run;
use crate::workspace::Workspace;

pub fn run(app_name: &str, extra: &[String]) -> Result<()> {
    let ws = Workspace::discover()?;

    let app = ws
        .apps
        .iter()
        .find(|a| a.name == app_name || a.name == app_name.replace('-', "_"))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "App '{}' not found. Available apps: {}",
                app_name,
                ws.apps.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", ")
            )
        })?;

    // Find the entry point script name from pyproject.toml [project.scripts]
    let entry = app
        .pyproject
        .project
        .as_ref()
        .and_then(|p| p.scripts.as_ref())
        .and_then(|scripts| scripts.keys().next().cloned())
        .unwrap_or_else(|| app.name.clone());

    display::info(&format!("Running '{}' via uv", entry));
    println!();

    let status = uv_run(&app.path, &entry, extra, &ws.root)?;

    if !status.success() {
        bail!("App '{}' exited with non-zero status", app.name);
    }

    Ok(())
}

use anyhow::{bail, Result};

use crate::display;
use crate::uv::uv_build;
use crate::workspace::Workspace;

pub fn run(app_name: &str) -> Result<()> {
    let ws = Workspace::discover()?;

    let app = ws
        .apps
        .iter()
        .find(|a| a.name == app_name || a.name == app_name.replace('-', "_"))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "App '{}' not found. Available apps: {}",
                app_name,
                ws.apps
                    .iter()
                    .map(|a| a.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;

    display::section_header(&format!("Building app: {}", app.name));
    println!();

    let status = uv_build(&app.path, &ws.root)?;
    println!();

    if !status.success() {
        bail!("Build failed for app '{}'", app.name);
    }

    display::success(&format!("App '{}' built successfully", app.name));
    Ok(())
}

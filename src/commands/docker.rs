use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::display;
use crate::workspace::{Brick, Workspace};

// ── helpers ──────────────────────────────────────────────────────────────────

/// Compute the image tag for an app.
/// Priority: explicit --tag > per-app registry > workspace registry > bare name:version
fn image_tag(ws: &Workspace, app: &Brick, override_tag: Option<&str>) -> String {
    if let Some(t) = override_tag {
        return t.to_string();
    }

    let version = app
        .pyproject
        .project
        .as_ref()
        .and_then(|p| p.version.as_deref())
        .unwrap_or("latest");

    let app_name = app.name.replace('_', "-");

    // Per-app registry from [tool.pascal.docker] in pyproject.toml
    let per_app_registry = app
        .pyproject
        .tool
        .as_ref()
        .and_then(|t| t.pascal.as_ref())
        .and_then(|p| p.docker.as_ref())
        .and_then(|d| d.registry.as_deref());

    // Workspace-level registry from [workspace.docker] in pascal.toml
    let ws_registry = ws
        .config
        .workspace
        .docker
        .as_ref()
        .and_then(|d| d.registry.as_deref());

    match per_app_registry.or(ws_registry) {
        Some(r) => format!("{}/{}:{}", r.trim_end_matches('/'), app_name, version),
        None => format!("{}:{}", app_name, version),
    }
}

/// Find the workspace packages this app directly depends on.
fn workspace_deps<'a>(ws: &'a Workspace, app: &Brick) -> Vec<&'a Brick> {
    let member_names = ws.member_names();

    let deps = app
        .pyproject
        .project
        .as_ref()
        .map(|p| p.dependencies.clone())
        .unwrap_or_default();

    deps.iter()
        .filter_map(|d| {
            let name = d
                .split(['>', '<', '=', '[', ';', ' '])
                .next()
                .unwrap_or(d)
                .replace('-', "_");
            if member_names.contains(&name) {
                ws.packages.iter().find(|b| b.name == name)
            } else {
                None
            }
        })
        .collect()
}

fn find_app<'a>(ws: &'a Workspace, app_name: &str) -> Result<&'a Brick> {
    ws.apps
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
        })
}

// ── public commands ───────────────────────────────────────────────────────────

pub fn build(
    app_name: &str,
    override_tag: Option<&str>,
    push: bool,
    platforms: &[String],
) -> Result<()> {
    let ws = Workspace::discover()?;
    let app = find_app(&ws, app_name)?;
    let tag = image_tag(&ws, app, override_tag);
    let deps = workspace_deps(&ws, app);

    display::section_header(&format!("Docker build: {}", app.name));
    println!();

    // Scaffold a Dockerfile if one doesn't exist yet
    let dockerfile_path = app.path.join("Dockerfile");
    if !dockerfile_path.exists() {
        let dep_names: Vec<&str> = deps.iter().map(|b| b.name.as_str()).collect();
        let content =
            crate::template::dockerfile(&app.name, &ws.config.workspace.python, &dep_names);
        std::fs::write(&dockerfile_path, content)
            .with_context(|| format!("Failed to write {}", dockerfile_path.display()))?;
        display::created(&dockerfile_path.to_string_lossy());
        println!("  (Dockerfile scaffolded — review before committing)");
        println!();
    }

    // Resolve platforms: CLI flag > per-app config > nothing
    let resolved_platforms: Vec<String> = if !platforms.is_empty() {
        platforms.to_vec()
    } else {
        app.pyproject
            .tool
            .as_ref()
            .and_then(|t| t.pascal.as_ref())
            .and_then(|p| p.docker.as_ref())
            .map(|d| d.platforms.clone())
            .unwrap_or_default()
    };

    // Print what we're about to do
    display::info(&format!("Tag:     {tag}"));
    if !deps.is_empty() {
        display::info(&format!(
            "Deps:    {}",
            deps.iter()
                .map(|b| b.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if !resolved_platforms.is_empty() {
        display::info(&format!("Platforms: {}", resolved_platforms.join(", ")));
    }
    println!();

    // Build the docker command
    let mut cmd = Command::new("docker");
    cmd.arg("build");
    cmd.args(["-t", &tag]);
    cmd.args([
        "-f",
        &app.path
            .join("Dockerfile")
            .strip_prefix(&ws.root)
            .unwrap_or(&app.path.join("Dockerfile"))
            .to_string_lossy(),
    ]);
    if !resolved_platforms.is_empty() {
        cmd.args(["--platform", &resolved_platforms.join(",")]);
    }
    if push {
        cmd.arg("--push");
    }
    cmd.arg("."); // build context = workspace root
    cmd.current_dir(&ws.root);

    let status = cmd
        .status()
        .with_context(|| "Failed to spawn `docker build`")?;

    println!();
    if !status.success() {
        bail!("docker build failed for '{}'", app.name);
    }

    display::success(&format!("Built:  {tag}"));
    if push {
        display::success("Pushed");
    }
    Ok(())
}

pub fn push(app_name: &str, override_tag: Option<&str>) -> Result<()> {
    let ws = Workspace::discover()?;
    let app = find_app(&ws, app_name)?;
    let tag = image_tag(&ws, app, override_tag);

    display::section_header(&format!("Docker push: {}", app.name));
    println!();
    display::info(&format!("Pushing: {tag}"));
    println!();

    let status = Command::new("docker")
        .args(["push", &tag])
        .current_dir(&ws.root)
        .status()
        .with_context(|| "Failed to spawn `docker push`")?;

    println!();
    if !status.success() {
        bail!("docker push failed for '{}'", app.name);
    }

    display::success(&format!("Pushed: {tag}"));
    Ok(())
}

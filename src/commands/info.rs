use anyhow::Result;
use colored::Colorize;

use crate::display;
use crate::workspace::Workspace;

pub fn run() -> Result<()> {
    let ws = Workspace::discover()?;

    display::section_header("Pascal Workspace");
    println!();

    display::kv("name", &ws.config.workspace.name);
    display::kv("python", &ws.config.workspace.python);
    if let Some(desc) = &ws.config.workspace.description {
        if !desc.is_empty() {
            display::kv("description", desc);
        }
    }
    display::kv("root", &ws.root.to_string_lossy());

    // Packages
    println!();
    println!(
        "  {} ({})",
        "Packages".bold().bright_blue(),
        ws.packages.len()
    );
    if ws.packages.is_empty() {
        println!("    {}", "(none)".dimmed());
    } else {
        for pkg in &ws.packages {
            let version = pkg
                .pyproject
                .project
                .as_ref()
                .and_then(|p| p.version.as_deref())
                .unwrap_or("?");
            let dep_count = pkg
                .pyproject
                .project
                .as_ref()
                .map(|p| p.dependencies.len())
                .unwrap_or(0);
            display::tree_item(1, "◆", &pkg.name, &format!("v{version}  {dep_count} deps"));
        }
    }

    // Apps
    println!();
    println!("  {} ({})", "Apps".bold().bright_blue(), ws.apps.len());
    if ws.apps.is_empty() {
        println!("    {}", "(none)".dimmed());
    } else {
        for app in &ws.apps {
            let version = app
                .pyproject
                .project
                .as_ref()
                .and_then(|p| p.version.as_deref())
                .unwrap_or("?");
            let deps = app
                .pyproject
                .project
                .as_ref()
                .map(|p| p.dependencies.clone())
                .unwrap_or_default();

            display::tree_item(
                1,
                "▶",
                &app.name,
                &format!("v{version}  {} deps", deps.len()),
            );

            // Show dependencies
            for dep in &deps {
                display::tree_item(2, "└─", dep, "");
            }
        }
    }

    println!();

    Ok(())
}

use anyhow::Result;
use colored::Colorize;

use crate::display;
use crate::git::{changed_files_since, latest_tag};
use crate::workspace::{Brick, Workspace};

pub fn run(since: Option<String>) -> Result<()> {
    let ws = Workspace::discover()?;

    // Resolve the git ref to compare against
    let git_ref = match since {
        Some(r) => r,
        None => {
            match latest_tag(&ws.root)? {
                Some(tag) => {
                    display::info(&format!("Comparing against latest tag: {tag}"));
                    tag
                }
                None => {
                    // Fall back to comparing against first commit
                    get_first_commit(&ws.root)?
                }
            }
        }
    };

    let changed = changed_files_since(&ws.root, &git_ref)?;

    display::section_header(&format!("Changes since '{git_ref}'"));
    println!();

    let all_bricks: Vec<&Brick> = ws.packages.iter().chain(ws.apps.iter()).collect();
    let _member_names: Vec<String> = all_bricks.iter().map(|b| b.name.clone()).collect();

    let mut changed_bricks: Vec<String> = Vec::new();

    for brick in &all_bricks {
        // Check if any changed file belongs to this brick
        let brick_rel = ws.root.join("packages")
            .join(&brick.name)
            .strip_prefix(&ws.root)
            .unwrap_or_else(|_| std::path::Path::new(""))
            .to_string_lossy()
            .into_owned();

        let brick_rel_app = ws.root.join("apps")
            .join(&brick.name)
            .strip_prefix(&ws.root)
            .unwrap_or_else(|_| std::path::Path::new(""))
            .to_string_lossy()
            .into_owned();

        // Also compute relative path of the brick from root
        let brick_path_rel = brick.path
            .strip_prefix(&ws.root)
            .unwrap_or(&brick.path)
            .to_string_lossy()
            .into_owned();

        let affected = changed.iter().any(|f| {
            f.starts_with(&brick_path_rel)
                || f.starts_with(&brick_rel)
                || f.starts_with(&brick_rel_app)
        });

        if affected {
            changed_bricks.push(brick.name.clone());
        }
    }

    if changed_bricks.is_empty() {
        display::info("No packages or apps changed");
        return Ok(());
    }

    println!("  {} Changed bricks:", "◈".bright_blue());
    for name in &changed_bricks {
        let kind = if ws.packages.iter().any(|b| &b.name == name) { "package" } else { "app" };
        println!("    {} {}  {}", "◆".yellow(), name.bold(), format!("[{kind}]").dimmed());
    }

    // Find apps that depend on changed packages
    let affected_apps: Vec<&Brick> = ws.apps.iter().filter(|app| {
        let deps = app
            .pyproject
            .project
            .as_ref()
            .map(|p| p.dependencies.clone())
            .unwrap_or_default();

        deps.iter().any(|d| {
            let name = d
                .split(['>', '<', '=', '[', ';', ' '])
                .next()
                .unwrap_or(d)
                .replace('-', "_");
            changed_bricks.contains(&name)
        })
    }).collect();

    if !affected_apps.is_empty() {
        println!();
        println!("  {} Apps affected by changed packages:", "◈".bright_blue());
        for app in &affected_apps {
            if !changed_bricks.contains(&app.name) {
                println!("    {} {}  {}", "▶".cyan(), app.name.bold(), "[app — transitive]".dimmed());
            }
        }
    }

    println!();

    Ok(())
}

fn get_first_commit(repo_path: &std::path::Path) -> Result<String> {
    use git2::Sort;

    let repo = git2::Repository::open(repo_path)?;
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

    let first = walk
        .next()
        .ok_or_else(|| anyhow::anyhow!("No commits in repository"))??;

    Ok(first.to_string())
}

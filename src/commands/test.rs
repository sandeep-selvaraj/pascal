use anyhow::{bail, Result};
use colored::Colorize;

use crate::display;
use crate::git::{changed_files_since, latest_tag};
use crate::uv::uv_test;
use crate::workspace::{Brick, Workspace};

pub fn run(changed: bool, name: Option<String>, extra: &[String]) -> Result<()> {
    let ws = Workspace::discover()?;

    let all_bricks: Vec<&Brick> = ws.packages.iter().chain(ws.apps.iter()).collect();

    // Determine which bricks to test
    let to_test: Vec<&Brick> = if let Some(ref target) = name {
        let brick = ws
            .find_brick(target)
            .ok_or_else(|| anyhow::anyhow!("Brick '{}' not found", target))?;
        vec![brick]
    } else if changed {
        // Find changed bricks since last tag
        let git_ref = match latest_tag(&ws.root)? {
            Some(tag) => {
                display::info(&format!("Detecting changes since tag: {tag}"));
                tag
            }
            None => {
                display::warning("No git tags found; testing all bricks");
                return run_all(&ws, extra);
            }
        };

        let changed_files = changed_files_since(&ws.root, &git_ref)?;
        all_bricks
            .iter()
            .copied()
            .filter(|brick| {
                let brick_rel = brick
                    .path
                    .strip_prefix(&ws.root)
                    .unwrap_or(&brick.path)
                    .to_string_lossy()
                    .into_owned();
                changed_files.iter().any(|f| f.starts_with(&brick_rel))
            })
            .collect()
    } else {
        all_bricks.clone()
    };

    if to_test.is_empty() {
        display::info("No bricks to test.");
        return Ok(());
    }

    display::section_header("Running Tests");
    println!();

    let mut failures = 0usize;

    for brick in &to_test {
        let tests_dir = brick.path.join("tests");
        if !tests_dir.exists() {
            display::warning(&format!("{}: no tests/ directory, skipping", brick.name));
            continue;
        }

        println!("  {} {}", "Testing".bold(), brick.name.bright_blue().bold());
        let status = uv_test(&brick.path, extra, &ws.root)?;
        if !status.success() {
            display::error(&format!("{} FAILED", brick.name));
            failures += 1;
        } else {
            display::success(&format!("{} passed", brick.name));
        }
        println!();
    }

    if failures > 0 {
        bail!("{} brick(s) had test failures", failures);
    }

    Ok(())
}

fn run_all(ws: &Workspace, extra: &[String]) -> Result<()> {
    let all: Vec<&Brick> = ws.packages.iter().chain(ws.apps.iter()).collect();
    let mut failures = 0;
    for brick in all {
        let tests_dir = brick.path.join("tests");
        if !tests_dir.exists() {
            continue;
        }
        println!("  {} {}", "Testing".bold(), brick.name.bright_blue().bold());
        let status = uv_test(&brick.path, extra, &ws.root)?;
        if !status.success() {
            failures += 1;
        }
    }
    if failures > 0 {
        bail!("{} brick(s) had test failures", failures);
    }
    Ok(())
}

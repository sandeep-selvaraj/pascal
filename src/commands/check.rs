use std::collections::HashMap;

use anyhow::Result;
use petgraph::algo::is_cyclic_directed;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::display;
use crate::workspace::{Brick, Workspace};

pub fn run() -> Result<()> {
    let ws = Workspace::discover()?;
    let all_bricks: Vec<&Brick> = ws.packages.iter().chain(ws.apps.iter()).collect();
    let member_names: Vec<String> = all_bricks.iter().map(|b| b.name.clone()).collect();

    display::section_header("Pascal Workspace Check");
    println!();

    let mut errors = 0usize;
    let mut warnings = 0usize;

    // Build dependency graph for cycle detection
    let mut g: DiGraph<String, ()> = DiGraph::new();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for brick in &all_bricks {
        let idx = g.add_node(brick.name.clone());
        node_map.insert(brick.name.clone(), idx);
    }

    for brick in &all_bricks {
        let deps = brick
            .pyproject
            .project
            .as_ref()
            .map(|p| p.dependencies.clone())
            .unwrap_or_default();

        for dep in &deps {
            let dep_norm = dep
                .split(['>', '<', '=', '[', ';', ' '])
                .next()
                .unwrap_or(dep)
                .replace('-', "_");

            if member_names.contains(&dep_norm) {
                if let (Some(&src), Some(&dst)) =
                    (node_map.get(&brick.name), node_map.get(&dep_norm))
                {
                    g.add_edge(src, dst, ());
                }
            }
        }
    }

    // Check for circular dependencies
    if is_cyclic_directed(&g) {
        display::error("Circular dependency detected in workspace graph");
        errors += 1;
    } else {
        display::success("No circular dependencies");
    }

    // Check that each app dependency on a workspace member is declared in [tool.uv.sources]
    for app in &ws.apps {
        let deps = app
            .pyproject
            .project
            .as_ref()
            .map(|p| p.dependencies.clone())
            .unwrap_or_default();

        let sources = app
            .pyproject
            .tool
            .as_ref()
            .and_then(|t| t.uv.as_ref())
            .and_then(|u| u.sources.as_ref());

        for dep in &deps {
            let dep_norm = dep
                .split(['>', '<', '=', '[', ';', ' '])
                .next()
                .unwrap_or(dep)
                .replace('-', "_");

            if member_names.contains(&dep_norm) {
                let in_sources = sources
                    .map(|s| {
                        s.contains_key(&dep_norm) || s.contains_key(&dep_norm.replace('_', "-"))
                    })
                    .unwrap_or(false);

                if !in_sources {
                    display::warning(&format!(
                        "{}: '{}' is a workspace dep but missing from [tool.uv.sources]",
                        app.name, dep_norm
                    ));
                    warnings += 1;
                }
            }
        }
    }

    // Check that each brick directory has the expected src layout
    for brick in &all_bricks {
        let snake = brick.name.replace('-', "_");
        let src_dir = brick.path.join("src").join(&snake);
        if !src_dir.exists() {
            display::warning(&format!(
                "{}: expected src/{snake}/ directory not found",
                brick.name
            ));
            warnings += 1;
        }
    }

    // Check for bricks with no pyproject.toml
    for brick in &all_bricks {
        if !brick.path.join("pyproject.toml").exists() {
            display::error(&format!("{}: missing pyproject.toml", brick.name));
            errors += 1;
        }
    }

    println!();

    if errors == 0 && warnings == 0 {
        display::success("Workspace is healthy");
    } else {
        if errors > 0 {
            display::error(&format!("{errors} error(s) found"));
        }
        if warnings > 0 {
            display::warning(&format!("{warnings} warning(s) found"));
        }
    }

    if errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}

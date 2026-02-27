use std::collections::HashMap;

use anyhow::Result;
use colored::Colorize;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::display;
use crate::workspace::{Brick, Workspace};

pub fn run(graph: bool) -> Result<()> {
    let ws = Workspace::discover()?;
    let all_bricks: Vec<&Brick> = ws.packages.iter().chain(ws.apps.iter()).collect();
    let member_names: Vec<String> = all_bricks.iter().map(|b| b.name.clone()).collect();

    // Build node index
    let mut g: DiGraph<String, ()> = DiGraph::new();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for brick in &all_bricks {
        let idx = g.add_node(brick.name.clone());
        node_map.insert(brick.name.clone(), idx);
    }

    // Add edges for workspace-internal dependencies
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

            if let Some(&dep_idx) = node_map.get(&dep_norm) {
                if let Some(&src_idx) = node_map.get(&brick.name) {
                    g.add_edge(src_idx, dep_idx, ());
                }
            }
        }
    }

    if graph {
        display::section_header("Dependency Graph");
        println!();
        print_graph(&g, &node_map, &all_bricks);
    } else {
        display::section_header("Dependencies");
        println!();
        print_list(&ws, &member_names);
    }

    Ok(())
}

fn print_list(ws: &Workspace, member_names: &[String]) {
    for brick in ws.packages.iter().chain(ws.apps.iter()) {
        let deps = brick
            .pyproject
            .project
            .as_ref()
            .map(|p| p.dependencies.clone())
            .unwrap_or_default();

        let internal: Vec<String> = deps
            .iter()
            .filter_map(|d| {
                let name = d
                    .split(['>', '<', '=', '[', ';', ' '])
                    .next()
                    .unwrap_or(d)
                    .replace('-', "_");
                if member_names.contains(&name) { Some(name) } else { None }
            })
            .collect();

        let external: Vec<String> = deps
            .iter()
            .filter_map(|d| {
                let name = d
                    .split(['>', '<', '=', '[', ';', ' '])
                    .next()
                    .unwrap_or(d)
                    .replace('-', "_");
                if !member_names.contains(&name) { Some(d.clone()) } else { None }
            })
            .collect();

        let kind_icon = if ws.packages.iter().any(|b| b.name == brick.name) {
            "◆"
        } else {
            "▶"
        };

        println!("  {} {}", kind_icon, brick.name.bold().bright_blue());

        if internal.is_empty() && external.is_empty() {
            println!("    {}", "(no dependencies)".dimmed());
        }
        for dep in &internal {
            println!("    {} {}", "→".green(), dep.green());
        }
        for dep in &external {
            println!("    {} {}", "→".dimmed(), dep.dimmed());
        }
        println!();
    }
}

fn print_graph(
    g: &DiGraph<String, ()>,
    node_map: &HashMap<String, NodeIndex>,
    all_bricks: &[&Brick],
) {
    // Print adjacency list style
    for brick in all_bricks {
        let idx = match node_map.get(&brick.name) {
            Some(&i) => i,
            None => continue,
        };
        let neighbors: Vec<String> = g
            .neighbors(idx)
            .map(|n| g[n].clone())
            .collect();

        print!("  {}", brick.name.bold());
        if neighbors.is_empty() {
            println!(" {}", "(no internal deps)".dimmed());
        } else {
            println!(" {} {}", "→".dimmed(), neighbors.join(", ").green());
        }
    }
    println!();
}

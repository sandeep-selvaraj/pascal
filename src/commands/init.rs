use anyhow::{bail, Result};

use crate::display;
use crate::template;

pub fn run(name: Option<String>, python: &str) -> Result<()> {
    let cwd = std::env::current_dir()?;

    // Determine workspace name
    let workspace_name = name.unwrap_or_else(|| {
        cwd.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    });

    // Validate name
    if workspace_name.is_empty() {
        bail!("Workspace name cannot be empty");
    }

    // Check if already initialized
    if cwd.join("pascal.toml").exists() {
        bail!("Workspace already initialized (pascal.toml exists)");
    }

    display::section_header("Pascal — Initializing Workspace");
    println!();

    // Create directory structure
    let dirs = ["packages", "apps"];
    for dir in &dirs {
        let path = cwd.join(dir);
        std::fs::create_dir_all(&path)?;
        display::created(dir);
    }

    // Write pascal.toml
    let pascal_toml = template::pascal_toml(&workspace_name, python);
    std::fs::write(cwd.join("pascal.toml"), &pascal_toml)?;
    display::created("pascal.toml");

    // Write root pyproject.toml
    let root_pyproject = template::root_pyproject(&workspace_name, python);
    std::fs::write(cwd.join("pyproject.toml"), &root_pyproject)?;
    display::created("pyproject.toml");

    // Write .gitignore if it doesn't exist
    let gitignore_path = cwd.join(".gitignore");
    if !gitignore_path.exists() {
        std::fs::write(
            &gitignore_path,
            "# Python\n__pycache__/\n*.pyc\n*.pyo\n.venv/\ndist/\n*.egg-info/\n\n# UV\n.uv/\nuv.lock\n",
        )?;
        display::created(".gitignore");
    }

    println!();
    display::success(&format!("Workspace '{}' initialized", workspace_name));
    println!();
    println!("  Next steps:");
    println!("    pascal create package <name>   — add a reusable package");
    println!("    pascal create app <name>       — add a deployable app");
    println!("    pascal info                    — view workspace overview");

    Ok(())
}

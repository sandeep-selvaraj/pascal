use anyhow::Result;

use crate::display;
use crate::template;
use crate::workspace::Workspace;

pub fn run() -> Result<()> {
    let ws = Workspace::discover()?;

    display::section_header("Pascal Sync");
    println!();

    // Regenerate root pyproject.toml
    let root_pyproject_path = ws.root.join("pyproject.toml");
    let content = template::root_pyproject(&ws.config.workspace.name, &ws.config.workspace.python);
    std::fs::write(&root_pyproject_path, &content)?;
    display::modified(&root_pyproject_path.to_string_lossy());

    println!();
    display::success("Root pyproject.toml regenerated");
    display::info("Run `uv sync` in the workspace root to update the lockfile.");

    Ok(())
}

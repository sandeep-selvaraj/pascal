use std::path::Path;

use anyhow::{bail, Result};

use crate::display;
use crate::template;
use crate::workspace::Workspace;

pub fn run_package(name: &str) -> Result<()> {
    let ws = Workspace::discover()?;
    validate_name(name)?;

    let pkg_dir = ws.root.join("packages").join(name);
    if pkg_dir.exists() {
        bail!("Package '{}' already exists at {}", name, pkg_dir.display());
    }

    display::section_header(&format!("Creating package: {name}"));
    println!();

    let python = &ws.config.workspace.python;
    scaffold_brick(&pkg_dir, name, python, false)?;

    println!();
    display::success(&format!("Package '{}' created", name));
    println!("  Run `pascal sync` to update the UV workspace.");

    Ok(())
}

pub fn run_app(name: &str) -> Result<()> {
    let ws = Workspace::discover()?;
    validate_name(name)?;

    let app_dir = ws.root.join("apps").join(name);
    if app_dir.exists() {
        bail!("App '{}' already exists at {}", name, app_dir.display());
    }

    display::section_header(&format!("Creating app: {name}"));
    println!();

    let python = &ws.config.workspace.python;
    scaffold_brick(&app_dir, name, python, true)?;

    println!();
    display::success(&format!("App '{}' created", name));
    println!("  Run `pascal sync` to update the UV workspace.");

    Ok(())
}

fn scaffold_brick(dir: &Path, name: &str, python: &str, is_app: bool) -> Result<()> {
    let snake = name.replace('-', "_");

    // Create directories
    let src_pkg = dir.join("src").join(&snake);
    let tests_dir = dir.join("tests");
    std::fs::create_dir_all(&src_pkg)?;
    std::fs::create_dir_all(&tests_dir)?;

    let rel = |path: &Path| -> String { path.to_string_lossy().into_owned() };

    // pyproject.toml
    let pyproject_content = if is_app {
        template::app_pyproject(name, python)
    } else {
        template::package_pyproject(name, python)
    };
    let pyproject_path = dir.join("pyproject.toml");
    std::fs::write(&pyproject_path, pyproject_content)?;
    display::created(&rel(&pyproject_path));

    // src/<name>/__init__.py
    let init_path = src_pkg.join("__init__.py");
    std::fs::write(&init_path, template::init_py(name))?;
    display::created(&rel(&init_path));

    // src/<name>/main.py (apps only)
    if is_app {
        let main_path = src_pkg.join("main.py");
        std::fs::write(&main_path, template::app_main_py(name))?;
        display::created(&rel(&main_path));
    }

    // tests/test_<name>.py
    let test_path = tests_dir.join(format!("test_{snake}.py"));
    std::fs::write(&test_path, template::test_stub_py(name))?;
    display::created(&rel(&test_path));

    Ok(())
}

fn validate_name(name: &str) -> Result<()> {
    let valid = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if !valid || name.is_empty() {
        bail!(
            "Invalid name '{}': must contain only lowercase letters, digits, underscores, or hyphens",
            name
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_name_accepts_simple_name() {
        assert!(validate_name("cart").is_ok());
    }

    #[test]
    fn validate_name_accepts_hyphens_and_underscores() {
        assert!(validate_name("my-pkg").is_ok());
        assert!(validate_name("my_pkg").is_ok());
        assert!(validate_name("my-pkg_v2").is_ok());
    }

    #[test]
    fn validate_name_accepts_digits() {
        assert!(validate_name("pkg2").is_ok());
    }

    #[test]
    fn validate_name_rejects_empty() {
        assert!(validate_name("").is_err());
    }

    #[test]
    fn validate_name_rejects_spaces() {
        assert!(validate_name("my pkg").is_err());
    }

    #[test]
    fn validate_name_rejects_dots() {
        assert!(validate_name("my.pkg").is_err());
    }

    #[test]
    fn validate_name_rejects_slashes() {
        assert!(validate_name("a/b").is_err());
    }
}

//! Templates for generated files

pub fn pascal_toml(name: &str, python: &str) -> String {
    format!(
        r#"[workspace]
name = "{name}"
python = "{python}"
description = ""

# Auto-discovered from packages/ and apps/ directories.
# Uncomment and edit to pin specific members:
# packages = ["packages/cart", "packages/auth"]
# apps     = ["apps/api_service"]
"#
    )
}

pub fn package_pyproject(name: &str, python: &str) -> String {
    let normalized = name.replace('_', "-");
    format!(
        r#"[project]
name = "{normalized}"
version = "0.1.0"
requires-python = ">={python}"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#
    )
}

pub fn app_pyproject(name: &str, python: &str) -> String {
    let normalized = name.replace('_', "-");
    let snake = name.replace('-', "_");
    format!(
        r#"[project]
name = "{normalized}"
version = "0.1.0"
requires-python = ">={python}"
dependencies = []

[project.scripts]
{snake} = "{snake}.main:main"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
"#
    )
}

pub fn init_py(name: &str) -> String {
    let snake = name.replace('-', "_");
    format!(
        r#"__version__ = "0.1.0"
__all__ = ["{snake}"]
"#
    )
}

pub fn app_main_py(name: &str) -> String {
    let snake = name.replace('-', "_");
    format!(
        r#"def main() -> None:
    print("Hello from {snake}!")


if __name__ == "__main__":
    main()
"#
    )
}

pub fn test_stub_py(name: &str) -> String {
    let snake = name.replace('-', "_");
    format!(
        r#"def test_{snake}() -> None:
    assert True
"#
    )
}

pub fn root_pyproject(workspace_name: &str, python: &str) -> String {
    format!(
        r#"[project]
name = "{workspace_name}"
version = "0.1.0"
requires-python = ">={python}"

[tool.uv.workspace]
members = ["packages/*", "apps/*"]
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── pascal_toml ──────────────────────────────────────────────────────────

    #[test]
    fn pascal_toml_contains_name_and_python() {
        let out = pascal_toml("my-ws", "3.12");
        assert!(out.contains("my-ws"));
        assert!(out.contains("3.12"));
    }

    #[test]
    fn pascal_toml_is_valid_toml() {
        let out = pascal_toml("ws", "3.11");
        toml::from_str::<toml::Value>(&out).expect("should be valid TOML");
    }

    // ── package_pyproject ────────────────────────────────────────────────────

    #[test]
    fn package_pyproject_normalizes_underscores_to_hyphens() {
        // project name in pyproject.toml should use hyphens
        let out = package_pyproject("my_pkg", "3.12");
        assert!(out.contains("my-pkg"));
    }

    #[test]
    fn package_pyproject_contains_python_constraint() {
        let out = package_pyproject("cart", "3.11");
        assert!(out.contains(">=3.11"));
    }

    #[test]
    fn package_pyproject_uses_hatchling() {
        let out = package_pyproject("cart", "3.12");
        assert!(out.contains("hatchling"));
    }

    #[test]
    fn package_pyproject_is_valid_toml() {
        let out = package_pyproject("cart", "3.12");
        toml::from_str::<toml::Value>(&out).expect("should be valid TOML");
    }

    // ── app_pyproject ────────────────────────────────────────────────────────

    #[test]
    fn app_pyproject_has_scripts_section() {
        let out = app_pyproject("api_service", "3.12");
        assert!(out.contains("[project.scripts]"));
    }

    #[test]
    fn app_pyproject_script_key_uses_snake_case() {
        let out = app_pyproject("api-service", "3.12");
        assert!(out.contains("api_service"));
    }

    #[test]
    fn app_pyproject_normalizes_hyphens_in_name() {
        let out = app_pyproject("my_app", "3.12");
        assert!(out.contains("my-app"));
    }

    #[test]
    fn app_pyproject_is_valid_toml() {
        let out = app_pyproject("api_service", "3.12");
        toml::from_str::<toml::Value>(&out).expect("should be valid TOML");
    }

    // ── init_py ──────────────────────────────────────────────────────────────

    #[test]
    fn init_py_contains_version_var() {
        let out = init_py("cart");
        assert!(out.contains("__version__"));
    }

    #[test]
    fn init_py_snake_cases_name() {
        let out = init_py("my-pkg");
        assert!(out.contains("my_pkg"));
        assert!(!out.contains("my-pkg"));
    }

    // ── app_main_py ──────────────────────────────────────────────────────────

    #[test]
    fn app_main_py_has_main_function() {
        let out = app_main_py("api_service");
        assert!(out.contains("def main()"));
    }

    #[test]
    fn app_main_py_has_dunder_main_guard() {
        let out = app_main_py("api_service");
        assert!(out.contains("__main__"));
    }

    // ── test_stub_py ─────────────────────────────────────────────────────────

    #[test]
    fn test_stub_py_function_matches_name() {
        let out = test_stub_py("cart");
        assert!(out.contains("def test_cart()"));
    }

    #[test]
    fn test_stub_py_snake_cases_hyphenated_name() {
        let out = test_stub_py("my-pkg");
        assert!(out.contains("def test_my_pkg()"));
    }

    // ── root_pyproject ───────────────────────────────────────────────────────

    #[test]
    fn root_pyproject_has_uv_workspace_members() {
        let out = root_pyproject("my-ws", "3.12");
        assert!(out.contains("[tool.uv.workspace]"));
        assert!(out.contains("packages/*"));
        assert!(out.contains("apps/*"));
    }

    #[test]
    fn root_pyproject_is_valid_toml() {
        let out = root_pyproject("my-ws", "3.12");
        toml::from_str::<toml::Value>(&out).expect("should be valid TOML");
    }
}

/// Templates for generated files

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

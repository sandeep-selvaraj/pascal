//! Integration tests: spawn the real `pascal` binary against a temp directory.

use std::path::Path;
use std::process::{Command, Output};

// Cargo injects the path to the compiled binary for integration tests.
fn pascal_bin() -> &'static str {
    env!("CARGO_BIN_EXE_pascal")
}

/// Run `pascal <args>` with `cwd` as the working directory.
fn run(args: &[&str], cwd: &Path) -> Output {
    Command::new(pascal_bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("failed to spawn pascal")
}

/// Assert the command succeeded, printing stdout/stderr on failure.
#[track_caller]
fn assert_ok(out: &Output) {
    if !out.status.success() {
        eprintln!("--- stdout ---\n{}", String::from_utf8_lossy(&out.stdout));
        eprintln!("--- stderr ---\n{}", String::from_utf8_lossy(&out.stderr));
        panic!("pascal exited with {}", out.status);
    }
}

/// Assert the command failed (non-zero exit).
#[track_caller]
fn assert_err(out: &Output) {
    assert!(
        !out.status.success(),
        "expected pascal to fail, but it exited with {}",
        out.status
    );
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// Init a workspace named `name` inside `dir`, return the workspace root.
fn init_workspace(dir: &Path, name: &str) -> std::path::PathBuf {
    let ws_dir = dir.join(name);
    std::fs::create_dir_all(&ws_dir).unwrap();
    assert_ok(&run(&["init", name], &ws_dir));
    ws_dir
}

// ── pascal init ───────────────────────────────────────────────────────────────

#[test]
fn init_creates_pascal_toml() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "my-ws");
    assert!(ws.join("pascal.toml").exists());
}

#[test]
fn init_creates_root_pyproject_toml() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "my-ws");
    assert!(ws.join("pyproject.toml").exists());
}

#[test]
fn init_creates_packages_and_apps_dirs() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "my-ws");
    assert!(ws.join("packages").is_dir());
    assert!(ws.join("apps").is_dir());
}

#[test]
fn init_embeds_workspace_name_in_pascal_toml() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "cool-ws");
    let contents = std::fs::read_to_string(ws.join("pascal.toml")).unwrap();
    assert!(contents.contains("cool-ws"));
}

#[test]
fn init_fails_if_already_initialized() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "my-ws");
    // second init in the same directory should fail
    assert_err(&run(&["init", "my-ws"], &ws));
}

// ── pascal create package ────────────────────────────────────────────────────

#[test]
fn create_package_generates_expected_files() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "package", "cart"], &ws));

    let pkg = ws.join("packages").join("cart");
    assert!(pkg.join("pyproject.toml").exists());
    assert!(pkg.join("src").join("cart").join("__init__.py").exists());
    assert!(pkg.join("tests").join("test_cart.py").exists());
}

#[test]
fn create_package_hyphenated_name_uses_snake_case_for_src_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "package", "my-pkg"], &ws));

    // src dir should be snake_case
    assert!(ws
        .join("packages")
        .join("my-pkg")
        .join("src")
        .join("my_pkg")
        .join("__init__.py")
        .exists());
}

#[test]
fn create_package_fails_on_duplicate() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "package", "cart"], &ws));
    assert_err(&run(&["create", "package", "cart"], &ws));
}

#[test]
fn create_package_fails_on_invalid_name() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_err(&run(&["create", "package", "bad name!"], &ws));
}

// ── pascal create app ────────────────────────────────────────────────────────

#[test]
fn create_app_generates_expected_files() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "app", "api"], &ws));

    let app = ws.join("apps").join("api");
    assert!(app.join("pyproject.toml").exists());
    assert!(app.join("src").join("api").join("__init__.py").exists());
    assert!(app.join("src").join("api").join("main.py").exists());
    assert!(app.join("tests").join("test_api.py").exists());
}

#[test]
fn create_app_pyproject_contains_scripts_section() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "app", "api"], &ws));

    let content =
        std::fs::read_to_string(ws.join("apps").join("api").join("pyproject.toml")).unwrap();
    assert!(content.contains("[project.scripts]"));
}

// ── pascal add ───────────────────────────────────────────────────────────────

#[test]
fn add_inserts_dependency_into_app_pyproject() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "package", "cart"], &ws));
    assert_ok(&run(&["create", "app", "api"], &ws));
    assert_ok(&run(&["add", "cart", "--to", "api"], &ws));

    let content =
        std::fs::read_to_string(ws.join("apps").join("api").join("pyproject.toml")).unwrap();
    assert!(content.contains("cart"));
}

#[test]
fn add_inserts_uv_sources_entry() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "package", "cart"], &ws));
    assert_ok(&run(&["create", "app", "api"], &ws));
    assert_ok(&run(&["add", "cart", "--to", "api"], &ws));

    let content =
        std::fs::read_to_string(ws.join("apps").join("api").join("pyproject.toml")).unwrap();
    // toml::to_string_pretty inlines the key: [tool.uv.sources.cart]
    assert!(content.contains("tool.uv.sources"));
    assert!(content.contains("workspace = true"));
}

#[test]
fn add_is_idempotent() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "package", "cart"], &ws));
    assert_ok(&run(&["create", "app", "api"], &ws));
    assert_ok(&run(&["add", "cart", "--to", "api"], &ws));
    // second add should succeed (no-op with a warning, not an error)
    assert_ok(&run(&["add", "cart", "--to", "api"], &ws));
}

#[test]
fn add_fails_for_unknown_package() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "app", "api"], &ws));
    assert_err(&run(&["add", "ghost", "--to", "api"], &ws));
}

// ── pascal check ─────────────────────────────────────────────────────────────

#[test]
fn check_passes_on_fresh_workspace() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    assert_ok(&run(&["create", "package", "cart"], &ws));
    assert_ok(&run(&["create", "app", "api"], &ws));
    // check exits 0 even with warnings about missing src dirs on a fresh create
    // (the src dirs do exist, so it should be clean)
    let out = run(&["check"], &ws);
    assert_ok(&out);
}

#[test]
fn check_fails_outside_workspace() {
    let tmp = tempfile::tempdir().unwrap();
    // no pascal.toml → should fail
    assert_err(&run(&["check"], tmp.path()));
}

// ── pascal sync ───────────────────────────────────────────────────────────────

#[test]
fn sync_regenerates_root_pyproject() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "ws");

    // overwrite the root pyproject to something wrong
    std::fs::write(ws.join("pyproject.toml"), "# corrupted").unwrap();

    assert_ok(&run(&["sync"], &ws));

    let content = std::fs::read_to_string(ws.join("pyproject.toml")).unwrap();
    assert!(content.contains("[tool.uv.workspace]"));
}

// ── full workflow ─────────────────────────────────────────────────────────────

#[test]
fn full_workflow_init_create_add_check() {
    let tmp = tempfile::tempdir().unwrap();
    let ws = init_workspace(tmp.path(), "shop");

    assert_ok(&run(&["create", "package", "cart"], &ws));
    assert_ok(&run(&["create", "package", "auth"], &ws));
    assert_ok(&run(&["create", "app", "storefront"], &ws));
    assert_ok(&run(&["add", "cart", "--to", "storefront"], &ws));
    assert_ok(&run(&["add", "auth", "--to", "storefront"], &ws));
    assert_ok(&run(&["check"], &ws));
    assert_ok(&run(&["info"], &ws));
    assert_ok(&run(&["deps"], &ws));
    assert_ok(&run(&["sync"], &ws));

    // storefront should declare both deps
    let content =
        std::fs::read_to_string(ws.join("apps").join("storefront").join("pyproject.toml")).unwrap();
    assert!(content.contains("cart"));
    assert!(content.contains("auth"));
}

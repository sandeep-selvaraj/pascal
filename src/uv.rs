use std::path::Path;
use std::process::{Command, ExitStatus};

use anyhow::{bail, Context, Result};

/// Run a uv subcommand in the given working directory.
/// Streams stdout/stderr directly to the terminal.
pub fn run_uv(args: &[&str], cwd: &Path) -> Result<ExitStatus> {
    let status = Command::new("uv")
        .args(args)
        .current_dir(cwd)
        .status()
        .with_context(|| format!("Failed to spawn `uv {}`", args.join(" ")))?;
    Ok(status)
}

/// Like run_uv but captures stdout and returns it as a String.
#[allow(dead_code)]
pub fn capture_uv(args: &[&str], cwd: &Path) -> Result<String> {
    let output = Command::new("uv")
        .args(args)
        .current_dir(cwd)
        .output()
        .with_context(|| format!("Failed to spawn `uv {}`", args.join(" ")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("uv {} failed:\n{}", args.join(" "), stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// `uv run --project <dir> <entry_point> [extra_args]`
pub fn uv_run(
    project_dir: &Path,
    entry: &str,
    extra: &[String],
    workspace_root: &Path,
) -> Result<ExitStatus> {
    let mut args: Vec<String> = vec![
        "run".into(),
        "--project".into(),
        project_dir.to_string_lossy().into_owned(),
        entry.into(),
    ];
    args.extend(extra.iter().cloned());
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_uv(&arg_refs, workspace_root)
}

/// `uv build --project <dir>`
pub fn uv_build(project_dir: &Path, workspace_root: &Path) -> Result<ExitStatus> {
    let dir_str = project_dir.to_string_lossy().into_owned();
    run_uv(&["build", "--project", &dir_str], workspace_root)
}

/// `uv run pytest <dir> [extra_args]`
pub fn uv_test(project_dir: &Path, extra: &[String], workspace_root: &Path) -> Result<ExitStatus> {
    let mut args: Vec<String> = vec![
        "run".into(),
        "--project".into(),
        project_dir.to_string_lossy().into_owned(),
        "pytest".into(),
        project_dir.join("tests").to_string_lossy().into_owned(),
    ];
    args.extend(extra.iter().cloned());
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_uv(&arg_refs, workspace_root)
}

/// `uv sync`
#[allow(dead_code)]
pub fn uv_sync(workspace_root: &Path) -> Result<ExitStatus> {
    run_uv(&["sync"], workspace_root)
}

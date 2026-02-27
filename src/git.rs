use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use git2::{Repository, Sort};

/// Find all files changed since a given ref (commit SHA, tag, or branch).
/// Returns relative paths from the repo root.
pub fn changed_files_since(repo_path: &Path, since_ref: &str) -> Result<HashSet<String>> {
    let repo = Repository::open(repo_path)
        .with_context(|| format!("Failed to open git repo at {}", repo_path.display()))?;

    let since_obj = repo
        .revparse_single(since_ref)
        .with_context(|| format!("Failed to resolve ref '{since_ref}'"))?;
    let since_commit = since_obj.peel_to_commit()?;
    let since_tree = since_commit.tree()?;

    // Get HEAD tree
    let head = repo.head()?.peel_to_commit()?;
    let head_tree = head.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&since_tree), Some(&head_tree), None)?;

    let mut paths = HashSet::new();
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                paths.insert(path.to_string_lossy().into_owned());
            }
            if let Some(path) = delta.old_file().path() {
                paths.insert(path.to_string_lossy().into_owned());
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(paths)
}

/// Find the latest tag in the repository.
pub fn latest_tag(repo_path: &Path) -> Result<Option<String>> {
    let repo = Repository::open(repo_path)?;

    let mut tag_names: Vec<(git2::Time, String)> = Vec::new();

    repo.tag_foreach(|oid, name_bytes| {
        let name = String::from_utf8_lossy(name_bytes)
            .trim_start_matches("refs/tags/")
            .to_string();
        if let Ok(obj) = repo.find_object(oid, None) {
            let time = if let Ok(tag) = obj.peel_to_commit() {
                tag.time()
            } else {
                git2::Time::new(0, 0)
            };
            tag_names.push((time, name));
        }
        true
    })?;

    tag_names.sort_by(|a, b| b.0.seconds().cmp(&a.0.seconds()));
    Ok(tag_names.into_iter().next().map(|(_, name)| name))
}

/// Find commits since a ref using the revwalk
#[allow(dead_code)]
pub fn commits_since(repo_path: &Path, since_ref: &str) -> Result<Vec<String>> {
    let repo = Repository::open(repo_path)?;

    let since_oid = repo.revparse_single(since_ref)?.id();
    let head_oid = repo.head()?.peel_to_commit()?.id();

    let mut walk = repo.revwalk()?;
    walk.push(head_oid)?;
    walk.set_sorting(Sort::TIME)?;
    walk.hide(since_oid)?;

    let commits = walk
        .filter_map(|r| r.ok())
        .filter_map(|oid| repo.find_commit(oid).ok())
        .map(|c| c.summary().unwrap_or("").to_string())
        .collect();

    Ok(commits)
}

# pascal sync

Regenerate the UV workspace root `pyproject.toml` from `pascal.toml`.

```
pascal sync
```

## What it does

Rewrites `<workspace-root>/pyproject.toml` with the correct UV workspace config:

```toml
[project]
name = "<workspace-name>"
version = "0.1.0"
requires-python = ">= <python>"

[tool.uv.workspace]
members = ["packages/*", "apps/*"]
```

## When to run

| Situation | Action |
|---|---|
| Added a new package or app with `pascal create` | `pascal sync && uv sync` |
| Changed `pascal.toml` (workspace name, python version) | `pascal sync && uv sync` |
| Root `pyproject.toml` got corrupted or manually edited | `pascal sync` |

!!! tip
    `pascal sync` regenerates the UV workspace config. To also install all packages and update the lockfile, follow it with `uv sync`.

## Difference from `uv sync`

| Command | What it does |
|---|---|
| `pascal sync` | Rewrites the root `pyproject.toml` (workspace config only) |
| `uv sync` | Reads `pyproject.toml` and `uv.lock`, installs packages, updates lockfile |

They're complementary: run `pascal sync` first, then `uv sync`.

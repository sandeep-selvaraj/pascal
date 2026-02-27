# UV Integration

Pascal is designed to sit **alongside** UV, not replace it. The division of responsibility is clean:

| Concern | Tool |
|---|---|
| Workspace structure, scaffolding, dependency wiring | **pascal** |
| Package installation, lockfile management, virtual envs | **uv** |
| Running scripts, building wheels, publishing | **uv** (via pascal shims) |

## How they fit together

```
pascal init        →  writes pascal.toml + UV workspace root pyproject.toml
pascal create      →  adds a new UV workspace member
pascal add         →  edits pyproject.toml, adds [tool.uv.sources] entry
pascal sync        →  regenerates UV workspace root pyproject.toml
pascal test        →  calls: uv run pytest
pascal build       →  calls: uv build
pascal run         →  calls: uv run <entry-point>
```

You can always drop into raw `uv` commands — pascal only ever writes standard UV workspace files.

## UV workspace model

A UV workspace is a monorepo layout where multiple `pyproject.toml` files share a single `uv.lock`. Pascal generates and maintains the workspace root:

```toml
# <workspace-root>/pyproject.toml
[tool.uv.workspace]
members = ["packages/*", "apps/*"]
```

Each member is a standard Python package with its own `pyproject.toml`. UV resolves all members together into a single lockfile.

## Path dependencies via `[tool.uv.sources]`

When you run `pascal add cart --to storefront`, pascal writes:

```toml
# apps/storefront/pyproject.toml
[project]
dependencies = ["cart"]

[tool.uv.sources]
cart = { workspace = true }
```

The `workspace = true` source tells UV to resolve `cart` from the local workspace rather than downloading it from PyPI.

## Typical workflow

```bash
# Day 1: set up
pascal init my-ws && cd my-ws
pascal create package cart
pascal create app storefront
pascal add cart --to storefront
uv sync                          # installs everything, creates uv.lock

# Day N: add a dependency
pascal add auth --to storefront
uv sync                          # updates lockfile

# Day N: run things
pascal test
pascal run storefront

# CI: targeted testing
pascal test --changed --since origin/main
```

## Lock file

`uv.lock` is managed entirely by UV. Pascal never touches it. Commit it to git — it ensures reproducible installs across machines and CI.

## Virtual environments

UV creates a single `.venv` at the workspace root shared by all members. You don't need to activate it manually — `uv run` handles it automatically.

```bash
# These all work without activating a venv:
uv run python -c "import cart"
uv run pytest packages/cart/tests/
pascal test cart
```

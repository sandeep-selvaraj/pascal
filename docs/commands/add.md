# pascal add

Add a workspace package as a dependency of another brick.

```
pascal add <PACKAGE> --to <TARGET>
```

## Arguments

| Argument | Description |
|---|---|
| `PACKAGE` | Name of the workspace package to add |
| `--to TARGET` | Name of the app (or package) that should depend on it |

## What it does

1. Appends `<package>` to the `dependencies` list in `<TARGET>/pyproject.toml`
2. Adds a `[tool.uv.sources]` entry so UV resolves it from the workspace rather than PyPI:

```toml
[tool.uv.sources]
cart = { workspace = true }
```

3. Prints a reminder to run `uv sync`

!!! note
    `pascal add` does **not** call `uv sync` automatically. Run `uv sync` after wiring dependencies to update the lockfile.

## Example

```bash
pascal create package cart
pascal create app storefront
pascal add cart --to storefront

# then install
uv sync
```

Resulting `apps/storefront/pyproject.toml`:

```toml
[project]
name = "storefront"
dependencies = ["cart"]

[tool.uv.sources.cart]
workspace = true
```

## Idempotent

Running `pascal add cart --to storefront` a second time is safe — pascal detects the dependency is already present and exits cleanly with a warning.

## Errors

| Condition | Message |
|---|---|
| Package not in workspace | `Package 'x' not found in workspace` |
| Target not in workspace | `Target 'x' not found in workspace` |
| Target has no `[project]` section | `Target 'x' has no [project] section` |

# pascal init

Bootstrap a new Pascal workspace in the current directory.

```
pascal init [NAME] [--python VERSION]
```

## Arguments

| Argument | Default | Description |
|---|---|---|
| `NAME` | current directory name | Workspace name written into `pascal.toml` |
| `--python` | `3.12` | Minimum Python version for the workspace |

## What it creates

```
./
  pascal.toml      # workspace manifest
  pyproject.toml   # UV workspace root
  .gitignore       # created if not already present
  packages/        # empty directory
  apps/            # empty directory
```

## Example

```bash
mkdir my-workspace && cd my-workspace
pascal init my-workspace --python 3.11
```

!!! note
    `pascal init` initialises the **current directory** as the workspace root. It does not create a subdirectory. Create and `cd` into your directory first.

## Errors

| Condition | Message |
|---|---|
| `pascal.toml` already exists | `Workspace already initialized (pascal.toml exists)` |
| Empty name | `Workspace name cannot be empty` |

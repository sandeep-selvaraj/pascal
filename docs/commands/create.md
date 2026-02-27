# pascal create

Scaffold a new package or app inside the workspace.

```
pascal create package <NAME>
pascal create app     <NAME>
```

## Subcommands

### `create package`

Creates a reusable library under `packages/<NAME>/`.

**Files generated:**

```
packages/<NAME>/
  pyproject.toml
  src/
    <name_snake>/
      __init__.py
  tests/
    test_<name_snake>.py
```

### `create app`

Creates a deployable application under `apps/<NAME>/`.

**Files generated:**

```
apps/<NAME>/
  pyproject.toml              # includes [project.scripts]
  src/
    <name_snake>/
      __init__.py
      main.py
  tests/
    test_<name_snake>.py
```

## Name rules

- Allowed characters: ASCII letters, digits, hyphens (`-`), underscores (`_`)
- Hyphens and underscores are equivalent for lookup; the `pyproject.toml` name uses hyphens, the `src/` directory uses underscores

## Examples

```bash
pascal create package cart
pascal create package my-utils
pascal create app storefront
pascal create app data-pipeline
```

## After creating

Run `pascal sync` to update the UV workspace root `pyproject.toml` and then `uv sync` to install the new member:

```bash
pascal sync
uv sync
```

## Errors

| Condition | Message |
|---|---|
| Directory already exists | `Package 'x' already exists at …` |
| Invalid name | `Invalid name 'x': must contain only …` |
| Not inside a workspace | `pascal.toml not found` |

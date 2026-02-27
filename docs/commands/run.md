# pascal run

Run an app's entry-point via `uv run`.

```
pascal run <APP> [-- <ARGS>]
```

## Arguments

| Argument | Description |
|---|---|
| `APP` | Name of the app to run |
| `-- <ARGS>` | Arguments forwarded to the app's entry-point |

## What it does

Resolves the app's `[project.scripts]` entry and calls:

```bash
uv run --project apps/<APP> <entry-point> [args]
```

## Example

```bash
# Run the app
pascal run storefront

# Pass arguments to the app
pascal run storefront -- --port 8080 --debug
```

## Entry-point resolution

The entry-point is derived from the app's `[project.scripts]` in `pyproject.toml`. For an app named `storefront`, the generated script is:

```toml
[project.scripts]
storefront = "storefront.main:main"
```

So `pascal run storefront` calls `storefront.main:main`.

!!! note
    `pascal run` requires `uv` to be available on `PATH`. The app and its workspace dependencies are automatically available in the UV-managed environment.

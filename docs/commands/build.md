# pascal build

Build a wheel and/or sdist for an app.

```
pascal build <APP>
```

## Arguments

| Argument | Description |
|---|---|
| `APP` | Name of the app to build |

## What it does

Runs `uv build` in the app's directory, producing a wheel (`.whl`) and source distribution (`.tar.gz`) under `apps/<APP>/dist/`.

## Example

```bash
pascal build storefront
```

```
  Building app: storefront

  uv build → apps/storefront/dist/storefront-0.1.0-py3-none-any.whl
  uv build → apps/storefront/dist/storefront-0.1.0.tar.gz

✓ Build complete
```

## Output location

```
apps/<APP>/
  dist/
    <app>-<version>-py3-none-any.whl
    <app>-<version>.tar.gz
```

!!! note
    `pascal build` only applies to apps, not packages. Packages are typically published as library wheels — run `uv build` directly inside `packages/<name>/` if needed.

# Quickstart

This guide walks you through building a small Python monorepo from scratch using pascal. It takes about five minutes.

## Prerequisites

- `pascal` installed ([see Installation](installation.md))
- `uv` installed (`curl -LsSf https://astral.sh/uv/install.sh | sh`)

---

## Step 1 — Bootstrap a workspace

```bash
mkdir shop && cd shop
pascal init shop
```

```
┌──────────────────────────────────────────┐
│       Pascal — Initializing Workspace    │
└──────────────────────────────────────────┘

  create  packages/
  create  apps/
  create  pascal.toml
  create  pyproject.toml
  create  .gitignore

✓ Workspace 'shop' initialized
```

This creates:

```
shop/
  pascal.toml      # workspace manifest
  pyproject.toml   # UV workspace root (auto-managed)
  .gitignore
  packages/
  apps/
```

---

## Step 2 — Add a reusable package

```bash
pascal create package cart
```

```
  create  packages/cart/pyproject.toml
  create  packages/cart/src/cart/__init__.py
  create  packages/cart/tests/test_cart.py

✓ Package 'cart' created
```

---

## Step 3 — Add a deployable app

```bash
pascal create app storefront
```

```
  create  apps/storefront/pyproject.toml
  create  apps/storefront/src/storefront/__init__.py
  create  apps/storefront/src/storefront/main.py
  create  apps/storefront/tests/test_storefront.py

✓ App 'storefront' created
```

---

## Step 4 — Wire the package into the app

```bash
pascal add cart --to storefront
```

This updates `apps/storefront/pyproject.toml` to declare `cart` as a dependency and adds the `[tool.uv.sources]` entry so UV resolves it from the workspace:

```toml
[project]
name = "storefront"
dependencies = ["cart"]

[tool.uv.sources]
cart = { workspace = true }
```

Then sync the UV lockfile:

```bash
uv sync
```

---

## Step 5 — Inspect and validate

```bash
pascal info
```

```
  Workspace: shop  (python 3.12)
  ├── packages
  │   └── cart  0.1.0
  └── apps
      └── storefront  0.1.0
          └── depends on: cart
```

```bash
pascal check
```

```
✓ No circular dependencies
✓ Workspace is healthy
```

```bash
pascal deps
```

```
  ◆ cart
    (no dependencies)

  ▶ storefront
    → cart
```

---

## Step 6 — Run tests

```bash
pascal test
```

Pascal calls `uv run pytest` for each brick in dependency order.

To run tests only for packages that changed since the last git tag:

```bash
pascal test --changed
```

---

## What's next?

- Explore the full [command reference](commands/index.md)
- Understand the [workspace layout](workspace.md)
- See how to use pascal in [CI/CD pipelines](ci-cd.md)

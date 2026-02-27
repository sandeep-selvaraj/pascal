# pascal

[![PyPI version](https://img.shields.io/pypi/v/pascal-cli.svg)](https://pypi.org/project/pascal-cli/)
[![CI](https://github.com/sandeep-selvaraj/pascal/actions/workflows/ci.yml/badge.svg)](https://github.com/sandeep-selvaraj/pascal/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Fast Python monorepo manager powered by Rust and UV.**

Pascal handles workspace scaffolding, dependency wiring, and UV workspace sync — so you can focus on code, not configuration.

**[Documentation](https://sandeep-selvaraj.github.io/pascal)** · [Installation](https://sandeep-selvaraj.github.io/pascal/installation/) · [Quickstart](https://sandeep-selvaraj.github.io/pascal/quickstart/) · [Commands](https://sandeep-selvaraj.github.io/pascal/commands/)

> Heavily inspired by [Polylith](https://polylith.gitbook.io/polylith) — *packages* map to Polylith's components, *apps* to its bases, brought to the Python/UV ecosystem with a minimal footprint.

---

## Installation

```bash
uv tool install pascal-cli   # recommended
pipx install pascal-cli
pip install pascal-cli
```

```bash
pascal --version
```

---

## Quickstart

```bash
# Bootstrap a workspace
mkdir shop && cd shop
pascal init shop

# Add a library and an app
pascal create package cart
pascal create app storefront

# Wire them together
pascal add cart --to storefront
uv sync

# Validate and inspect
pascal check
pascal info
pascal test
```

---

## Workspace layout

After the quickstart above, your workspace looks like this:

```
shop/
  pascal.toml          # workspace manifest
  pyproject.toml       # UV workspace root — managed by pascal
  uv.lock              # lockfile — commit to git
  packages/
    cart/
      pyproject.toml
      src/cart/__init__.py
      tests/test_cart.py
  apps/
    storefront/
      pyproject.toml   # depends on cart
      src/storefront/__init__.py
      src/storefront/main.py
      tests/test_storefront.py
```

**`pascal.toml`**

```toml
[workspace]
name = "shop"
python = "3.12"
description = "My Python monorepo"

# Optional — pascal auto-discovers from packages/ and apps/ if omitted
packages = ["packages/cart"]
apps     = ["apps/storefront"]
```

**`apps/storefront/pyproject.toml`** (after `pascal add cart --to storefront`)

```toml
[project]
name = "storefront"
version = "0.1.0"
requires-python = ">=3.12"
dependencies = ["cart"]

[project.scripts]
storefront = "storefront.main:main"

[tool.uv.sources]
cart = { workspace = true }

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

---

## Command reference

| Command | Description |
|---|---|
| `pascal init [name]` | Bootstrap a new workspace |
| `pascal create package <name>` | Scaffold a reusable library |
| `pascal create app <name>` | Scaffold a deployable app |
| `pascal add <pkg> --to <app>` | Add a workspace package as a dependency |
| `pascal info` | Print workspace overview |
| `pascal deps [--graph]` | Show the dependency tree |
| `pascal check` | Validate workspace health |
| `pascal diff [--since <ref>]` | Show changed packages since a git ref |
| `pascal test [--changed] [name]` | Run tests via UV |
| `pascal build <app>` | Build an app wheel |
| `pascal run <app> [-- args]` | Run an app entry-point |
| `pascal sync` | Regenerate UV workspace config |

---

## Development

### Prerequisites

| Tool | Purpose |
|---|---|
| [Rust ≥ 1.75](https://rustup.rs) | Compile the binary |
| [pre-commit](https://pre-commit.com) | Git hook runner (`uv tool install pre-commit`) |
| [maturin](https://maturin.rs) *(optional)* | Test PyPI packaging locally |

### Build

```bash
git clone https://github.com/sandeep-selvaraj/pascal
cd pascal
cargo build
```

### Test

```bash
cargo test          # unit + integration
cargo test --lib    # unit tests only
cargo test --test integration  # integration tests only
```

Integration tests in `tests/integration.rs` spawn the real binary against `tempfile` directories — no mocking.

### Lint and format

```bash
cargo fmt
cargo clippy -- -D warnings
```

### Pre-commit hooks

Install once after cloning:

```bash
pre-commit install
```

Every `git commit` will then run `cargo fmt` and `cargo clippy` automatically.

### Test PyPI packaging

```bash
pip install maturin
maturin build
pip install target/wheels/*.whl
pascal --version
```

### Serve docs locally

```bash
pip install zensical
zensical serve
```
---

## License

MIT — see [LICENSE](LICENSE).

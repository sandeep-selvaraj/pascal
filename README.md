# pascal

[![PyPI version](https://img.shields.io/pypi/v/pascal-cli.svg)](https://pypi.org/project/pascal-cli/)
[![CI](https://github.com/sandeep-selvaraj/pascal/actions/workflows/ci.yml/badge.svg)](https://github.com/sandeep-selvaraj/pascal/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Fast Python monorepo manager powered by Rust and UV.**

Pascal handles workspace scaffolding, dependency wiring, and UV workspace sync — so you can focus on code, not configuration.

**[Documentation](https://sandeep-selvaraj.github.io/pascal)** · [Installation](https://sandeep-selvaraj.github.io/pascal/installation/) · [Quickstart](https://sandeep-selvaraj.github.io/pascal/quickstart/) · [Commands](https://sandeep-selvaraj.github.io/pascal/commands/)

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
pip install mkdocs-material
mkdocs serve
```

### Project layout

```
src/
  main.rs          # CLI entry-point
  cli.rs           # clap argument structs
  config.rs        # serde types (pascal.toml, pyproject.toml)
  workspace.rs     # workspace discovery and loading
  template.rs      # generated file content
  display.rs       # coloured output helpers
  uv.rs            # uv subprocess wrappers
  git.rs           # git2 helpers
  commands/        # one module per subcommand
tests/
  integration.rs   # end-to-end CLI tests
docs/              # MkDocs source (mkdocs.yml at root)
.github/workflows/
  ci.yml           # test on push / PR
  release.yml      # publish to PyPI on git tag
```

---

## License

MIT — see [LICENSE](LICENSE).

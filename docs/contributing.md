# Contributing

## Prerequisites

| Tool | Purpose | Install |
|---|---|---|
| **Rust ≥ 1.75** | Compile the binary | [rustup.rs](https://rustup.rs) |
| **cargo** | Build and test runner | Included with Rust |
| **pre-commit** | Git hook runner | `uv tool install pre-commit` |
| **maturin** *(optional)* | Test PyPI packaging locally | `uv tool install maturin` |

## Clone and build

```bash
git clone https://github.com/sandeep-selvaraj/pascal
cd pascal
cargo build
```

The debug binary is at `target/debug/pascal`.

For a release build:

```bash
cargo build --release
# target/release/pascal
```

## Running tests

### Unit tests

```bash
cargo test --lib
```

Tests live alongside the source in `#[cfg(test)]` modules inside each `.rs` file.

### Integration tests

```bash
cargo test --test integration
```

Integration tests are in `tests/integration.rs`. Each test spawns the real `pascal` binary against a temporary directory using `CARGO_BIN_EXE_pascal`.

### All tests

```bash
cargo test
```

## Pre-commit hooks

Install the hooks once after cloning:

```bash
pre-commit install
```

After that, every `git commit` automatically runs:

1. `cargo fmt` — formats changed Rust files
2. `cargo clippy -- -D warnings` — lints the whole crate

To run hooks manually without committing:

```bash
pre-commit run --all-files
```

## Code style

- Formatting is enforced by `rustfmt` (via `cargo fmt`)
- Lints are enforced by `clippy -D warnings` — no warnings allowed
- Match the style of the surrounding code for new contributions

## Project structure

```
pascal/
  Cargo.toml             # dependencies, binary definition
  pyproject.toml         # maturin packaging config (PyPI)
  mkdocs.yml             # docs site config
  .pre-commit-config.yaml
  src/
    main.rs              # CLI entry-point, dispatch
    cli.rs               # clap structs — Commands enum and args
    error.rs             # PascalError type
    config.rs            # serde types for pascal.toml and pyproject.toml
    workspace.rs         # workspace discovery and loading
    template.rs          # file content templates
    display.rs           # coloured terminal output helpers
    uv.rs                # uv subprocess wrappers
    git.rs               # git2 helpers (diff, latest tag)
    commands/
      mod.rs
      init.rs
      create.rs
      add.rs
      info.rs
      deps.rs
      check.rs
      diff.rs
      test.rs
      build.rs
      run.rs
      sync.rs
  tests/
    integration.rs       # end-to-end CLI tests
  docs/                  # MkDocs source
  .github/
    workflows/
      ci.yml             # build + test on push/PR
      release.yml        # publish to PyPI on git tag
```

## Testing PyPI packaging locally

```bash
pip install maturin
maturin build
pip install target/wheels/*.whl
pascal --version
```

## Serving docs locally

```bash
pip install mkdocs-material
mkdocs serve
# open http://127.0.0.1:8000
```

## Opening a PR

1. Open an issue first for non-trivial changes so we can agree on the approach
2. Fork the repo, create a branch from `master`
3. Make your changes — make sure `cargo test` and `cargo clippy` pass
4. Submit a pull request with a clear description of what changed and why

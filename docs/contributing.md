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
pip install zensical
zensical serve
# open http://127.0.0.1:8000
```

## Commit messages — Conventional Commits

Pascal uses [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). This is what drives automatic `CHANGELOG.md` generation via [git-cliff](https://git-cliff.org/).

```
<type>[optional scope]: <description>

[optional body]

[optional footer]
```

### Types

| Type | When to use | Changelog section |
|---|---|---|
| `feat` | New user-facing feature | **Features** |
| `fix` | Bug fix | **Bug Fixes** |
| `perf` | Performance improvement | **Performance** |
| `refactor` | Internal restructuring, no behaviour change | **Refactoring** |
| `docs` | Documentation only | **Documentation** |
| `test` | Adding or fixing tests | **Tests** |
| `ci` | CI/CD workflow changes | **CI** |
| `chore` | Tooling, deps, release plumbing | **Miscellaneous** |
| `revert` | Reverts a previous commit | **Reverts** |

### Breaking changes

Add `!` after the type, or include a `BREAKING CHANGE:` footer:

```
feat!: remove --python flag from init (use pascal.toml instead)

BREAKING CHANGE: The --python flag is no longer accepted by `pascal init`.
Set `python` in pascal.toml instead.
```

Breaking changes appear in bold in the changelog and bump the major version.

### Examples

```
feat: add pascal diff --stat flag
fix: handle workspaces with no packages directory
docs: add UV integration page to docs site
test: add integration test for pascal sync
chore: bump clap to 4.5
ci: cache Rust build artifacts in release workflow
```

## Generating the changelog locally

Install git-cliff (`cargo install git-cliff`) and run:

```bash
# Preview what the next release entry will look like
git cliff --unreleased

# Regenerate the full CHANGELOG.md
git cliff --output CHANGELOG.md
```

The release workflow runs this automatically when a `v*` tag is pushed.

## Release process

1. Bump `version` in `Cargo.toml` and `pyproject.toml` to the new semver
2. Commit: `chore: release v0.2.0`
3. Tag: `git tag v0.2.0 && git push origin v0.2.0`
4. The release workflow builds wheels, publishes to PyPI, commits an updated `CHANGELOG.md`, and creates a GitHub release with the generated notes

## Opening a PR

1. Open an issue first for non-trivial changes so we can agree on the approach
2. Fork the repo, create a branch from `master`
3. Make your changes — make sure `cargo test` and `cargo clippy` pass
4. Write conventional commit messages
5. Submit a pull request with a clear description of what changed and why

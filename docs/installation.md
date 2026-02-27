# Installation

Pascal ships as a native binary wrapped in a Python wheel, so it installs like any other Python tool — no Rust toolchain required for end users.

## pip / uv / pipx

=== "uv (recommended)"

    ```bash
    uv tool install pascal-cli
    ```

    This installs pascal into an isolated uv-managed environment and puts the `pascal` binary on your `PATH`. Recommended because uv itself is what pascal delegates package operations to.

=== "pipx"

    ```bash
    pipx install pascal-cli
    ```

=== "pip"

    ```bash
    pip install pascal-cli
    ```

    Installing into a virtualenv or system Python works, but `uv tool` or `pipx` are better choices for CLI tools.

## Verify

```bash
pascal --version
```

## From source (cargo)

If you have the Rust toolchain installed:

```bash
cargo install --git https://github.com/sandeep-selvaraj/pascal
```

Or clone and build locally:

```bash
git clone https://github.com/sandeep-selvaraj/pascal
cd pascal
cargo build --release
# binary is at target/release/pascal
```

## System requirements

| Requirement | Notes |
|---|---|
| **uv** | Required at runtime for `test`, `build`, `run`, and `sync` commands. Install from [astral.sh/uv](https://astral.sh/uv). |
| **Python ≥ 3.8** | Only needed if you install via pip/pipx. The pascal binary itself has no Python dependency. |
| **git** | Required for `pascal diff`. Optional otherwise. |

## Shell completions

Pascal uses `clap` and can generate shell completions:

```bash
# bash
pascal --generate-shell-completion bash >> ~/.bashrc

# zsh
pascal --generate-shell-completion zsh >> ~/.zshrc

# fish
pascal --generate-shell-completion fish > ~/.config/fish/completions/pascal.fish
```

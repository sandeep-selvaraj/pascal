# pascal

**Fast Python monorepo manager powered by Rust and UV.**

[![PyPI version](https://img.shields.io/pypi/v/pascal-cli.svg)](https://pypi.org/project/pascal-cli/)
[![CI](https://github.com/sandeep-selvaraj/pascal/actions/workflows/ci.yml/badge.svg)](https://github.com/sandeep-selvaraj/pascal/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/sandeep-selvaraj/pascal/blob/master/LICENSE)

---

Pascal is a CLI tool that makes managing Python monorepos straightforward. It handles workspace scaffolding, dependency wiring, cross-package testing, and UV workspace sync — so you can focus on code, not configuration.

## Why pascal?

<div class="grid cards" markdown>

- :zap: **Single binary, zero overhead**

    Written in Rust. No Python runtime needed to run the CLI itself — just install and go.

- :package: **UV-native**

    All package operations delegate to `uv`. Pascal manages the structure; `uv` manages the packages.

- :mag: **Monorepo-aware**

    Understands the difference between reusable *packages* and deployable *apps*. Tracks cross-brick dependencies automatically.

- :wrench: **Zero config for simple cases**

    Drop a `pascal.toml` at the root and run. Pascal auto-discovers `packages/` and `apps/` — no manifest required.

</div>

## Quick look

```bash
# Bootstrap a workspace
pascal init my-workspace && cd my-workspace

# Add a library and an app
pascal create package cart
pascal create app storefront

# Wire them together
pascal add cart --to storefront

# Validate, inspect, run
pascal check
pascal info
pascal test
```

## Concepts

| Term | Meaning |
|---|---|
| **workspace** | The root of your monorepo. Contains `pascal.toml` and a UV workspace root `pyproject.toml`. |
| **package** | A reusable library under `packages/`. Installable by other packages or apps. |
| **app** | A deployable entry-point under `apps/`. Has a `[project.scripts]` entry and may depend on workspace packages. |
| **brick** | Internal term for any workspace member (package or app). |

## Next steps

- [Install pascal](installation.md)
- [Follow the quickstart](quickstart.md)
- [Browse all commands](commands/index.md)

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

## Inspiration

Pascal is heavily inspired by [Polylith](https://polylith.gitbook.io/polylith) — an architecture concept that encourages decomposing a system into small, focused, reusable *components* and *bases*, all living in a single repository.

Polylith's core idea is that code should be organised around **what it does** (components) rather than **which service it belongs to** — making it easy to compose different combinations of components into deployable artifacts. Pascal brings this same philosophy to the Python ecosystem with a simpler surface area: *packages* map to Polylith's components, and *apps* map to its bases.

Where Polylith is an opinionated full-stack architecture, pascal is deliberately minimal — it handles the scaffolding and dependency wiring and then gets out of the way, leaving the rest to UV and standard Python tooling.

If you find the ideas here compelling, the [Polylith documentation](https://polylith.gitbook.io/polylith) and its [Python tooling](https://davidvujic.github.io/python-polylith-docs/) are well worth reading.

## Next steps

- [Install pascal](installation.md)
- [Follow the quickstart](quickstart.md)
- [Browse all commands](commands/index.md)

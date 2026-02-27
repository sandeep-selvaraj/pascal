# CI/CD

## Testing changed packages only

Pascal's `--changed` flag makes pull-request CI fast — only bricks touched by the PR get tested.

```yaml
# .github/workflows/test.yml  (in your USER workspace repo, not the pascal source)
name: Test

on:
  pull_request:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0          # required for git history used by pascal diff/test

      - uses: astral-sh/setup-uv@v4

      - name: Install pascal
        run: uv tool install pascal-cli

      - name: Validate workspace
        run: pascal check

      - name: Test changed packages (PR)
        if: github.event_name == 'pull_request'
        run: pascal test --changed --since origin/${{ github.base_ref }}

      - name: Test all packages (push to main)
        if: github.event_name == 'push'
        run: pascal test
```

## Full matrix test

For thorough CI on pushes to main or release branches:

```yaml
jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        python: ["3.10", "3.11", "3.12"]
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v4
        with:
          python-version: ${{ matrix.python }}
      - name: Install pascal
        run: uv tool install pascal-cli
      - run: pascal test
```

## Building and publishing an app

```yaml
name: Release app

on:
  push:
    tags: ["v*"]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: astral-sh/setup-uv@v4
      - name: Install pascal
        run: uv tool install pascal-cli
      - name: Build
        run: pascal build storefront
      - uses: actions/upload-artifact@v4
        with:
          name: dist
          path: apps/storefront/dist/
```

## Caching

Add UV caching to speed up CI:

```yaml
      - uses: astral-sh/setup-uv@v4
        with:
          enable-cache: true
          cache-dependency-glob: "**/uv.lock"
```

## Key tips

!!! tip "Always pass `fetch-depth: 0`"
    `pascal diff` and `pascal test --changed` need full git history to compute what changed. Without `fetch-depth: 0`, GitHub Actions does a shallow clone and pascal falls back to comparing against the first commit.

!!! tip "Commit `uv.lock`"
    Commit your `uv.lock` file. This ensures CI uses exactly the same package versions as your local machine and makes installs faster (UV can skip resolution).

# pascal test

Run tests for workspace packages and apps using `uv run pytest`.

```
pascal test [NAME] [--changed] [--since <REF>] [-- <PYTEST_ARGS>]
```

## Arguments and flags

| Argument / Flag | Description |
|---|---|
| `NAME` | Run tests only for this brick (package or app name) |
| `--changed` | Only run tests for bricks changed since `--since` ref |
| `--since REF` | Git ref for `--changed` comparison (default: latest tag) |
| `-- <args>` | Extra arguments forwarded to pytest |

## Examples

```bash
# Run all tests
pascal test

# Run tests for one brick
pascal test cart

# Run tests only for changed bricks
pascal test --changed

# Run only for bricks changed since a branch
pascal test --changed --since origin/main

# Pass extra pytest flags
pascal test -- -x -v --tb=short
```

## Under the hood

For each brick being tested, pascal runs:

```bash
uv run --project <brick-dir> pytest tests/
```

Tests run in dependency order — if `storefront` depends on `cart`, `cart` is tested first.

## Exit codes

| Code | Meaning |
|---|---|
| `0` | All tests passed |
| Non-zero | At least one test suite failed |

!!! tip
    Use `pascal test --changed` in pull request CI to avoid re-running tests for unmodified packages.

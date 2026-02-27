# pascal check

Validate workspace health.

```
pascal check
```

## What it checks

| Check | Severity |
|---|---|
| Circular dependencies in the workspace graph | Error |
| Workspace package missing from `[tool.uv.sources]` | Warning |
| Missing `src/<name>/` directory in a brick | Warning |
| Missing `pyproject.toml` in a declared brick | Error |

## Output

```
┌──────────────────────────────────┐
│    Pascal Workspace Check        │
└──────────────────────────────────┘

✓ No circular dependencies
⚠ storefront: 'cart' is a workspace dep but missing from [tool.uv.sources]
⚠ auth: expected src/auth/ directory not found

1 warning(s) found
```

## Exit codes

| Code | Meaning |
|---|---|
| `0` | No errors (warnings are allowed) |
| `1` | One or more errors found |

!!! tip
    Run `pascal check` in CI before building or deploying to catch dependency wiring mistakes early.

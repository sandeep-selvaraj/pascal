# pascal deps

Print the dependency tree for all workspace members.

```
pascal deps [--graph]
```

## Flags

| Flag | Description |
|---|---|
| `--graph` | Print an adjacency-list style graph instead of the tree view |

## Output (default)

Internal workspace dependencies are shown in green. External (PyPI) dependencies are dimmed.

```
  ◆ cart
    (no dependencies)

  ◆ auth
    (no dependencies)

  ▶ storefront
    → cart        # workspace dep (green)
    → auth        # workspace dep (green)
    → httpx       # external dep (dimmed)
```

Icons:

- `◆` — package
- `▶` — app

## Output (`--graph`)

```
  cart        (no internal deps)
  auth        (no internal deps)
  storefront  → cart, auth
```

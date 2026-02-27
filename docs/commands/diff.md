# pascal diff

Show which packages and apps have changed since a git ref.

```
pascal diff [--since <REF>]
```

## Flags

| Flag | Default | Description |
|---|---|---|
| `--since REF` | latest git tag | Git ref (tag, branch, commit SHA) to compare against |

## Behaviour

1. Uses `git2` to find all files changed between `REF` and `HEAD`
2. Maps changed file paths back to workspace bricks
3. Also reports apps that **transitively** depend on changed packages

## Output

```
  Changes since 'v0.2.0'

  ◈ Changed bricks:
    ◆ cart     [package]

  ◈ Apps affected by changed packages:
    ▶ storefront  [app — transitive]
```

## Examples

```bash
# Compare against latest tag (default)
pascal diff

# Compare against a specific tag
pascal diff --since v0.1.0

# Compare against a branch
pascal diff --since origin/main

# Compare against a commit SHA
pascal diff --since abc1234
```

## Use in CI

```bash
# In a pull request workflow:
pascal diff --since origin/main
```

Combined with `pascal test --changed`:

```bash
pascal test --changed --since origin/main
```

!!! note
    `pascal diff` requires the workspace to be inside a git repository. It exits with an error if no git repo is found.

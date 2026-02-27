# Command Reference

All pascal commands follow the pattern `pascal <command> [args] [flags]`.

Run `pascal --help` or `pascal <command> --help` for built-in help text.

## Overview

| Command | Description |
|---|---|
| [`pascal init`](init.md) | Bootstrap a new workspace |
| [`pascal create package`](create.md) | Scaffold a new reusable library |
| [`pascal create app`](create.md) | Scaffold a new deployable app |
| [`pascal add`](add.md) | Wire a package into an app |
| [`pascal info`](info.md) | Print workspace overview |
| [`pascal deps`](deps.md) | Show the dependency tree |
| [`pascal check`](check.md) | Validate workspace health |
| [`pascal diff`](diff.md) | Show changed packages since a git ref |
| [`pascal test`](test.md) | Run tests via UV |
| [`pascal build`](build.md) | Build an app wheel |
| [`pascal run`](run.md) | Run an app entry-point |
| [`pascal sync`](sync.md) | Regenerate UV workspace config |

## Global behaviour

- **Workspace detection**: every command (except `init`) walks up from the current directory to find `pascal.toml`. You can run commands from any subdirectory.
- **Exit codes**: `0` on success, `1` on error. `pascal check` exits `1` if errors (not warnings) are found.
- **Colour**: output is coloured by default. Pipe to a file or set `NO_COLOR=1` to disable.

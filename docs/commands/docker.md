# pascal docker

Build and push Docker images for apps.

```
pascal docker build <APP> [flags]
pascal docker push <APP> [flags]
```

## Subcommands

### build

Build a Docker image for an app.

```
pascal docker build <APP> [--tag <TAG>] [--platform <PLATFORMS>] [--push]
```

| Argument / Flag | Description |
|---|---|
| `APP` | Name of the app to build an image for |
| `--tag <TAG>` | Override the computed image tag (e.g. `myorg/myapp:latest`) |
| `--platform <P>` | Target platform(s), comma-separated (e.g. `linux/amd64,linux/arm64`) |
| `--push` | Push the image to the registry immediately after a successful build |

### push

Push a previously built image.

```
pascal docker push <APP> [--tag <TAG>]
```

| Argument / Flag | Description |
|---|---|
| `APP` | Name of the app to push |
| `--tag <TAG>` | Override the computed image tag |

## Image tag resolution

The final tag is computed in priority order:

1. `--tag` CLI flag (always wins)
2. `[tool.pascal.docker] registry` in the app's `pyproject.toml`
3. `[workspace.docker] registry` in `pascal.toml`
4. Bare `<app-name>:<version>` (fallback)

When a registry is configured the tag becomes `<registry>/<app-name>:<version>`.

## Dockerfile scaffolding

If `apps/<APP>/Dockerfile` does not exist, pascal generates one automatically on the first `docker build`.
The generated Dockerfile:

- Uses `python:<version>-slim` matching the workspace Python version
- Copies in `uv` from the official image
- Copies only the workspace packages the app directly depends on
- Installs dependencies with `uv sync --frozen --no-dev`

Review the scaffolded Dockerfile before committing.

## Registry configuration

### Workspace-level (pascal.toml)

```toml
[workspace]
name = "my-workspace"
python = "3.12"

[workspace.docker]
registry = "ghcr.io/myorg"
```

All apps will default to `ghcr.io/myorg/<app-name>:<version>`.

### Per-app (apps/\<name\>/pyproject.toml)

```toml
[tool.pascal.docker]
registry = "myorg.azurecr.io"
platforms = ["linux/amd64", "linux/arm64"]
```

Per-app settings override the workspace registry.

## Examples

```bash
# Build with auto-computed tag
pascal docker build storefront

# Build for multiple platforms and push
pascal docker build storefront --platform linux/amd64,linux/arm64 --push

# Build with an explicit tag
pascal docker build storefront --tag ghcr.io/myorg/storefront:v1.2.0

# Push a previously built image
pascal docker push storefront
```

```
  Docker build: storefront

  Tag:       ghcr.io/myorg/storefront:0.1.0
  Deps:      cart, auth
  Platforms: linux/amd64, linux/arm64

[docker build output…]

✓ Built:  ghcr.io/myorg/storefront:0.1.0
✓ Pushed
```

!!! note
    `pascal docker` is a thin wrapper around the `docker` CLI.
    Docker (or a compatible runtime like Podman aliased to `docker`) must be installed and running.
    For multi-platform builds, `docker buildx` must be available.

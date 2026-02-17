# Docker Layers

This directory contains layered container targets for tinyverse.

- `common`: shared runtime base with Bun.
- `build`: installs Rust/proto/moon toolchain and builds `tinyverse_cli`.
- `run`: minimal runtime image with `tinyverse` binary.
- `agentbox`: interactive CLI image for local automation workflows.
- `ci`: CI/test image that defaults to `bun scripts/ci.sh.ts docker`.
- `devcontainer`: dev UX image with shell/editor tools.

Compose stack: `docker/compose.devcontainers.yml`.

Helper scripts:

- `bun scripts/docker_build.sh.ts`
- `bun scripts/docker_ci.sh.ts`
- `bun scripts/docker_agentbox.sh.ts run -- moon run tinyverse_cli:check`
- `bun scripts/docker_devcontainer.sh.ts attach`

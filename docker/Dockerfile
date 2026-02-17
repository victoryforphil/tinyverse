# syntax=docker/dockerfile:1.7

ARG UBUNTU_VERSION=24.04

FROM ubuntu:${UBUNTU_VERSION} AS common

ENV DEBIAN_FRONTEND=noninteractive
ENV BUN_INSTALL=/opt/bun
ENV PATH="${BUN_INSTALL}/bin:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
  bash \
  ca-certificates \
  curl \
  git \
  gzip \
  tini \
  unzip \
  xz-utils \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir -p "${BUN_INSTALL}" \
  && curl -fsSL https://bun.sh/install | bash -s -- bun-v1.3.9 \
  && ln -sf "${BUN_INSTALL}/bin/bun" /usr/local/bin/bun \
  && ln -sf "${BUN_INSTALL}/bin/bunx" /usr/local/bin/bunx

WORKDIR /workspace

FROM common AS build

ENV PROTO_HOME=/opt/proto
ENV CARGO_HOME=/opt/cargo
ENV RUSTUP_HOME=/opt/rustup
ENV PATH="${CARGO_HOME}/bin:${PROTO_HOME}/bin:${PROTO_HOME}/shims:${PATH}"

RUN apt-get update && apt-get install -y --no-install-recommends \
  build-essential \
  libsqlite3-dev \
  libssl-dev \
  pkg-config \
  sqlite3 \
  tmux \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir -p "${PROTO_HOME}" \
  && curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- --yes --no-profile

RUN mkdir -p "${CARGO_HOME}" "${RUSTUP_HOME}" \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable

COPY .prototools /workspace/.prototools
RUN proto install

COPY . /workspace

RUN moon :install
RUN cargo build --release -p tinyverse_cli

FROM common AS run

RUN apt-get update && apt-get install -y --no-install-recommends \
  tmux \
  && rm -rf /var/lib/apt/lists/*

COPY --from=build /workspace/target/release/tinyverse_cli /usr/local/bin/tinyverse

ENTRYPOINT ["tini", "--"]
CMD ["tinyverse", "--help"]

FROM build AS agentbox

RUN apt-get update && apt-get install -y --no-install-recommends \
  fd-find \
  gh \
  jq \
  less \
  procps \
  ripgrep \
  zsh \
  && rm -rf /var/lib/apt/lists/*

RUN ln -sf /usr/bin/fdfind /usr/local/bin/fd

ENV TERM=xterm-256color

ENTRYPOINT ["tini", "--"]
CMD ["sleep", "infinity"]

FROM build AS ci

ENV CI=true
WORKDIR /workspace

ENTRYPOINT ["tini", "--"]
CMD ["bun", "scripts/ci.sh.ts", "docker"]

FROM agentbox AS devcontainer

RUN apt-get update && apt-get install -y --no-install-recommends \
  fzf \
  neovim \
  && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["tini", "--"]
CMD ["sleep", "infinity"]

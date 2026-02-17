----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/install
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, install
- Summary: BunDenoGoNode.jsPHPPythonRubyRust
----

Source: https://moonrepo.dev/docs/install

BunDenoGoNode.jsPHPPythonRubyRust
- [Home](/)
- [Install moon](/docs/install)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Install moon

2 min

The following guide can be used to install moon and integrate it into an existing repository (with
or without incremental adoption), or to a fresh repository.

## Installing[​](#installing)

The entirety of moon is packaged and shipped as a single binary. It works on all major operating
systems, and does not require any external dependencies. For convenience, we provide the following
scripts to download and install moon.

### proto[​](#proto)

moon can be installed and managed in [proto's toolchain](/proto). This will install moon to
`~/.proto/tools/moon` and make the binary available at `~/.proto/bin`.

```
proto install moon
```

Furthermore, the version of moon can be pinned on a per-project basis using the
[`.prototools` config file](/docs/proto/config).

.prototools

```
moon = "1.31.0"
```

info

We suggest using proto to manage moon (and other tools), as it allows for multiple versions to be
installed and used. The other installation options only allow for a single version (typically the
last installed).

### Linux, macOS, WSL[​](#linux-macos-wsl)

In a terminal that supports Bash, run:

```
bash info

If you are using Git Bash on Windows, you can run the [Unix commands](#linux-macos-wsl) above.

### npm[​](#npm)

moon is also packaged and shipped as a single binary through the
[`@moonrepo/cli`](https://www.npmjs.com/package/@moonrepo/cli) npm package. Begin by installing this
package at the root of the repository.

- Yarn
- Yarn (classic)
- npm
- pnpm
- Bun

```
yarn add --dev @moonrepo/cli
```

```
yarn add --dev @moonrepo/cli# If using workspacesyarn add --dev -W @moonrepo/cli
```

```
npm install --save-dev @moonrepo/cli
```

```
pnpm add --save-dev @moonrepo/cli# If using workspacespnpm add --save-dev -w @moonrepo/cli
```

```
bun install --dev @moonrepo/cli
```

If you are installing with Bun, you'll need to add `@moonrepo/cli` as a
[trusted dependency](https://bun.sh/docs/install/lifecycle#trusteddependencies).

info

When a global `moon` binary is executed, and the `@moonrepo/cli` binary exists within the
repository, the npm package version will be executed instead. We do this because the npm package
denotes the exact version the repository is pinned it.

### Other[​](#other)

moon can also be downloaded and installed manually, by downloading an asset from
[https://github.com/moonrepo/moon/releases](https://github.com/moonrepo/moon/releases). Be sure to
rename the file after downloading, and apply the executable bit (`chmod +x`) on macOS and Linux.

## Upgrading[​](#upgrading)

If using proto, moon can be upgraded using the following command:

```
proto install moon --pin
```

Otherwise, moon can be upgraded with the [`moon upgrade`](/docs/commands/upgrade) command. However, this
will only upgrade moon if it was installed in `~/.moon/bin`.

```
moon upgrade
```

Otherwise, you can re-run the installers above and it will download, install, and overwrite with the
latest version.

## Canary releases[​](#canary-releases)

moon supports canary releases, which are built and published for every commit to our development
branches. These releases will include features and functionality that have not yet landed on master.
When using a canary release, you'll need to download and execute the binaries manually:

- Using our npm package [`@moonrepo/cli`](https://www.npmjs.com/package/@moonrepo/cli?activeTab=versions) under the `canary` tag. Releases are versioned by date.

- From a [GitHub prerelease](https://github.com/moonrepo/moon/releases/tag/canary) using the `canary` tag. This tag always represents the latest development release.

## Nightly releases[​](#nightly-releases)

moon supports nightly releases, which are built and published once a day from the latest commit on
master. When using a nightly release, you'll need to download and execute the binaries manually.

- Using our npm package [`@moonrepo/cli`](https://www.npmjs.com/package/@moonrepo/cli?activeTab=versions) under the `nightly` tag. Releases are versioned by date.

- From a [GitHub prerelease](https://github.com/moonrepo/moon/releases/tag/nightly) using the `nightly` tag. This tag always represents the latest stable release.

## Next steps[​](#next-steps)

[Setup workspace](/docs/setup-workspace)

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/install.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

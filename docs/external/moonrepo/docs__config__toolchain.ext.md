----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/config/toolchain
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, config, toolchain
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/config/toolchain

- [Home](/)
- [Config files](/docs/config)
- [.moon/toolchains](/docs/config/toolchain)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# .moon/toolchains

The `.moon/toolchains.yml` file configures the toolchain and the workspace development environment.
This file is optional.

Managing tool version's within the toolchain ensures a deterministic environment across any machine
(whether a developer, CI, or production machine).

## `extends`[​](#extends)

Defines one or many external `.moon/toolchains.yml`'s to extend and inherit settings from. Perfect
for reusability and sharing configuration across repositories and projects. When defined, this
setting must be an HTTPS URL or relative file system path that points to a valid YAML document!

.moon/toolchains.yml

```
extends: 'https://raw.githubusercontent.com/organization/repository/master/.moon/toolchains.yml'
```

caution

Settings will be merged recursively for blocks, with values defined in the local configuration
taking precedence over those defined in the extended configuration.

## `moon`v1.29.0[​](#moon)

Configures how moon will receive information about latest releases and download locations.

### `manifestUrl`[​](#manifesturl)

Defines an HTTPS URL in which to fetch the current version information from.

.moon/toolchains.yml

```
moon:  manifestUrl: 'https://proxy.corp.net/moon/version'
```

### `downloadUrl`[​](#downloadurl)

Defines an HTTPS URL in which the moon binary can be downloaded from. The download file name is
hard-coded and will be appended to the provided URL.

Defaults to downloading from GitHub: [https://github.com/moonrepo/moon/releases](https://github.com/moonrepo/moon/releases)

.moon/toolchains.yml

```
moon:  downloadUrl: 'https://github.com/moonrepo/moon/releases/latest/download'
```

## `proto`v1.39.0[​](#proto)

Configures how moon integrates with and utilizes [proto](/proto).

### `version`[​](#version)

The version of proto to install and run toolchains with. If proto or this version of proto has not
been installed yet, it will be installed automatically when running a task.

.moon/toolchains.yml

```
proto:  version: '0.51.0'
```

## Go[​](#go)

## `go`v1.38.0[​](#go-1)

Run `moon toolchain info go` for all available settings.

## JavaScript[​](#javascript)

## `javascript`v1.40.0[​](#javascript-1)

Run `moon toolchain info javascript` for all available settings.

## `bun`v1.40.0[​](#bun)

Run `moon toolchain info bun` for all available settings.

info

This toolchain requires the [`javascript`](#javascript) toolchain to also be enabled.

## `deno`v1.41.0[​](#deno)

Run `moon toolchain info deno` for all available settings.

info

This toolchain requires the [`javascript`](#javascript) toolchain to also be enabled.

## `node`v1.40.0[​](#node)

Run `moon toolchain info node` for all available settings.

info

This toolchain requires the [`javascript`](#javascript) toolchain to also be enabled.

## `npm`v1.40.0[​](#npm)

Run `moon toolchain info npm` for all available settings.

info

This toolchain requires the [`node`](#node) toolchain to also be enabled.

## `pnpm`v1.40.0[​](#pnpm)

Run `moon toolchain info pnpm` for all available settings.

info

This toolchain requires the [`node`](#node) toolchain to also be enabled.

## `yarn`v1.40.0[​](#yarn)

Run `moon toolchain info yarn` for all available settings.

info

This toolchain requires the [`node`](#node) toolchain to also be enabled.

## `typescript`[​](#typescript)

Run `moon toolchain info typescript` for all available settings.

## Pythonv1.30.0[​](#python)

caution

Python support is currently a work in progress for v2!

## `python`[​](#python-1)

Run `moon toolchain info python` for all available settings.

## Rust[​](#rust)

## `rust`v1.37.0[​](#rust-1)

Run `moon toolchain info rust` for all available settings.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/config/toolchain.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/install
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, install
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/install

- [Home](/)
- [Install proto](/docs/proto/install)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Install proto

1 min

The following guide can be used to install proto into your environment.

## Requirements[​](#requirements)

- Git - for fetching available versions/tags

- tar, unzip, gz, xz - for unpacking archives

```
# macOSbrew install git unzip gzip xz# Ubuntu / Debianapt-get install git unzip gzip xz-utils# RHEL-based / Fedoradnf install git unzip gzip xz
```

## Installing[​](#installing)

The entirety of proto is packaged and shipped as 2 binaries. It works on most operating systems,
and does not require any external dependencies. For convenience, we provide the following scripts to
download and install proto.

info

The install location can be customized with the `PROTO_HOME` environment variable. If not provided,
the default location is `~/.proto`.

### Linux, macOS, WSL[​](#linux-macos-wsl)

In a terminal that supports Bash, run the following command. This will download and install proto,
then open an interactive prompt to complete the installation.

```
bash administrator Powershell or Windows Terminal, run the following command. This will download
and install proto, then open an interactive prompt to complete the installation.

```
irm https://moonrepo.dev/install/proto.ps1 | iex
```

You may also need to run the following command for shims to be executable:

```
Set-ExecutionPolicy RemoteSigned# Without admin privilegesSet-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

### Other[​](#other)

proto can also be downloaded and installed manually, by downloading an asset from
[https://github.com/moonrepo/proto/releases](https://github.com/moonrepo/proto/releases). Be sure to
rename the file after downloading, and apply the executable bit (`chmod +x`) on macOS and Linux.

## Upgrading[​](#upgrading)

To upgrade proto, run the [`proto upgrade`](/docs/proto/commands/upgrade) command, or re-run the install
scripts above.

## Uninstalling[​](#uninstalling)

To uninstall proto, delete the `~/.proto` directory, and remove any `PROTO_HOME` references from
your shell profile.

## Canary releases[​](#canary-releases)

proto supports canary releases, which are built and published for every commit to our development
branches. These releases will include features and functionality that have not yet landed on master.
Canary releases are available as a
[GitHub prerelease](https://github.com/moonrepo/proto/releases/tag/canary) using the `canary` tag.

## Nightly releases[​](#nightly-releases)

proto supports nightly releases, which are built and published once a day from the latest commit on
master. Nightly releases are available as a
[GitHub prerelease](https://github.com/moonrepo/proto/releases/tag/nightly) using the `nightly` tag.

## Next steps[​](#next-steps)

[Choose a workflow](/docs/proto/workflows)[Learn about `.prototools`](/docs/proto/config)

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/install.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

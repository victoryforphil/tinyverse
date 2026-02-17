----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/offline-mode
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, guides, offline mode
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/guides/offline-mode

- [Home](/)
- [Offline mode](/docs/guides/offline-mode)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# Offline mode

moon assumes that an internet connection is always available, as we download and install tools into
the toolchain, resolve versions against upstream manifests, and automatically install dependencies.
While this is useful, having a constant internet connection isn't always viable.

To support workflows where internet isn't available or is spotty, moon will automatically check for
an active internet connection, and drop into offline mode if necessary.

## What's disabled when offline[​](#whats-disabled-when-offline)

When offline, moon will skip or disable the following:

- Automatic dependency installation will be skipped.

- Toolchain will skip resolving, downloading, and installing tools, and instead use the local cache. If no local cache available, will fallback to binaries found on `PATH`.

- If not available on `PATH`, will fail to run.

- Upgrade and version checks will be skipped.

## Toggling modes[​](#toggling-modes)

While we automatically check for an internet connection, both online and offline modes can be forced
with the `PROTO_OFFLINE` environment variable. Setting the variable to `1` or `true` will force
offline mode, while `0` and `false` will force online mode.

## Environment variables[​](#environment-variables)

Some additional variables to interact with offline checks.

- `PROTO_OFFLINE_TIMEOUT` - Customize the timeout for offline checks (in milliseconds). Defaults to `750`.

- `PROTO_OFFLINE_HOSTS` - Customize additional hosts/IPs to check for offline status. Separate multiple hosts with a `,`.

- `PROTO_OFFLINE_IP_VERSION` - Customize which IP version to support, `4` or `6`. If not defined, supports both.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/guides/offline-mode.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

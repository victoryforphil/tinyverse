----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/upgrade
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, upgrade
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/upgrade

- [Home](/)
- Commands
- [upgrade](/docs/proto/commands/upgrade)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# upgrade

The `proto upgrade` (or `proto up`) command can be used to upgrade your current proto binary to the
latest version, or check if you're currently outdated.

```
$ proto upgrade# Up/downgrade to a specific version$ proto upgrade 0.39.0
```

info

The previous binary will be moved to `~/.proto/tools/proto/`, while the new binary will be
installed to `~/.proto/bin`.

### Arguments[​](#arguments)

- `` - The version of proto to explicitly upgrade or downgrade to. v0.39.3

### Options[​](#options)

- `--check` - Check if there's a new version without executing the upgrade.

- `--json` - Print the upgrade information as JSON.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/upgrade.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/alias
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, alias
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/alias

- [Home](/)
- Commands
- [alias](/docs/proto/commands/alias)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# alias

The `proto alias   ` (or `proto a`) command will define a custom alias that
maps to a specific version for the provided tool. Aliases can be used anywhere a version is
accepted.

```
$ proto alias node work 16.16
```

By default this will update the local [`./.prototools`](/docs/proto/config) file. Pass `--to` to customize
the location.

### Arguments[​](#arguments)

- `` - Type of tool.

- `` - Name of the alias. Supports alphanumeric chars.

- `` - Version to map to the alias.

## Options[​](#options)

- `--to` - [Location of `.prototools`](/docs/proto/config#locations) to update. Supports `global`, `local`, and `user`. v0.41.0

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/alias.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

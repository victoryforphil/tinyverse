----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/versions
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, versions
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/versions

- [Home](/)
- Commands
- [versions](/docs/proto/commands/versions)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# versions

v0.44.0

The `proto versions ` command will list available versions by resolving versions from the
tool's remote release manifest. Furthermore, if a version has been installed locally, it will be
denoted with a timestamp.

```
$ proto versions node...22.0.022.1.022.2.022.3.022.4.022.4.122.5.0 - installed 12/25/2422.5.122.6.022.7.022.8.022.9.022.10.022.11.022.12.023.0.023.1.023.2.023.3.023.4.0 - installed 12/19/2423.5.0 - installed 12/25/24
```

### Arguments[​](#arguments)

- `` - Type of tool.

### Options[​](#options)

- `--aliases` - Include aliases in the list.

- `--installed` - Only display installed versions.

- `--json` - Print the versions and aliases in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/versions.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/templates
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, templates
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/templates

- [Home](/)
- [Commands](/docs/commands)
- [templates](/docs/commands/templates)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# templates

v1.24.0

The `moon templates` command will list all templates available for [code generation](/docs/commands/generate).
This list will include the template title, description, default destination, where it's source files
are located, and more.

```
$ moon templates
```

### Options[​](#options)

- `--filter` - Filter templates by a search term.

- `--json` - Print templates in JSON format.

### Configuration[​](#configuration)

- [`generator`](/docs/config/workspace#generator) in `.moon/workspace.yml`

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/templates.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

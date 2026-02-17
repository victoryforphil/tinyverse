----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/query/affected
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, commands, query, affected
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/commands/query/affected

- [Home](/)
- [Commands](/docs/commands)
- [query](/docs/commands/query)
- [affected](/docs/commands/query/affected)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# query affected

v2.0.0

Use the `moon query affected` sub-command to query for all affected projects and tasks based on the
state of the workspace and VCS.

```
# Return affected$ moon query affected# Return affected including dependency relationships$ moon query affected --upstream deep
```

This will output a map of projects and tasks as JSON. The output has the following structure:

```
{	projects: Record,	tasks: Record,}
```

### Options[​](#options)

- `--downstream` - Include downstream dependents. Supports "none" (default), "direct", "deep".

- `--upstream` - Include upstream dependencies. Supports "none", "direct", "deep" (default).

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/commands/query/affected.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

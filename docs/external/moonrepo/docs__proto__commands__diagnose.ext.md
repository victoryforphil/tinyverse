----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/diagnose
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, diagnose
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/diagnose

- [Home](/)
- Commands
- [diagnose](/docs/proto/commands/diagnose)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# diagnose

v0.37.0

The `proto diagnose` command will diagnose your proto installation for any potential issues. Issues
are categorized into errors and warnings, with the former being a must fix, and the latter being a
maybe fix (depending on your usage of proto).

```
$ proto diagnoseShell: zshShell profile: /Users/name/.zshrcErrors ────────────────────────────────────────────────────────────────────  - Issue: Bin directory /Users/name/.proto/bin was found BEFORE the shims directory /Users/name/.proto/shims on PATH    Resolution: Ensure the shims path comes before the bin path in your shell    Comment: Runtime version detection will not work correctly unless shims are used
```

### Options[​](#options)

- `--shell` - The shell to diagnose (will detect automatically).

- `--json` - Print the diagnosis in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/diagnose.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

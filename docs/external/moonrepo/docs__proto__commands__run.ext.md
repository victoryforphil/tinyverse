----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/run
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, run
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/run

- [Home](/)
- Commands
- [run](/docs/proto/commands/run)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# run

The `proto run  [version]` (or `proto r`) command will run a tool after
[detecting a version](/docs/proto/detection) from the environment.

```
# Run and detect version from environment$ proto run bun# Run with explicit version$ proto run bun 0.5.3# Run with version from environment variable$ PROTO_BUN_VERSION=0.5.3 proto run bun
```

Arguments can be passed to the underlying tool binary by providing additional arguments after `--`.

```
$ proto run bun -- run ./script.ts# When using the binary on PATH$ bun run ./script.ts
```

### Arguments[​](#arguments)

- `` - Type of tool.

- `[version]` - Version of tool. If not provided, will attempt to detect the version from the environment.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/run.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

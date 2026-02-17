----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/regen
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, regen
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/regen

- [Home](/)
- Commands
- [regen](/docs/proto/commands/regen)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# regen

v0.27.0

The `proto regen` command can be used to regenerate all shims in the `~/.proto/shims` directory.
This command will also clean the shims directory before regenerating, in an effort to remove
unexpected or broken shims.

```
$ proto regen
```

By default this will only regenerate shims. If you want to regenerate bins in `~/.proto/bin` as
well, pass the `--bin` flag. This will also clean the bins directory before regenerating.

```
$ proto regen --bin
```

Only versions pinned in `~/.proto/.prototools` will be linked as bins.

## Options[​](#options)

- `--bin` - Also recreate `~/.proto/bin` symlinks.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/regen.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

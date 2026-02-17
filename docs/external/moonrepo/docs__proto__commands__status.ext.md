----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/status
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, status
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/status

- [Home](/)
- Commands
- [status](/docs/proto/commands/status)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# status

v0.34.0

The `proto status` command will list all tools that are currently active for a target directory,
what versions of those tools are resolved to, and the configuration file in which they are defined.

```
$ proto status╭───────────────────────────────────────────────────────────────────────────────────────────────────────╮│ Tool      Configured Resolved  Installed                           Config                             ││───────────────────────────────────────────────────────────────────────────────────────────────────────││ bun       1.1.42     1.1.42    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                bun/1.1.42                                                             ││ deno      1.43.1     1.43.1    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                deno/1.43.1                                                            ││ node      23.5.0     23.5.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                node/23.5.0                                                            ││ npm       ~10.7      10.7.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                npm/10.7.0                                                             ││ python    3.12.0     3.12.0    /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                python/3.12.0                                                          ││ yarn      3.6.3      3.6.3     /Users/name/.proto/tools/          /Users/name/.proto/.prototools      ││                                yarn/3.6.3                                                             │╰───────────────────────────────────────────────────────────────────────────────────────────────────────╯
```

By default, this command does not check tools for versions pinned in the global
`~/.proto/.prototools` file. Pass `--config-mode all` to include them.

### Options[​](#options)

- `--json` - Print the list in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/status.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

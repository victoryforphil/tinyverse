----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/outdated
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, outdated
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/outdated

- [Home](/)
- Commands
- [outdated](/docs/proto/commands/outdated)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# outdated

v0.19.0

The `proto outdated` command will load all [`.prototools`](/docs/proto/config) files and check for newer
(matching configured range) and latest versions of each configured tool. Will also include the
configuration file in which the version has been configured.

```
$ proto outdated╭───────────────────────────────────────────────────────────────────────╮│ Tool      Current Newest  Latest  Config                              ││───────────────────────────────────────────────────────────────────────││ bun       1.1.42  1.1.42  1.1.42  /Users/name/.proto/.prototools      ││ node      23.5.0  23.5.0  23.5.0  /Users/name/.proto/.prototools      ││ npm       10.7.0  10.7.0  11.0.0  /Users/name/.proto/.prototools      ││ rust      1.83.0  1.83.0  1.83.0  /Users/name/.proto/.prototools      ││ yarn      3.6.3   3.8.7   4.5.1   /Users/name/.proto/.prototools      │╰───────────────────────────────────────────────────────────────────────╯
```

By default, this command does not check tools for versions pinned in the global
`~/.proto/.prototools` file. Pass `--config-mode all` to include them.

### Options[​](#options)

- `--json` - Print the list in JSON format.

- `--latest` - When updating versions with `--update`, use the latest version instead of newest.

- `--update` - Update and write newest/latest versions to their respective configuration.

- `--yes` - Avoid and confirm all prompts. v0.44.0

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/outdated.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

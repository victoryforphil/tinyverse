----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/debug/config
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, debug, config
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/debug/config

- [Home](/)
- Commands
- [debug](/docs/proto/commands/debug)
- [config](/docs/proto/commands/debug/config)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# debug config

v0.25.0

The `proto debug config` command will list all `.prototools` configuration files (in TOML format)
that have been loaded, in order of precedence, with the final merged configuration printed at the
end.

```
$ proto debug config/Users/name/.proto/.prototools ───────────────────────────────────────────  node = "20.0.0"  npm = "bundled"  [tools.node.aliases]  stable = "~20"  [settings]  auto-clean = falseFinal configuration ───────────────────────────────────────────────────────  node = "20.0.0"  npm = "bundled"  [tools.node.aliases]  stable = "~20"  [plugins.tools]  node = "https://github.com/moonrepo/node-plugin/releases/download/v0.6.1/node_plugin.wasm"  [settings]  auto-clean = false  auto-install = false  detect-strategy = "first-available"  [settings.http]  allow-invalid-certs = false  proxies = []
```

### Options[​](#options)

- `--json` - Print the list in JSON format.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/debug/config.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

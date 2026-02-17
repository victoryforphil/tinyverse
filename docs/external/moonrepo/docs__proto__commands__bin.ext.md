----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/bin
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, bin
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/bin

- [Home](/)
- Commands
- [bin](/docs/proto/commands/bin)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# bin

The `proto bin  [version]` command will return an absolute path to a tool's binary within the
toolchain. When a tool has not been installed, or a version cannot be resolved, the command will
exit with a failure.

```
$ proto bin node 16.10.0/Users/example/.proto/tools/node/16.10.0/bin/node
```

This command can also return directories using the `--dir` option.

```
$ proto bin node 16.10.0 --dir exes/Users/example/.proto/tools/node/16.10.0/bin$ proto bin node 16.10.0 --dir globals/Users/example/.proto/tools/node/globals/bin
```

### Arguments[​](#arguments)

- `` - Type of tool.

- `[version]` - Version of tool. If not provided, will attempt to [detect the version](/docs/proto/detection).

### Options[​](#options)

- `--all` - Return multiple paths, separated by newlines, instead of the first path. v0.50.0

- `--dir ` - Return a directory instead of of the main file. v0.50.0 `exes` - Returns the executable's directory.

- `globals` - Returns the globals/packages directory.

- `--bin` - When applicable, return the `~/.proto/bin` path.

- `--shim` - When applicable, return the `~/.proto/shims` path.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/bin.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

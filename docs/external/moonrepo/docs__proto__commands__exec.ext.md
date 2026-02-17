----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/exec
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, exec
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/exec

- [Home](/)
- Commands
- [exec](/docs/proto/commands/exec)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# exec

v0.53.0

The `proto exec  -- ` (or `proto x`) command will activate a temporary
environment by loading and initializing any number of tools, and then execute an arbitrary command
within that environment.

```
$ proto exec node pnpm -- pnpm run dev
```

Tools will automatically detect a version to execute with based on loaded `.prototools`, but the
version can be provided inline by suffixing the tool with `@`.

```
$ proto exec node@24.2 pnpm@10 -- pnpm run dev
```

### Shell support[​](#shell-support)

By default, the command will not be executed in a shell, and will be excuted in the context of the
parent process. If you want to execute the command within a shell (using `-c`), you can use the
`--shell` option.

```
$ proto exec node pnpm --shell bash --
```

If your command contains special characters, complex expressions, or shell specific syntax, you may
need to pass `--raw` to avoid quoting/escaping issues.

```
$ proto exec node pnpm --shell bash --raw --
```

Furthermore, if you want to launch an interactive shell session with the activated environment, you
can pass the shell command itself as the exec command.

```
$ proto exec node pnpm -- bash
```

### Arguments[​](#arguments)

- `` - List of tool identifiers with optional version.

- `` - Command to execute within the environment. Must be passed after a `--` separator.

### Options[​](#options)

- `--tools-from-config` - Inherit tools to initialize from `.prototools` configs, instead of passing an explicit list.

- `--raw` - Execute the command as-is without quoting or escaping when using `--shell`.

- `--shell` - Shell to execute the command with (e.g. `bash` or `pwsh`).

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/exec.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

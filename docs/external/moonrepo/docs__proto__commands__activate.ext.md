----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/activate
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, activate
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/activate

- [Home](/)
- Commands
- [activate](/docs/proto/commands/activate)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# activate

v0.38.0

The `proto activate ` command will activate proto for the current shell session, by exporting
environment variables and prepending `PATH` for each tool configured in the current directory.
Activation is ran each time the current directory changes using a shell hook.

info

Learn more about
[shell activation in the official workflow documentation](/docs/proto/workflows#shell-activation)!

### Arguments[​](#arguments)

- `` - The shell to activate for.

### Options[​](#options)

- `--export` - Print the activate instructions in shell-specific syntax.

- `--json` - Print the activate instructions in JSON format.

- `--no-bin` - Do not include `~/.proto/bin` when appending `PATH`.

- `--no-shim` - Do not include `~/.proto/shims` when prepending `PATH`.

- `--no-init` - Do not trigger activation when initialized in the shell, and instead wait for a cd/prompt change. v0.50.0

### Caveats[​](#caveats)

- Only tools that have a [version configured in `.prototools`](/docs/proto/config#pinning-versions) will be activated.

- Tool versions configured in the global `~/.proto/.prototools` are not included by default. Pass `--config-mode all` during activation to include them. Do note that this will worsen performance depending on the number of tools.

### Setup[​](#setup)

The following activation steps should be added after all environment variable and `PATH`
modifications have happened in your shell, typically at the end of your shell profile.

#### Bash[​](#bash)

Add the following line to the end of your `~/.bashrc` or `~/.bash_profile`.

```
eval "$(proto activate bash)"
```

#### Elvish[​](#elvish)

Generate the hook:

```
proto activate elvish > ~/.elvish/lib/proto-hook.elv
```

Then add the following line to your `~/.elvish/rc.elv` file.

```
use proto-hook
```

#### Fish[​](#fish)

Add the following line to the end of your `~/.config/fish/config.fish`.

```
proto activate fish | source
```

#### Murex[​](#murex)

Add the following line to the end of your `~/.murex_profile`.

```
proto activate murex -> source
```

#### Nu[​](#nu)

Generate the hook:

```
(proto activate nu) | save ~/.config/nushell/proto-hook.nu
```

Then add the following line to your `~/.config/nushell/config.nu` file.

```
use proto-hook.nu
```

#### Pwsh[​](#pwsh)

Add the following line to the end of your profile (`$PROFILE`).

```
proto activate pwsh | Out-String | Invoke-Expression
```

#### Zsh[​](#zsh)

Add the following line to the end of your `~/.zshrc`.

```
eval "$(proto activate zsh)"
```

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/activate.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

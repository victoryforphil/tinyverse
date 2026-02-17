----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/setup
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, setup
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/setup

- [Home](/)
- Commands
- [setup](/docs/proto/commands/setup)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# setup

The `proto setup` command will setup proto in your current shell by modifying an applicable profile
file and appending proto's bin directory to `PATH`. If a shell could not be detected, you'll be
prompted to select one.

```
$ proto setup
```

During setup, the following profiles will be searched or prompted for.

- Bash `~/.bash_profile`

- `~/.bashrc`

- `~/.profile`

- Elvish `~/.elvish/rc.elv`

- `~/.config/elvish/rc.elv`

- Fish `~/.config/fish/config.fish`

- Ion `~/.config/ion/initrc`

- Murex `~/.murex_preload`

- `~/.murex_profile`

- Nu `~/.config/nushell/env.nu`

- `~/.config/nushell/config.nu`

- PowerShell Windows `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`

- `~\Documents\PowerShell\Profile.ps1`

- Unix `~/.config/powershell/Microsoft.PowerShell_profile.ps1`

- `~/.config/powershell/profile.ps1`

- Xonsh `~/.config/xonsh/rc.xsh`

- `~/.xonshrc`

- Zsh `~/.zprofile`

- `~/.zshenv`

- `~/.zshrc`

### Windows support[​](#windows-support)

In addition to updating a shell profile file (most likely PowerShell), we'll also modify the `PATH`
(or `Path`) system environment variable, by prepending the `~/.proto/shims` and `~/.proto/bin`
paths.

If you would like to opt-out of this behavior, pass the `--no-modify-path` flag.

### Options[​](#options)

- `--shell` - Shell to explicitly setup for.

- `--no-modify-profile` / `PROTO_NO_MODIFY_PROFILE` - Don't update a shell profile file.

- `--no-modify-path` / `PROTO_NO_MODIFY_PATH` - Don't update the system `PATH` environment variable (Windows only).

- `--yes` - Avoid interactive prompts and use defaults.

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/setup.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

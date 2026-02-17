----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-17T03:11:54.185Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/completions
- Keywords: moonrepo, docs, monorepo, task runner, toolchain, proto, commands, completions
- Summary: Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).
----

Source: https://moonrepo.dev/docs/proto/commands/completions

- [Home](/)
- Commands
- [completions](/docs/proto/commands/completions)

warning

Documentation is currently for [moon v2](/blog/moon-v2-alpha) and latest proto. Documentation for moon v1 has been frozen and can be [found here](https://moonrepo.github.io/website-v1/).

# completions

The `proto completions` command will generate proto command and argument completions for your
current shell. This command will write to stdout, which can then be redirected to a file of your
choice.

```
$ proto completions > ./path/to/write/to
```

### Options[​](#options)

- `--shell` - Shell to explicitly generate for.

### Examples[​](#examples)

- Bash
- Fish
- Zsh

If using [bash-completion](https://github.com/scop/bash-completion).

```
mkdir -p ~/.bash_completion.dproto completions > ~/.bash_completion.d/proto.sh
```

Otherwise write the file to a common location, and source it in your profile.

```
mkdir -p ~/.bash_completionsproto completions > ~/.bash_completions/proto.sh# In your profilesource ~/.bash_completions/proto.sh
```

Write the file to Fish's completions directory.

```
mkdir -p ~/.config/fish/completionsproto completions > ~/.config/fish/completions/proto.fish
```

If using [oh-my-zsh](https://ohmyz.sh/) (the `_` prefix is required).

```
mkdir -p ~/.oh-my-zsh/completionsproto completions > ~/.oh-my-zsh/completions/_proto# Reload shell (or restart terminal)omz reload
```

[Edit this page](https://github.com/moonrepo/moon/tree/master/website/docs/proto/commands/completions.mdx)

----
## Notes / Comments / Lessons

- Collection method: sitemap-first discovery scoped to moonrepo docs.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----

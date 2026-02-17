---
description: Create clean, grouped commits for all current changes
agent: build
---

Use the `gitter-commit` skill from `.opencode/skills/gitter-commit/SKILL.md` and route commit execution through `@gitter`.

Goal:

- Turn the current working tree into a clean set of meaningful commits (not a single `git add -A` dump)
- End with a clean `git status`

Process:

1. Summarize the current thread and work performed to provide context for the commits.
2. Inspect the repository state (`git status`, staged/unstaged diffs, and recent commit titles for style).
3. Group changes into logical commit units (by feature, fix, docs, config, refactor, etc.).
4. (Optional) For complex sets of changes, spawn parallel `@gitter` subagents to summarize and process specific change groups.
5. For each commit unit:
   - Stage only the relevant files.
   - Write a commit title using this repo format:
     - `{Component/Meta} // {Optional Addition} // {Short Description} (Optional,Tags)`
   - Add a short commit body rationale and signature when possible.
6. Repeat until all intended tracked changes are committed.
7. Confirm final `git status` is clean.

Rules:

- Prefer 1-3 commits total for most runs.
- Ideal default is 1-2 commits unless there is clearly separate work.
- Only exceed 3 commits when there was substantial parallel or truly independent change streams.
- It is okay to combine related changes with `+` or summarize into a single cohesive commit.
- Do not commit likely secret files (`.env`, credentials, private keys).
- Do not push.
- If no commit-worthy changes exist, report that clearly.

Return:

- Commit list in order (hash + title + files)
- Any skipped files and why
- Final `git status` summary

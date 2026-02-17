---
description: Triage failing GitHub Actions and babysit reruns to green
agent: babysitter
---

Use the `gha-babysit` skill from `.opencode/skills/gha-babysit/SKILL.md`.

Goal:

- Monitor GitHub Actions reruns without parent timeout.
- Provide high-signal watch status and escalation guidance.

Process:

1. List recent runs for the current branch (`gh run list --branch <branch> --limit 10`).
2. Inspect the target run summary (`gh run view <run-id>`).
3. Watch live progress in PTY (`gh run watch <run-id> --interval 10`).
4. Report concise status updates (running/completed/failed/cancelled/timed out).
5. If the run fails, collect failure logs (`gh run view <run-id> --log-failed`) and escalate engineering work to:
   - `@.opencode/agents/developer_jr.md` for simple/scoped fixes
   - `@.opencode/agents/developer_senior.md` for complex/ambiguous fixes
6. When fixes are ready to submit, route git staging/commit/push through `gitter-commit` + `@.opencode/agents/gitter.md` for predictable CI-only submission.

Return:

- Run IDs inspected and watched
- Current CI state (green/failing/blocked)
- Failed job names and first failure signal (if failing)
- Clear escalation target (`developer_jr` or `developer_senior`) when fixes are needed

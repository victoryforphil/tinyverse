---
name: gha-babysit
description: Triage failing GitHub Actions runs, apply fixes, push, and watch reruns in a repeatable loop.
---

## What I do

- Standardize the CI recovery loop: inspect failed run -> fix -> verify -> commit/push -> watch rerun.
- Prefer `gh` CLI for run discovery/log extraction and PTY-based watch for long-running status streams.
- Keep fixes minimal and tied to concrete failing commands from logs.
- For long-running watch mode, prefer the `@babysitter` agent (for example via `/gha_babysit`).
- Keep git submission predictable by routing commit/push steps through the `gitter-commit` skill and `@gitter` agent.

## When to use me

Use this when:

- The user asks to "fix CI" or "make GitHub Actions green".
- A push run failed and we need iterative repair.
- We need a predictable handoff/checklist for repeated GHA triage.

## Workflow

1. **List recent runs**
   - `gh run list --branch <branch> --limit 10`
2. **Inspect failure details**
   - `gh run view <run-id> --log-failed`
3. **Patch root cause**
   - Fix only what is required by the failing logs.
4. **Verify locally**
   - Run targeted local checks relevant to the failure path.
5. **Commit + push**
   - Use the `gitter-commit` skill and `@gitter` agent for staging/commit/push steps.
   - Prefer submitting only CI-related work touched in this loop; accidental WIP in a push is acceptable, but avoidable in most cases.
   - Use a focused commit message describing the failure class and fix intent.
6. **Watch rerun with PTY**
   - `gh run watch <new-run-id>` in PTY to avoid timeout and keep live output.
7. **Repeat until green**
   - Iterate log -> fix -> push -> watch until all required jobs pass.

## Guardrails

- Do not guess root cause without failed logs.
- Avoid broad refactors while recovering CI unless explicitly requested.
- If repeated failures point to secrets, permissions, or external outage, escalate with a clear blocker note.
- If code changes are required during a watch flow, route implementation to `@.opencode/agents/developer_jr.md` or `@.opencode/agents/developer_senior.md`.
- Do not do ad-hoc git submission from babysit flow when `gitter-commit`/`@gitter` can be used.

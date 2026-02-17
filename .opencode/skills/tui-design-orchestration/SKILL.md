---
name: tui-design-orchestration
description: Orchestrate Ratatui design critique, component planning, parallel implementation, and final polish with @tui_designer.
---

## What I do

- Route Ratatui design work through `@tui_designer` with a repeatable critique-to-build loop.
- Require `@explore` context gathering before major design decisions.
- Encourage parallel component implementation, then parent-agent integration and final design QA.

## When to use me

Use this skill when the user asks for TUI redesign, visual polish, UX critique, componentization, or Ratatui layout/widget improvements.

## Required workflow

1. Start with context sweeps via `@explore` in parallel:
   - Code context pass: locate current UI/rendering/component files and constraints.
   - Docs context pass: search `docs/external/ratatui/` for relevant patterns (layout, widgets, component architecture, state, event handling).
2. Invoke `@tui_designer` for design critique of the current project.
3. Invoke `@tui_designer` for a component plan:
   - list proposed components
   - boundaries and responsibilities
   - implementation order
4. Spawn multiple `@tui_designer` tasks in parallel, one focused component per task.
5. Parent agent integrates all component work, resolves overlap, and validates behavior.
6. Run one final `@tui_designer` pass for consistency feedback and finishing advice.

## Handoff template

Use this structure when delegating to `@tui_designer`:

- Goal: <design objective>
- Scope: <file paths or screen areas>
- Context: <findings from `@explore` and ratatui docs snapshots>
- Constraints: <behavioral, style, keyboard UX, performance>
- Deliverable: <critique, component plan, implementation, or final review>

## Return checklist

- Critique findings and priority order
- Component backlog and ownership split
- Parallel task list (component per subtask)
- Integration notes for parent agent
- Final polish feedback from closing design pass

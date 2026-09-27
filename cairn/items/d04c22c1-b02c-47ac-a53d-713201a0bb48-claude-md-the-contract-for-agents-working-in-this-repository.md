---
id: d04c22c1-b02c-47ac-a53d-713201a0bb48
title: 'CLAUDE.md: the contract for agents working in this repository'
type: docs
status: planned
milestone: v0.1
created: 2026-09-26
updated: 2026-09-26
priority: p1
effort: s
area: docs
---

## Problem

An agent joining this repository needs the workflow and the conventions in one place, or it will invent its own and commit to the default branch.

## Proposal

`CLAUDE.md`, written as instructions rather than prose about the project: the `scripts/task` seam, the one worktree per branch rule, Conventional Commits, that the backlog is cairn and the board is the plan, and that no commit, comment or pull request names an assistant. Under 200 lines. Anything language-specific goes in `.claude/rules/` with `paths:` frontmatter instead, and anything task-shaped becomes a skill.

## Acceptance criteria

- [x] it names the seam, the workflow, the commit convention, and the attribution ban
- [x] it is under 200 lines and contains nothing that duplicates `CONTRIBUTING.md`
- [x] it tells an agent to run `cairn next` to find work

---
id: 9ef6fdb5-71dc-4b3d-9072-77e88d3dc1b8
title: Prove the whole loop end to end once
type: chore
status: planned
milestone: v0.1
depends_on:
- c362d83c-6a8e-4fac-b711-5dcb9a45f6d9
- fd2d1eef-b964-46c9-9ab1-ab256212e3a0
created: 2026-09-26
updated: 2026-09-26
priority: p0
effort: s
area: infra
---

## Problem

Files written is not a workflow proven. Every step below has to be watched passing, because the ones that break are the ones nobody exercised.

## Proposal

`scripts/agent doctor` clean, `start` a throwaway branch, make one real change, `commit`, `pr`, watch CI go green with `gh pr checks --watch`, confirm a direct push to the default branch is rejected both locally and on the server, squash merge, `done`, and confirm the worktree, local branch and remote branch are all gone.

## Acceptance criteria

- [ ] every step above was run and watched, not assumed
- [ ] the default branch log shows the squashed commit and no attribution anywhere
- [ ] `scripts/agent list` is empty of stray worktrees afterwards

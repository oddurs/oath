---
id: 9ef6fdb5-71dc-4b3d-9072-77e88d3dc1b8
title: Prove the whole loop end to end once
type: chore
status: done
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

- [x] every step above was run and watched, not assumed
- [x] the default branch log shows the squashed commit and no attribution anywhere
- [x] `scripts/agent list` is empty of stray worktrees afterwards

## 2026-09-26

Verified end to end on 2026-09-27: doctor clean, agent start created the worktree, agent commit refused an unstaged tree then committed, agent pr pushed and opened #3, both platforms plus required went green, a direct push to main was refused locally by pre-push and by the server with GH006, the squash merge landed as ce800d7, and no attribution appears anywhere in the history.

Two rough edges surfaced and were fixed in the process: pre-push blocked the very first push of main, since refusing a push to the default branch only makes sense once it exists; and agent commit fell through to git's own 'no changes added to commit'.

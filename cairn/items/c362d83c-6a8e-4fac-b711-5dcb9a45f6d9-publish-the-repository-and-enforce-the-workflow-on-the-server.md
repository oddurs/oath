---
id: c362d83c-6a8e-4fac-b711-5dcb9a45f6d9
title: Publish the repository and enforce the workflow on the server
type: chore
status: done
milestone: v0.1
depends_on:
- 19e43dee-07bc-438f-a397-d0460f4df7fd
- 90e139dc-d2c2-4f83-85ac-82b859d459d1
- b878db8a-6bb0-4b38-a4c6-55e463652fc6
created: 2026-09-26
updated: 2026-09-26
priority: p0
effort: m
area: infra
---

## Problem

Every rule this project relies on is currently local and therefore optional. Branch protection is what makes the incorrect thing impossible rather than merely discouraged.

## Proposal

`gh repo create oddurs/oath --public --source=. --remote=origin --push`, then configure and verify each of:

- squash merge only, merge commits and rebase merges disabled, head branches deleted on merge
- default workflow permissions read-only
- branch protection on the default branch: a pull request required, the `required` status check must pass, branches up to date, conversation resolution required, force pushes and deletions blocked, stale approvals dismissed, and 0 required approvals so a solo owner is not deadlocked
- private vulnerability reporting and Dependabot alerts enabled
- repository topics

Anything the token cannot do is reported with the exact command rather than skipped silently.

## Acceptance criteria

- [x] `gh repo view` shows squash-only and delete-on-merge
- [x] a direct push to the default branch is rejected by the server, watched and recorded
- [x] a pull request cannot merge until the `required` check passes
- [x] `gh api` confirms private vulnerability reporting is on

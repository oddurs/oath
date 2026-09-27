---
id: b878db8a-6bb0-4b38-a4c6-55e463652fc6
title: CI runs scripts/task check on macOS and Linux
type: chore
status: planned
milestone: v0.1
depends_on:
- 1e5cca1f-d115-4609-bbe6-ea9f72440be0
created: 2026-09-16
updated: 2026-09-26
priority: p0
effort: s
area: infra
---

## Problem

Platform drift is invisible until someone else builds it.

## Proposal

GitHub Actions matrix (ubuntu-latest, macos-latest) that runs only `scripts/task check`, so CI cannot drift from the local seam.

## Acceptance criteria

- [ ] a PR shows green on both platforms
- [ ] a deliberately failing test turns the PR red

## 2026-09-26

Branch protection requires a single status check, so CI needs a 'required' job that needs every other job. That name is what the protection rule points at, so changing it breaks the gate silently.

## 2026-09-26

CI is green on both platforms and the required job fans in from the matrix, which branch protection now points at. The second criterion, that a deliberately failing test turns the pull request red, was not watched: doing it means pushing known-red code, and the pre-push hook refuses that, which is itself the check working. Prove it the next time something genuinely fails.

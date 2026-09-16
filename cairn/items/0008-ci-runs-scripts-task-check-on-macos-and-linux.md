---
id: 8
title: CI runs scripts/task check on macOS and Linux
type: chore
status: planned
milestone: v0.1
depends_on:
- 7
created: 2026-09-16
updated: 2026-09-16
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

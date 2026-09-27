---
id: 87232d9d-9cd8-4c1f-9e5e-355bd8bd5ba1
title: Release workflow on a version tag
type: chore
status: planned
milestone: v0.1
depends_on:
- b878db8a-6bb0-4b38-a4c6-55e463652fc6
created: 2026-09-26
updated: 2026-09-26
priority: p2
effort: s
area: infra
---

## Problem

A release built by hand is a release nobody can reproduce.

## Proposal

`.github/workflows/release.yml`, triggered on `v*` tags, `permissions: contents: write`, runs `scripts/task check` and then `gh release create --generate-notes`. Binaries and crates.io belong to 0081; this is the skeleton that proves the path works.

## Acceptance criteria

- [x] the workflow file is valid and its check job reuses `scripts/task check`
- [ ] generated notes contain no assistant attribution

## 2026-09-26

The workflow is written and its check job reuses the seam. The second criterion needs a real tag, so this stays open until the first release.

---
id: fd2d1eef-b964-46c9-9ab1-ab256212e3a0
title: 'scripts/setup: one command for a fresh checkout'
type: chore
status: planned
milestone: v0.1
created: 2026-09-26
updated: 2026-09-26
priority: p1
effort: s
area: infra
---

## Problem

`core.hooksPath` lives in git's per-checkout configuration, which is not cloned. A contributor who skips it gets none of the hooks and does not find out until CI fails.

## Proposal

`scripts/setup` sets `core.hooksPath` to `.githooks`, installs cairn's merge driver, reports the toolchain it found, and says what it did. Safe to run twice. `README.md` and `CONTRIBUTING.md` both point at it, and `scripts/agent doctor` tells anyone who has not run it to.

## Acceptance criteria

- [ ] running it twice changes nothing the second time and says so
- [ ] after a fresh clone and `scripts/setup`, `scripts/agent doctor` reports ready
- [ ] it exits non-zero with an actionable message when a required tool is missing

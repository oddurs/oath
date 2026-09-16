---
id: 25
title: 'README: install from source and the first sworn program'
type: docs
status: planned
milestone: v0.1
depends_on:
- 23
- 24
- 42
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: docs
---

## Problem

Without this, v0.1 is not installable by a stranger.

## Proposal

README with: the one-sentence pitch, `cargo install --path .`, a 20-line program (an oath, a sworn keeper, an unsworn keeper), the `oath swear` output, the `oath run` output, and the four things v0.1 does not do.

## Acceptance criteria

- [ ] every command in the README is copied from a golden test so it cannot rot
- [ ] the program in the README is `examples/readme.oath` and is in the golden suite

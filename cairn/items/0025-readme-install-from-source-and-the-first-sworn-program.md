---
id: d73c6445-ed8c-465f-bf91-1020fc732f04
title: 'README: install from source and the first sworn program'
type: docs
status: planned
milestone: v0.1
depends_on:
- 00b844e9-f272-448c-aa30-2d4595d7b3df
- 4950905e-b874-422e-b613-61ab15297f2e
- 55fcecb3-5ef2-46d1-a952-7139eb533e58
created: 2026-09-16
updated: 2026-09-26
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

## 2026-09-26

The bootstrap README lands earlier, at publication, describing what runs today. This item is the update that adds the first sworn program once the loop works. Do not write a second README; extend that one.

---
id: 7f1da788-89c6-4077-9b8e-09f84800c143
title: Automatic undo journal of cell writes
type: feature
status: backlog
milestone: v0.2
depends_on:
- 2d1cc97f-2183-4421-abbb-7a1bfce2ab0b
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: l
area: effects
---

## Problem

Performing an op must leave enough behind to reverse it, and the reversal must be correct without trusting the user.

## Proposal

Every write to a state cell pushes `(cell, old value)` on the journal. Every `external` perform pushes `(op, args)`. A journal mark is taken at `try` entry. Rewind pops to the mark: cell entries restore the old value, external entries run the declared `undo`. The journal is per evaluation, never global.

## Acceptance criteria

- [ ] property test: for random sequences of cell writes and ops, perform then rewind leaves every cell at its initial value
- [ ] an `undo` that itself fails aborts with a diagnostic naming the op; no partial rewinds are hidden
- [ ] two `oath run` invocations never share a journal

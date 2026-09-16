---
id: 59
title: 'oath trace: step backwards through the journal'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 28
- 35
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: l
area: effects
---

## Problem

The journal is a time machine; expose it.

## Proposal

`oath trace file.oath` runs to completion recording the journal, then a prompt: `b` undo one op, `f` redo, `p` print all cells, `q`. Uses the same `undo` clauses, so the debugger proves they are correct.

## Acceptance criteria

- [ ] golden test with scripted stdin steps back twice and prints a cell value
- [ ] stepping back past the start says so and stays put

## 2026-09-16

Decision (round 3): undo for cells is derived, so trace is a debugger and the check on external compensations, not a check on hand-written cell inverses.

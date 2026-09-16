---
id: 22
title: Swearing by examples; oath swear subcommand
type: feature
status: planned
milestone: v0.1
depends_on:
- 18
- 21
- 96
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

A keeper is sworn when it keeps the oath. In v0.1 the evidence is the examples.

## Proposal

For each oath, for each keeper: evaluate every `eg` with the keeper bound as the oath's implementation. All match → sworn. `oath swear file.oath` prints one line per keeper: `sworn`/`unsworn`, the oath hash, the label, and for unsworn the first failing example with expected and actual.

## Acceptance criteria

- [ ] exit 0 when every keeper is sworn, 1 otherwise
- [ ] a keeper that errors at runtime on an example is unsworn with the diagnostic attached
- [ ] an oath with zero keepers is reported as `unkept`, and is not an error in `swear`
- [ ] golden tests for all three outcomes

## 2026-09-16

Decision (round 3): everything callable is an oath, including `main`. An oath with no examples and no laws is sworn vacuously and `oath swear` prints `main: sworn (no evidence)` as a warning. There is no second kind of definition.

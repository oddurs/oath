---
id: 53
title: Reaching a hole raises the Keep effect
type: feature
status: backlog
milestone: v0.3
depends_on:
- 23
- 37
- 99
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: holes
---

## Problem

Filling a hole is a decision. In Oath, decisions are effects, so the program can own its policy.

## Proposal

Builtin `effect Keep { keep : Oath -> Keeper }`, irreversible. Evaluating a hole keeper, or calling an oath with no sworn keeper, performs `Keep.keep` with the oath value. The handler returns a keeper; the runtime swears it and, if sworn, resumes with it bound. Unhandled `Keep` is a runtime error naming the oath and its hash.

## Acceptance criteria

- [ ] a user handler that returns a fixed body fills the hole in a golden test
- [ ] a handler returning an unsworn body causes a runtime error saying which check failed

## 2026-09-16

Decision (round 3): `Keep : Str -> Str`. The handler receives the oath pretty-printed (item 0051) and returns source text for one keeper body; the runtime parses, swears, and stores it. No quoted-code values on the v1.0 path.

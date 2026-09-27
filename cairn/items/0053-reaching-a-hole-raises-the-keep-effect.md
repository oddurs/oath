---
id: f84f6083-ba76-4381-bd94-ad1db39522af
title: Reaching a hole raises the Keep effect
type: feature
status: backlog
milestone: v0.3
depends_on:
- 00b844e9-f272-448c-aa30-2d4595d7b3df
- 2127e390-7dba-42c3-a471-d1cb327d0737
- 7f9eae9c-21d2-4660-b008-f807c1a79f04
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
- [ ] the pretty-printed oath handed to the handler ends with `at call: sort [5,2,9]` when reached from a call with those arguments

## 2026-09-16

Decision (round 3): `Keep : Str -> Str`. The handler receives the oath pretty-printed (item 0051) and returns source text for one keeper body; the runtime parses, swears, and stores it. No quoted-code values on the v1.0 path.

## 2026-09-16

Added criterion (from Hazel's hole closures, docs/research/05): the environment at the hole is the live call. The Keep payload appends the actual arguments of the call that reached the hole as a candidate example, marked as such, so a person or a command sees a concrete input for free.

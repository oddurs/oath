---
id: 38
title: Small-scope value generator
type: feature
status: backlog
milestone: v0.2
depends_on:
- 12
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

Laws need inputs. Exhaustive small inputs find more bugs per second than random ones and give minimal counterexamples for free.

## Proposal

Enumerate values of a type up to a depth: Int in `-depth..=depth`, Bool, lists up to `depth` long, tuples, user types by constructor. `oath swear --depth N`, default 4. Ordered smallest first.

## Acceptance criteria

- [ ] enumeration of `List Int` at depth 3 matches a hand-written expected list
- [ ] depth 6 for `List Int -> List Int` completes under one second

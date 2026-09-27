---
id: 3cb4f026-cd20-453e-8663-54a249fc36fb
title: try/else rolls back to the block entry
type: feature
status: backlog
milestone: v0.2
depends_on:
- 7f1da788-89c6-4077-9b8e-09f84800c143
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: effects
---

## Problem

The user-visible payoff of reversible effects is a transaction that cannot half-apply.

## Proposal

`try e else f`: on `fail` inside `e`, rewind to the mark and evaluate `f`. Nested `try` nests marks. A runtime error inside `try` also rewinds, then propagates.

## Acceptance criteria

- [ ] golden test: a transfer that fails on insufficient funds leaves both balances unchanged
- [ ] nested `try` rewinds only the inner block

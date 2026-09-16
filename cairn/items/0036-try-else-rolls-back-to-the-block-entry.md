---
id: 36
title: try/else rolls back to the block entry
type: feature
status: backlog
milestone: v0.2
depends_on:
- 35
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

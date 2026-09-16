---
id: 34
title: Irreversible ops are refused inside try and during swearing
type: feature
status: backlog
milestone: v0.2
depends_on:
- 97
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: effects
---

## Problem

Rollback is a promise only if nothing inside a rolled-back region is unrecoverable.

## Proposal

A lexical pass: `perform` of an `irreversible` op inside `try` is a check error at the perform's span. A keeper that performs an irreversible op anywhere in its body, transitively through other oaths, is `unswearable` and `oath swear` says which op and where. `print` is the canonical irreversible op.

## Acceptance criteria

- [ ] golden tests for both errors
- [ ] transitive detection: a keeper calling an oath whose only keeper prints is unswearable, message names the chain

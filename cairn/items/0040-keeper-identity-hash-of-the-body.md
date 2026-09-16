---
id: 40
title: 'Keeper identity: hash of the body'
type: feature
status: planned
milestone: v0.1
depends_on:
- 13
- 21
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: oath
---

## Problem

The store needs to name keepers, and the same body under two labels is the same keeper.

## Proposal

Canonicalise and hash keeper bodies with the same machinery as oaths. `oath swear` prints `<oath-hash> <keeper-hash> <label> <verdict>`.

## Acceptance criteria

- [ ] renaming a bound variable in a keeper does not change its hash
- [ ] two labels with identical bodies are reported as the same keeper

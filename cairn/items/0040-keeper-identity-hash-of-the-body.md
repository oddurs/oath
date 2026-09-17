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

The store needs to name keepers, and the same body under two labels is the same keeper. A keeper's identity must also move when a promise it calls moves, or its evidence would outlive its meaning.

## Proposal

Canonicalise and hash keeper bodies with the same machinery as oaths: bound variables by position, every called oath substituted by its oath hash (not its keeper's hash; a keeper depends on promises, not on bodies). `oath swear` prints `<oath-hash> <keeper-hash> <label> <verdict>`.

## Acceptance criteria

- [ ] renaming a bound variable in a keeper does not change its hash
- [ ] two labels with identical bodies are reported as the same keeper
- [ ] changing the promise of `insert` changes the hash of every `sort` keeper that calls it; changing `insert`'s keeper does not (tests)

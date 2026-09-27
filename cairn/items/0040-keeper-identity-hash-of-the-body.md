---
id: 73ab165c-be8f-461f-ac76-e03220dc8218
title: 'Keeper identity: hash of the body'
type: feature
status: planned
milestone: v0.1
depends_on:
- d9310c65-7ebb-40db-8ed0-41a5c7ecb153
- 5d43d6fa-435e-436f-b8b9-262f3891e383
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

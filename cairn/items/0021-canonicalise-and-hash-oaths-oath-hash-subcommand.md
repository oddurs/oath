---
id: 21
title: Canonicalise and hash oaths; oath hash subcommand
type: feature
status: planned
milestone: v0.1
depends_on:
- 20
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

The whole idea is that a call site names a promise by its hash, and a promise means nothing without the promises it refers to.

## Proposal

Implement the spike's answer. Following Unison, the canonical form substitutes every referenced oath (in the signature's user types, in laws, in examples) with that oath's hash, so the hash of `sort` pins the promise of `sorted` and `length` that its laws rely on. The name is not included. `oath hash file.oath` prints `<name> <hash>` per oath.

## Acceptance criteria

- [ ] renaming an oath does not change its hash (test)
- [ ] reordering examples, changing whitespace, or renaming a bound variable does not change its hash (tests)
- [ ] changing the type or any example changes the hash (tests)
- [ ] changing the promise of an oath referenced in a law changes this oath's hash; changing only that oath's keeper does not (tests)
- [ ] adding or changing a `requires` clause changes the oath's hash
- [ ] two oaths in one file with the same hash but different names is a check-time warning

## 2026-09-16

Added criterion (from design by contract, docs/research/02): preconditions are part of the promise.

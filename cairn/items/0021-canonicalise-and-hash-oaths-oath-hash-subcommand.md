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

The whole idea is that a call site names a promise by its hash.

## Proposal

Implement the spike's answer. `oath hash file.oath` prints `<name> <hash>` per oath. Hash is over the canonical form only; the name is not included.

## Acceptance criteria

- [ ] renaming an oath does not change its hash (test)
- [ ] reordering examples, changing whitespace, or renaming a bound variable does not change its hash (tests)
- [ ] changing the type or any example changes the hash (tests)
- [ ] two oaths in one file with the same hash but different names is a check-time warning

---
id: 42
title: Oath change un-swears its keepers visibly
type: feature
status: planned
milestone: v0.1
depends_on:
- 41
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

The reason for contract addressing: a changed promise cannot silently keep its old evidence.

## Proposal

A changed oath has a new hash and therefore an empty store entry. `oath swear` detects that the previous hash for this name existed and prints `sort: oath changed (old → new); 2 keepers un-sworn`. Old records are kept for `oath who`.

## Acceptance criteria

- [ ] golden test: adding an example and re-swearing shows the un-sworn message and re-swears
- [ ] `oath run` after an oath change without re-swearing fails at the first call with the same message

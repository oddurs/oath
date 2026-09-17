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

The reason for contract addressing: a changed promise cannot silently keep its old evidence, and neither can anything built on that promise.

## Proposal

A changed oath has a new hash and therefore an empty store entry. Because keeper hashes include the hashes of the oaths they call, every keeper downstream of the change also gets a new hash and an empty entry. `oath swear` reports both, with the chain: `sort by "reference": un-sworn because insert's promise changed (old → new)`. Old records are kept for `oath who`.

## Acceptance criteria

- [ ] golden test: adding an example and re-swearing shows the un-sworn message and re-swears
- [ ] golden test: changing `insert`'s example un-swears `sort`'s keepers and the message names `insert`
- [ ] `oath run` after an oath change without re-swearing fails at the first call with the same message

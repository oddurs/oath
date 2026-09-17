---
id: 78
title: Pin golden hashes for the examples
type: chore
status: backlog
milestone: v1.0
depends_on:
- 21
- 29
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: oath
---

## Problem

A change to canonicalisation silently un-swears everyone's keepers.

## Acceptance criteria

- [ ] `tests/hashes.txt` lists the hash of every example oath; CI fails on any change
- [ ] the file's header says that changing it is a breaking change requiring a major version
- [ ] tests/hashes.txt includes every prelude oath

## 2026-09-16

Added criterion: the prelude's oath hashes are pinned too, since every user oath whose laws use the prelude depends on them.

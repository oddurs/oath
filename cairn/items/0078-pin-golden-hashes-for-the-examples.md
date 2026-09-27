---
id: f2ae4108-fa38-4a68-b31e-06e5a8361e27
title: Pin golden hashes for the examples
type: chore
status: backlog
milestone: v1.0
depends_on:
- 5d43d6fa-435e-436f-b8b9-262f3891e383
- e79e4682-c8e2-45db-8f45-8f2b05c490d0
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

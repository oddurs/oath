---
id: 29
title: User-defined data types
type: feature
status: backlog
milestone: v0.2
depends_on:
- 18
- 38
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: eval
---

## Problem

A language with only lists and tuples cannot express a bank account.

## Proposal

`type Shape = Circle Int | Rect Int Int`. Constructors, patterns, exhaustiveness warning. Type declarations are hashed and an oath's hash includes the hashes of the types it mentions.

## Acceptance criteria

- [ ] constructors and matching in the golden suite
- [ ] changing a type's definition changes the hash of every oath mentioning it (test)
- [ ] the small-scope generator can enumerate values of a user type

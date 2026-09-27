---
id: 470bab2e-32de-400e-804c-68b1d16ce8d6
title: 'Generator: function arguments from a table, polymorphic oaths at Int'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 908186e3-49b5-49ee-bb45-d7f5efe3187e
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: oath
---

## Problem

`map : (a -> b) -> List a -> List b` has laws, and the generator cannot enumerate functions.

## Proposal

Type variables are instantiated at `Int` for generation (documented; `--instantiate Bool` for a second pass). Function-typed arguments are drawn from a small table per signature: `Int -> Int` gives `+1 *2 negate const0 id`, `Int -> Bool` gives `even positive const-true`, `a -> a -> a` gives `+ max first`. Tables are Oath source in the prelude so users can extend them.

## Acceptance criteria

- [ ] prelude `map` and `filter` laws swear at depth 4
- [ ] a signature with no table entry is reported as `laws skipped: no generator for (Int -> Int -> Int -> Int)`, never silently sworn

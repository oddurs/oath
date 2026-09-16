---
id: 96
title: Runtime contract check at the oath boundary
type: feature
status: planned
milestone: v0.1
depends_on:
- 12
- 18
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

Oath has no static type checker on the v1.0 path. The signature is still a promise, so it must be checked somewhere.

## Proposal

Every call through an oath checks the arguments against the declared argument types and the result against the declared result type, at runtime. A violation is a contract error that blames the keeper (result) or the caller (argument), with both spans. v0.1 types: `Int`, `Bool`, `List t`, tuples, `a -> b` (functions are wrapped and checked on call), rigid type variables checked for consistency within one call. `Str` and `Unit` join in v0.2 through the same table.

## Acceptance criteria

- [ ] a keeper returning `Int` for an oath promising `List Int` is unsworn with `contract: sort by "bad" returned Int, promised List Int`
- [ ] a caller passing `true` to `sort` fails at the call site, not inside the keeper
- [ ] the check costs nothing measurable at depth 4 swearing (bench noted in the PR)

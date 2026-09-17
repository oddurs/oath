---
id: 121
title: 'Preconditions: requires clauses on oaths'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 27
- 38
- 96
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

Design by contract has three parts: preconditions (the caller's obligation), postconditions (the supplier's), and invariants. Oath has types and laws, which are postconditions. It has no preconditions, so a partial oath cannot be promised at all:

```
oath head : List a -> a
  law \xs -> elem (head xs) xs      -- false for []
```

The generator will produce `[]`, the law will fail, and no keeper of `head` can ever be sworn. Today the only workaround is to weaken every law with a guard, which wastes enumeration and still lets a caller pass `[]` with no diagnostic.

## Proposal

```
oath head : List a -> a
  requires \xs -> length xs > 0
  law      \xs -> elem (head xs) xs
```

`requires` is part of the promise, so it is in the canonical form and the hash (item 0021). It does three things:

- **Filters generation.** Laws and examples run only on inputs satisfying every `requires`. Enumeration skips the rest, and swearing reports how many inputs were skipped so a `requires` that excludes everything is visible rather than silently vacuous.
- **Blames the caller.** At runtime, a call whose arguments fail a `requires` is a contract error against the caller, alongside the argument type check (item 0096). A keeper never runs on input it was not promised.
- **Bounds a keeper's obligation.** A keeper is sworn against the oath's laws only where `requires` holds, which is what makes partial oaths keepable.

## Acceptance criteria

- [ ] `head` with the precondition above swears, and its keeper is never run on `[]` during swearing
- [ ] calling `head []` at runtime is a contract error blaming the caller, naming the failing `requires` and its span
- [ ] `oath swear` prints `head: 12 inputs, 5 skipped by requires` at depth 4
- [ ] a `requires` no generated input satisfies is reported as `requires excludes every input at depth N`, and the keeper is unsworn rather than vacuously sworn
- [ ] adding or changing a `requires` changes the oath's hash

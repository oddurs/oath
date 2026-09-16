---
id: 32
title: Parse effect declarations with state cells, external ops, try
type: feature
status: backlog
milestone: v0.2
depends_on:
- 11
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Problem

Effects need syntax for state, operations, compensation for the rare external op, and the places they are handled.

## Proposal

```
effect Counter
  state count : Int = 0
  incr : Int -> Unit
    do n -> count := count + n

effect Disk
  external write : Str -> Str -> Unit
    do   path s -> builtin.write path s
    undo path s -> builtin.remove path

effect Console
  irreversible print : Str -> Unit
```
State cells are the only mutable thing and need no `undo`; the journal derives it. An `external` op must declare `undo`, a compensation. An `irreversible` op declares neither and may not be performed inside `try` or during swearing. `perform Counter.incr 3`, `handle e with { Counter.incr n -> ... }`, `try e else e`, `fail`.

## Acceptance criteria

- [ ] a plain op with an `undo` clause is a parse error with the hint "undo is derived for state; mark the op external"
- [ ] an `external` op without `undo` is a parse error
- [ ] `try` without `else` is a parse error with a hint

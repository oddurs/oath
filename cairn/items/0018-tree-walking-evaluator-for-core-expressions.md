---
id: 37875bb2-b37c-4093-9ed7-3d0e723277e8
title: Tree-walking evaluator for core expressions
type: feature
status: planned
milestone: v0.1
depends_on:
- 45551b47-c978-4bd3-bc2c-a262d9451c24
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: l
area: eval
---

## Problem

Nothing runs yet.

## Proposal

Environment-passing evaluator over the typed AST. Values: Int (i64, checked arithmetic), Bool, List, Tuple, Closure. Builtins: arithmetic, comparison, `print`. Top-level `main` is evaluated by `oath run`; its value is printed if it is not Unit.

## Acceptance criteria

- [ ] fib 25 evaluates correctly in the golden suite
- [ ] deep recursion (10k frames) does not overflow the Rust stack; use an explicit stack or a large thread stack, documented
- [ ] `print` writes to stdout with a newline and returns Unit

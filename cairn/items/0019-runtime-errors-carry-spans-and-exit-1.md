---
id: 19
title: Runtime errors carry spans and exit 1
type: feature
status: planned
milestone: v0.1
depends_on:
- 14
- 18
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: s
area: eval
---

## Problem

Division by zero and a non-exhaustive match must be errors with positions, never panics.

## Proposal

Evaluator returns `Result<Value, Diagnostic>`; every failure point knows its span from the AST node.

## Acceptance criteria

- [ ] golden tests for division by zero, integer overflow, and a match with no matching arm
- [ ] exit code 1 and the caret under the failing expression

---
id: 686d01b4-97bf-46b6-99b3-af41cdd090f4
title: Runtime errors carry spans and exit 1
type: feature
status: planned
milestone: v0.1
depends_on:
- 1c5423ff-b3cb-4bc2-b16c-188246cad23f
- 37875bb2-b37c-4093-9ed7-3d0e723277e8
created: 2026-09-16
updated: 2026-09-27
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

## 2026-09-27

Rendering is done: Diagnostic::render (0014) prints file:line:col, the source line and a caret, and cannot panic on any span. What is left here is giving every evaluator failure point a real span and mapping a runtime error to exit 1 against a source error's exit 2, which --help already documents and a test already pins.

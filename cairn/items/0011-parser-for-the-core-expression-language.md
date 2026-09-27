---
id: 45551b47-c978-4bd3-bc2c-a262d9451c24
title: Parser for the core expression language
type: feature
status: planned
milestone: v0.1
depends_on:
- 7ac5641a-b839-4707-a40a-58f3744eb3b6
created: 2026-09-16
updated: 2026-09-27
priority: p0
effort: l
area: syntax
---

## Problem

The MVP needs enough expression language to write sort, fib, and their keepers.

## Proposal

Pratt parser producing a spanned AST. Expressions: literals (Int, Bool, `[]`, list literal), variables, `let x = e in e`, `fn x -> e`, application, infix arithmetic and comparison, `if`, `match e with | pat -> e`, tuples. Patterns: literal, variable, wildcard, `x:xs`, `[]`, tuple.

## Acceptance criteria

- [ ] precedence table documented in a comment and covered by tests (`1 + 2 * 3`, `f x y`, `x:xs`)
- [ ] parse errors carry the span of the offending token
- [ ] AST is `Debug`-printable and used by the type checker and evaluator without conversion

## 2026-09-27

Parse errors render through Diagnostic::render (0014), which already prints file:line:col with a caret and a help line. A parse error needs no new rendering, only a Diagnostic with the right span; caret placement is covered by 0014's snapshots.

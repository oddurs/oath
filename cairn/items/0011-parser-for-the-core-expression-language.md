---
id: 11
title: Parser for the core expression language
type: feature
status: planned
milestone: v0.1
depends_on:
- 10
created: 2026-09-16
updated: 2026-09-16
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

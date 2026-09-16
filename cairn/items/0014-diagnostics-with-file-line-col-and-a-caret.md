---
id: 14
title: Diagnostics with file:line:col and a caret
type: feature
status: planned
milestone: v0.1
depends_on:
- 10
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Problem

A stranger's first experience of Oath will be an error message.

## Proposal

One `Diagnostic { span, message, hint: Option<String> }` type shared by lexer, parser, checker and evaluator. Renderer prints `file:line:col: message`, the source line, and a caret under the span. Parse and type errors exit 2; runtime errors exit 1.

## Acceptance criteria

- [ ] golden test for a parse error, a type error and a runtime error, each with the caret on the right column
- [ ] no `panic!`/`unwrap` on user input anywhere in the binary (grep in CI)
- [ ] exit codes documented in `oath --help`

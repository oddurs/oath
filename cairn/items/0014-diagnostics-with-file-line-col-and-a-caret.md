---
id: 1c5423ff-b3cb-4bc2-b16c-188246cad23f
title: Diagnostics with file:line:col and a caret
type: feature
status: planned
milestone: v0.1
depends_on:
- 7ac5641a-b839-4707-a40a-58f3744eb3b6
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Problem

A stranger's first experience of Oath will be an error message.

## Proposal

One `Diagnostic { span, message, hint: Option<String> }` type shared by every stage. The renderer prints `file:line:col: message`, the source line, and a caret under the span:

```
sort.oath:2:14: unterminated string at the end of the line
  |
2 | keep sort by "fast
  |              ^^^^^
  = help: add a closing `"`
```

Source errors exit 2; runtime errors exit 1.

## Acceptance criteria

- [ ] the caret lands on the right column, with snapshot tests covering a plain line, a wide span, a tab-indented line, a multibyte line, an empty span, an offset past the last newline, a span crossing lines, and a two-digit line number
- [ ] rendering cannot panic whatever span it is handed, including one that splits a character, proved by rendering every offset pair over a set of sources
- [ ] a real lexer error renders end to end through the same path
- [ ] `unwrap`, `panic!`, `todo!`, `unimplemented!` and `dbg!` are refused outside test modules, enforced by `scripts/task lint` and therefore by CI
- [ ] exit codes are documented in `oath --help` and pinned by a test

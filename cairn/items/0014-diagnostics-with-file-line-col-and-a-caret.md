---
id: 1c5423ff-b3cb-4bc2-b16c-188246cad23f
title: Diagnostics with file:line:col and a caret
type: feature
status: done
milestone: v0.1
depends_on:
- 7ac5641a-b839-4707-a40a-58f3744eb3b6
created: 2026-09-16
updated: 2026-09-27
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

- [x] the caret lands on the right column, with snapshot tests covering a plain line, a wide span, a tab-indented line, a multibyte line, an empty span, an offset past the last newline, a span crossing lines, and a two-digit line number
- [x] rendering cannot panic whatever span it is handed, including one that splits a character, proved by rendering every offset pair over a set of sources
- [x] a real lexer error renders end to end through the same path
- [x] `unwrap`, `panic!`, `todo!`, `unimplemented!` and `dbg!` are refused outside test modules, enforced by `scripts/task lint` and therefore by CI
- [x] exit codes are documented in `oath --help` and pinned by a test

## 2026-09-27

The original first criterion asked for golden tests of a parse error, a type error and a runtime error. None was possible: there is no parser yet (45551b47), no evaluator, and no static type checker at all now that typing is deferred to 0114 in favour of a runtime contract at the oath boundary. It was rewritten to what this item can actually prove, and the parser and evaluator items carry notes to render through this path.

Deviation on enforcement: the ticket asked for a grep in CI. Clippy restriction lints do it better, because they read code rather than text, so unwrap_used, panic, todo, unimplemented and dbg_macro are denied in Cargo.toml and allowed back inside test modules. Verified by adding an unwrap to library code and watching lint fail. `expect` is deliberately left allowed: with a message it states an invariant, which is the sanctioned way to say why a case cannot happen.

The panic-proofing criterion earned its place immediately. Rendering every offset pair over a set of sources found a real bug: slicing the source by a span landing inside a multibyte character panicked, in the one code path that is already the error path. Offsets are now snapped back to a character boundary.

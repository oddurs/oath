---
id: 7ac5641a-b839-4707-a40a-58f3744eb3b6
title: Lexer with source spans
type: feature
status: done
milestone: v0.1
depends_on:
- 1e5cca1f-d115-4609-bbe6-ea9f72440be0
created: 2026-09-16
updated: 2026-09-27
priority: p0
effort: m
area: syntax
---

## Problem

Every diagnostic needs a position; the parser needs tokens.

## Proposal

Hand-written lexer producing tokens with byte spans. Tokens: identifiers, integer literals, keywords (`oath eg keep by let in if then else match with fn`), operators, brackets, `->`, `=>`, `:`, `,`, `|`, `--` line comments. Layout is not significant; statements are separated by newlines or `;`.

## Acceptance criteria

- [x] unit tests cover every token kind and comments
- [x] an illegal character produces an error with its span, not a panic
- [x] spans round-trip: slicing the source by a token's span yields its text

## 2026-09-27

Three deviations from the proposal, each deliberate:

Omitted `=>`. The ticket listed it, but no syntax in the design uses it: functions and match arms both use `->`, and so do effect handlers. A token nothing can parse is scaffolding, so it arrives if a syntax ever needs it.

Added string literals, which the ticket did not list. v0.1 needs them: a keeper label is quoted, as in `keep sort by "reference"` (0013). Four escapes resolve (\" \\ \n \t) because you cannot know where a quoted string ends without handling \" at minimum. The Str type and its operations remain 0028's work.

Introduced Span and Diagnostic in a new `diagnostic` module, since the lexer cannot report an illegal character without them. Rendering to file:line:col with a caret stays 0014's job, which is why it depends on this.

One behaviour worth knowing: `1--2` lexes as `1` followed by a comment, because `--` always opens one. Every language with `--` comments has this; a special case in the scanner would surprise the reader more than the rule does. Covered by a test that says so.

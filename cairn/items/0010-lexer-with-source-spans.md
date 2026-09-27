---
id: 7ac5641a-b839-4707-a40a-58f3744eb3b6
title: Lexer with source spans
type: feature
status: planned
milestone: v0.1
depends_on:
- 1e5cca1f-d115-4609-bbe6-ea9f72440be0
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Problem

Every diagnostic needs a position; the parser needs tokens.

## Proposal

Hand-written lexer producing tokens with byte spans. Tokens: identifiers, integer literals, keywords (`oath eg keep by let in if then else match with fn`), operators, brackets, `->`, `=>`, `:`, `,`, `|`, `--` line comments. Layout is not significant; statements are separated by newlines or `;`.

## Acceptance criteria

- [ ] unit tests cover every token kind and comments
- [ ] an illegal character produces an error with its span, not a panic
- [ ] spans round-trip: slicing the source by a token's span yields its text

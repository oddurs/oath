---
id: 7f9eae9c-21d2-4660-b008-f807c1a79f04
title: Parse hole keepers
type: feature
status: backlog
milestone: v0.3
depends_on:
- d9310c65-7ebb-40db-8ed0-41a5c7ecb153
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: syntax
---

## Problem

A hole in Oath is an oath with no keeper. There are no expression-level holes on the v1.0 path; that needs a type checker and lives in `later`.

## Proposal

`keep sort by "fast" ?` declares a keeper whose whole body is a hole, so the label and producer can exist before the body does. An oath with zero keepers is already legal since v0.1.

## Acceptance criteria

- [ ] a hole keeper is recorded in the AST distinctly from a missing keeper
- [ ] `?` anywhere else is a parse error with the hint "holes are keepers; see docs/holes.md"

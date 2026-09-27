---
id: 2d1cc97f-2183-4421-abbb-7a1bfce2ab0b
title: Structural checks for effects and laws
type: feature
status: backlog
milestone: v0.2
depends_on:
- 437b3e57-8a55-42f9-a262-58c58155f48d
- 0d5c6a89-6668-48d4-a6ed-4ee0671c7d31
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Problem

Without a type checker, the shape errors that would otherwise be silent until swearing need a pass of their own.

## Proposal

A resolve pass after parsing: each `law` is a lambda whose arity equals the oath's; `do` and `undo` of one op have the same arity; a `handle` block names only ops of the effect it handles; `perform` names a declared op. Errors are diagnostics with spans and exit 2.

## Acceptance criteria

- [ ] golden tests for each of the four errors
- [ ] the pass is the single place that resolves op and oath names; the evaluator never looks names up by string

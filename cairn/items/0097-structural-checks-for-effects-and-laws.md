---
id: 97
title: Structural checks for effects and laws
type: feature
status: backlog
milestone: v0.2
depends_on:
- 27
- 32
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

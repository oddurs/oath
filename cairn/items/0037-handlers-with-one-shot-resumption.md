---
id: 37
title: Handlers with one-shot resumption
type: feature
status: backlog
milestone: v0.2
depends_on:
- 35
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: l
area: effects
---

## Problem

An effect's meaning is given by its handler, not by the op.

## Proposal

`handle e with { Op args -> body }` where `body` may call `resume v` at most once. The default handler for an effect is its `do` clause. Resuming after a rewind is a runtime error.

## Acceptance criteria

- [ ] a handler that counts ops without performing them (mocking) in the golden suite
- [ ] resuming twice is a runtime error with the handler's span

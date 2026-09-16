---
id: 54
title: 'Keeper source: pause into a REPL'
type: feature
status: backlog
milestone: v0.3
depends_on:
- 51
- 53
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: l
area: holes
---

## Problem

The first keeper source is the programmer.

## Proposal

Default `Keep` handler when stdin is a TTY: print the oath, read a body until a blank line, swear it, and either resume or re-prompt with the failure. `:skip` fails the enclosing `try` or aborts.

## Acceptance criteria

- [ ] golden test with scripted stdin fills a hole and continues
- [ ] the accepted keeper is written to the store with `producer = repl`

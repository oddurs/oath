---
id: bb9fda78-12fe-4dfb-8d38-9a509b8b0f05
title: 'Keeper source: pause into a REPL'
type: feature
status: backlog
milestone: v0.3
depends_on:
- ca1c532d-8707-4a3c-ad89-bddcd1dcdd2f
- f84f6083-ba76-4381-bd94-ad1db39522af
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

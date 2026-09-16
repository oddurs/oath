---
id: 55
title: 'Keeper source: search the store by oath'
type: feature
status: backlog
milestone: v0.3
depends_on:
- 41
- 53
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: holes
---

## Problem

Someone may already have sworn a keeper for this promise under another name.

## Proposal

`--keep-with library`: find store entries whose oath has the same signature, swear each candidate against this oath's examples and laws, adopt the first that passes.

## Acceptance criteria

- [ ] golden test: `sort2`, a renamed copy of `sort`, is kept by `sort`'s keeper without writing a body

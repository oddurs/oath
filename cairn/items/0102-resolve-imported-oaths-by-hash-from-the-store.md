---
id: f61ea3f6-fc28-47ba-8439-d12a67494c5e
title: Resolve imported oaths by hash from the store
type: feature
status: backlog
milestone: v0.4
depends_on:
- 35a4f75b-4dea-4681-b3c1-927cdd162ff4
- 82e2c71c-8669-4079-978b-f0e97a90664e
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: l
area: oath
---

## Problem

This is where content addressing stops being a printout and becomes linkage.

## Proposal

`use x = oath:<hash>` binds `x` to the oath with that hash in the store, including its signature, laws, and examples; its sworn keepers are whatever the store holds. Calls through `x` dispatch like any local oath. A hash not in the store is a resolve error that says where the store was looked for.

## Acceptance criteria

- [ ] golden test: file B imports A's `sort` by hash and never mentions the name `sort`
- [ ] deleting A's source after swearing does not break B (the store is the source of truth)
- [ ] `oath swear B.oath` reports imported oaths as `imported` with their evidence

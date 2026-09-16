---
id: 105
title: Imported oath with no sworn keeper fires Keep
type: feature
status: backlog
milestone: v0.4
depends_on:
- 53
- 102
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: holes
---

## Problem

Someone can share a promise without sharing a body. That is the intended way to hand work to a colleague or a model.

## Proposal

An imported oath whose store entry has no sworn keeper is an unkept oath; calling it performs `Keep` exactly as a local one does. All three keeper sources work unchanged.

## Acceptance criteria

- [ ] golden test: B imports a hash whose only keeper is unsworn, runs with `--keep-with` the canned script, and the new keeper is stored under the imported hash

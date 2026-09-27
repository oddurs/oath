---
id: dbf78240-015d-4fa1-864b-64a73b29eae0
title: Imported oath with no sworn keeper fires Keep
type: feature
status: backlog
milestone: v0.4
depends_on:
- f84f6083-ba76-4381-bd94-ad1db39522af
- f61ea3f6-fc28-47ba-8439-d12a67494c5e
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

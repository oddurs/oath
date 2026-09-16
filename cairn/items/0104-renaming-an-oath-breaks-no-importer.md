---
id: 104
title: Renaming an oath breaks no importer
type: feature
status: backlog
milestone: v0.4
depends_on:
- 102
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: oath
---

## Problem

The promise of content addressing is that names are labels.

## Acceptance criteria

- [ ] golden test: rename `sort` to `order` in A; B, which imports by hash, runs unchanged with no re-swearing
- [ ] `oath who` shows both names for the hash, newest first

## 2026-09-16

Leading option (round 3): the store is committed with the project, so a path import's pinned hash lives in the store record and travels with the repo.

---
id: 35a4f75b-4dea-4681-b3c1-927cdd162ff4
title: Keeper store on disk with sworn records
type: feature
status: planned
milestone: v0.1
depends_on:
- 077ae825-1ca6-45c7-af09-fb4c147bad02
- 73ab165c-be8f-461f-ac76-e03220dc8218
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: l
area: oath
---

## Problem

Swearing is evidence, and evidence should persist.

## Proposal

`.oath/store/<oath-hash>/<keeper-hash>.json`: `{ label, producer, sworn_at, checks: { examples: n, laws: n, depth }, timing_ns }` plus the canonical body. `oath swear` reads the store and skips keepers already sworn at ≥ the requested depth unless `--force`.

## Acceptance criteria

- [ ] second `oath swear` on an unchanged file does no evaluation and says `cached`
- [ ] the store is plain files and survives `git clean -X` documentation in README
- [ ] a corrupt record is reported with its path, not a panic
- [ ] the sworn record lists dependency keeper hashes used during swearing

## 2026-09-16

Decision (round 3): `.oath/store` is meant to be committed. It is the evidence and works like a lockfile for a team. README says so; there is no gitignore entry.

## 2026-09-16

Added criterion (from Unison's dependency-graph caching, docs/research/01): a sworn record lists the keeper hashes of every dependency oath that ran during swearing, so oath who can say which insert the evidence for sort was gathered against.

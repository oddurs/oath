---
id: 0ef2a5d2-b1b1-4048-bd83-824b8b44f24d
title: 'oath pack / unpack: hand a store bundle to someone'
type: feature
status: backlog
milestone: v0.4
depends_on:
- f61ea3f6-fc28-47ba-8439-d12a67494c5e
created: 2026-09-16
updated: 2026-09-16
priority: p2
effort: m
area: oath
---

## Proposal

`oath pack <hash>... > bundle.oathpack` writes the oaths, their keepers and sworn records. `oath unpack` imports them with `sworn` downgraded to `claimed` until re-sworn locally, unless `--trust <producer>`.

## Acceptance criteria

- [ ] round-trip test
- [ ] unpacked keepers are not eligible for dispatch until sworn locally
- [ ] `oath pack sort` includes every oath that sort's hash depends on, and unpack refuses a bundle with a missing dependency, naming the hash

## 2026-09-16

Added criterion (from Unison's sync of missing hashes, docs/research/01): a bundle carries the dependency closure.

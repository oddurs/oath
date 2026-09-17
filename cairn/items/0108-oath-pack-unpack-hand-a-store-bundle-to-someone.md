---
id: 108
title: 'oath pack / unpack: hand a store bundle to someone'
type: feature
status: backlog
milestone: v0.4
depends_on:
- 102
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

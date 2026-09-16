---
id: 107
title: Global store with project overlay
type: feature
status: backlog
milestone: v0.4
depends_on:
- 100
- 102
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: oath
---

## Proposal

Per the spike. Provisionally: `~/.oath/store` is read for imports; `.oath/store` in the project is written to and read first. `OATH_STORE` overrides. `oath store path` prints the resolution.

## Acceptance criteria

- [ ] a hash present only in the global store resolves
- [ ] the same hash in both stores prefers the project's record and says so in `oath who`

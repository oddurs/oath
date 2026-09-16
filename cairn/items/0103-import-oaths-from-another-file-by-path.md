---
id: 103
title: Import oaths from another file by path
type: feature
status: backlog
milestone: v0.4
depends_on:
- 101
- 102
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Proposal

`use ./lib.oath` parses, swears and stores every oath in that file, then binds each by name. Per the spike, the resolved hashes are recorded so a later change to `lib.oath` is reported as an oath change, not silently followed.

## Acceptance criteria

- [ ] golden test: editing `lib.oath`'s examples makes `oath swear main.oath` print the un-sworn message for the importer too
- [ ] import cycles are a resolve error listing the cycle

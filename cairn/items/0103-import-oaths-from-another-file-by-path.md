---
id: c333a4b2-1611-40cc-b6ac-65f1d2ccb7ae
title: Import oaths from another file by path
type: feature
status: backlog
milestone: v0.4
depends_on:
- 82e2c71c-8669-4079-978b-f0e97a90664e
- f61ea3f6-fc28-47ba-8439-d12a67494c5e
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

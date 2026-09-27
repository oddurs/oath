---
id: f0a8ea58-96f0-4286-b596-bc0a2ea56460
title: CHANGELOG is true, one entry per milestone
type: docs
status: backlog
milestone: v1.0
created: 2026-09-16
updated: 2026-09-26
priority: p0
effort: s
area: docs
---

## Problem

The licence and the contributing guide now land at publication, not here, because a public repository needs them on day one. What is left for v1.0 is the changelog actually being true.

## Proposal

`CHANGELOG.md` in Keep a Changelog form, with a real entry per milestone written as that milestone closed rather than reconstructed at the end. v1.0 is where it gets checked against the git history and the gaps filled.

## Acceptance criteria

- [ ] every milestone from v0.1 has an entry naming what a user can do that they could not before
- [ ] the entries were written as the milestones closed, not invented at the end
- [ ] no entry names an assistant

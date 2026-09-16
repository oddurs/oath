---
id: 51
title: Pretty-printer for oaths and keepers
type: feature
status: backlog
milestone: v0.3
depends_on:
- 21
- 27
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: syntax
---

## Problem

The command keeper source sends an oath to an external process as text; the pause source shows it to a human.

## Proposal

`oath show <name>` prints the canonical text form: signature, laws, examples, and the list of sworn keepers by label. Round-trips through the parser.

## Acceptance criteria

- [ ] property test: parse(print(x)) has the same hash as x for every oath in the suite

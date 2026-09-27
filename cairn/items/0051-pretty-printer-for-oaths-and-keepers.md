---
id: ca1c532d-8707-4a3c-ad89-bddcd1dcdd2f
title: Pretty-printer for oaths and keepers
type: feature
status: backlog
milestone: v0.3
depends_on:
- 5d43d6fa-435e-436f-b8b9-262f3891e383
- 437b3e57-8a55-42f9-a262-58c58155f48d
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
- [ ] `oath show sort` prints the declaration of any user type in its signature and the signature line of `sorted` and `length`

## 2026-09-16

Added criterion (from the typed-holes LLM finding that type definitions in context mattered most, docs/research/05): the printed form includes, transitively, the user type declarations the signature mentions and the signatures of the oaths its laws and examples reference.

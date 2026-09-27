---
id: d51033b2-f681-491e-a99c-5ff3acebc8da
title: Swear effectful keepers under rollback
type: feature
status: backlog
milestone: v0.2
depends_on:
- a3a03c1d-43b6-4883-807f-2853d3d4cc24
- 3cb4f026-cd20-453e-8663-54a249fc36fb
- e29e8d64-cba1-44f8-bd19-168b0cc88bb9
- 5037e1bb-34f0-4bb9-9ae8-5bd9871d7e0f
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

Most useful code performs effects. Swearing it must not change the world.

## Proposal

Each example and law run is wrapped in an implicit `try`; the journal is rewound after each. The sworn record lists the effects performed.

## Acceptance criteria

- [ ] golden test: swearing a keeper that increments a counter leaves the counter at 0 afterwards
- [ ] `oath who` shows `effects: [Counter.incr]` for that keeper

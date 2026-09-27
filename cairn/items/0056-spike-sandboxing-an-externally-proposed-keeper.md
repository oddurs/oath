---
id: c35dc1a0-54d3-461a-93c6-af1faacb740e
title: 'Spike: sandboxing an externally proposed keeper'
type: spike
status: backlog
milestone: v0.3
depends_on:
- d51033b2-f681-491e-a99c-5ff3acebc8da
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: holes
---

## Question

A keeper proposed by an external command runs during swearing. Rollback already prevents lasting effects from reversible ops, and irreversible ops are refused during swearing. What remains: runaway evaluation (timeout, step limit), memory, and whether the proposed body may itself contain holes.

## Timebox

Half a day.

## Options considered

- step budget + wall-clock timeout per example, no holes in proposals
- allow nested holes, recursive Keep with a depth limit

## Answer

_(fill in before closing; a spike without an answer was wasted)_

## Spawns

-

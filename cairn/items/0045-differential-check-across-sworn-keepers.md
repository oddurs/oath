---
id: 45
title: Differential check across sworn keepers
type: feature
status: backlog
milestone: v0.3
depends_on:
- 39
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: oath
---

## Problem

Two sworn keepers that disagree on an input the laws do not cover means the oath is underspecified.

## Proposal

During swearing, if an oath has ≥2 sworn keepers, run them on the generated inputs and report the first divergence as a warning with the input and both outputs.

## Acceptance criteria

- [ ] golden test with a stable-sort and unstable-sort pair of keepers on pairs shows the divergence
- [ ] divergence is a warning, exit code unchanged

---
id: 3dceef27-319e-4ba2-be50-57978efedcda
title: Random generation with shrinking, beyond small scope
type: feature
status: backlog
milestone: later
created: 2026-09-16
updated: 2026-09-16
priority: p2
effort: l
area: oath
---

## Why it is here and not on the v1.0 path

Small-scope enumeration gives minimal counterexamples for free and makes a sworn record deterministic: the same depth always yields the same verdict, which is what lets evidence be cached by hash. SmallCheck's own documentation concedes that random generators with shrinking now offer a better experience for inputs that must be large.

The honest limit is recorded rather than hidden: swearing stores the depth reached and `oath who` shows it, so nobody mistakes depth 4 for proof.

## Shape when it comes

A second evidence kind in the sworn record (`random: seed, count`) rather than a replacement for depth, because a seed and a count are reproducible and a wall-clock budget is not. Shrinking is then required, since a random counterexample is not minimal.

---
id: 91
title: Parallel evaluation of pure keepers
type: feature
status: backlog
milestone: later
created: 2026-09-16
updated: 2026-09-16
priority: p2
effort: xl
area: eval
---

Pure keepers are candidates for an interaction-net style runtime; see HVM/Bend.

## 2026-09-16

Risk (docs/research/04): parallel evaluation reintroduces the saga isolation problem for the journal; two concurrent try blocks can observe each other's un-rewound writes. Needs a design before any of this ships.

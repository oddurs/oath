---
id: 77
title: Versioned store format with migration
type: feature
status: backlog
milestone: v1.0
depends_on:
- 41
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Acceptance criteria

- [ ] records carry `format`; an unknown format is refused with the path
- [ ] `oath store migrate` upgrades v0 records; tested with a fixture store

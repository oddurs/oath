---
id: 5182643a-7c06-44ba-a167-77bfe0f62c7a
title: Fuzz parser and canonicaliser
type: chore
status: backlog
milestone: v1.0
depends_on:
- ca1c532d-8707-4a3c-ad89-bddcd1dcdd2f
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: infra
---

## Acceptance criteria

- [ ] cargo-fuzz targets for parse and for parse∘print∘hash idempotence
- [ ] ran 1 hour each with no crash before tagging; crashes found are filed as bugs with the input

---
id: 3a57635b-c4b1-40a0-87ca-80437b400c59
title: Freeze the CLI and remove experimental flags
type: chore
status: backlog
milestone: v1.0
depends_on:
- 77339e8e-2cc0-4970-963f-5688387173b4
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: cli
---

## Acceptance criteria

- [ ] every flag in `--help` is in the reference; anything not is removed
- [ ] `oath --help` golden test

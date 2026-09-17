---
id: 79
title: Error message audit
type: chore
status: backlog
milestone: v1.0
depends_on:
- 75
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: cli
---

## Acceptance criteria

- [ ] every diagnostic in the golden suite reviewed: says what, where, and one thing to try
- [ ] every error involving an oath or keeper prints its hash

## 2026-09-16

From Racket's warning that nested contract boundaries blame unintuitively (docs/research/02): Oath's mitigation is that there is exactly one kind of boundary and every contract error names the oath, the keeper label, and both hashes. Verify that during the audit.

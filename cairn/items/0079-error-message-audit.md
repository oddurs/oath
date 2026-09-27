---
id: 77339e8e-2cc0-4970-963f-5688387173b4
title: Error message audit
type: chore
status: backlog
milestone: v1.0
depends_on:
- 258b99b1-54df-427e-a389-6a11af504ad6
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

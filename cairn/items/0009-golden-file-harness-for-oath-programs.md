---
id: 7523f3ee-4360-4adf-bcf1-8c79cf9e2a37
title: Golden-file harness for .oath programs
type: chore
status: planned
milestone: v0.1
depends_on:
- 1e5cca1f-d115-4609-bbe6-ea9f72440be0
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: infra
---

## Problem

Every language feature needs an end-to-end test that is trivial to add.

## Proposal

`tests/cases/<name>.oath` with sibling `<name>.stdout` and optional `<name>.exit`. A Rust test iterates the directory, runs the binary with `run` and `swear` as the case's first line directs, and diffs. `UPDATE_GOLDEN=1` rewrites expectations.

## Acceptance criteria

- [ ] adding a new case is one `.oath` file and one `.stdout` file, no Rust
- [ ] a mismatch prints a unified diff
- [ ] the harness runs under `scripts/task test`

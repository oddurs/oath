---
id: 7
title: Single crate oath-lang, binary oath, scripts/task seam
type: chore
status: planned
milestone: v0.1
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: infra
---

## Problem

There is no code. Every later item needs a place to land and one command to check it.

## Proposal

One crate `oath-lang` (the name `oath` is taken on crates.io) with binary `oath`, modules named for the areas in cairn.toml: `syntax`, `eval`, `oath`, `effects`, `holes`, `cli`, `stdlib`. No workspace; a split earns its place when compile times demand it. `scripts/task` with verbs `fmt fmt:check lint test build check`; `lint` is clippy with warnings denied. `scripts/agent` for the worktree workflow. Git hooks call `scripts/task check`.

## Acceptance criteria

- [ ] `scripts/task check` runs fmt:check, lint, test, build and exits 0 on the empty crate
- [ ] `cargo run -- --version` prints a version
- [ ] pre-commit hook refuses a commit when `scripts/task check` fails

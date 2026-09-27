---
id: 4950905e-b874-422e-b613-61ab15297f2e
title: 'oath binary: run, swear, hash, --help'
type: feature
status: planned
milestone: v0.1
depends_on:
- 1c5423ff-b3cb-4bc2-b16c-188246cad23f
- 37875bb2-b37c-4093-9ed7-3d0e723277e8
- 077ae825-1ca6-45c7-af09-fb4c147bad02
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: cli
---

## Problem

A stranger needs one binary with obvious verbs.

## Proposal

`oath run <file>`, `oath swear <file>`, `oath hash <file>`, `oath --version`, `oath --help`. clap-derived. Unknown subcommand exits 2 with usage.

## Acceptance criteria

- [ ] `--help` for every subcommand lists exit codes
- [ ] `oath run` on a missing file exits 2 with the path in the message, covered by a test
- [ ] `oath` with no arguments prints usage and exits 2

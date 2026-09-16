---
id: 24
title: 'oath binary: run, swear, hash, --help'
type: feature
status: planned
milestone: v0.1
depends_on:
- 14
- 18
- 22
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

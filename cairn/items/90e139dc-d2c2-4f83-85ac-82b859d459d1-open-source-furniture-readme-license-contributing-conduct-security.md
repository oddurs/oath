---
id: 90e139dc-d2c2-4f83-85ac-82b859d459d1
title: 'Open-source furniture: README, LICENSE, CONTRIBUTING, conduct, security'
type: docs
status: done
milestone: v0.1
created: 2026-09-26
updated: 2026-09-26
priority: p0
effort: m
area: docs
---

## Problem

A public repository without these is hostile: nobody can tell what the project is, whether they may use it, how to contribute, or how to report a vulnerability. They have to exist at publication, not at v1.0.

## Proposal

- `README.md` — what Oath is in two sentences, the one idea (contract-addressed code), install from source, the state of it today, a Development section pointing at `scripts/agent`, and CI and licence badges. It must describe what runs today, not the roadmap.
- `LICENSE` — MIT, 2026, Oddur Sigurdsson.
- `CONTRIBUTING.md` — the branch, worktree and pull request workflow, Conventional Commits, `scripts/task check`, and the fact that required approvals are 0 because this is a solo repository.
- `CODE_OF_CONDUCT.md` — Contributor Covenant 2.1, reporting through the owner's GitHub profile rather than a published email address.
- `SECURITY.md` — private reporting through GitHub Security Advisories, and what response to expect.

## Acceptance criteria

- [x] every file exists with real content and no placeholder left in it
- [x] the README's install and run commands are copy-pasteable and were actually run
- [x] the README says plainly that the language does not yet execute a program
- [x] no email address is published anywhere in them

---
id: 6da39fe2-47e0-4325-9127-5c362b8b8cef
title: Repository templates and hygiene files
type: chore
status: planned
milestone: v0.1
created: 2026-09-26
updated: 2026-09-26
priority: p1
effort: s
area: infra
---

## Problem

Issues and pull requests arrive shapeless without a template, and editors disagree about whitespace without a declared configuration.

## Proposal

- `.github/PULL_REQUEST_TEMPLATE.md` — problem, approach, what a reviewer should read sceptically, and a checklist tied to `scripts/task check`.
- `.github/ISSUE_TEMPLATE/bug_report.yml` and `feature_request.yml` as YAML forms, plus `config.yml` disabling blank issues.
- `.github/CODEOWNERS` — `* @oddurs`.
- `.github/dependabot.yml` — `github-actions` and `cargo`, weekly, minor and patch grouped into one pull request.
- `.editorconfig` matching rustfmt: 4 spaces for Rust, tabs for shell, LF, final newline.
- `.gitattributes` gains `* text=auto eol=lf`, keeping the cairn merge driver lines.

## Acceptance criteria

- [ ] opening a pull request pre-fills the template
- [ ] the issue forms render as forms on GitHub, and blank issues are refused
- [x] `.editorconfig` does not contradict `cargo fmt` on any file in the repository

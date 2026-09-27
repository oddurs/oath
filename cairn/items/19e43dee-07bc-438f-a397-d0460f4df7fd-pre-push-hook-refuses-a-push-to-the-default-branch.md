---
id: 19e43dee-07bc-438f-a397-d0460f4df7fd
title: pre-push hook refuses a push to the default branch
type: chore
status: done
milestone: v0.1
created: 2026-09-26
updated: 2026-09-26
priority: p0
effort: s
area: infra
---

## Problem

Branch protection stops a direct push to the default branch at the server, which is the real gate, but it stops it after the network round trip and with a message about refs. Catching it locally is faster and clearer.

## Proposal

`.githooks/pre-push` reads the ref lines on stdin, refuses any push whose remote ref is the default branch (resolved from the remote, not hardcoded), and otherwise runs `scripts/task check`. The pre-commit hook then drops to `fmt:check` and `lint` so committing stays fast and the full suite runs once, before the push.

## Acceptance criteria

- [x] a push to the default branch is refused locally with a message naming the branch
- [x] a push to a feature branch runs the full check and succeeds
- [x] committing no longer runs the test suite, and the timing difference is noted in the pull request

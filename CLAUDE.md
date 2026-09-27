# Oath

A language where a definition is a promise (an **oath**) identified by the hash of its
specification, and an implementation (a **keeper**) is *sworn* only once it demonstrably
keeps that promise. `docs/design.md` holds the decisions and why; `docs/research/` holds
the prior art behind each.

## Find work

The backlog is cairn, in `cairn/items/`. It is the plan; there is no other plan.

```sh
cairn next              # what is startable right now
cairn show <id>         # one item, with its acceptance criteria
```

Take the item, do exactly it, tick its criteria as you satisfy them. If the work turns
out to be two items, split it before it grows a shared history. `ROADMAP.md` is
generated — change items and run `cairn render`, never edit it by hand.

## Everything goes through the seam

```sh
./scripts/task check    # fmt:check, lint (warnings denied), test, build
```

CI and the git hooks call only these verbs. Anything worth checking belongs in
`scripts/task`, not in a workflow file, so the two can never disagree.

## One unit of work, one branch, one worktree, one pull request

```sh
./scripts/agent start feat/0010-lexer-with-spans
```

- **Never commit to the default branch.** It advances only through a merged pull
  request. The `pre-push` hook and branch protection both stop you.
- Branch `<type>/<slug>`, type one of `feat fix chore docs perf refactor test`, and the
  backlog id leads the slug.
- Worktrees live in `../.worktrees/oath/`. Two tasks never share a checkout; a second
  task is a second worktree, not a stash.
- Never `--no-verify`, never `|| true`. A hook that is wrong gets fixed.

## Commits

Conventional Commits. Imperative, subject at most 72 characters, no full stop. The body
says **why**; the diff already says what. A `Refs:` trailer names the item.

## Writing code here

Rust conventions live in `.claude/rules/` and load when you touch those files. Beyond
those:

- Make it work, make it right, make it small, in that order.
- Comments explain why. No dead scaffolding: no TODO stubs, no commented-out code, no
  abstraction for a second caller that does not exist yet. A module appears with the
  work that needs it.
- A dependency earns its place or is not added. Prefer the standard library.
- A bug fix arrives with the test that would have caught it.
- Diagnostics are the product's surface. Every error says what, where, and one thing to
  try, and anything involving an oath or a keeper prints its hash.

## Verify before claiming

Run the check and show what it printed. "Should work" is not a result. Say what you
skipped and why, and never report a step you did not watch pass.

## Attribution

No commit, comment, document, pull request or release note names an assistant, a model,
or an AI. The work is published under the owner's name. If a tool inserts attribution by
default, strip it before it reaches a git object or the GitHub API.

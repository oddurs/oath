# Contributing

## Once per checkout

```sh
./scripts/setup
```

This points git at `.githooks` and installs cairn's merge driver. Git does not clone
either, so a fresh checkout without this step has no hooks and finds out in CI.

## The one rule

**The default branch only ever advances through a merged pull request.** Branch
protection enforces it on the server; the `pre-push` hook catches it locally first.

Required approvals are **0**, because this is a solo repository and the owner would
otherwise be deadlocked. The status check is not optional: `required` must pass.

## One unit of work, one branch, one worktree, one pull request

```sh
./scripts/agent start feat/0010-lexer-with-spans
```

Branches are `<type>/<slug>` where type is one of `feat fix chore docs perf refactor
test`. When the work has a backlog item, its id leads the slug. Each branch gets its own
worktree under `../.worktrees/oath/`, so two pieces of work never share a checkout and
nothing needs stashing.

## Everything goes through the seam

```sh
./scripts/task fmt        # format in place
./scripts/task fmt:check  # fail on drift
./scripts/task lint       # clippy, warnings denied
./scripts/task test       # the suite
./scripts/task build
./scripts/task check      # all of the above
```

CI and the git hooks call only these verbs, so they cannot drift from what you run
locally. If something should be checked, it goes in the seam rather than in a workflow
file.

## Commits

Conventional Commits, imperative, subject at most 72 characters and no full stop. The
body explains *why*; the diff already says what. A `Refs:` trailer names the backlog
item.

```
feat(syntax): add the lexer with source spans

Every diagnostic needs a position, so tokens carry byte spans from the start
rather than being retrofitted later.

Refs: 0010
```

The `commit-msg` hook rejects anything that does not fit. Never `--no-verify`: a hook
that is wrong gets fixed, not bypassed.

## The backlog is the plan

Work is tracked in [cairn](https://github.com/oddurs/cairn) as Markdown files in
`cairn/items/`, reviewable in a pull request like anything else.

```sh
cairn next            # what is startable now
cairn show 10         # one item in full
cairn board
```

`ROADMAP.md` is generated. Change the items and re-render; never edit it by hand.

## Before you open a pull request

`./scripts/agent pr` runs the check, pushes, and fills the template. Say in the
description where the change is weak, so a reviewer does not have to find it.

## Attribution

Commits, comments, documentation and pull request descriptions do not name an assistant
or a model. The work is published under the author's name.

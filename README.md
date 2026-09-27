# Oath

A language where a definition is a **promise**, not a body. You write the promise once,
and any number of implementations compete to keep it.

[![ci](https://github.com/oddurs/oath/actions/workflows/ci.yml/badge.svg)](https://github.com/oddurs/oath/actions/workflows/ci.yml)
[![licence: MIT](https://img.shields.io/badge/licence-MIT-blue.svg)](LICENSE)

## The idea

In most languages a function is identified by its name, and in Unison it is identified
by the hash of its implementation. In Oath it is identified by the hash of its
*specification*: the type, the laws it must satisfy, the examples it must reproduce.
That is an **oath**. An implementation is a **keeper**, and a keeper becomes *sworn* when
it demonstrably keeps the oath.

```
oath sort : List Int -> List Int
  requires \xs -> length xs >= 0
  law      \xs -> sorted (sort xs)
  law      \xs -> length (sort xs) == length xs
  eg       sort [3,1,2] == [1,2,3]

keep sort by "reference"
  [] -> []
  (x:xs) -> insert x (sort xs)

keep sort by "fast" ?        -- a hole: an oath with no keeper yet
```

Call sites name the oath, never a body. Three things follow:

- **Renaming is free**, and changing a promise cannot silently keep its old evidence.
- **Several bodies can coexist** under one promise, so they can be checked against each
  other, and a disagreement means the promise is underspecified.
- **A missing body is not an error.** Reaching one raises an effect, and the program
  decides the policy: ask a person, search what is already sworn, or ask a model.

The design and the research behind each decision are in [docs/design.md](docs/design.md).

## State of it today

**The language does not run yet.** This is the beginning of the work, not a usable
interpreter. What exists today is the toolchain and the plan:

```
git clone https://github.com/oddurs/oath && cd oath
./scripts/setup                 # hooks and merge driver, once per checkout
./scripts/task check            # format, lint, test, build
cargo run -- --version          # oath 0.1.0
```

`oath run` and `oath swear` arrive with v0.1. What lands when is in
[ROADMAP.md](ROADMAP.md), which is generated from the backlog rather than written by
hand.

## Development

The backlog is the plan. `cairn next` says what is startable right now.

```
./scripts/agent doctor          # is this checkout ready
./scripts/agent start feat/0010-lexer
./scripts/agent check           # scripts/task check
./scripts/agent pr
```

One unit of work is one branch in one worktree with one pull request, and the default
branch only ever advances through a merged one. [CONTRIBUTING.md](CONTRIBUTING.md) has
the detail.

## Licence

MIT. See [LICENSE](LICENSE).

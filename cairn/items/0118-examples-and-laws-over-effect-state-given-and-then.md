---
id: 118
title: 'Examples and laws over effect state: given and then'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 35
- 39
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: m
area: oath
---

## Problem

`eg transfer 5 == Receipt` says nothing about the balances. Evidence for effectful oaths needs a starting world and an assertion about the world afterwards.

## Proposal

```
oath transfer : Int -> Receipt
  eg  given Bank.balance := 10
      transfer 5 == Receipt
      then Bank.balance == 5
  law given Bank.balance := b
      \amt -> try (transfer amt; true) else Bank.balance == b
```
`given` writes cells before the run (inside the implicit try, so it is rewound too). `then` is a Bool over cells after the run. Both optional; both allowed on `eg` and `law`. Variables bound in `given` (like `b`) are generated like arguments.

## Acceptance criteria

- [ ] golden test: the transfer example swears; a keeper that forgets to debit fails the `then` with both values printed
- [ ] `given` on an oath with no effects is a check warning

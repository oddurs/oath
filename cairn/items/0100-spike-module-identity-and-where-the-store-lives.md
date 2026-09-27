---
id: b1f23c1d-b2bf-4bc6-bf19-08dc000fd28f
title: 'Spike: module identity and where the store lives'
type: spike
status: backlog
milestone: v0.4
depends_on:
- 35a4f75b-4dea-4681-b3c1-927cdd162ff4
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: s
area: oath
---

## Question

How does one file name an oath from another? By path (`use ./lib.oath`) is convenient; by hash (`use sort = oath:7f3a…`) is the point of the language. Both, with path imports resolved to hashes at swear time and recorded? And is the store project-local (`.oath/`), global (`~/.oath/`), or a global store with a project overlay?

## Timebox

One day; read Unison's "big idea" doc and its codebase-manager notes first.

## Options considered

- path imports only, hashes are informational
- hash imports only, paths never appear in source
- both; a path import pins to a hash in a lockfile-like record in the store

## Answer

_(fill in before closing; a spike without an answer was wasted)_

## Spawns

-

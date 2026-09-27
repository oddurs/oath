---
id: 03dcafbc-be06-42ed-9376-ed140e18c4d7
title: 'Keeper source: external command'
type: feature
status: backlog
milestone: v0.3
depends_on:
- ca1c532d-8707-4a3c-ad89-bddcd1dcdd2f
- f84f6083-ba76-4381-bd94-ad1db39522af
- c35dc1a0-54d3-461a-93c6-af1faacb740e
created: 2026-09-16
updated: 2026-09-16
priority: p0
effort: l
area: holes
---

## Problem

This is where a model plugs in, without the language knowing what a model is.

## Proposal

`--keep-with 'cmd ...'`: pipe `oath show` output to the command's stdin, read one keeper body from stdout, swear it under the sandbox limits. Adopt only if sworn; store with `producer = cmd:<argv[0]>`. On failure, retry up to `--keep-retries N` with the failure appended to stdin.

## Acceptance criteria

- [ ] `examples/keep-with-script.sh` (a shell script with canned bodies) fills the `sort` hole in the golden suite
- [ ] a script returning a body that fails a law leaves the store and the world untouched
- [ ] the retry prompt includes the counterexample

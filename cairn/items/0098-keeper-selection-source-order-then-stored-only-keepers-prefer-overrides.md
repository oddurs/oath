---
id: 98
title: 'Keeper selection: source order, then stored-only keepers; --prefer overrides'
type: feature
status: backlog
milestone: v0.2
depends_on:
- 23
- 41
created: 2026-09-16
updated: 2026-09-16
priority: p1
effort: m
area: eval
---

## Problem

With several sworn keepers, which one runs? "Most evidence" cannot decide, because every keeper of an oath is sworn at the same depth in one run. "Fastest" on depth-4 inputs would be a lie.

## Proposal

The order a person wrote keepers in the file is a deliberate ranking: the first sworn keeper in source order runs. Keepers that exist only in the store (filled through `Keep`, unpacked from a bundle) rank after every source keeper, newest sworn first. `oath run --prefer <label>` overrides. Unsworn keepers are never eligible. `oath swear` prints which keeper would run and why.

## Acceptance criteria

- [ ] golden test: with keepers `reference` then `fast` both sworn, `reference` runs; `--prefer fast` flips it
- [ ] golden test: a store-only keeper never outranks a source keeper
- [ ] `--prefer` naming an unsworn keeper fails before running

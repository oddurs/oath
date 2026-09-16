# Content-addressed code

## Prior art: Unison

Unison identifies each definition by a hash of its syntax tree. The tree is normalised before hashing: named arguments become positionally numbered references, and every dependency is replaced by its own hash, so "the hash of `increment` uniquely identifies its exact implementation and pins down all its dependencies." Names are "just separately stored metadata that don't affect the function's hash." Renaming is a change to the name-to-hash mapping; "a Unison codebase is never in a broken state, even midway through a refactoring."

Tests are watch expressions whose results are cached: Unison "caches test results unless the functions in a particular test's dependency graph receive a new hash." Tests that perform IO cannot be cached this way, because they may differ per run.

The stated tension: definitions are immutable, but people need to evolve which definitions they use, so a codebase manager mediates between content-addressed storage and mutable name bindings.

## What Oath takes

- **Hash a canonical tree, not text.** Alpha-renaming, whitespace, comments and example order do not change identity (item 0020).
- **Names are labels.** The name is not in the hash; renames break nothing (items 0021, 0104).
- **Cached evidence keyed by hash.** A sworn record is exactly Unison's cached test result, keyed by oath hash and keeper hash, invalidated only when a hash changes (item 0041). Unison's caveat about IO is answered differently: effectful keepers are sworn under rollback, so their runs are repeatable (item 0046).

## What Oath inverts

Unison hashes the implementation. Oath hashes the **promise** and attaches any number of implementations to it. The consequences:

- A refactor that keeps the oath is invisible to every caller.
- A change to the oath is a new identity, so it cannot silently keep old evidence (item 0042).
- Several bodies coexist under one promise: a reference, a fast one, a model-written one. That is what makes differential checking (item 0045) and the `Keep` effect (item 0053) possible.

## What Oath does differently, and the risk

Unison keeps the codebase in a database managed by a tool. Oath keeps source in ordinary files and evidence in a plain-file store that is committed to git. This is simpler and reviewable in a pull request, but the "never broken mid-refactor" property does not hold: an editor can leave a file unparseable. Oath accepts that; it is a text-first language.

Identity churn is the open cost. Because examples are in the hash (decision 3), every strengthening of evidence is a new promise. Path imports exist for those who want to follow changes; hash imports exist for those who want exactly what they tested against. Spike 0020 must answer this consciously.

## Sources

- [Unison: the big idea](https://www.unison-lang.org/docs/the-big-idea/)
- [Unison: testing and cached results](https://www.unison-lang.org/docs/usage-topics/testing/)
- [Unison 1.0 announcement coverage, InfoWorld](https://www.infoworld.com/article/4100673/futuristic-unison-functional-language-debuts.html)

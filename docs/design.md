# Oath: design decisions

Oath is an interpreter where every call resolves to a body sworn against a hashed contract, and effects roll back on failure. Each decision below names the research note that grounds it and the backlog item that carries it.

## 1. A definition is a promise plus keepers

An *oath* is a name, a type signature, laws and examples. A *keeper* is a body. Call sites bind to the oath's hash, never to a body. The runtime supplies a sworn keeper.

Grounded in Unison's content addressing ([research/01](research/01-content-addressed-code.md)), inverted: Unison hashes the implementation, Oath hashes the specification and lets implementations come and go. Carried by items 0020, 0021, 0023.

## 2. The name is not in the hash

Renaming is free. Two oaths with different names and the same hash are the same promise. Straight from Unison. Items 0021, 0104.

## 3. Examples are part of the promise

Adding an example changes the hash. Importers by hash stay on the promise they tested against; importers by path are told the promise changed. Principled, at the cost of identity churn. Recorded as the question spike 0020 must answer, with the alternative (hash type plus laws only) stated.

## 4. Evidence is dynamic; the signature is a runtime contract

There is no static type checker on the v1.0 path. Arguments and results are checked at the oath boundary, blaming the caller or the keeper. Function-typed values are wrapped and checked when called.

Grounded in Racket's contract boundaries and Eiffel's design by contract ([research/02](research/02-contracts-and-blame.md)). Item 0096. Static types are deferred with reasons on item 0114.

## 5. Swearing is small-scope enumeration, smallest first

Laws are checked against every value up to a depth, smallest first, so the first failure is the minimal counterexample without a shrinking step. Function arguments come from a table per signature. Several sworn keepers are run differentially and their disagreements reported.

Grounded in Jackson's small scope hypothesis, SmallCheck, and McKeeman's differential testing ([research/03](research/03-evidence-and-small-scope.md)). Items 0038, 0039, 0119, 0045.

## 6. Sworn records are cached by hash and committed

A sworn record is keyed by the oath hash and the keeper hash. Nothing re-runs unless a hash changes. The store is committed with the project, like a lockfile.

Unison caches test results by hash the same way ([research/01](research/01-content-addressed-code.md)). Items 0041, 0042, 0100.

## 7. Undo is derived, not written

State cells are the only mutable thing; every write journals the old value, so rollback is correct by construction. `external` ops carry a user-written compensation. `irreversible` ops are refused inside `try` and during swearing.

Grounded in Verse's transactional failure, the saga pattern, and reversible languages ([research/04](research/04-reversible-effects.md)). Items 0032, 0034, 0035, 0036.

## 8. Handlers resume once

One-shot continuations, as in OCaml 5. Multi-shot resumption interacts badly with a journal and is deferred (item 0093). Item 0037.

## 9. A hole is an oath with no keeper

Reaching one performs `Keep : Str -> Str`. The handler decides the policy: pause for a person, search the store, ask an external command, or refuse. Proposals are sworn under rollback before they are adopted, so a bad proposal leaves no trace.

Grounded in Hazel's typed holes and the typed-holes-for-LLMs work ([research/05](research/05-holes-and-keepers.md)), narrowed: expression-level holes need a type checker and are deferred (item 0115). Items 0053, 0054, 0055, 0057.

## 10. Selection is source order

The order a person wrote keepers is a ranking. Store-only keepers rank after every source keeper. `--prefer` overrides. Neither "fastest" nor "most evidence" can decide honestly at small scope. Item 0098.

## 11. Deferred with reasons

Static types, expression holes, effect rows, multi-party oaths, a registry, a faster backend. Each carries why it is not on the v1.0 path ([research/06](research/06-deferred.md)).

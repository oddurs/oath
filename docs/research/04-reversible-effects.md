# Reversible effects

## Prior art

**Reversible languages** (Janus; Yokoyama and Glück's invertible self-interpreter) make every statement invertible by construction: assignments are restricted to forms like `x += e` where `x` does not occur in `e`, and conditionals carry exit assertions so the inverse knows which branch was taken. Reversibility is bought by forbidding information loss in the language itself.

**Verse** makes failure a control-flow primitive and runs `<transacts>` methods as transactions: when one fails, "changes are automatically reverted," and "any failure below rolls back the gold as well as the item." Effects that cannot be rolled back are kept out of failure contexts.

**Sagas** handle the distributed case without a global transaction: a sequence of local transactions, each with a compensating transaction that reverses it if a later step fails. The stated limit is isolation: concurrent sagas can observe each other's partial state.

**Effect handlers** in OCaml 5 capture a delimited continuation at `perform`; "every captured continuation must be resumed either with a continue or discontinue exactly once," because "one-shot continuations are sufficient for almost all concurrent programming needs" and programs "manipulate linear resources such as sockets and file descriptors." An unhandled effect raises at the point of `perform`. Koka's documentation likewise treats tail-resumptive, one-shot operations as the common case.

## What Oath takes

- **Rollback is Verse's, not Janus's.** Oath does not restrict the language to invertible statements. It journals: state cells are the only mutable thing, every write records the old value, and `try` rewinds to its mark. The user writes no inverse for state (item 0035). This is the round-three decision: a hand-written inverse can be wrong, and a wrong inverse silently breaks the one thing rollback promises.
- **Compensation is the saga case.** An `external` op declares an `undo` because the runtime cannot know how to reverse a file write. The journal replays compensations in reverse order (items 0032, 0035).
- **Irreversible ops are the pivot.** `print` and its kind can be performed neither inside `try` nor during swearing; the check is lexical and transitive (item 0034). Verse's exclusion of unrollable effects from failure contexts is the same rule.
- **One-shot handlers.** OCaml's rationale applies directly, and a second resumption after a rewind has no coherent meaning against the journal (item 0037).

## What this buys

- **Swearing effectful code is safe.** Each example runs inside an implicit `try` and is rewound, which is why Unison's "IO tests cannot be cached" caveat does not apply (item 0046).
- **A proposal from outside leaves no trace.** A keeper from an external command is sworn under the same rollback (item 0057).
- **A time-travel debugger for free.** The journal is a tape; `oath trace` steps it (item 0059).

## The limit Oath inherits

Sagas' missing isolation. Oath's runtime is single-threaded in v1.0, so it does not arise; it will the moment roles or parallel evaluation arrive, and it is noted on those deferred items.

## Sources

- [A reversible programming language and its invertible self-interpreter (Yokoyama, Glück)](https://dl.acm.org/doi/10.1145/1244381.1244404)
- [Reversible computing research groups](https://reversibility.org/programming/academia/)
- [Book of Verse: overview](https://verselang.github.io/book/00_overview/)
- [Bringing Verse transactional memory semantics to C++](https://www.unrealengine.com/en-US/tech-blog/bringing-verse-transactional-memory-semantics-to-c)
- [Saga pattern](https://microservices.io/patterns/data/saga.html)
- [OCaml 5 manual: effect handlers](https://ocaml.org/manual/5.3/effects.html)
- [Koka book](https://koka-lang.github.io/koka/doc/book.html)

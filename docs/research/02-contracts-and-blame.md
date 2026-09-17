# Contracts and blame

## Prior art: Eiffel and Racket

Design by contract comes from Meyer's Eiffel (1986): preconditions are the client's obligation, postconditions the supplier's, and "the contract is semantically equivalent to a Hoare triple." It "does not replace regular testing strategies" but adds self-checks that act as a test oracle.

Racket puts contracts at module boundaries: "whenever a value crosses this boundary, the contract monitoring system performs contract checks," and a violation "blames the module for breaking its promises." Higher-order values are not checked eagerly; a function crossing a boundary is wrapped, its argument contract checked when it is called and its result contract when it returns. Blame for an argument violation goes to the caller, for a result violation to the callee. Racket warns that nested boundaries "may have unexpected performance implications or blame a party that may seem unintuitive."

## What Oath takes

- **The oath boundary is a contract boundary.** Arguments are checked against the declared argument types and blame the caller; results are checked against the declared result type and blame the keeper (item 0096).
- **Higher-order values are wrapped, not inspected.** A function passed through an oath is checked on call, exactly as Racket does.
- **Laws are postconditions over generated inputs.** A law is a supplier obligation; swearing is the oracle run ahead of time rather than at every call (item 0039).

## The gap this research found

Design by contract has three parts. Oath had two: the signature and laws are supplier obligations, and there was nothing for the client's. That makes every partial oath unkeepable, because the generator produces the inputs the promise was never meant to cover:

```
oath head : List a -> a
  law \xs -> elem (head xs) xs      -- false for []
```

`requires` closes it (item 0121). It is part of the promise and therefore in the hash; it filters generation, so a keeper is never sworn against input it was not promised; and at runtime it blames the caller, exactly as Eiffel intends. Invariants, the third part, have no home in Oath because an oath owns no state; the nearest thing is a law over an effect's cells via `given`/`then` (item 0118).

## What Oath decides against

A static type checker on the v1.0 path. Every other form of evidence in Oath is dynamic, and the signature is declared, so nothing needs inference. A mistyped keeper fails swearing before it can be called. Static types return when expression-level holes need an expected type; see [06-deferred](06-deferred.md).

## The risk Racket names

Blame at nested boundaries can be unintuitive. Oath has exactly one kind of boundary, the oath, and every diagnostic prints the oath and keeper hashes involved (item 0079), which is the mitigation.

## Sources

- [Design by contract](https://en.wikipedia.org/wiki/Design_by_contract)
- [Racket Guide: contracts and boundaries](https://docs.racket-lang.org/guide/contract-boundaries.html)
- [Racket Guide: contracts on functions](https://docs.racket-lang.org/guide/contracts-general-functions.html)

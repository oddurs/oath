# Evidence: small scope, enumeration, and disagreement

## Prior art

**The small scope hypothesis** (Jackson, Alloy): "systems that fail on large instances almost always fail on small ones with similar properties. Whether these small instances are common in the actual executing system doesn't matter." It is the justification for Alloy's bounded analysis and has found bugs in long-standing distributed protocols.

**SmallCheck** applies it to property testing: "if a program fails to meet its specification in some cases, it almost always fails in some simple case," so it "tests properties for all the finitely many values up to some depth, progressively increasing the depth." Function-typed arguments use "a measure combining the depth to which arguments may be evaluated and the depth of possible results." SmallCheck's own documentation notes that random generators with shrinking now offer a better user experience for large inputs.

**Differential testing** (McKeeman, 1998): "use the difference between two or more implementations of the same specification as an oracle for testing." When two implementations disagree, at least one is wrong, even with no specification oracle. Reduction then shrinks the disagreeing input to a minimal case.

## What Oath takes

- **Exhaustive small scope, smallest first.** Laws are checked against every value up to a depth, enumerated smallest first, so the first failure is already the minimal counterexample and there is no shrinking step to write (items 0038, 0039).
- **A table for function arguments.** SmallCheck's depth measure for functions is elegant and opaque. Oath uses a small, user-extensible table of named functions per signature, written in Oath, so a counterexample says `f = negate` rather than printing a partial function (item 0119).
- **Disagreement as an oracle.** Several sworn keepers of one oath are run on the same inputs; a divergence is reported with the input and both outputs. This is McKeeman's oracle applied inside one program, and it is how Oath notices an underspecified oath (item 0045).
- **Polymorphism instantiated at Int.** A documented default with a flag for a second pass, rather than a generator for every type variable.

## What Oath decides against

- Random generation and shrinking in v1.0. Small scope is enough for the programs a hobby language runs, and it gives determinism for free: the same depth always produces the same verdict, which the sworn record depends on.
- Timing as evidence. Nothing measured on depth-4 inputs says anything about real inputs; "fastest" was removed from the selection policy (item 0098).

## The known limit

Small scope misses bugs that only appear at size. The sworn record stores the depth reached, `oath who` shows it, and the depth is the honest measure of how much a keeper has been tested.

## Sources

- [Evaluating the small scope hypothesis (Andoni, Daniliuc, Khurshid, Marinov)](https://www.semanticscholar.org/paper/Evaluating-the-%E2%80%9C-Small-Scope-Hypothesis-%E2%80%9D-Andoni-Daniliuc/0c6d97fbc3c753f59e7fb723725639f1b18706bb)
- [Alloy: a lightweight object modelling notation (Jackson)](https://groups.csail.mit.edu/sdg/pubs/2002/alloy-journal.pdf)
- [SmallCheck](https://hackage.haskell.org/package/smallcheck)
- [Differential testing for software (McKeeman)](https://www.cs.tufts.edu/comp/150FP/archive/bill-mckeeman/DifferentailTesting.pdf)

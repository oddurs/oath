# Deferred, with reasons

Each of these is a real idea that was in an earlier draft of the plan. Each was moved off the v1.0 path for a stated reason, and the cairn item carries it.

## Static types (item 0114)

The draft spent a spike and two large items on inference in v0.1, effect typing in v0.2, hole typing in v0.3 and located types in v0.4. Every form of evidence in Oath is dynamic and every oath declares its signature, so a runtime contract at the boundary gives the same blame for far less code ([02](02-contracts-and-blame.md)). Static types earn their place when expression-level holes need an expected type (item 0115) or effect rows are wanted on prelude oaths (item 0116). POPL 2026's "Rows and Capabilities as Modal Effects" is the reference for the latter.

## Multi-party oaths (item 0117)

Choreographic programming writes one program for all participants and projects it per endpoint; "source choreographies do not allow for pairing send and receive actions incorrectly," giving deadlock freedom by construction. The hard parts are knowledge of choice at conditionals and keeping the projected code decentralised. This was the third pillar of the original pitch and remains the v1.1 headline. It is deferred because it is absent from the v1.0 sentence and is the work most likely to run three times over: projection, a transport, and rollback across roles, which reintroduces the saga isolation problem ([04](04-reversible-effects.md)). The former spikes' questions are recorded on the item.

## Parallel evaluation (item 0091)

Pure keepers are natural candidates for an interaction-net runtime such as HVM. Only worth it once the tree-walker is too slow for a program someone actually wrote.

## Proof terms as evidence (item 0087)

A keeper sworn by a checked proof rather than enumeration, with the sworn record citing the proof hash. The right long-term form of evidence; needs a type system first.

## Registry (item 0088)

Fetching a keeper by oath hash from elsewhere and trusting it by producer. `oath pack` (item 0108) is the file-based precursor; unpacked keepers are re-sworn locally by default.

## Sources

- [Rows and capabilities as modal effects, POPL 2026](https://popl26.sigplan.org/details/POPL-2026-popl-research-papers/34/Rows-and-Capabilities-as-Modal-Effects)
- [Choreographic programming (Montesi)](https://www.fabriziomontesi.com/bliki/ChoreographicProgramming)
- [Chorex: restartable, language-integrated choreographies](https://arxiv.org/pdf/2511.15820)
- [HVM2 and Bend](https://github.com/HigherOrderCO/Bend)

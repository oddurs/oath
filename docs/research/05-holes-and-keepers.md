# Holes and keepers

## Prior art: Hazel

Hazel gives every incomplete program both a static and a dynamic meaning. "Evaluation proceeds around holes, tracking the closure around each hole instance as it flows through the remainder of the program." A hole closure captures the environment at the hole, and "fill-and-resume" lets evaluation continue after the hole is filled without restarting.

The typed-holes-for-LLMs work uses the hole as the prompt boundary: the language server supplies "the type and typing context of the hole being filled, even in the presence of errors," and proposals go through "iterative refinement via further dialog with the language server" rather than being accepted as returned. Contextualisation with type definitions was the most impactful ingredient.

## What Oath takes

- **Incomplete programs run.** An oath with no sworn keeper is legal and the program runs until it is called (items 0022, 0023).
- **Fill-and-resume.** Reaching an unkept oath performs `Keep`; the handler returns a body; the runtime swears it and resumes with it bound. This is Hazel's fill-and-resume expressed as an effect (item 0053).
- **The oath is the prompt.** The typed-holes work found that the type and surrounding definitions are what a model needs. Oath hands the handler the whole promise, pretty-printed: signature, laws, examples, and the sworn keepers that already exist (items 0051, 0057).
- **Iterative refinement.** A rejected proposal is retried with the counterexample appended (item 0057). The language server's role is played by swearing.
- **Policy belongs to the program.** Because `Keep` is an effect, a program can pause for a person, search the store, call a command, or refuse, and the language does not know what a model is (items 0054, 0055, 0057).

## What Oath narrows

Hazel's holes appear anywhere an expression can. That needs a type checker to know what to ask for at the hole. Oath's own idea is narrower: a hole is an oath with no keeper. It has a signature, laws and examples already, so the prompt is complete without inference. Expression-level holes are deferred with static types (item 0115).

`Keep` is `Str -> Str`. First-class code values would need quotation syntax; pretty-printed text in and source text out needs none, and the external-command source already speaks text.

## The safety argument

A proposal is sworn under rollback before adoption, with a step budget and no irreversible ops (spike 0056). An unsworn proposal leaves nothing in the world or the store. A sworn one is recorded with its producer, so `oath who` always says which keepers a model wrote (item 0043).

## Sources

- [Hazel](https://hazel.org/)
- [Live functional programming with typed holes (Omar et al.)](https://arxiv.org/abs/1805.00155)
- [Statically contextualizing large language models with typed holes](https://arxiv.org/abs/2409.00921)

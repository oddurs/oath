---
paths:
  - "**/*.rs"
  - "**/Cargo.toml"
---

# Rust in Oath

General Rust conventions apply. These are the ones specific to an interpreter whose
error messages are half the product.

- **No panic on user input, ever.** `unwrap` and `expect` belong in tests and in `main`.
  Source that does not parse, a broken store record and a failed contract are all
  `Result`, never a panic. CI greps for it, so a new one will be caught.
- **Every failure carries a span.** A diagnostic without a position is a bug report the
  user cannot act on. Thread spans through the AST rather than retrofitting them.
- **One `Diagnostic` type** shared by lexer, parser, checker and evaluator, so the
  renderer stays the only place that knows how a message looks.
- **Anything naming an oath or a keeper prints its hash.** Two promises can share a name;
  the hash is what identifies them.
- **Exit codes are part of the interface:** 0 success, 1 runtime error, 2 usage or source
  error. They are documented in `--help` and pinned by a test.
- **Hashing is load-bearing.** Anything touching canonicalisation changes identity for
  every user, so it needs a test proving what does and does not change the hash.
- Modules appear with the work that needs them. An empty module named for a future
  subsystem is scaffolding.

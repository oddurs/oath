---
id: 114
title: Static type system
type: feature
status: backlog
milestone: later
created: 2026-09-16
updated: 2026-09-16
priority: p2
effort: xl
area: types
---

## Why it is here and not on the v1.0 path

Every piece of evidence in Oath is dynamic: examples, laws, differential runs, rollback. The oath signature is declared, so generation never needs inference, and a mistyped keeper fails swearing. A runtime contract at the oath boundary (v0.1) gives the same blame with a tenth of the code. Static types earn their place when expression-level holes need an expected type, or when effect rows are wanted on prelude oaths. Both are listed here and depend on this.

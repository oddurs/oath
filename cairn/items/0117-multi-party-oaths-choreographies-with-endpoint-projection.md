---
id: 9330d99e-a3f7-430a-a8cf-8aa640c6efe0
title: 'Multi-party oaths: choreographies with endpoint projection'
type: feature
status: backlog
milestone: later
created: 2026-09-16
updated: 2026-09-16
priority: p2
effort: xl
area: choreo
---

## The third pillar, deferred

An oath naming roles, written once, projected per role, with an oath-hash handshake before any message. It is the v1.1 headline, not a v1.0 item, because it is absent from the v1.0 sentence and it is the work most likely to run three times over: knowledge of choice in projection, a transport, and rollback across roles.

## Questions to answer first (former spikes)

- projection strategy and the choreographable subset: straight-line plus located `if` with explicit selection is the smallest thing that runs a bank example
- rollback across roles: per-role `try` only, or an abort broadcast with every role rewinding to the choreography's entry mark

## Shape when it comes

parse roles → location check as a syntactic pass during projection → `oath project` → in-process transport → hash handshake → TCP on localhost → swear role oaths in-process → deadlock-freedom test over every example.

## 2026-09-16

Risk (docs/research/04): rollback across roles is a saga and inherits its missing isolation; the cross-role spike question must include it.

# M4 Policy-Decision Replay Request Summary

## Requested slice

Add a bounded library record that re-evaluates scripted-agent decisions from
actor-visible observations and keeps expected versus declared-anomalous cases
inspectable.

## Required boundaries

- Version the record as `m4-scripted-agent-replay-v1`.
- Retain only actor-visible observation input, policy decision, expected intent,
  disposition, and optional seed provenance.
- Replay through the existing default or seeded policy path and reject a
  mismatch with a bounded error.
- Keep host transition, execution, history, state hashes, and durable stores
  outside the record.

## Evidence target

An expected initial decision and a declared-anomalous expectation both replay
deterministically; a tampered recorded decision is rejected. The evidence is a
library-only inspection set, not a degenerate-policy population or outcome
report.

## Non-goals

No host-history integration, artifact persistence, population sampling,
scenario-level replay, strategic-quality claim, or human-behavior claim.

## Verification

Focused agent tests cover expected/anomalous classification and tamper
rejection. Full repository checks remain required before handoff.

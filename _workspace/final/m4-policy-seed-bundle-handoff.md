# M4 Policy Seed-Bundle Handoff

## Delivered

- Added the versioned `m4-scripted-agent-random-v1` seed bundle with explicit
  policy `StreamId`/`DrawId` identity and caller-provided seed.
- Added opt-in `ScriptedAgent::choose_with_seed` using
  `max-score-seeded-tie-v1` only among equal top-score candidates.
- Preserved default `choose` behavior under
  `max-score-stable-order-v1`; seeded decisions retain their bundle and rule
  identity for reproduction.
- Synchronized canonical/workspace docs, CHANGELOG, ARCHITECTURE, and
  LESSONS.md.

## Verification

The focused agent suite contains 13 tests. The full suite contains 167 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test,
plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Domain QA disposition

Pass for the bounded library-only policy-edge slice. Seeded requests remain
actor-bound and host-valid; no hidden state, wall clock, global RNG, transition,
execution, history, or replay authority moved into the agent.

## Open boundaries

The current fixture has unique public profile maxima, so draw-sensitive tie
variation is covered with synthetic equal-score candidates while public seeded
request validation covers the no-tie fixture. Seed persistence in artifacts,
broader sampling, top-k/nucleus selection, experiment manifests, populations,
outcomes, strategic quality, and human behavioral realism remain open.

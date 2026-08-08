# M4 Three-Profile Catalog Request Summary

## Requested slice

Extend the bounded scripted-agent catalog to three deterministic profiles and
compare them on one identical actor-visible initial observation.

## In scope

- Add `yielding-laner-v1` with a versioned
  `yield-first-fixed-score-v1` evaluation rule.
- Reuse shared actor-visible candidate generation, stable first-max selection,
  policy errors, and host validation.
- Assert stable candidate sequences, profile rule IDs, distinct selected
  intents, repeated decisions, and validator acceptance for all three profiles.
- Synchronize M4/core docs, QA/handoff, changelog, and `LESSONS.md`.

## Out of scope

- New scenarios, execution inputs, memory, communication, randomness, role
  populations, outcome metrics, external adapters, strategic quality, or
  human realism.

## Success evidence

- The same observation selects `Stabilize`, `Contest`, and `Yield` for the
  cautious, risk-taking, and yielding profiles.
- All profiles share the same advertised candidate sequence and exact rule IDs,
  repeat deterministically, and pass the same host validator.

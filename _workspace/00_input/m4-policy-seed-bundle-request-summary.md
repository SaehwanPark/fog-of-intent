# M4 Policy Seed-Bundle Request Summary

## Requested slice

Add an explicit, reproducible policy seed bundle for the remaining bounded M4
randomness item without changing the deterministic scripted-profile baseline.

## Required boundaries

- Carry a versioned seed plus policy `StreamId`/`DrawId` as an explicit input.
- Keep the existing `choose` path stable-order deterministic.
- Use seeded randomness only for equal top-score candidates in an opt-in
  `choose_with_seed` path.
- Keep candidate generation, scoring, host validation, transition authority,
  history, and true state unchanged.

## Evidence target

Identical observations and identical seed bundles reproduce the same decision;
changing the policy draw can select a different member of an equal-score tie;
the resulting request remains host-valid. The seed bundle and selection rule
are inspectable without exposing hidden state.

## Non-goals

This slice does not add broad random sampling, top-k/nucleus selection,
population experiments, experiment manifests, model-provider randomness,
scenario outcomes, or human-behavior claims.

## Verification

Focused agent tests cover bundle identity, seeded decision reproducibility,
stream/draw-scoped tie selection, and host validation. Full repository checks
remain required before handoff.

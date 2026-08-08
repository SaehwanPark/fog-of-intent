# M6 Matched Scenario Sample Set Request Summary

## Target slice

Compose a bounded, caller-supplied set of matched observation pairs over the
existing `ScriptedAgentMatchedSample` contract. Preserve pair order and
observation identity while keeping scenario/population generation outside the
library.

## Required behavior

- Accept a non-empty list of at most four matched observation pairs.
- Require one shared actor and globally distinct observation IDs across all
  pairs before policy evaluation.
- Reuse the existing two-observation sample and seeded batch runner in stable
  pair/manifest order.
- Return only bounded nested selected-intent samples and fixed schema/observer
  metadata; repeated construction must be equal.
- Reject empty, over-capacity, mixed-actor, and duplicate-ID inputs without
  running policies.

## Non-goals

This slice does not generate scenarios or populations, choose a distribution,
aggregate outcomes, emit metrics, persist samples, or claim strategic quality,
balance, provider compatibility, or human behavior.

## Verification

Use two caller-supplied pairs with distinct IDs, repeat the sample, assert pair
and manifest order, and cover all bounded rejection branches. Run the pinned
Rust, repository, Python, and diff gates.

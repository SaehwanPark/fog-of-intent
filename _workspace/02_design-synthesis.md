# Design Synthesis — M2 Matched-Input Strategy Fixtures

## Decision

Add three explicit diagnostic fixture descriptors over the existing one-window
lane, allied-coordination, and terminal-objective contracts:
`HappyPath`, `RiskTaking`, and `Conservative`. They are immutable, matched-input
bundles, not an additional policy engine or scenario runtime.

The production path remains host-owned: fixture construction selects explicit
inputs, existing validation and transition APIs commit the one record, and the
existing objective review classifies the committed result. No fixture bypasses
actor-visible observations, proposal-ID binding, coordination validation,
execution separation, replay, or state hashing.

## Resolved Cases

- Happy path: `Contest` + accepted support, self damage `0`, opponent damage
  `2`, wave `Advanced` -> `HeldSpace` and `GoalAchieved`.
- Risk-taking: `Contest` + rejected support, self damage `3`, opponent damage
  `0`, wave `Lost` -> legal `YieldedSpace` and `GoalMissed`.
- Conservative: `Stabilize` + rejected support, no damage, wave `Held` ->
  deliberate `YieldedSpace` and `GoalMissed`.

Each response uses the proposal ID generated from the canonical allied actor
input. The fixture runner appends through `CoordinatedLaneHistory`, then derives
`ObjectiveReviewRecord`; expected outcomes are checks, never transition
inputs.

## Evidence and Limits

Tests establish exact fixture reproducibility, distinct declared input/output
cases, replay, validation, state/hash preservation, and legal-unfavorable
behavior. They do not establish strategy quality, balance, optimality, human
preference, enjoyment, accessibility, trust, or behavioral validity.

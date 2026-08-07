# Simulation Design — M2 Automatic-Advance Contract

## Goal and Boundary

Define when a bounded lane window still requires a decision. The contract is a
pure synchronous value-level rule; the existing host remains responsible for
obtaining a commit and resolving explicit execution inputs.

## Contract

`LaneAdvanceCondition::OnCommit` advances only after a decision has been
committed. `LaneAdvanceCondition::WhenNoLegalIntent` advances only when the
host supplies an explicit zero count of currently legal intents. Otherwise the
decision remains required. The evaluation returns
`LaneAdvanceDecision::{DecisionRequired, AdvanceAutomatically}` and has no
side effects.

Current `LaneWindow::OneBeat` and `LaneWindow::TwoBeats` use `OnCommit`, so
existing transition, hash, observation, and replay behavior is unchanged. The
no-legal-intent condition is defined for a later host integration slice; this
change does not synthesize execution outcomes or close a window by itself.

## Verification Contract

- Both window durations expose the current commit-required condition.
- Commit and no-legal-intent inputs map to the documented decisions.
- Existing transition/replay hashes and M1 fixtures remain unchanged.

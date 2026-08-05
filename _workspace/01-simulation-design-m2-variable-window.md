# Simulation Design — M2 Bounded Variable-Duration Window

## Goal and Boundary

This slice adds one explicit `TwoBeats` decision-window duration to the
existing synchronous lane transition. The selected window duration is
authoritative state, appears in current player/allied observations, advances
the committed turn deterministically, and closes automatically when the
existing transition commits its resolved execution input.

The prior `OneBeat` contract remains the default. No manual tick command,
adaptive pacing, third duration, threat-damage rule, or new actor policy is
introduced.

## Window Contract

```text
LaneWindow::OneBeat  -> turn span 1, closes on transition commit
LaneWindow::TwoBeats -> turn span 2, closes on transition commit
```

`LaneSnapshot::new` continues to create a `OneBeat` state. A bounded
`new_with_window` constructor creates a `TwoBeats` state for the diagnostic
case. `transition_lane` advances the turn by the selected span and retains the
window kind in the resolved state. The resolved phase is the automatic
close-on-commit condition; no separate advance command is needed.

One-beat state hashes remain byte-compatible with the prior contract. Two-beat
state hashes include an explicit duration tag and therefore cannot collide with
the corresponding one-beat snapshot solely because of duration. Existing
one-beat record identities and replay results remain unchanged.

## Observation and Coordination

Player and allied observations carry the current `LaneWindow`. The allied
scripted policy accepts either bounded duration but keeps the same two
`Stabilize`/`Contest` candidates, scores, and support semantics. The duration
is included in the allied visible input digest only for `TwoBeats`, preserving
prior one-beat policy identities while binding longer-window policy input.

Player intent, conditional Withdraw availability, last-known threat reporting,
objective attribution, branch checks, scenario reopening, and final-debrief
projections use their existing authority. A future scenario can choose a
duration explicitly at its host boundary; this slice only proves the typed
two-beat transition contract.

## Verification Contract

Focused tests cover duration propagation to both observations, distinct
two-beat state hashing, two-turn advancement, automatic resolved closure,
unchanged allied candidate bounds, and exact history replay. Existing M1/M2
tests remain passing.

Evidence establishes one bounded TwoBeats duration only. It does not establish
adaptive pacing, automatic execution outcomes, communication, strategy
quality, balance, or a complete playable lane scenario.

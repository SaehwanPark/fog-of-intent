# Design Synthesis — M2 Bounded Two-Window Scenario Wrapper

## Decision

Compose two existing one-window lane transitions with one explicit deterministic
reopen boundary. The base transition and all prior one-window/branch/
coordination/objective identities stay unchanged. `LaneScenarioHistory` owns
only sequence state, per-window start-state evidence, and the reopen boundary.

The public reopen boundary accepts the opaque committed transition result,
verifies its state hash/outcome consistency, then copies valid player,
opponent, wave, and hidden-threat values into an `Open` snapshot at the already
advanced turn and clears the per-window terminal outcome. The wrapper records
this state and requires it as the second window's starting point. The second
result remains resolved and is the two-window terminal state.

## Resolved Contract

`m2-two-window-scenario-v1` accepts at most `First` and `Second` records. Each
record stores its exact start state and complete `LaneTransitionRecord`; the
first additionally stores the reopened state. Replay regenerates observations,
validates commands, reruns the base transition, reconstructs the reopen state,
and compares all stored values. A third append, resolved current window, bad
reopen state, or tampered record fails.

The wrapper uses ordinary player records in this slice. Existing allied
coordination, objective, fixture, and branch APIs remain valid; a future
version can compose coordination across multiple windows once that boundary
has a demonstrated need.

## Evidence and Limits

Tests cover valid reopen invariants, two sequential commits, terminal state,
objective preservation, third-window rejection, repeated replay, and reopen
tamper detection. This establishes a deterministic two-window composition only;
variable pacing, recall, gank response, communication, a complete scenario,
strategy quality, balance, and human evidence remain open.

# Design Synthesis — M2 Scenario Goal and Terminal Objective

## Decision

The next M2 slice is a single diagnostic goal, `HoldLaneSpaceThroughWindow`,
evaluated after an ordinary or coordinated one-window record is committed.
The objective is a pure host-owned review projection. It does not add a second
window, a new mechanic, a state field, a transition event/effect, or a general
goal framework.

The simulation contract and prior coordination contract agree on these
boundaries:

- `transition_lane` remains the only state/outcome/hash authority;
- objective inputs are derived from committed record facts, not caller-picked
  hidden state or inferred policy quality;
- ordinary records keep `NotApplicable` coordination, while coordinated
  records preserve their exact disposition;
- objective input identity is provenance and includes source replay identity,
  hashes, and a canonical digest, but the visible `ObjectiveReport` omits
  source hashes and private receipts;
- replay compares the complete derived objective inputs and review, while the
  source record identity detects neutral-input tampering;
- the result is a diagnostic classification, never a new `LaneOutcome` or
  persistent state mutation.

## Resolved Contract

The two criteria are `SpaceHeld` (next player position is `Center`) and
`SurvivedBeat` (next player health is nonzero). `GoalAchieved` requires both;
`GoalPartiallyAchieved` is the explicit held-space/zero-health combination;
all other combinations are `GoalMissed`. The evaluator records player intent,
wave result, coordination disposition, and execution trace solely for causal
attribution; it never labels an action optimal.

`ObjectiveReviewRecord` stores the goal, source replay/record identity,
canonical objective inputs, and `TerminalObjectiveReview`. Ordinary and
coordinated constructors derive those values from their respective committed
records. `verify_lane` and `verify_coordinated` reject altered objective facts,
source identity, review, or coordinated base-record provenance.

## Evidence and Limits

Focused tests cover achieved, missed, partial, forced-out, yielded-space,
ordinary, and accepted-coordination cases; report hash redaction; replay
tamper; unsupported replay identity; unchanged state hash; and deterministic
typed evaluation. This establishes software-level objective bookkeeping for
one window only. It does not establish a complete scenario, optimality,
balance, trust, enjoyment, accessibility, behavioral validity, or human
preference.

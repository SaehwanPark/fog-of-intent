# Simulation Design — M9 Match-Level Pivotal-Decision Detection

## Goal and Roadmap Milestone

M9 — Bounded Multi-Lane Match Prototype. This slice closes the open scope
item "Add match-level pivotal-decision detection" nested under the comeback
mechanics work. It provides a deterministic, pure evaluation boundary that
identifies which declared match decisions most changed the match's value —
the turning points a match-level debrief should surface.

## Slice Boundary and Non-Goals

**Boundary.** One versioned contract `m9-pivotal-decision-v1` plus a
benchmark catalog `m9-pivotal-catalog-v1`, following the established M9
pure-evaluation pattern (`m9-comeback-mechanics-v1`):

- Inputs are fully explicit, caller-declared decision samples: decision id,
  turn, acting side, and Allied-perspective net match value immediately
  before and after the decision, in integer basis points.
- Detection classifies swing magnitude, swing direction, lead changes, and
  whether the swing aligned with or against the acting side.
- All arithmetic is exact integer math on `i32`/`u32`; no floating point, no
  randomness, no I/O, no wall clock, no authoritative state access.

**Non-goals.**

- No automatic trajectory derivation from authoritative match state.
- No counterfactual branch execution from a pivotal decision.
- No decision-quality, optimality, or human-debrief-usefulness claims.
- No host/CLI/MCP integration into the runnable fixture.

## Actors and Authority

No actor gains authority from this contract. The caller (host debrief code,
tests, or research tooling in later slices) owns the value trajectory and
declares it explicitly. The evaluator is a pure function and cannot mutate
any state, emit authoritative events, or authorize commands.

## True State, Beliefs, Observations, and Reports

- **True state**: untouched; this contract never reads `MatchMapState`,
  structures, objectives, or any authoritative match value.
- **Belief/observation**: not projected here; value trajectories are declared
  inputs that a future host boundary may derive from committed facts.
- **Report**: `PivotalDecisionReport::render_markdown()` produces a
  structured debrief summary containing only derived classification facts —
  no hashes, resolved inputs, execution traces, or private chain-of-thought.

## Plans, Commands, and Validation

There are no commands. Input validation is fail-closed with a typed error
`PivotalDecisionError`, checked in a fixed order before any classification:

1. `EmptyTrajectory` — no samples.
2. `ValueOutOfRange { index }` — any `value_before_bp`/`value_after_bp`
   outside `[-10,000..=10,000]`. Detection thresholds depend on exact
   magnitudes, so out-of-range inputs are rejected rather than clamped.
3. `NonMonotonicTurn { index }` — turns must be strictly increasing; equal or
   decreasing turns are caller errors.

Validation failure is distinct from any modeled outcome; there are no
"unfavorable but legal" paths inside detection itself — a large unfavorable
swing is a valid, classifiable finding.

## Resolved Inputs and Random Streams

No stochastic inputs. Determinism is total: identical sample slices produce
identical reports (`Eq`-comparable). Tie-breaking is structural (largest
absolute swing; earliest turn on ties), never random.

## Events, Effects, and Transition

None. This slice emits no events or effects and performs no transition; it is
an evaluation boundary layered beside `evaluate_comeback_opportunity`.

## History, Replay, and Branching

No history changes. The report is reproducible from its declared inputs,
which is the replay property this slice provides. Branching stays with the
existing M2/M9 branch boundaries.

## Debrief and Causal Explanation

Per-decision finding (`PivotalDecisionFinding`):

- `swing_bp = value_after_bp - value_before_bp` (Allied perspective).
- `SwingDirection`: `AlliedFavorable` / `OpposingFavorable` / `Neutral`.
- `PivotalTier` from absolute swing, with explicit thresholds mirroring the
  comeback deficit thresholds' granularity:
  - `Routine`: `|swing| <= 500` bp — ordinary play.
  - `Notable`: `501..=1,500` bp — meaningful, not decisive.
  - `Pivotal`: `1,501..=3,500` bp — major turning point.
  - `MatchDefining`: `> 3,500` bp — game-deciding swing.
- `lead_changed`: strict sign flip of the Allied-perspective value
  (`before > 0 && after < 0` or `before < 0 && after > 0`); passing to or
  from exact parity is not a lead change.
- `DecisionAlignment`: `SwingWithActor` / `SwingAgainstActor` /
  `NeutralSwing` — separates "pivotal because the acting side made it
  count" from "pivotal because the acting side threw", echoing the M8 rule
  that attribution must not collapse into raw outcome sign.

Report aggregates: findings in turn order, `most_pivotal` (max absolute
swing, earliest turn on ties), `pivotal_count` (Pivotal + MatchDefining),
ranked `pivotal_findings()` helper, `lead_change_turns`, `final_value_bp`
(last sample's after value), and `total_absolute_swing_bp` (saturating sum).

## Verification Contract

Focused tests named before implementation:

- Tier boundaries: 500/501, 1,500/1,501, 3,500/3,501, and a 0-swing.
- Direction and alignment classification matrix (Allied/Opposing actor with
  favorable/unfavorable/neutral swings).
- Lead-change sign-flip matrix: both flips true; zero-crossings-into-parity
  false.
- Ranking determinism and earliest-turn tie-break.
- Fail-closed: empty, non-monotonic and duplicate turns, out-of-range
  values, with correct sample indices.
- Reproducibility: identical inputs yield equal reports; `final_value_bp`
  and `total_absolute_swing_bp` aggregate correctly.
- Catalog: all three benchmark scenarios meet expectations; unknown id
  fails; Markdown rendering contains the debrief labels and no hidden
  fields.

Catalog scenarios (`m9-pivotal-catalog-v1`):

1. `scenario-base-race-decisive-swing-v1` — one MatchDefining swing decides
   an uncontested base race.
2. `scenario-baron-throw-comeback-v1` — an Opposing decision swings the
   match to Allied (SwingAgainstActor + lead change), followed by a Notable
   consolidation swing.
3. `scenario-stable-slow-burn-v1` — only Notable/Routine swings, zero
   pivotal decisions, no lead changes.

## Open Questions

- Should a future host boundary derive value trajectories automatically from
  committed match history? Deferred with comeback auto-detection.
- Should pivotal reports integrate into the M9 match debrief projections?
  Deferred until a host integration slice requests it.

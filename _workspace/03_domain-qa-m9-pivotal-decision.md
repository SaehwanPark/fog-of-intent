# Domain QA — M9 Match-Level Pivotal-Decision Detection

## Status

`pass` — the bounded contract passed review. Balance, enjoyment,
accessibility, behavioral validity, and intellectual-property posture are not
validated by this review.

## Reviewed Inputs

- Branch `feat/m9-pivotal-decision-detection`: `src/map/pivotal.rs`,
  `src/map/pivotal_catalog.rs`, `src/map/tests/pivotal.rs`, `src/map/mod.rs`,
  `src/map/tests/mod.rs`.
- `_workspace/00_input/m9-pivotal-decision-request-summary.md` and
  `_workspace/01_simulation-design-m9-pivotal-decision-detection.md`.
- `ROADMAP.md` Phase 9 scope; `SPEC.md` M9 evidence;
  `src/map/comeback.rs` / `comeback_catalog.rs` as the established sibling
  pattern.
- Verification: `cargo +1.96.0 fmt --all -- --check`,
  `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`,
  `cargo +1.96.0 test --locked` — 434 lib (21 new), 7 binary, 3 doc tests
  pass.

## Scope and Roadmap Findings

- The slice maps exactly to the open M9 item "Add match-level
  pivotal-decision detection" nested under the comeback mechanics evidence;
  no unrelated scope was added and sibling open items (decision density,
  profiling, broader tests) remain untouched.
- Follows the M9 pure-evaluation pattern established by
  `m9-comeback-mechanics-v1`: explicit caller-declared inputs, pure
  deterministic function, benchmark catalog, focused tests.

## Authority and Information-Boundary Findings

- `detect_pivotal_decisions` reads only its argument slice; it cannot touch
  `MatchMapState`, structures, objectives, or any authoritative value. No
  command, transition, event, or effect authority is introduced.
- The report is derived classification only. `render_markdown` emits
  decision ids, turns, tiers, directions, alignments, lead changes, and
  aggregates; no hashes, resolved inputs, execution traces, receipts, or
  private chain-of-thought appear, and the test asserts this.
- Values are Allied-perspective by documented contract; an Opposing
  perspective requires caller-side negation. This matches the comeback
  module's explicit-input stance and avoids a second perspective parameter.

## Determinism, Replay, and Reproducibility Findings

- No randomness, wall clock, I/O, async, threads, or global mutable state;
  all arithmetic is saturating integer math on bounded inputs.
- Iteration is over the caller slice in order; ranking ties break
  structurally (first-encountered maximum equals earliest turn because
  validation enforces strictly increasing turns).
- `PivotalDecisionReport` derives `Eq` and the reproducibility test compares
  two full evaluations. Validation is fail-closed and precedes
  classification, with typed errors carrying the offending sample index.

## Behavior and Playtest Findings

- This is debrief tooling, not an agent policy: no profile, utility, or
  selection behavior is introduced, so no agent-ecology design was required.
- Attribution (`DecisionAlignment`) classifies outcome direction relative to
  the acting side; it does not and must not be read as decision quality. The
  design doc, module docs, and evidence limits state this.

## Gameplay and Debrief Findings

- Tier thresholds (500 / 1,500 / 3,500 bp) are named public constants with
  boundary tests at both sides of each threshold.
- Lead change is a strict sign flip; passing to or from exact parity is not
  counted, tested at both zero crossings.
- The catalog covers three defensible shapes: a match-defining decisive
  swing, an against-actor throw with a lead change, and a stable match with
  zero pivotal decisions — more than one defensible trajectory is
  representable.

## Evidence and Claim Limits

- Library-only evaluation over declared trajectories; automatic derivation
  from authoritative match history, host/CLI/MCP integration, and
  counterfactual branching remain open, consistent with the roadmap.
- No human-debrief-usefulness, decision-quality, or playability claim is
  made anywhere in the code or tests.

## Required Fixes

None. Two non-blocking observations recorded for the PR reviewer:

1. `PivotalDecisionError` implements `Display` but not `std::error::Error`,
   matching every sibling map error (`MapGraphError`, `ObjectiveError`,
   `StructureError`, `TravelError`, `VisionError`, `RoleActionError`).
2. `sample_count` duplicates `findings.len()` for consumer convenience,
   mirroring aggregate fields on sibling M9 report types.

## Residual Risks

- Threshold constants are declared, not calibrated; treat tiers as bounded
  classification evidence, not validated turning-point semantics.
- A caller declaring a fabricated trajectory gets a well-formed report; the
  contract trusts declared inputs by design and must be composed with
  host-derived trajectories in a future slice to become evidence-bearing.

## Verification Evidence

- Full suite green on Rust 1.96.0 (fmt check, clippy `-D warnings`, tests).
- New focused tests: 21, covering tier boundaries, direction/alignment
  matrices, strict lead-change semantics, ranking tie-break, fail-closed
  validation (empty, non-monotonic/duplicate turns, out-of-range values with
  indices), reproducibility, aggregates, ranked filtering, all three catalog
  scenarios, unknown-id failure, and Markdown label hygiene.

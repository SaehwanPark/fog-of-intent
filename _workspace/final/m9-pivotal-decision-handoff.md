# M9 Pivotal-Decision Detection Handoff

## Summary

This slice closes the open M9 roadmap item "Add match-level pivotal-decision
detection" nested under the comeback-mechanics evidence. It follows the
established M9 pure-evaluation pattern (`m9-comeback-mechanics-v1`):

1. **Detection contract (`src/map/pivotal.rs`, `m9-pivotal-decision-v1`)**:
   - `PivotalDecisionSample`: explicit caller-declared decision measurement —
     decision id, strictly increasing turn, acting side, Allied-perspective
     net match value before/after in `[-10,000..=10,000]` bp. No
     authoritative match state is consulted.
   - `PivotalTier`: `Routine` / `Notable` / `Pivotal` / `MatchDefining` from
     absolute swing magnitude at explicit 500/1,500/3,500 bp thresholds.
   - `SwingDirection` and `DecisionAlignment`
     (`SwingWithActor`/`SwingAgainstActor`/`NeutralSwing`): outcome direction
     kept separate from acting-side attribution.
   - Strict lead-change detection: only a value-sign flip counts; to/from
     exact parity does not.
   - `detect_pivotal_decisions`: pure function; fail-closed typed errors
     (`EmptyTrajectory`, `ValueOutOfRange { index }`,
     `NonMonotonicTurn { index }`) validated before classification.
   - `PivotalDecisionReport`: findings in turn order, `most_pivotal` (largest
     absolute swing, earliest-turn tie-break), `pivotal_count`, ranked
     `pivotal_findings()`, `lead_change_turns`, `final_value_bp`, saturating
     `total_absolute_swing_bp`, and a structured Markdown debrief rendering
     with zero private chain-of-thought.

2. **Benchmark catalog (`src/map/pivotal_catalog.rs`,
   `m9-pivotal-catalog-v1`)**: `PivotalCatalog` registers 3 canonical
   scenarios with fail-closed lookup and verifiable expectations:
   - `scenario-base-race-decisive-swing-v1`: one `MatchDefining` +4,400 bp
     swing decides an uncontested base race.
   - `scenario-baron-throw-comeback-v1`: an Opposing against-actor throw
     swings +3,000 bp and flips the lead at turn 14.
   - `scenario-stable-slow-burn-v1`: only routine/notable swings; zero
     pivotal decisions, no lead changes.

3. **Tests (`src/map/tests/pivotal.rs`)**: 21 focused tests covering tier
   boundaries, direction/alignment matrices, strict lead-change semantics,
   ranking tie-break, fail-closed validation with indices, reproducibility,
   aggregates, ranked filtering, all catalog scenarios, unknown-id failure,
   and Markdown label hygiene.

## Design artifacts

- `_workspace/00_input/m9-pivotal-decision-request-summary.md`
- `_workspace/01_simulation-design-m9-pivotal-decision-detection.md`
- `_workspace/03_domain-qa-m9-pivotal-decision.md` (status: `pass`)

## Verification

- `cargo +1.96.0 fmt --all -- --check` — pass.
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
  — pass.
- `cargo +1.96.0 test --locked` — 434 lib (21 new), 7 binary, 3 doc tests
  pass.

## Documentation reconciled

- `CHANGELOG.md` — `0.1.197` entry.
- `ROADMAP.md` — scope item checked, new evidence section, deferred list and
  baseline version updated, review date refreshed.
- `SPEC.md` — new "Delivered in the bounded pivotal-decision detection
  follow-up" subsection; comeback deferral sentence updated.
- `README.md` — package version and M9 library-contract row updated.

## Evidence limits

This establishes deterministic detection over caller-declared value
trajectories only. Automatic trajectory derivation from authoritative match
history, host/CLI/MCP debrief integration, counterfactual branching from a
pivotal decision, threshold calibration, decision quality, and human-debrief
usefulness remain open and are stated as deferred in the roadmap and spec.

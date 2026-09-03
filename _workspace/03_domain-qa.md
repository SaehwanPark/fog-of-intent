# Domain QA Review: Project Truth Reconciliation, Roster Claim Correction, Ward Vision, Identity ADR

**Milestone:** M9 / M10 boundaries and audit follow-up
**Reviewer:** Domain QA & Verification
**Date:** 2026-08-30
**Branch:** `docs/reconcile-project-truth-20260830` (`1585be6`, `d26b969`, `b0c9060`)

## Status

`pass` for the delivered contract: claims now track evidence, one real
player-facing defect is fixed with projection-level proof, and the open items stay
visible as open. This is **not** validation of balance, enjoyment, accessibility,
human usability, or the product identity itself.

## Reviewed Inputs

- `docs/audit_report_20260828.md` (Priorities 1-4), `README.md`, `ROADMAP.md`,
  `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `HOW_TO_PLAY.md`, `LESSONS.md`,
  `AGENTS.md`, `docs/adr/0004-cargo-workspace-partitioning.md`,
  `docs/adr/0005-product-identity-hybrid.md`
- `crates/foi-map/src/state.rs`, `crates/foi-map/src/complete_match.rs`,
  `crates/foi-map/src/vision.rs`, `crates/foi-map/src/tests/observation.rs`,
  `src/host/match_host.rs`, `src/terminal.rs`, `src/presentation.rs`,
  `src/mcp/server.rs`, `src/mcp/tools.rs`, `src/command_loop.rs`
- `_workspace/00_input/request-summary-20260830-truth-and-5v5.md`
- Verification output in `Verification Evidence` below

## Scope and Roadmap Findings

- Audit Priority 1, 2 (one defect), and 4 are addressed within the existing M9/M10
  slices. No new milestone, crate, dependency, or persistence service was added, and
  no Phase 11/12 work was opened.
- The 5v5 roadmap exit is **not** marked complete. `ROADMAP.md` Phase 9 now states the
  roster the runner actually starts (3 allied actors, 1 opposing actor) and why a larger
  roster would not change the outcome, and Phase 10 states that the delivered
  framework is a study framework and that no human study has run. Deferring the
  actor-presence decision to a human mechanics decision is recorded, not hidden.

## Authority and Information-Boundary Findings

- Boundary respected: the deterministic transition is untouched. `observe_with_wards`
  is a read-side projection in `foi-map`; hosts still own authority and adapters
  consume projections only.
- Ward coverage is paired with its owning `TeamSide` and other-team entries are
  dropped inside the projection, so a caller that holds latent opposing ward positions
  cannot convert them into allied sight. Pinned by
  `crates/foi-map/src/tests/observation.rs::opposing_ward_coverage_never_becomes_allied_sight`.
- Resolution stays inside the projection: `CliMatchHost` passes vision state in rather
  than the terminal or MCP renderer re-deriving visibility, consistent with
  `LESSONS.md` ("keep text renderers downstream of redacted projections").
- Known boundary gap, deliberately left open and now documented in `ROADMAP.md`
  Phase 9 and `HOW_TO_PLAY.md`: the observation prints all 26 structures with exact
  health regardless of sight, and `DataSensitivityLevel` has no structure/health entry.
  A structure-vision projection needs a `(lane, tier)` to `MapLocation` mapping that
  does not exist, i.e. new model surface, so it was not improvised here.

## Determinism, Replay, and Reproducibility Findings

- No randomness, wall-clock, provider, or iteration-order dependence introduced;
  `observe_with_wards` is a pure function of state plus the caller-supplied coverage.
- Run-directory save/load, history, state hashes, and replay verification all pass
  unchanged (`tests/binary_run_dir.rs`, 46 tests).
- Schema identifiers are unchanged: this is a projection input, not a new artifact type.

## Behavior and Playtest Findings

- No agent policy, bounded-rationality parameter, or batch metric changed, so no
  historical AI-playtest numeric claim was reinterpreted.
- The defect found here is the kind the audit warns about: a capability that satisfied
  library tests while never reaching the player. wards changed authoritative vision
  state that no projection consumed; `ward` printed `events=0 effects=0` and an
  opponent directly above a fresh ward stayed `location=unknown`.

## Gameplay and Debrief Findings

- `ward` is now a decision with a visible payoff: spend 25 gold to reveal a sector,
  confirmed live (`actor: id=4 team=opposing location=lane:mid:far-side`) and by a
  host test asserting both the reveal and the non-reveal case.
- `HOW_TO_PLAY.md` no longer promises a stale last-known location, because the
  projection reports only `unknown` or an observed position; `OpponentSighting::LastKnown`
  exists in the model and is still never emitted by the match projection.
- Unchanged friction, accepted for this slice: the structure dump dominates `observe`,
  `siege`/`contest` damage tokens ignore the cost profile, and legitimate no-ops still
  report only `events=0 effects=0` without a reason line.

## Evidence and Claim Limits

- "Implemented", "technically verified", "AI-agent validated", "human validated", and
  "release-ready" are separated in `README.md`; no claim crosses a boundary the
  evidence does not support.
- Identity is recorded as **open** (`docs/adr/0005-product-identity-hybrid.md` is
  `Proposed - owner ratification required`). The repository does not claim a hybrid
  product, and the ADR's rule ("a subsystem that can name neither an audience nor a
  promotion path is not built") is a proposal pending owner ratification.
- No human session, cohort, participant, accessibility accommodation, or study result
  is claimed anywhere in the changed files. `docs/audit_report_20260825.md` is marked
  historical and non-authoritative for current claims.

## Required Fixes

- None blocking for this slice.
- Owner decision required before either roadmap roster exit or the Phase 10
  human-validated label can move: (a) whether objective/siege resolution should depend
  on actor presence, and (b) the product identity.
- Follow-up slice, ordered by player impact: structure projection through vision
  (needs the location mapping and a sensitivity classification), reason lines for
  legitimate no-ops, and cost-profile-aware damage tokens.

## Residual Risks

- The interaction menu still reads "Interactive Multi-Lane Tactical Match Playthrough"
  while a `"5v5"` selection alias remains for input continuity; a player who typed that
  alias before may expect a 5v5. Mitigated by the roadmap and README statements; not
  removed to avoid breaking existing scripts.
- `SPEC.md` M1 still lists migration support as Deferred while the host reports
  version mismatches as errors. Unresolved and visible; building a migration framework
  with no second artifact version in existence would be speculative infrastructure.
- Phase 9 exit criteria still list a 5v5 match. It stays unchecked, so a reader could
  mistake the Phase 9 label for completed scope; the limit section is the mitigation.
- Human playtest remains the M10 stop gate and is untouched by this branch.

## Verification Evidence

- `cargo +1.96.0 fmt --all -- --check` -> exit 0
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings` -> exit 0
- `cargo +1.96.0 test --locked` -> 239 passed, 0 failed (root; includes the 2 new host tests)
- `cargo +1.96.0 test --locked --workspace` -> all targets ok, 0 failures
  (188 root lib, 196 `foi-map` incl. 5 new `tests::observation` tests, 46 binary tests)
- `python scripts/check_repository.py` -> `Repository format, links, currentness, and dependency-free package policy: ok`
- Live: `printf 'ward allied 3 mid_far_side 3\ncommit\nadvance\nobserve\nquit' | fog-of-intent --scenario m9-interactive-match-v1`
  -> `actor: id=4 team=opposing location=lane:mid:far-side` (previously `location=unknown`)
- Live: `fog-of-intent --list-scenarios` (16 entries, interactive match label) and
  `fog-of-intent-mcp --tools` (25 tools, multi-lane match labels, no `5v5` in tool descriptions)

# M2 Codebase Review

## Review Scope and Method

Reviewed the repository instructions, active M2 roadmap and specification,
architecture and compatibility contracts, terminology, all Rust production
modules under `src/`, all lane and kernel tests, repository scripts, recent git
history, public module exports, and the current GitHub PR state. The review
traced authoritative state through observations, validation, resolved inputs,
transition evaluation, event/effect projection, history, replay, branching,
coordination, objectives, and debriefs.

Baseline checks on `main` at `64336f7` all passed:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` — 156 Rust tests
- `python3 scripts/check_repository.py`
- repository Python tests — 7 tests

The package is `0.1.49`, dependency-free, and has no release/tag or open PR
that exposes the M2 lane artifacts.

## Evidence Inventory

- Production: `src/lib.rs`, `src/main.rs`, `src/kernel.rs`,
  `src/serialization.rs`, and every module under `src/lane/` (`branch`,
  `coordination`, `encoding`, `evaluation`, `history`, `intent`, `objective`,
  `observation`, `projection`, `result`, `scenario`, `state`, `transition`,
  `validation`, and `values`).
- Tests: the kernel and serialization groups plus every lane test group
  (`state`, `observation`, `intent`, `transition`, `resources`,
  `coordination`, `history`, `objective`, `scenario`, and `branch`).
- Repository automation: `scripts/check_repository.py`, its nine unit tests,
  CI/toolchain metadata, and dependency policy files.
- Canonical state: `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`,
  `CHANGELOG.md`, `docs/COMPATIBILITY.md`, ADR/terminology sources, and the
  Fog of Intent harness team specification.
- History/public surface: recent commits from the clean `main` base, the
  branch/PR state, `src/lib.rs` exports, and the `crate::lane::*` re-export
  boundary.

The state trace followed `LaneSnapshot` through actor-specific observation,
host validation, resolved inputs, authoritative evaluation, ordered
events/effects, history, replay, branching, coordination, objective review,
scenario composition, and final debrief projection. M1 kernel/codec/fixture
paths were compared independently and remained outside the change set.

## Findings

### Medium — M2 compatibility identities are reused after authoritative changes

Evidence: `src/lane/values.rs:3-8` fixes the ruleset, observation schemas,
replay IDs, and allied profile at v1 while the git history adds many state
fields, hash tags, observations, events, effects, execution inputs, debrief
fields, and record-identity inputs. The same issue appears in the v1 literals in
`src/lane/branch.rs`, `src/lane/scenario.rs`, and `src/lane/objective.rs`.

Trigger: replay or record data produced before and after one of those changes is
interpreted under the same identity. Impact: replay comparability and the
repository's own compatibility policy are no longer trustworthy once M2 data is
treated as an artifact.

Disposition: blocking for the M2 correction. Retire the unsupported internal v1
contract and issue a v2 ruleset/schema/replay/profile identity; add explicit
record replay identity and fail-closed tests. M1 fixtures remain unchanged.

### Medium — Speculative resource surface has become authoritative state

Evidence: `PlayerLaneState`, `PlayerResources`, `LaneExecutionInputs`,
`ResourceExecutionDeltas`, observations, events/effects, errors, debriefs, and
record identity all repeat fields for bounty, level, minion kills, shield, ward,
and sixteen interchangeable consumables (`src/lane/state.rs:37-63`,
`src/lane/transition.rs:11-58`, `512-601`, `1285-1470`, `1494-1550`,
`1951-2273`). The named M2 strategy fixtures and goal use none of those
resources, while `ROADMAP.md:717-721` explicitly defers a general item catalog.

Trigger: adding a single unused counter requires edits across every state,
observation, hash, transition, event, error, debrief, replay, test, and project
document layer. Impact: scope drift, omission risk, and a large accidental
public contract that does not contribute to the current decision loop.

Disposition: blocking for the M2 correction. Retain the scenario's minimum
health/mana/gold/experience/cooldown model and remove unsupported counters until
a scenario contract demonstrates them.

### Medium — Public state constructors allow correlated or weakly valid states

Evidence: `LaneSnapshot` stores `window`, `phase`, and `terminal_outcome` as
independent fields (`src/lane/state.rs:629-641`), and `is_valid_lane_state`
checks their relationship only after construction (`927-933`). `LaneSnapshot`
constructors are public and accept arbitrary combinations. `PlayerLaneState`
also exposes an escalating constructor ladder (`222-390`) created by successive
resource additions.

Trigger: a caller constructs `LanePhase::Open` with a terminal outcome (or a
resolved phase without one), or selects the wrong constructor for a partial
resource state. Impact: invalid states can exist outside the transition
boundary, and every history/validation entry point must defend against them.

Disposition: blocking for the M2 correction. Replace the correlated fields with
`LaneStatus::{Open, Resolved(LaneOutcome)}` and use one aggregate-based player
resource constructor.

### Low — Cooldown ticking narrows arbitrary `u32` beats to `u8`

Evidence: `LaneCooldown::tick` casts `beats as u8` before subtracting
(`src/lane/values.rs:253-255`).

Trigger: `LaneCooldown::new(5).unwrap().tick(256)` returns 5 instead of zero.
The current two-beat window does not trigger it, but the public method claims a
`u32` input.

Disposition: fix adjacent to the v2 type correction and add a boundary test.

### Medium — Zero-delay effects are accepted but are not immediate

Evidence: `LaneDelayedEffect::new` accepts any `u8`, including zero
(`src/lane/state.rs:567-585`). New effects are queued after existing effects
are ticked (`src/lane/transition.rs:2330-2364`), so a zero-delay effect is not
resolved until a later transition.

Trigger: queue `LaneDelayedEffect::new(0, ...)`. Impact: the public delay value
does not match the observed resolution timing and can produce ambiguous causal
debriefs.

Disposition: reject zero with a dedicated non-zero delay value and test one- and
two-beat resolution.

### Medium — Canonical project state is stale while currentness checks pass

Evidence: `README.md:79` reports package `0.1.25`; Cargo reports `0.1.49`.
`SPEC.md:610-620` still lists delayed effects, cooldown, gold, and experience as
unfinished despite delivered sections above. `ARCHITECTURE.md:188-191` says
only immediate effects are emitted and `269-282` lists implemented resources as
absent. `scripts/check_repository.py:72-134` checks milestone alignment but not
package-version or capability consistency.

Trigger: a contributor follows canonical current-state documents after a green
repository check. Impact: scope decisions, compatibility decisions, and future
handoffs are based on false repository state.

Disposition: blocking for the audit slice. Correct the documents and add a
Cargo/README version-consistency guard; preserve historical changelog entries.

## Deferred or Non-Blocking Observations

- FNV-1a state hashes are adequate for the current internal stale-state guard but
  are not an external cryptographic integrity mechanism; revisit before
  externally supported artifacts.
- Public error enums do not yet implement a complete presentation/error trait
  surface; defer to the CLI/application-host slice.
- Exact cumulative test counts in historical specification prose are brittle;
  current verification commands are the authoritative evidence.

## Required Fix Order

1. Correct currentness and durable audit artifacts.
2. Version and simplify the M2 contract while preserving M1.
3. Decompose the transition implementation only after v2 characterization
   coverage exists.

## Evidence Limits

The review establishes code, replay, information-boundary, scope, and document
risks only. It does not establish balance, enjoyment, accessibility, human
trust, or human-like behavior.

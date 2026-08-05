# Changelog

All meaningful contributor- and user-visible changes are recorded here. The
project uses the versioning policy in `README.md`; documentation-only changes do
not increment the package version.

## Unreleased

### Added

- Explicit MIT source license, contributor policy, code of conduct, and
  unofficial/noncommercial project notice with an original-setting fallback and
  conservative distribution boundary.
- Concise design principles, authoritative terminology, and ADR-0001 for the
  host-owned deterministic transition boundary.
- Pinned Rust `1.96.0` toolchain and binary package lockfile, with ADR-0002
  keeping M1 in one Cargo package.
- Minimum artifact/replay compatibility and dependency, security, and license
  policy documents for the pre-implementation-to-M1 boundary.
- Canonical evidence-gated project roadmap with milestone dependencies, exit
  evidence, explicit deferrals, and maintenance rules.
- Lightweight specification and architecture state documents that distinguish
  the current placeholder from planned capabilities.
- Repo-wide `AGENTS.md` guidance and a portable Fog of Intent agent harness for
  simulation design, agent-ecology design, synthesis, and domain QA.
- Deterministic `_workspace/` handoff conventions for substantial work.

### Changed

- M0 is promoted to complete after the hosted clean-checkout CI run passed; the
  first bounded M1 deterministic-kernel fixture is now the active project-state
  slice.
- M1 is promoted to complete after its replay, codec, determinism, and bounded
  invariant evidence passed; the first bounded M2 lane decision-window slice is
  now active.

## 0.1.23 — 2026-08-05

### Added

- A bounded `LaneLevel` player resource abstraction with initial default 1, player and allied observation projections, state/digest hash binding for non-initial level, execution `level_gained` resolution, direct-immediate `LevelGained`/`LevelChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.23` for the bounded level-resource slice; ability point trees and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.22 — 2026-08-05

### Added

- A bounded `LaneBounty` player resource abstraction with zero default, player and allied observation projections, state/digest hash binding for non-zero bounty, execution `bounty_earned` resolution, direct-immediate `BountyEarned`/`BountyChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.22` for the bounded bounty-resource slice; item catalog and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.21 — 2026-08-05

### Added

- A bounded `LaneCooldown` player resource abstraction with zero (ready) default, tick reduction by window beats, player and allied observation projections, state/digest hash binding for non-zero cooldowns, execution `cooldown_set` resolution, direct-immediate `CooldownSet`/`CooldownTicked`/`CooldownChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.21` for the bounded cooldown-resource slice; item catalog and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.20 — 2026-08-05

### Added

- A bounded `LaneExperience` player resource with zero default, player and allied observation projections, state/digest hash binding for non-zero experience, execution experience-gaining resolution, direct-immediate `ExperienceGained`/`ExperienceChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.20` for the bounded experience-resource slice; cooldowns, item catalog, and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.19 — 2026-08-05

### Added

- A bounded `LaneGold` player resource with full/zero compatibility defaults, player and allied observation projections, state/digest hash binding for non-zero gold, execution gold-earning resolution, direct-immediate `GoldEarned`/`GoldChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.19` for the bounded gold-resource slice; cooldowns, experience, item catalog, and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.18 — 2026-08-05

### Added

- A bounded player-facing `Yield` intent in `LanerObservation` and `transition_lane`, resolving deterministically to `NearTower` with zero damage and zero mana spent.
- Yield availability, execution validation, mana-spend rejection, replay, and objective-review tests while preserving existing intent tags and state-hash contracts.

### Changed

- The package version advances to `0.1.18` for the bounded Yield-intent slice; the executable remains the documented placeholder.

## 0.1.17 — 2026-08-04

### Added

- A bounded player-only opponent report: hidden `FarSide` truth projects as a
  current-turn `LastKnown` position while Center/NearTower remain Unknown.
- FarSide report, hidden health/posture, allied uncertainty, and history-replay
  coverage without changing lane state, transition inputs, or hashes.

### Changed

- The package version advances to `0.1.17` for the bounded opponent
  last-known-report slice; complete vision and belief updates remain deferred
  and the executable remains the documented placeholder.

## 0.1.16 — 2026-08-04

### Added

- A bounded `LaneMana` player resource with full-resource compatibility
  defaults, player/allied observation projections, and non-full state/digest
  binding.
- Contest-only explicit mana spending with fail-closed validation, ordered
  `ManaSpent`/`ManaChanged` attribution, debrief recording, and replay tests.
- Mana is included in lane record identity; matched-parent branches apply and
  record an intent-aware normalization when a Contest-only spend crosses to a
  non-Contest alternate.

### Changed

- The package version advances to `0.1.16` for the bounded mana-resource
  slice; cooldowns, gold, experience, regeneration, and abilities remain
  deferred and the executable remains the documented placeholder.

## 0.1.15 — 2026-08-04

### Added

- Explicit `LaneEffectProvenance` relationship/timing labels for emitted lane
  effects: direct-immediate for explicit execution/intent changes and
  indirect-immediate for Contest fallback movement.
- Direct/indirect effect provenance and no-delayed-emission tests while
  retaining existing cause/trace attribution and replay behavior.

### Changed

- The package version advances to `0.1.15` for the bounded effect-provenance
  slice; the executable remains the documented placeholder.

## 0.1.14 — 2026-08-04

### Added

- A bounded `LaneWindow::TwoBeats` duration in the authoritative snapshot,
  actor observations, allied proposal input, and transition turn advancement.
- Automatic close-on-commit and distinct two-beat state hashing with replay
  coverage while preserving the one-beat hash/identity behavior.

### Changed

- The package version advances to `0.1.14` for the bounded variable-duration
  window slice; the executable remains the documented placeholder.

## 0.1.13 — 2026-08-04

### Added

- A conditional player `Withdraw` response authorized only by a current
  RiverSide last-known threat report, with deterministic NearTower movement and
  explicit wave/execution preservation.
- Withdraw availability, unknown/stale/resolved rejection, attribution,
  unfavorable execution, replay, and objective tests while preserving the
  allied two-intent policy boundary.

### Changed

- The package version advances to `0.1.13` for the bounded gank-response slice;
  the executable remains the documented placeholder.

## 0.1.12 — 2026-08-04

### Added

- A bounded player-visible `LastKnown` RiverSide threat report with explicit
  observation-turn provenance while Absent and hidden current InLane truth
  remain Unknown.
- Last-known/unknown boundary and RiverSide replay tests while preserving the
  existing transition, intent, state-hash, and replay contracts.

### Changed

- The package version advances to `0.1.12` for the bounded last-known
  threat-report slice; the executable remains the documented placeholder.

## 0.1.11 — 2026-08-04

### Added

- A bounded player-facing `Recall` intent in the existing one-window lane
  command and transition contract, with explicit NearTower movement, wave and
  execution preservation, and ordinary YieldedSpace/ForcedOut outcomes.
- Recall legality, observation-boundary, attribution, and unfavorable
  execution tests while preserving the allied policy's two-intent candidate
  set and existing replay identities.

### Changed

- The package version advances to `0.1.11` for the bounded Recall-intent
  slice; the executable remains the documented placeholder.

## 0.1.10 — 2026-08-04

### Added

- A committed-facts `m2-two-window-final-debrief-v1` projection with per-window
  intent/coordination/execution/objective summaries, final objective
  aggregation, privileged source provenance, and a redacted visible report.
- Final-debrief replay, incomplete-history, tamper, and provenance-redaction
  tests while retaining all existing M2 window, branch, coordination,
  objective, fixture, and two-window tests.

### Changed

- The package version advances to `0.1.10` for the bounded final-debrief
  slice; the executable remains the documented placeholder.

## 0.1.9 — 2026-08-04

### Added

- A bounded `m2-two-window-scenario-v1` history that composes two existing
  one-beat lane transitions, reopens only a valid resolved first window, and
  stores exact sequence/reopen state for replay.
- Two-window append, terminal-state, invalid-reopen, third-window, and replay
  tamper tests while retaining all existing one-window, branch, coordination,
  objective, and strategy-fixture contracts.

### Changed

- The package version advances to `0.1.9` for the bounded two-window scenario
  slice; the executable remains the documented placeholder.

## 0.1.8 — 2026-08-04

### Added

- Named `HappyPath`, `RiskTaking`, and `Conservative` matched-input strategy
  fixtures that run through the existing host validation, coordination,
  execution, history, and terminal-objective contracts.
- Repeated-run, distinct-outcome, legal-unfavorable, replay, and tampered
  expectation tests for the three diagnostic cases.

### Changed

- The package version advances to `0.1.8` for the one-window strategy-fixture
  slice; the executable remains the documented placeholder.

## 0.1.7 — 2026-08-04

### Added

- A bounded `HoldLaneSpaceThroughWindow` scenario goal with deterministic
  `SpaceHeld`/`SurvivedBeat` criteria, achieved/partial/missed dispositions,
  committed-facts attribution, and a redacted visible objective report.
- Versioned objective input/source-record identities plus ordinary and
  coordinated objective review/replay verification with tamper detection.
- Focused objective, coordination-attribution, state-hash, report-redaction,
  and replay tests while retaining the existing M2 window, branch, and
  coordination fixtures.

### Changed

- The package version advances to `0.1.7` for the one-window scenario-goal and
  terminal-objective slice; the executable remains the documented placeholder.

## 0.1.6 — 2026-08-04

### Added

- A deterministic proposal-only allied actor projection with versioned
  profile/input identities, bounded candidate scores, hidden-state-safe
  observations, and stable proposal identity.
- One host-owned support offer, accept/reject/counter response boundary, five
  explicit coordination follow-through outcomes, coordination-attributed
  events/effects/debrief data, and one-record coordinated replay with tamper
  detection.
- Focused policy, information-boundary, coordination, execution-separation,
  state-hash, and coordinated-history tests while retaining the existing lane
  window and counterfactual branch fixtures.

### Changed

- The package version advances to `0.1.6` for the one-window allied
  proposal/coordination slice; the executable remains the documented
  placeholder.

## 0.1.5 — 2026-08-04

### Added

- A bounded one-window counterfactual branch with immutable parent history,
  matched-parent or explicitly regenerated execution inputs, stable branch
  traces, replay identity, and comparison limits that separate decision from
  execution changes.
- Branch validation, replay, tamper, parent-immutability, and causal-review
  tests while preserving the existing M2 lane transition contract.

### Changed

- The package version advances to `0.1.5` for the bounded branch slice; the
  executable remains the documented placeholder.

## 0.1.4 — 2026-08-04

### Added

- Internal M2 lane decision-window contracts for bounded lane state,
  actor-visible observations, `Stabilize`/`Contest` intent validation,
  explicit execution inputs, attributed events/effects, one-window debriefs,
  and append-only replay.
- Focused information-boundary, unfavorable-execution, validation,
  determinism, stream-isolation, and replay tests for the first lane slice.

### Changed

- The package version advances to `0.1.4` for the first M2 code slice; the
  executable remains the documented placeholder.

## 0.1.3 — 2026-08-04

### Added

- Strict dependency-free `1.0.0` snapshot/history text codecs with explicit
  hash-representation versioning, checked-in M1 fixtures, replay-backed
  deserialization, and fail-closed malformed/tampered-input tests.
- Exhaustive bounded spend/yield tests for energy bounds, conservation, and
  score/yield invariants.

## 0.1.2 — 2026-08-04

### Added

- Initial M1 `fog_of_intent::kernel` fixture with typed state, command
  validation, explicit resolved-input categories, deterministic transitions,
  attributed effects, authoritative hashes, append-only in-memory history, and
  replay verification.

### Changed

- The first M1 transition fixture is implemented and verified as an internal
  library surface; serialization, scenario mechanics, and user-facing adapters
  remain deferred.
- README now presents the project thesis, current pre-implementation status,
  initial vertical slice, canonical documents, and contributor workflow.
- The original proposal roadmap is labeled as a design source; `ROADMAP.md` is
  the canonical execution plan.

## 0.1.1 — 2026-08-04

### Added

- Dependency-free repository currentness/link checker, focused parser tests,
  and a pinned GitHub Actions workflow for clean-checkout verification.

## 0.1.0 — 2026-08-04

### Added

- Initial Rust 2024 binary package.
- Comprehensive project proposal for a turn-based, AI-native team-strategy
  simulation.
- Rust-first technology-stack analysis.

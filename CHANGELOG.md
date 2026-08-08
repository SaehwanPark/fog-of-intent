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
- Reconciled the M2 minimum lane/wave/position/health/resource checklist item
  with the existing bounded v2 implementation; no package version increment or
  runtime change was needed.
- Reconciled the M2 bounded intent/commitment/focus/communication/abort/fallback
  definition with existing v2 request, observation, validation, and replay
  evidence; free-form communication remains deferred.
- Reconciled M2 causal/information evidence for effect provenance, non-binary
  outcomes, hidden-state/report coverage, and complete-replay inspection;
  vision/belief remains deferred; the bounded automatic-advance condition
  contract is now explicit while host scheduling remains deferred.
- Reconciled the M3 terminal-rendering boundary with source evidence: the
  application host remains the sole simulation authority, the pure kernel/lane
  modules evaluate validated inputs, and the current CLI adapter owns no
  terminal I/O, rendering loop, or mutable runtime presentation state; a future
  renderer remains an outer adapter concern.

## 0.1.64 — 2026-08-08

### Added

- Added the versioned `m3-cli-information-labels-v1` vocabulary for
  `observed`, `believed`, `inferred`, `reported`, and `unknown` actor-visible
  information.
- Added generic `CliInformation<T>` values whose `Unknown` form carries no
  payload, with focused tests for canonical names, redaction, borrowing, and
  explicit extraction.

### Known limits

- The labels are a pure adapter contract; terminal rendering, host execution,
  inference, persistence, and human usability evidence remain deferred.

## 0.1.65 — 2026-08-08

### Added

- Added the versioned `m3-cli-precommit-draft-v1` contract with typed local
  staging for message, plan, and contingency edits.
- Added clear-all `CliDraft::undo()` and a consuming `CliCommittedDraft`
  read-only marker; empty payloads and commit/advance staging fail closed.
- Added focused tests for last-write-wins edits, undo isolation, malformed
  staging, and committed-choice readback.

### Known limits

- Drafts remain adapter-local borrowed values; host command execution,
  persistence, transcript acceptance, and authoritative history are deferred.

## 0.1.66 — 2026-08-08

### Added

- Added the versioned `m3-cli-run-id-v1` borrowed identifier contract with
  bounded human-readable syntax and typed malformed-ID errors.
- Applied validated `CliRunId` values to session save/load, in-session replay,
  and top-level replay/export adapter requests with focused mapping tests.

### Known limits

- Run IDs remain adapter syntax only; generation, persistence, uniqueness,
  resume behavior, and human discoverability remain deferred.

## 0.1.70 — 2026-08-08

### Added

- Added the versioned `m3-cli-command-loop-v1` line-oriented stdin/stdout edge
  adapter and wired the binary to the deterministic two-window fixture host.
- The loop renders plain text results and bounded errors, continues after
  malformed commands, and exits cleanly on `quit` or end-of-input.

### Known limits

- The binary remains a deterministic fixture loop without scenario selection,
  persistent storage, branch execution, prompt styling, or human
  keyboard/screen-reader evidence.

## 0.1.69 — 2026-08-08

### Added

- Added the versioned `m3-cli-terminal-text-v1` pure projection for every
  actor-valid host output and bounded host error. It emits stable labeled
  plain text, sanitizes echoed control characters, and performs no terminal
  I/O or hidden-state lookup.

### Known limits

- The projection is library-only; a command loop, terminal integration,
  persistent backend, keyboard/focus inspection, and screen-reader evidence
  remain open.

## 0.1.68 — 2026-08-08

### Added

- Added the dependency-free `m3-cli-host-v1` synchronous host fixture. It
  maps CLI grammar commands to an explicit-input two-window scenario and
  verifies actor-visible observe/history, pre-commit staging and undo,
  in-memory save/load, replay, and debrief projections.

### Known limits

- The host is library-only and deterministic in memory; it does not provide a
  terminal renderer, persistent backend, branch execution, keyboard-only flow,
  or screen-reader evidence.

## 0.1.67 — 2026-08-08

### Added

- Added grammar-level transcript acceptance tests covering a representative
  read/write/process/session sequence and common parser/request errors.

### Known limits

- These tests do not claim a host-backed complete run, save/resume, replay,
  debrief, terminal output, or human keyboard/screen-reader evidence.

## 0.1.63 — 2026-08-08

### Added

- Added repository-wide two-space formatting policy, hard-tab rejection, and
  dependency-free checker tests for Rust, Python, and authored text.
- Added the verified contributor lessons ledger in `LESSONS.md`.

### Changed

- Converted textual lane test inclusions into formatter-visible test modules
  without changing production contracts or test behavior.
- Replaced unchecked numeric casts and data-dependent transition assertions with
  checked bounded operations and typed error paths; Clippy now denies
  `as_conversions`.

### Known limits

- Markdown syntax-sensitive indentation and versioned compatibility fixtures
  remain formatting-policy exceptions; hard tabs remain forbidden.

## 0.1.50 — 2026-08-06

### Changed

- Audited the current M2 implementation and reconciled README, specification,
  architecture, and repository-currentness claims with the verified internal
  kernel and replay fixtures.
- The repository checker now rejects a stale README package version.

### Known limits

- The M2 lane contract remains an internal diagnostic fixture; the complete
  scenario, CLI, MCP, persistence, and human-evidence work remain deferred.

## 0.1.51 — 2026-08-06

### Changed

- Replaced the experimental M2 v1 resource surface with the versioned M2 v2
  contract: retained resources use `LaneResources` and `LaneResourceInputs`,
  lifecycle uses `LaneStatus`, and delayed effects require non-zero `LaneDelay`.
- Retired bounty, level, minion kills, shield, ward, and the sixteen
  experimental consumable slices from state, observations, execution inputs,
  events/effects, debriefs, errors, hashes, and replay identities.
- Versioned current M2 ruleset, observations, replay/profile/strategy fixtures,
  and base transition-record identities. M2 v1 has no migration because it was
  never an external or supported artifact; M1 fixtures and codec remain exact.
- Bound delayed-effect execution inputs into the v2 lane record identity and
  made objective verification reject retired record IDs.
- Updated canonical project-state documents to distinguish current v2 evidence
  from retired v1 history without marking the complete M2 exit criteria done.

## 0.1.62 — 2026-08-07

### Added

- Added typed top-level process commands for `play`, `replay`, `branch`, `experiment`,
  `export`, `validate`, `mcp`, `help`, and `version`.
- Added `CliInteractionMode` (`Guided` default and `Expert`) and `CliVerbosity`
  (`Concise`, `Standard` default, `Explanatory`, `Research`) policies.
- Added `CliPrivilegeLevel` (`Unprivileged` and `Privileged`), enforcing that research
  verbosity and unredacted exports fail closed under standard unprivileged contexts.
- Added pure, dependency-free parsing and validation for top-level arguments and flags.
- Added `CliTopLevelHelpCatalog` and focused top-level command, mode, verbosity, privilege,
  and catalog unit tests.

## 0.1.61 — 2026-08-07

### Added

- Added typed borrowed adapter session requests for `save`, `load`, `undo`, and
  `quit` verbs with run identifier and payload-free boundaries.
- Added focused session-request mapping tests; persistence, save/load execution,
  uncommitted choice editing, and session lifecycle remain outside the adapter.
  Help metadata now identifies these four verbs as session-adapter requests.

## 0.1.60 — 2026-08-07

### Added

- Added typed borrowed adapter process requests for `review`, `debrief`,
  `replay`, and `branch` verbs with optional run and point identifier boundaries.
- Added focused process-request mapping tests; host execution, history inspection,
  and branch derivation remain outside the adapter. Help metadata now identifies
  these four verbs as process-adapter requests.

## 0.1.59 — 2026-08-06

### Added

- Added typed borrowed adapter write requests for `message`, `plan`,
  `contingency`, `commit`, and `advance`, with distinct payload and commitment
  boundaries; empty direct-construction payloads fail closed.
- Added focused write-request mapping tests; domain intent mapping, legality,
  execution, and history mutation remain outside the adapter. Help metadata now
  identifies these five verbs as write-adapter requests.

## 0.1.58 — 2026-08-06

### Added

- Added typed read-only adapter requests for `observe`, bounded `inspect`, and
  contextual `help`, with a static catalog of stable grammar verbs.
- Added actor-visible inspect-target restrictions and read-mapping tests without
  terminal I/O, hidden-state access, or domain mutation.

## 0.1.57 — 2026-08-06

### Added

- Added the dependency-free typed M3 CLI grammar for stable help, observe,
  inspect, planning, review, replay, branch, save/load, undo, and quit verbs.
- Added bounded parse errors and borrowed-payload transcript tests; terminal
  I/O, rendering, and domain authorization remain outside the adapter.

## 0.1.56 — 2026-08-06

### Added

- Added report-derived `LaneBelief<T>` values for unknown, observed, and
  last-known information with an explicit no-decay update rule.
- Added focused opponent/threat report, malformed-pair, and redaction-boundary
  tests without changing observation schemas, authoritative state, or replay
  identities.

## 0.1.55 — 2026-08-06

### Added

- Added typed deterministic `LaneAdvanceCondition` and
  `LaneAdvanceDecision` values for commit-required and no-legal-intent
  evaluation; current one- and two-beat windows remain commit-required.
- Added focused condition-mapping tests without changing authoritative state,
  replay identities, or M1 behavior.

## 0.1.54 — 2026-08-06

### Added

- Retained each delayed lane effect's originating execution trace through
  queueing, ticking, state hashing, branch/history identity, replay,
  resolution event/effect attribution, lane debriefs, and final debrief
  reports.
- Versioned the current internal M2 ruleset, observation, replay, profile,
  strategy, scenario, debrief, and branch identities from v2 to v3; unsupported
  older M2 inputs fail closed while M1 fixtures remain unchanged.
- Added focused origin-trace retention, hash/identity tamper, delayed-resolution
  attribution, and debrief projection tests.

## 0.1.53 — 2026-08-06

### Added

- Added the fixed M2 `LaneActorRoster` and `LaneActorRole` contract for one
  human laner, one opposing laner, one allied autonomous actor, and one
  abstract opposing jungle threat.
- Exposed role identity through player and allied observations while retaining
  hidden opponent/jungle redaction and excluding fixed roster metadata from
  authoritative lane hashes.
- Added focused actor-roster completeness and information-boundary tests.

## 0.1.52 — 2026-08-06

### Changed

- Decomposed the retained M2 transition into private authoritative evaluation
  and ordered event/effect projection modules behind the unchanged `lane`
  facade and v2 contract.
- Added characterization coverage for v2 hashes, replay identity, lifecycle,
  retained resource bounds, delayed effects, observations, branches,
  coordination, scenarios, strategy fixtures, and final debrief replay.

## 0.1.49 — 2026-08-06

### Added

- Added `LanePoultice` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_POULTICE_HASH_TAG` state-hash binding).
- Exposed `self_poultice` in `LanerObservation` and `laner_poultice` in `AlliedLaneObservation`.
- Supported `poultice_gained` and `poultice_spent` resolution during execution with direct-immediate `PoulticeGained`/`PoulticeSpent`/`PoulticeChanged` events and effects, debrief recording, and replay verification.
- Rejection of poultice overflow (`PoulticeOverflow`) or spending without available poultices (`InsufficientPoultice`) before state mutation.

## 0.1.48 — 2026-08-06

### Added

- Added `LaneSalve` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_SALVE_HASH_TAG` state-hash binding).
- Exposed `self_salve` in `LanerObservation` and `laner_salve` in `AlliedLaneObservation`.
- Supported `salve_gained` and `salve_spent` resolution during execution with direct-immediate `SalveGained`/`SalveSpent`/`SalveChanged` events and effects, debrief recording, and replay verification.
- Rejection of salve overflow (`SalveOverflow`) or spending without available salves (`InsufficientSalve`) before state mutation.

## 0.1.47 — 2026-08-06

### Added

- Added `LaneIncense` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_INCENSE_HASH_TAG` state-hash binding).
- Exposed `self_incense` in `LanerObservation` and `laner_incense` in `AlliedLaneObservation`.
- Supported `incense_gained` and `incense_spent` resolution during execution with direct-immediate `IncenseGained`/`IncenseSpent`/`IncenseChanged` events and effects, debrief recording, and replay verification.
- Rejection of incense overflow (`IncenseOverflow`) or spending without available incenses (`InsufficientIncense`) before state mutation.

## 0.1.46 — 2026-08-06

### Added

- Added `LaneFlask` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_FLASK_HASH_TAG` state-hash binding).
- Exposed `self_flask` in `LanerObservation` and `laner_flask` in `AlliedLaneObservation`.
- Supported `flask_gained` and `flask_spent` resolution during execution with direct-immediate `FlaskGained`/`FlaskSpent`/`FlaskChanged` events and effects, debrief recording, and replay verification.
- Rejection of flask overflow (`FlaskOverflow`) or spending without available flasks (`InsufficientFlask`) before state mutation.

## 0.1.45 — 2026-08-06

### Added

- Added `LanePhial` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_PHIAL_HASH_TAG` state-hash binding).
- Exposed `self_phial` in `LanerObservation` and `laner_phial` in `AlliedLaneObservation`.
- Supported `phial_gained` and `phial_spent` resolution during execution with direct-immediate `PhialGained`/`PhialSpent`/`PhialChanged` events and effects, debrief recording, and replay verification.
- Rejection of phial overflow (`PhialOverflow`) or spending without available phials (`InsufficientPhial`) before state mutation.

## 0.1.44 — 2026-08-06

### Added

- Added `LaneAmulet` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_AMULET_HASH_TAG` state-hash binding).
- Exposed `self_amulet` in `LanerObservation` and `laner_amulet` in `AlliedLaneObservation`.
- Supported `amulet_gained` and `amulet_spent` resolution during execution with direct-immediate `AmuletGained`/`AmuletSpent`/`AmuletChanged` events and effects, debrief recording, and replay verification.
- Rejection of amulet overflow (`AmuletOverflow`) or spending without available amulets (`InsufficientAmulet`) before state mutation.

## 0.1.43 — 2026-08-06

### Added

- Added `LaneTalisman` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_TALISMAN_HASH_TAG` state-hash binding).
- Exposed `self_talisman` in `LanerObservation` and `laner_talisman` in `AlliedLaneObservation`.
- Supported `talisman_gained` and `talisman_spent` resolution during execution with direct-immediate `TalismanGained`/`TalismanSpent`/`TalismanChanged` events and effects, debrief recording, and replay verification.
- Rejection of talisman overflow (`TalismanOverflow`) or spending without available talismans (`InsufficientTalisman`) before state mutation.

## 0.1.42 — 2026-08-06

### Added

- Added `LaneSigil` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_SIGIL_HASH_TAG` state-hash binding).
- Exposed `self_sigil` in `LanerObservation` and `laner_sigil` in `AlliedLaneObservation`.
- Supported `sigil_gained` and `sigil_spent` resolution during execution with direct-immediate `SigilGained`/`SigilSpent`/`SigilChanged` events and effects, debrief recording, and replay verification.
- Rejection of sigil overflow (`SigilOverflow`) or spending without available sigils (`InsufficientSigil`) before state mutation.

## 0.1.41 — 2026-08-06

### Added

- Added `LaneRune` bounded player consumable resource abstraction (maximum 5, zero default, `LANE_RUNE_HASH_TAG` state-hash binding).
- Exposed `self_rune` in `LanerObservation` and `laner_rune` in `AlliedLaneObservation`.
- Supported `rune_gained` and `rune_spent` resolution during execution with direct-immediate `RuneGained`/`RuneSpent`/`RuneChanged` events and effects, debrief recording, and replay verification.
- Rejection of rune overflow (`RuneOverflow`) or spending without available runes (`InsufficientRune`) before state mutation.

## 0.1.40 — 2026-08-05

### Added

- Bounded `LaneTome` player consumable resource abstraction (`MAX_LANE_TOME = 5`) with zero default.
- Non-default `LaneTome` state-hash binding (`LANE_TOME_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player tome count (`self_tome`, `laner_tome`).
- `LaneExecutionInputs` support for `tome_gained` and `tome_spent` resolution.
- Direct-immediate `TomeGained`, `TomeSpent`, and `TomeChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::TomeOverflow` and `LaneExecutionError::InsufficientTome` fail-closed error handling.

## 0.1.39 — 2026-08-05

### Added

- Bounded `LaneScroll` player consumable resource abstraction (`MAX_LANE_SCROLL = 5`) with zero default.
- Non-default `LaneScroll` state-hash binding (`LANE_SCROLL_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player scroll count (`self_scroll`, `laner_scroll`).
- `LaneExecutionInputs` support for `scroll_gained` and `scroll_spent` resolution.
- Direct-immediate `ScrollGained`, `ScrollSpent`, and `ScrollChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::ScrollOverflow` and `LaneExecutionError::InsufficientScroll` fail-closed error handling.

## 0.1.38 — 2026-08-05

### Added

- Bounded `LaneCharm` player consumable resource abstraction (`MAX_LANE_CHARM = 5`) with zero default.
- Non-default `LaneCharm` state-hash binding (`LANE_CHARM_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player charm count (`self_charm`, `laner_charm`).
- `LaneExecutionInputs` support for `charm_gained` and `charm_spent` resolution.
- Direct-immediate `CharmGained`, `CharmSpent`, and `CharmChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::CharmOverflow` and `LaneExecutionError::InsufficientCharm` fail-closed error handling.

## 0.1.37 — 2026-08-05

### Added

- Bounded `LaneRelic` player consumable resource abstraction (`MAX_LANE_RELIC = 5`) with zero default.
- Non-default `LaneRelic` state-hash binding (`LANE_RELIC_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player relic count (`self_relic`, `laner_relic`).
- `LaneExecutionInputs` support for `relic_gained` and `relic_spent` resolution.
- Direct-immediate `RelicGained`, `RelicSpent`, and `RelicChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- `LaneExecutionError::RelicOverflow` and `LaneExecutionError::InsufficientRelic` fail-closed error handling.

## 0.1.36 — 2026-08-05

### Added

- Bounded `LaneTrinket` player consumable resource abstraction (`MAX_LANE_TRINKET = 5`) with zero default.
- Non-default `LaneTrinket` state-hash binding (`LANE_TRINKET_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player trinket count (`self_trinket`, `laner_trinket`).
- `LaneExecutionInputs` support for `trinket_gained` and `trinket_spent` resolution.
- Direct-immediate `TrinketGained`, `TrinketSpent`, and `TrinketChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `TrinketOverflow` and `InsufficientTrinket`.

## 0.1.35 — 2026-08-05

### Added

- Bounded `LaneElixir` player consumable resource abstraction (`MAX_LANE_ELIXIR = 5`) with zero default.
- Non-default `LaneElixir` state-hash binding (`LANE_ELIXIR_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player elixir count (`self_elixir`, `laner_elixir`).
- `LaneExecutionInputs` support for `elixir_gained` and `elixir_spent` resolution.
- Direct-immediate `ElixirGained`, `ElixirSpent`, and `ElixirChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `ElixirOverflow` and `InsufficientElixir`.

## 0.1.34 — 2026-08-05

### Added

- Bounded `LanePotion` player consumable resource abstraction (`MAX_LANE_POTION = 5`) with zero default.
- Non-default `LanePotion` state-hash binding (`LANE_POTION_HASH_TAG`).
- `LanerObservation` and `AlliedLaneObservation` exposure of player potion count (`self_potion`, `laner_potion`).
- `LaneExecutionInputs` support for `potion_gained` and `potion_spent` resolution.
- Direct-immediate `PotionGained`, `PotionSpent`, and `PotionChanged` events and effects during transition evaluation, debrief recording, and `LaneRecordIdentity` integration.
- Execution validation error handling for `PotionOverflow` and `InsufficientPotion`.

## 0.1.33 — 2026-08-05

### Added

- Bounded `LaneFallbackBehavior` player intent fallback abstraction (`MaintainPlan`, `RetreatToTower`, `SafeFarm`, `ConserveResources`) with `MaintainPlan` default.
- Non-default `LaneFallbackBehavior` state-hash binding (`LANE_FALLBACK_BEHAVIOR_HASH_TAG`).
- `LanerObservation` advertising of available fallback behaviors.
- Request/command integration with `fallback_behavior` getters and constructors while preserving existing constructors.
- Direct-immediate `FallbackBehaviorSelected`, `FallbackBehaviorSet`, and `FallbackBehaviorTriggered` events and effects during transition evaluation, debrief recording, and replay verification.

## 0.1.32 — 2026-08-05

### Added

- Bounded `LaneAbortCondition` player intent abort condition abstraction (`None`, `HealthThreshold`, `ThreatSpotted`, `ResourceDepleted`) with `None` default.
- Non-default `LaneAbortCondition` state-hash binding (`LANE_ABORT_CONDITION_HASH_TAG`).
- `LanerObservation` advertising of available abort conditions.
- Request/command integration with `abort_condition` getters and constructors while preserving existing constructors.
- Direct-immediate `AbortConditionSelected`, `AbortConditionSet`, and `AbortConditionTriggered` events and effects during transition evaluation, debrief recording, and replay verification.

## 0.1.31 — 2026-08-05

### Added

- Bounded `LanePingSignal` player intent communication signal abstraction (`None`, `Danger`, `OnMyWay`, `Assist`, `EnemyMissing`) with `None` default.
- Non-default `LanePingSignal` state-hash binding (`LANE_PING_SIGNAL_HASH_TAG`).
- `LanerObservation` advertising of available ping signals.
- Request/command integration with `ping_signal` getters and constructors while preserving existing constructors.
- Direct-immediate `PingSignalSelected` and `PingSignalSet` events and effects during transition resolution, debrief recording, and replay verification.

## 0.1.30 — 2026-08-05

### Added

- Bounded `LaneWard` player vision resource abstraction `[0, MAX_LANE_WARD=5]` with zero default.
- Non-zero `LaneWard` state-hash binding (`LANE_WARD_HASH_TAG`).
- Player (`self_ward`) and allied (`laner_ward`) observation projections without exposing opponent ward count.
- Resolution of explicit `ward_gained` execution inputs emitting direct-immediate `WardGained`/`WardChanged` events & effects, debrief recording, and replay verification.

## 0.1.29 — 2026-08-05

### Added

- A bounded `LaneShield` player defensive shield resource with zero default and `LANE_SHIELD_HASH_TAG` state-hash binding.
- `LanerObservation` and `AlliedLaneObservation` exposure for player shield (`self_shield`, `laner_shield`) while hiding opponent shield.
- `LaneExecutionInputs` support for explicit `shield_gained` resolution during execution with direct-immediate `ShieldGained`/`ShieldChanged` events and effects, debrief recording (`shield_gained`), and `LaneRecordIdentity` integration.
- `LaneExecutionError::ShieldOverflow` error when gaining shield beyond `MAX_LANE_SHIELD` (50).

### Changed

- The package version advances to `0.1.29` for the bounded shield-resource slice; complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.28 — 2026-08-05

### Added

- A bounded `LaneDelayedEffects` player delayed-effect queue abstraction (maximum 4 items) with `LANE_DELAYED_EFFECT_HASH_TAG` state-hash binding.
- `LaneExecutionInputs` support for `delayed_effect` resolution; queued effects tick on each transition beat and resolve when delay expires (health regen, mana regen, cooldown reduction).
- Direct/indirect `Delayed` provenance for resolved effects, `DelayedEffectQueued` and `DelayedEffectResolved` events and effects, debrief recording (`delayed_effects_queued`, `delayed_effects_resolved`), and replay verification through `LaneScenarioHistory`.
- `LaneExecutionError::DelayedEffectOverflow` error when queuing past maximum capacity.

### Changed

- The package version advances to `0.1.28` for the bounded delayed-effect slice; complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.27 — 2026-08-05

### Added

- A bounded `LaneCommitment` player intent commitment abstraction with default `Standard`, explicit `Cautious` and `Aggressive` commitment options, observation advertising, request/command integration, state/record identity hash binding for non-default commitment, direct-immediate `CommitmentSelected`/`CommitmentSet` events and effects, debrief recording, and replay verification.

### Changed

- The package version advances to `0.1.27` for the bounded intent-commitment slice; commitment-based stat scaling and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.26 — 2026-08-05

### Added

- A bounded `LaneTargetFocus` player intent focus abstraction with default `Minions`, explicit `OpposingLaner` and `Tower` focus options, observation advertising, request/command integration, state/record identity hash binding for non-default target focus, direct-immediate `TargetFocusSelected`/`TargetFocusSet` events and effects, debrief recording, and replay verification.

### Changed

- The package version advances to `0.1.26` for the bounded intent-focus slice; multi-actor execution resolution and complete scenario mechanics remain deferred and the executable remains the documented placeholder.

## 0.1.25 — 2026-08-05

### Changed

- Split the internal lane implementation and tests into responsibility-oriented
  private modules behind the unchanged `crate::lane::*` facade, and clarified
  resource and transition data flow with private product types without
  changing hashes, events, errors, replay behavior, or the placeholder binary.

## 0.1.24 — 2026-08-05

### Added

- A bounded `LaneMinionKills` player resource abstraction with zero default, player and allied observation projections, state/digest hash binding for non-zero minion kills, execution `minion_kills_gained` resolution, direct-immediate `MinionKillsGained`/`MinionKillsChanged` events and effects, debrief recording, replay, and overflow error handling.

### Changed

- The package version advances to `0.1.24` for the bounded minion-kills-resource slice; minion wave spawn timing and last-hitting mechanics remain deferred and the executable remains the documented placeholder.

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

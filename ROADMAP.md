# Fog of Intent Project Roadmap

**Document role:** Canonical milestone order, scope, and promotion gates
**Status:** Active
**Current milestone:** M2 — One-Lane Vertical Slice
**Last reviewed:** 2026-08-16

This document is the authoritative execution roadmap. The project proposal
explains the broader vision and preserves the original roadmap concept; when its
sequencing or checklist differs from this file, this file governs current work.

## How to Use This Roadmap

- Milestones are evidence-gated, not date-gated. A checklist describes work, but
  only the named exit evidence promotes a milestone.
- A milestone authorizes nothing by itself. Work begins from a bounded user or
  contributor request and should select the smallest dependency-complete slice.
- Status values are `Planned`, `Active`, `Blocked`, or `Complete`. `Complete`
  requires repository evidence and a corresponding update to `SPEC.md`.
- Planned architecture and tooling are not shipped capabilities. `ARCHITECTURE.md`
  records what exists now and labels target boundaries separately.
- Technical progress may use automated and AI-agent evidence. Claims about human
  enjoyment, lived accessibility, trust, learning, or external behavioral
  validity require direct human evidence.
- Intellectual-property, licensing, and distribution checks are release gates;
  engineering progress does not imply public-release readiness.

## Current Baseline

| Surface | Current evidence | State |
| --- | --- | --- |
| Product direction | `docs/project-proposal.md` | Defined at proposal level |
| Technology direction | `docs/tech-stack-consideration.md` | Proposed, not adopted except Rust 2024 |
| Executable | `src/main.rs`, `src/command_loop.rs`, `src/presentation.rs`, `src/repl.rs` | Standalone package version reporting, a bounded fixture transcript with `--scenario m3-two-window-fixture-v1` (optional `--run-dir`, TTY prompt/completion, `--color`), a replay-verified complete-match transcript with `--scenario m9-complete-match-replay-v1`, a verified actor-visible HTML5 presentation document with `--scenario m11-gui-presentation-v1`, and a public alpha release readiness audit report with `--scenario m12-alpha-release-checks-v1` |
| Package | `Cargo.toml` | Version `0.1.219`, one deferred edge crate (`reedline`) |
| Canonical execution plan | `ROADMAP.md` | Active |
| Project-state docs | `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md` | Initialized |
| Agent workflow | `AGENTS.md`, `.agents/skills/`, `docs/harness/` | Initialized |
| Internal kernel/replay fixture | `src/kernel.rs`, `src/serialization.rs` | M1 complete; not playable |
| Scenario, CLI, MCP, research, GUI | Grammar, bounded host, pure text, fixture loop, injected file artifacts, explicit binary store wiring, and one-fixture scenario selection; broader scenario selection remains open | Not implemented as complete user-facing flows |

## Milestone Map

| Milestone | Outcome | Status | Required predecessor |
| --- | --- | --- | --- |
| M0 | Governed repository baseline | Complete | Repository inception |
| M1 | Replayable deterministic kernel | Complete | M0 |
| M2 | First complete one-lane scenario | Active | M1 |
| M3 | CLI reference experience | Planned | M2 |
| M4 | Interpretable bounded-agent population | Planned | M2; preferably M3 |
| M5 | Model-agnostic MCP play | Planned | M3 and stable actor contracts |
| M6 | Automated behavioral validation | Planned | M4 and M5 |
| M7 | Semantic-to-parametric calibration proof | Planned | M6 |
| M8 | Coordinated team decision play | Planned | M4 and M5 |
| M9 | Bounded multi-lane match prototype | Planned | M8 |
| M10 | Human-usable and accessibility-tested alpha | Planned (library complete) | Stable M9 candidate; informal checks start earlier |
| M11 | Optional shared-boundary GUI | Planned and optional | Demonstrated presentation need; stable host contracts |
| M12 | Public research-capable alpha | Planned | M10; M11 only if adopted |

Critical path:

```text
M0 -> M1 -> M2 -> M3 -> M5
             \-> M4 -> M6 -> M7
                   \-> M8 -> M9 -> M10 -> [M11] -> M12
```

M6 depends on both the baseline agent ecology and an external-agent interface.
M8 may begin from M4 behavior contracts, but its complete evidence requires the
stable actor and communication boundaries established by M5. M11 is optional
and must not delay CLI/MCP validation unless a user-facing need justifies it.

## Cross-Cutting Gates

Every milestone applies the relevant gates below in addition to its local exit
evidence.

### Scope and product coherence

- Keep the one-lane vertical slice authoritative until it demonstrates a
  complete, understandable decision loop.
- Require a concrete use case before adding a general framework, service,
  database, scripting runtime, GUI layer, or deployment surface.
- Preserve multiple defensible strategies; do not encode a hidden preferred
  route as the only viable path.

### Simulation authority and reproducibility

- The host owns true state, legality, ordering, resolved stochastic inputs,
  transition, history, replay, branching, and debrief generation.
- Identical prior state, validated commands, resolved inputs, and ruleset must
  produce identical events, effects, next state, and hash.
- Policy, observation, communication, coordination, execution, and environment
  uncertainty use explicit, versioned boundaries.
- Runtime logs never substitute for committed simulation history.

### Information and interface boundaries

- True state, actor belief, observation, reported state, and research inspection
  are distinct types or contracts.
- CLI, MCP, research, and any future GUI consume the same host-owned actions and
  actor-visible projections.
- Presentation and adapter layers do not reimplement legality, transitions,
  persistence, or hidden-state inference.

### Evidence and claim limits

- Software tests establish software properties only.
- Agent playtests may establish protocol usability, reproducibility, behavioral
  differences, exploit evidence, and strategy diversity within declared models.
- Human usability, accessibility, enjoyment, trust, and behavioral-validity
  claims require evidence from people relevant to the claim.
- AI behavior is a reference policy or experimental condition, not human ground
  truth.

### Documentation and compatibility

- `SPEC.md` distinguishes verified past, active work, and deferred future.
- `ARCHITECTURE.md` changes when ownership, data flow, dependencies, persistence,
  or a consequential invariant changes.
- `CHANGELOG.md` records contributor- or user-visible changes.
- Versioned schemas, rulesets, scenarios, prompts, profiles, and replay fixtures
  acquire explicit compatibility policies before external use.

## Phase 0 — Governed Repository Baseline

**Milestone:** M0
**Status:** Complete
**Depends on:** Repository inception

### Outcome

The repository has a clear product identity, canonical planning and state
documents, explicit authority and claim boundaries, repeatable local checks,
and enough governance to begin the deterministic kernel without ambiguity.

### Completed foundation

- [x] Create the Rust 2024 repository and `0.1.0` package scaffold.
- [x] Record the comprehensive product proposal.
- [x] Record a Rust-first technology analysis.
- [x] Establish this canonical roadmap.
- [x] Initialize `SPEC.md`, `ARCHITECTURE.md`, and `CHANGELOG.md`.
- [x] Define repo-wide agent guidance and a domain-oriented harness.
- [x] Define documentation-only version exclusions.

### Remaining scope

- [x] Select and add an explicit source license.
- [x] Add a contribution policy and code of conduct appropriate to the project.
- [x] Add a clear unofficial/noncommercial fan-project notice.
- [x] Document the original-setting fallback and content-isolation strategy.
- [x] Define the distribution boundary before publishing game content or assets.
- [x] Create a concise `DESIGN_PRINCIPLES.md` if the implementation needs a
  stable index separate from the proposal.
- [x] Create the first architecture decision record for deterministic authority,
  resolved inputs, and adapter boundaries.
- [x] Define authoritative terminology for state, belief, observation, report,
  intent, command, event, effect, and execution.
- [x] Decide whether to keep a single package or adopt a Cargo workspace for M1.
- [x] Pin the Rust toolchain and commit the binary package lockfile.
- [x] Establish formatting, lint, test, documentation-link, and currentness
  checks in CI.
- [x] Document dependency, security-advisory, and license-policy checks.
- [x] Define scenario, ruleset, schema, and replay compatibility conventions at
  the minimum depth M1 requires.

### Deliverables

- Canonical root documents and project proposal links.
- License, notices, and contribution documents.
- Initial ADR and terminology reference.
- Pinned toolchain, lockfile, and continuous-integration workflow.
- Passing local and hosted repository checks.

### Exit evidence

- All canonical documentation links resolve.
- Formatting, lint, and test commands pass in CI from a clean checkout.
- The initial ADR identifies one authoritative transition owner and forbids
  adapters from owning simulation truth.
- License and fan-project notices state what contributors and users may do.
- `SPEC.md` moves M0 to `Past` and identifies one bounded M1 slice.

### Explicit deferrals

- No game mechanics beyond what is needed to define M1 contracts.
- No MCP server, model-provider adapter, database, Python package, or GUI.
- No claim of public-release, legal, accessibility, or research readiness.

## Phase 1 — Deterministic Simulation Kernel

**Milestone:** M1
**Status:** Complete
**Depends on:** M0

### Initial bounded slice

Begin M1 with one internal kernel fixture: a typed world state with one bounded
resource, one host-validated command, explicit resolved inputs, one deterministic
transition, attributed event/effect output, an authoritative state hash, and an
append-only replay verification path. Keep the fixture small enough to prove the
boundary before adding lane mechanics, serialization formats, or adapters.

### Outcome

A small typed kernel resolves a scripted command sequence reproducibly, records
events and attributed effects, and verifies an append-only replay without I/O,
async, terminal, database, or model-provider dependencies in the core.

### Scope

- [x] Define stable identifiers, bounded numeric types, and normalized values
  for equality-sensitive state.
- [x] Define immutable `WorldState` and the minimum actor state required by the
  first fixture.
- [x] Define `Command`, `ValidatedCommand`, `ResolvedInputs`, `Event`, `Effect`,
  `TransitionResult`, and typed errors.
- [x] Define the ruleset identifier and version contract.
- [x] Implement command validation separately from modeled unfavorable outcomes.
- [x] Implement the pure or functionally pure transition boundary.
- [x] Define stable random-stream and draw identities without generating random
  values inside the transition.
- [x] Separate environment, observation, policy, coordination, and execution
  input categories even if the first fixture uses only a subset.
- [x] Implement append-only committed history records.
- [x] Implement deterministic state hashing over declared authoritative fields.
- [x] Implement snapshot and history serialization with versioned fixtures.
- [x] Implement replay verification from the initial state and committed inputs.
- [x] Add example-based transition tests.
- [x] Add invariant and exhaustive property-style tests for bounds and
  conservation rules.
- [x] Add malformed-command, duplicate-command, ordering, and version-mismatch
  tests.
- [x] Add repeated-run and random-stream-isolation tests.

### Deliverables

- Kernel modules and public domain types.
- One tiny scripted fixture with explicit resolved inputs.
- Versioned snapshot and history fixtures.
- State-hash and replay verifier.
- ADRs for any boundary that differs materially from Phase 0 assumptions.

### Exit evidence

- Repeated runs produce byte- or schema-equivalent events, effects, state, and
  hashes for identical inputs.
- Replay reconstructs and verifies the committed terminal state.
- Adding an unrelated input stream does not shift existing draw identities.
- Core dependency review confirms no I/O, async runtime, wall clock, terminal,
  database, MCP, model-provider, or hidden RNG dependency.
- Tests cover at least one invalid command and one legal but unfavorable outcome.

### Current slice evidence

- The internal `fog_of_intent::kernel` library fixture implements the checked
  items above without adding a dependency or changing binary behavior.
- Nineteen focused Rust tests pass: thirteen kernel tests cover validation,
  transitions, bounds, conservation, replay, ordering, repeated runs, and
  stream isolation; six codec tests cover fixtures, round trips, unsupported
  rulesets, and rejection.
- The checked-in `m1_snapshot_v1.txt` and `m1_history_v1.txt` fixtures use the
  explicit `1.0.0` schema and `fnv1a64-le-v1` hash representation.

### Promotion evidence

- The M1 checklist is complete and its exit evidence is recorded above.
- The locked Rust 1.96.0 format, clippy, test, repository-currentness, and
  focused checker suites pass on the merged `0.1.3` implementation.
- `SPEC.md` records M1 as past and identifies the bounded M2 slice now active.

### Explicit deferrals

- No complete lane model or promise of enjoyable play.
- No interactive shell, save browser, MCP transport, or batch experiment runner.
- No general entity-component system or arbitrary scenario scripting.

## Phase 2 — One-Lane Vertical Slice

**Milestone:** M2
**Status:** Active
**Depends on:** M1

### Initial bounded slice

Start with one deterministic lane decision window rather than the full
scenario: a typed lane snapshot, a host-owned observation for the human laner,
one legal intent command, explicit resolved execution input, and a replayable
transition that records a visible outcome without exposing latent opponent
state. Hold and pressure remain planned follow-up actions until this
boundary is verified.

### Outcome

A human can complete one short, text-first lane scenario with meaningful intent,
uncertainty, delegated execution, an allied proposal, a terminal objective, and
a causal debrief.

### Scope

- [x] Choose one scenario goal and simulated duration, such as surviving a weak
  lane, preparing a gank, or recalling with limited loss.
- [x] Define one human-controlled laner, one opposing laner, one allied autonomous
  actor, and one abstract opposing jungle threat.
- [x] Define the minimum lane, wave, position, health, mana, cooldown, gold, and
  experience abstractions needed by the scenario.
- [x] Define vision, last-known information, report-derived belief updates,
  unknowns, and report semantics without exposing latent values; map geometry,
  decay, and threat execution remain deferred.
- [x] Define variable-duration decision windows and automatic-advance conditions
  as explicit duration/condition values; host integration for a genuine
  no-choice automatic path remains deferred.
- [x] Define intent, commitment, target/focus, bounded communication, abort
  conditions, and fallback behavior.
- [x] Implement hold, pressure/trade, yield, recall, and gank-response decisions
  only where they create real tradeoffs.
- [x] Implement coordination and execution as distinct resolutions.
- [x] Record direct, indirect, immediate, and delayed effects with provenance
  through the bounded delayed-effect queue, replay, attribution, and debrief
  projections.
- [x] Define a terminal outcome that does not collapse evaluation to win/loss.
- [x] Produce immediate review and final debrief projections.
- [x] Support replay and a bounded counterfactual branch at a pivotal decision.
- [x] Add scripted happy-path, risk-taking, and conservative-strategy fixtures.
- [x] Test hidden-state leakage and actor-visible report completeness.
- [x] Inspect every transition in at least one complete replay manually.

> The bounded-slice, branch, coordination, objective, strategy, scenario,
> intent, report, window, effect, and retained-resource evidence sections below
> preserve the experimental M2 v1 history. The version corrections near the end of
> this evidence block is the current internal contract; the older entries are
> not release or M2-exit claims.

### Historical M2 v1 bounded slice evidence (retired)

- [x] Define one typed lane snapshot with bounded health, wave pressure,
  position, phase, hidden opponent truth, and hidden jungle-threat truth.
- [x] Project a player-laner observation with explicit unknown reports and no
  latent opponent values; the bounded RiverSide threat case is represented as
  an explicit last-known report while hidden current InLane truth remains
  unknown.
- [x] Validate `Stabilize` and `Contest` through one host-created intent
  command, including actor, turn, ruleset, observation, and prior-hash guards.
- [x] Resolve one deterministic window from explicit execution damage and wave
  inputs, with ordered events, attributed effects, outcome, and hash.
- [x] Commit and replay one append-only lane history record while preserving
  the M1 fixture contract.
- [x] Cover legal unfavorable execution, malformed inputs, determinism, stream
  isolation, hidden-state omission, validation rejection, and replay in nine
  focused lane tests.

This evidence promotes only the bounded M2 diagnostic slice. The unchecked
scope items remain required for the complete one-lane milestone.

### Historical M2 v1 bounded branch evidence (retired)

- [x] Branch only from the verified record-0 prefix before the original window,
  preserving the parent history and its replay result.
- [x] Support exact matched-parent execution inputs and explicitly regenerated
  execution inputs with stable branch-scoped stream identities.
- [x] Record versioned branch identity and bounded comparison metadata without
  adding branch fields to authoritative lane state or its hash.
- [x] Reject same-intent, wrong-actor, stale-observation, invalid-identity,
  malformed-parent, and tampered-branch inputs before accepting a result.
- [x] Cover matched/regenerated replay, parent immutability, attribution
  limits, hidden-state boundaries, and branch tamper detection.

### Historical M2 v1 allied proposal and coordination evidence (retired)

- [x] Project one proposal-only allied actor from an actor-valid observation
  with explicit unknown opponent/threat reports and no true-state hash.
- [x] Bind the scripted policy to versioned profile/input identities and
  deterministic candidate scores, selection, and proposal identity.
- [x] Present one typed support offer and validate accept, reject, or one
  bounded counter against the existing player intent request.
- [x] Resolve five closed response/follow-through outcomes from explicit host
  coordination input without allowing policy output to mutate lane state.
- [x] Compose coordination events/effects/debrief data around one unchanged
  `transition_lane` result and preserve the authoritative state hash.
- [x] Append and replay one coordinated record with tamper detection while
  retaining the old `LaneHistory` and `LaneBranch` contracts.

This evidence establishes a deterministic, modeled coordination boundary for
one window. It does not establish communication quality, trust, balance,
optimality, human behavior, or a playable multi-window scenario.

### Historical M2 v1 scenario-goal and terminal-objective evidence (retired)

- [x] Define the bounded `HoldLaneSpaceThroughWindow` scenario goal without
  changing authoritative lane state or mechanics.
- [x] Evaluate `SpaceHeld` and `SurvivedBeat` from committed visible result
  facts with explicit achieved/partial/missed dispositions.
- [x] Preserve ordinary `NotApplicable` coordination and coordinated
  dispositions as separate causal attribution facts.
- [x] Record versioned objective input identity and a source-record identity;
  replay reconstructs objective facts and rejects tampered reviews.
- [x] Provide a visible objective report that omits source-state hashes and
  private receipts while retaining a committed-facts attribution limit.

This evidence establishes one deterministic terminal-objective projection. It
does not establish a complete scenario, optimality, balance, or human
experience.

### Historical M2 v1 strategy-fixture evidence (retired)

- [x] Define named `HappyPath`, `RiskTaking`, and `Conservative` matched-input
  bundles over the existing observation, coordination, execution, and
  objective contracts.
- [x] Run each fixture through host validation and one coordinated history
  append; expected outcomes remain checks rather than hidden transition rules.
- [x] Verify repeated-run equality, distinct modeled outcomes, legal-unfavorable
  risk-taking behavior, objective replay, and tampered expectation rejection.

This evidence establishes three deterministic modeled cases for one window. It
does not establish strategy quality, balance, optimality, or human preference.

### Historical M2 v1 bounded two-window scenario evidence (retired)

- [x] Compose two sequential one-beat lane records under a versioned scenario
  replay identity without changing the existing transition or branch IDs.
- [x] Reopen only a valid resolved first-window state through an explicit
  deterministic host boundary that preserves domain values and clears only
  per-window phase/outcome status.
- [x] Store exact start/reopened states and complete base records; reject a
  third append, invalid reopen, resolved-window action, and tampered replay.
- [x] Reach and replay a resolved two-window terminal state while preserving
  the first-window objective and prior one-window/coordination/fixture tests.

This evidence establishes a bounded two-window composition. It does not
establish variable pacing, gank response, communication, or a complete playable
lane scenario.

### Historical M2 v1 bounded Recall-intent evidence (retired)

- [x] Advertise `Recall` only in the player-laner observation while preserving
  the allied policy's two-candidate `Stabilize`/`Contest` contract.
- [x] Validate Recall only when the current actor-visible observation advertises
  it; omitted, stale, and resolved-window requests remain rejected.
- [x] Resolve Recall deterministically to `NearTower` with explicit wave and
  execution inputs, intent-attributed position effects, and ordinary
  `YieldedSpace`/`ForcedOut` outcomes.
- [x] Preserve existing record identities, replay, branch, objective, and final
  debrief paths; add focused legality, observation-boundary, attribution, and
  unfavorable-execution tests.

This evidence establishes one bounded Recall plan. It does not establish recall
timing, resource restoration, variable pacing, gank response, communication,
strategy quality, balance, or a complete playable lane scenario.

### Historical M2 v1 bounded last-known threat-report evidence (retired)

- [x] Project only the bounded `RiverSide` threat case as
  `LastKnown { region, last_seen_turn }` in the player observation.
- [x] Keep `Absent` and current hidden `InLane` truth as `Unknown`, with no
  source-state hash, exact entity, execution result, or current-location claim.
- [x] Regenerate and replay a RiverSide observation through the existing
  history/transition authority without changing lane state, transition output,
  state hashes, or replay identities.
- [x] Preserve the existing player intent set and allied policy artifact while
  covering the unknown/last-known information boundary in focused tests.

This evidence establishes one bounded last-known threat report. It does not
establish complete vision, belief updates, gank response, variable pacing,
communication, strategy quality, balance, or a complete playable lane scenario.

### Historical M2 v1 bounded gank-response evidence (retired)

- [x] Advertise conditional `Withdraw` only when the current player observation
  carries `LastKnown { region: RiverSide, ... }`; Unknown threat reports do not
  authorize it.
- [x] Resolve Withdraw through the existing one-beat transition to `NearTower`,
  preserving explicit wave/damage/trace inputs and intent attribution without
  activating Contest fallback.
- [x] Reject stale, resolved-window, wrong-actor, malformed, and unsupported
  Withdraw commands before transition evaluation.
- [x] Replay Withdraw history and objective attribution while preserving the
  allied policy's two-candidate Stabilize/Contest artifact and hidden current
  InLane truth boundary.

This evidence establishes one conditional Withdraw response. It does not
establish automatic threat damage, complete vision/belief updates, variable
pacing, communication, strategy quality, balance, or a complete playable lane
scenario.

### Historical M2 v1 bounded variable-duration-window evidence (retired)

- [x] Add `LaneWindow::TwoBeats` as explicit snapshot state while retaining
  `OneBeat` as the compatibility default.
- [x] Propagate the duration to player/allied observations and bind the allied
  visible digest for TwoBeats without changing its two-candidate policy.
- [x] Advance a committed TwoBeats transition by exactly two turns and close
  it automatically at the existing resolved transition boundary.
- [x] Keep one-beat hashes/identities stable, make TwoBeats hashes distinct,
  and replay a two-beat history to the same resolved state.

This evidence establishes one bounded TwoBeats duration. It does not establish
adaptive pacing, a manual tick command, automatic execution outcomes,
communication, strategy quality, balance, or a complete playable lane
scenario.

### Historical M2 v1 effect-provenance evidence (retired)

- [x] Label all currently emitted lane effects as direct or indirect without
  replacing their existing cause/trace attribution.
- [x] Label current effects as immediate and declare delayed timing without
  emitting or storing delayed effects.
- [x] Mark explicit health, wave, and intent-position changes direct/immediate;
  mark Contest fallback movement indirect/immediate.
- [x] Verify explicit and fallback mappings, no delayed emission, and replay
  preservation without changing lane-state hashes or transition authority.

This historical evidence establishes provenance labels for the retired
immediate-effects slice only. It does not establish delayed effects, causal
completeness, adaptive pacing, automatic execution outcomes, communication,
strategy quality, balance, or a complete playable lane scenario.

### Historical M2 v1 bounded mana-resource evidence (retired)

- [x] Add bounded `LaneMana` to the player-laner state with a full-resource
  default and a tagged non-full state-hash representation.
- [x] Project player mana to the player observation and team-visible mana to the
  allied observation without adding opponent resource truth.
- [x] Permit only Contest execution to spend explicit resolved mana; reject
  wrong-intent and insufficient-resource inputs before state mutation.
- [x] Emit ordered `ManaSpent`/`ManaChanged` direct-immediate attribution,
  record the spend in debrief data, and replay the reduced resource exactly.
- [x] Bind non-full mana to the allied visible digest and cover bounds,
  information projection, malformed inputs, hash distinction, lane identity,
  intent-aware matched branching, and replay.

This evidence establishes one bounded mana resource and one Contest spend path.
It does not establish cooldowns, gold, experience, regeneration, abilities,
resource economy balance, communication, or a complete playable lane scenario.

### Historical M2 v1 bounded opponent last-known-report evidence (retired)

- [x] Project only hidden opponent `FarSide` as a player-facing
  `LastKnown { position, last_seen_turn }` report.
- [x] Keep hidden opponent Center/NearTower positions, health, and posture
  unknown; keep the allied opponent report unknown for all positions.
- [x] Preserve the existing observation receipt/state-hash binding without
  adding report state, transition inputs, events, effects, or commands.
- [x] Replay a FarSide observation through unchanged transition authority and
  cover visible/hidden projection boundaries.

This evidence establishes one player-only opponent sighting rule. It does not
establish complete vision, belief updates, memory decay, communication,
automatic threat timing, strategy quality, or a complete playable lane scenario.

### Historical M2 v1 bounded Yield-intent evidence (retired)

- [x] Advertise `Yield` in player observation alongside Stabilize, Contest, and Recall.
- [x] Resolve Yield deterministically to `NearTower` with zero damage and zero mana spent, producing outcome `YieldedSpace` and emitting intent-attributed position effects.
- [x] Reject mana spending for Yield during execution validation.
- [x] Replay Yield history and preserve objective attribution in focused tests.

This evidence establishes one bounded Yield plan. It does not establish complete
vision, belief updates, variable pacing, communication, strategy quality, balance,
or a complete playable lane scenario.

### Historical M2 v1 bounded gold-resource evidence (retired)

- [x] Add bounded `LaneGold` to player state with zero default and non-zero state-hash binding.
- [x] Expose `LaneGold` in player and allied observations without exposing opponent gold.
- [x] Resolve explicit gold earned during execution with direct-immediate `GoldEarned`/`GoldChanged` events and effects, debrief recording, and replay verification.
- [x] Reject gold overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded gold resource. It does not establish items, gold-driven scaling, experience, cooldowns, or a complete playable lane scenario.

### Historical M2 v1 bounded experience-resource evidence (retired)

- [x] Add bounded `LaneExperience` to player state with zero default and non-zero state-hash binding.
- [x] Expose `LaneExperience` in player and allied observations without exposing opponent experience.
- [x] Resolve explicit experience gained during execution with direct-immediate `ExperienceGained`/`ExperienceChanged` events and effects, debrief recording, and replay verification.
- [x] Reject experience overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded experience resource. It does not establish leveling curves, ability unlocks, cooldowns, or a complete playable lane scenario.

### Historical M2 v1 bounded cooldown-resource evidence (retired)

- [x] Add bounded `LaneCooldown` to player state with zero default and non-zero state-hash binding.
- [x] Expose `LaneCooldown` in player and allied observations without exposing opponent cooldown.
- [x] Resolve explicit `cooldown_set` and turn/window beat ticking during execution with direct-immediate `CooldownSet`/`CooldownTicked`/`CooldownChanged` events and effects, debrief recording, and replay verification.
- [x] Reject cooldown overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded cooldown resource. It does not establish complete ability catalogs, item active cooldowns, or a complete playable lane scenario.

### Historical M2 v2 contract correction

- [x] Replace the separate player-resource fields with one `LaneResources`
  aggregate and one `LaneResourceInputs` execution aggregate for mana, gold,
  experience, and cooldown, while retaining health as a direct lane field.
- [x] Replace correlated phase/outcome storage with `LaneStatus` and reject
  zero-delay effects through `LaneDelay`.
- [x] Keep direct/indirect and immediate/delayed effect provenance explicit;
  tick and resolve the bounded delayed-effect queue in transition order.
- [x] Version current M2 ruleset, observation, replay, profile, strategy, and
  base-record identities as v2; verify those identities during replay and
  branching.
- [x] Keep the M1 ruleset, codec, fixtures, hashes, and external behavior
  unchanged.

### Current M2 v3 delayed-origin correction

- [x] Retain each delayed effect's originating execution trace through queue
  ticking, state hashing, branch/history identity, replay, events, effects,
  lane debriefs, and final debrief reports.
- [x] Advance current internal M2 ruleset, observation, replay, profile,
  strategy, scenario, debrief, and branch identities to v3; unsupported v2 M2
  inputs fail closed.
- [x] Keep M1 ruleset, codec, fixtures, hashes, and external behavior
  unchanged.

### Current bounded automatic-advance contract

- [x] Define `LaneAdvanceCondition` and `LaneAdvanceDecision` for deterministic
  commit-required and no-legal-intent evaluation using explicit inputs.
- [x] Keep current one- and two-beat windows commit-required and preserve their
  transition, observation, hash, and replay behavior.

This defines the advance condition contract only. No-choice host scheduling,
automatic execution outcomes, timeout policy, or a complete playable scenario
is established.

### Current bounded belief/report contract

- [x] Define report-derived `LaneBelief<T>` states for `Unknown`, `Observed`,
  and `LastKnown` information over actor-authorized opponent-position and
  threat-region reports.
- [x] Retain prior belief on an unknown report under an explicit no-decay rule;
  malformed value/turn pairs fail closed and beliefs never enter authoritative
  lane state.

This establishes the bounded report and belief semantics only. Vision geometry,
memory decay, threat execution, communication, and a playable scenario remain
deferred.

### Current bounded actor-roster evidence

- [x] Define the fixed `LaneActorRoster` for the human laner, opposing laner,
  allied autonomous actor, and abstract opposing jungle threat with stable role
  identities.
- [x] Expose role identity through player and allied observations without adding
  hidden health, position, posture, policy, or threat truth to either view.
- [x] Keep the roster outside authoritative state hashing and verify that the
  existing transition/replay boundary remains unchanged.

This evidence establishes actor-role completeness for the bounded M2 contract;
it does not establish full vision geometry, memory decay, communication, threat
execution, pacing, or a playable scenario.

### Current bounded minimum-abstraction evidence

- [x] Define bounded lane positions, player/opponent health, wave pressure, and
  the `LaneResources` aggregate for mana, cooldown, gold, and experience.
- [x] Carry those values through the host-owned snapshot, state hash, and replay
  checks; expose only actor-authorized player fields and bounded reports, while
  explicit execution inputs carry resolved damage, wave, and resource changes.

This evidence establishes the minimum typed state needed by the current
diagnostic window only; it does not establish a complete economy, balance, or
playable scenario.

### Current bounded intent-contract evidence

- [x] Define `LaneIntent`, `LaneCommitment`, `LaneTargetFocus`,
  `LanePingSignal`, `LaneAbortCondition`, and `LaneFallbackBehavior` as typed
  request/command fields with defaults and bounded alternatives.
- [x] Advertise legal options in actor-visible observations and bind requests to
  the current observation, validation, record identity, and replay.

This evidence establishes a bounded intent and communication signal contract;
it does not establish free-form messaging, delivery, trust, negotiation, or a
complete communication system.

### Current bounded causal-information evidence

- [x] Preserve direct/indirect and immediate/delayed effect labels with complete
  originating cause/trace attribution through delayed-queue resolution.
- [x] Keep `LaneOutcome` and objective review distinct from binary win/loss
  scoring.
- [x] Test hidden-state redaction, actor-visible report completeness, and receipt
  privacy; inspect one complete two-window replay through debrief projection.

This evidence covers the bounded delayed-origin and advance-condition paths.
Vision/belief updates, no-choice host scheduling, communication transport,
balance, and playability remain open.

The following older resource-slice notes are retained as historical evidence
only. Bounty, level, minion kills, shield, ward, and the sixteen experimental
consumables are retired from the current M2 surface and are not M2 exit
criteria.

### Historical M2 v1 bounty-resource evidence (retired)

- [x] Add bounded `LaneBounty` to player state with zero default and non-zero state-hash binding.
- [x] Expose `LaneBounty` in player and allied observations without exposing opponent bounty.
- [x] Resolve explicit `bounty_earned` during execution with direct-immediate `BountyEarned`/`BountyChanged` events and effects, debrief recording, and replay verification.
- [x] Reject bounty overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded bounty resource. It does not establish complete bounty system scaling, multi-actor kills, or a complete playable lane scenario.

### Historical M2 v1 level-resource evidence (retired)

- [x] Add bounded `LaneLevel` to player state with initial default 1 and non-initial state-hash binding.
- [x] Expose `LaneLevel` in player and allied observations without exposing opponent level.
- [x] Resolve explicit `level_gained` during execution with direct-immediate `LevelGained`/`LevelChanged` events and effects, debrief recording, and replay verification.
- [x] Reject level overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded level resource. It does not establish complete leveling curves, ability point trees, or a complete playable lane scenario.

### Historical M2 v1 minion-kills-resource evidence (retired)

- [x] Add bounded `LaneMinionKills` to player state with zero default and non-zero state-hash binding.
- [x] Expose `LaneMinionKills` in player and allied observations without exposing opponent minion kills.
- [x] Resolve explicit `minion_kills_gained` during execution with direct-immediate `MinionKillsGained`/`MinionKillsChanged` events and effects, debrief recording, and replay verification.
- [x] Reject minion kills overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded minion kills resource. It does not establish minion wave spawn timing, last-hitting mechanics, or a complete playable lane scenario.

### Current bounded target-focus evidence

- [x] Add bounded `LaneTargetFocus` to player intent requests and commands with `Minions` default and non-default record-identity hash binding.
- [x] Expose available target focus options in player observations.
- [x] Emit direct-immediate `TargetFocusSelected`/`TargetFocusSet` events and effects during transition evaluation, with debrief recording and replay verification.

This evidence establishes one bounded intent focus abstraction. It does not establish multi-actor focus target switching or a complete playable lane scenario.

### Current bounded commitment evidence

- [x] Add bounded `LaneCommitment` to player intent requests and commands with `Standard` default and non-default record-identity hash binding.
- [x] Expose available commitment options in player observations.
- [x] Emit direct-immediate `CommitmentSelected`/`CommitmentSet` events and effects during transition evaluation, with debrief recording and replay verification.

This evidence establishes one bounded intent commitment abstraction. It does not establish commitment-based stat scaling or a complete playable lane scenario.

### Current bounded abort-condition evidence

- [x] Add bounded `LaneAbortCondition` to player intent requests and commands with `None` default and non-default record-identity hash binding.
- [x] Expose available abort condition options in player observations.
- [x] Emit direct-immediate `AbortConditionSelected`/`AbortConditionSet`/`AbortConditionTriggered` events and effects during transition evaluation, with debrief recording and replay verification.

This evidence establishes one bounded intent abort condition abstraction. It does not establish multi-beat contingency resolution or a complete playable lane scenario.

### Current bounded fallback-behavior evidence

- [x] Add bounded `LaneFallbackBehavior` to player intent requests and commands with `MaintainPlan` default and non-default record-identity hash binding.
- [x] Expose available fallback behavior options in player observations.
- [x] Emit direct-immediate `FallbackBehaviorSelected`/`FallbackBehaviorSet`/`FallbackBehaviorTriggered` events and effects during transition evaluation, with debrief recording and replay verification.

This evidence establishes one bounded intent fallback behavior abstraction. It does not establish complete contingency evaluation or a complete playable lane scenario.

### Historical M2 v1 shield-resource evidence (retired)

- [x] Add bounded `LaneShield` to player state with zero default and non-zero state-hash binding.
- [x] Expose `LaneShield` in player and allied observations without exposing opponent shield.
- [x] Resolve explicit `shield_gained` during execution with direct-immediate `ShieldGained`/`ShieldChanged` events and effects, debrief recording, and replay verification.
- [x] Reject shield overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded defensive shield resource. It does not establish complete shield degradation, absorption mechanics, or a complete playable lane scenario.

### Historical M2 v1 ward-resource evidence (retired)

- [x] Add bounded `LaneWard` to player state with zero default and non-zero state-hash binding.
- [x] Expose `LaneWard` in player and allied observations without exposing opponent ward count.
- [x] Resolve explicit `ward_gained` during execution with direct-immediate `WardGained`/`WardChanged` events and effects, debrief recording, and replay verification.
- [x] Reject ward overflow exceeding maximum bounds before transition evaluation.

This evidence establishes one bounded vision ward resource. It does not establish map ward placement locations, vision radius calculations, or a complete playable lane scenario.

### Historical M2 v1 flask-resource evidence (retired)

- [x] Add bounded `LaneFlask` to player state with zero default and non-zero state-hash binding (`LANE_FLASK_HASH_TAG`).
- [x] Expose `LaneFlask` in player (`self_flask`) and allied (`laner_flask`) observations without exposing opponent flask count.
- [x] Resolve explicit `flask_gained` and `flask_spent` during execution with direct-immediate `FlaskGained`/`FlaskSpent`/`FlaskChanged` events and effects, debrief recording, and replay verification.
- [x] Reject flask overflow exceeding maximum bounds (`MAX_LANE_FLASK`) or spending without available flasks (`InsufficientFlask`) before transition evaluation.

This evidence establishes one bounded flask consumable resource. It does not establish complete consumable item active usage or a complete playable lane scenario.

### Historical M2 v1 incense-resource evidence (retired)

- [x] Add bounded `LaneIncense` to player state with zero default and non-zero state-hash binding (`LANE_INCENSE_HASH_TAG`).
- [x] Expose `LaneIncense` in player (`self_incense`) and allied (`laner_incense`) observations without exposing opponent incense count.
- [x] Resolve explicit `incense_gained` and `incense_spent` during execution with direct-immediate `IncenseGained`/`IncenseSpent`/`IncenseChanged` events and effects, debrief recording, and replay verification.
- [x] Reject incense overflow exceeding maximum bounds (`MAX_LANE_INCENSE`) or spending without available incenses (`InsufficientIncense`) before transition evaluation.

This evidence establishes one bounded incense consumable resource. It does not establish complete consumable item active usage or a complete playable lane scenario.

### Historical M2 v1 poultice-resource evidence (retired)

- [x] Add bounded `LanePoultice` to player state with zero default and non-zero state-hash binding (`LANE_POULTICE_HASH_TAG`).
- [x] Expose `LanePoultice` in player (`self_poultice`) and allied (`laner_poultice`) observations without exposing opponent poultice count.
- [x] Resolve explicit `poultice_gained` and `poultice_spent` during execution with direct-immediate `PoulticeGained`/`PoulticeSpent`/`PoulticeChanged` events and effects, debrief recording, and replay verification.
- [x] Reject poultice overflow exceeding maximum bounds (`MAX_LANE_POULTICE`) or spending without available poultices (`InsufficientPoultice`) before transition evaluation.

This evidence establishes one bounded poultice consumable resource. It does not establish complete consumable item active usage or a complete playable lane scenario.

### Current final-debrief evidence

- [x] Build a versioned final debrief only from a replay-verified two-window
  history, with two per-window intent/coordination/execution/objective summaries.
- [x] Aggregate a final objective disposition without adding a new lane
  outcome, state field, event/effect, or hidden-state score.
- [x] Separate privileged provenance from a visible report that omits hashes,
  receipts, policy internals, and uncommitted choices.
- [x] Reject incomplete histories and tampered source identities, window
  summaries, objective reviews, terminal hashes, final disposition, or report.

### Active Developer Action Items (M2 Exit Promotion)

- [ ] Connect the full multi-window lane scenario to the interactive CLI runner.
- [ ] Validate three distinct playable strategy playthroughs (HappyPath, RiskTaking, Conservative) through the interactive runner.
- [ ] Verify automated advance condition integration in the interactive runner.
- [ ] Finalize M2 exit evidence review and promote M2 from Active to Complete in SPEC.md.

### Deliverables

- One versioned scenario and actor profile set.
- Complete scenario run and replay bundle.
- Actor-visible observation and debrief snapshots.
- Focused mechanics, determinism, and information-boundary tests.
- Short playtest note separating technical evidence from experience hypotheses.

### Exit evidence

- One scenario reaches a terminal state through only public player actions.
- At least three coherent strategies are representable and no single action
  dominates every declared starting condition.
- No ordinary player or agent action requires true-state access.
- Every decision window has a meaningful choice or an automatic-advance path.
- The final debrief separates intent, coordination, execution, and luck and can
  cite committed history.
- A replay reproduces the terminal hash; a branch documents whether exogenous
  inputs were matched or regenerated.

### Explicit deferrals

- No full champion roster, item catalog, three-lane map, or networked play.
- No empirical claim that simulated behavior resembles human players.
- No general framework beyond the needs demonstrated by this slice.

## Phase 3 — CLI Reference Experience

**Milestone:** M3
**Status:** Planned — early bounded grammar evidence
**Depends on:** M2

### Outcome

The command-line interface is a complete, keyboard-first reference client for
play, inspection, save/load, replay, branching, and debrief without developer
API access.

### Scope

- [x] Define stable top-level process commands.
- [x] Define stable in-session grammar as a typed, dependency-free adapter
  contract.
- [x] Implement `observe`, bounded actor-visible `inspect`, and contextual help
  as typed adapter read requests; plain text projection and fixture-loop I/O
  are covered while full client behavior remains open.
- [x] Define typed adapter requests for `message`, `plan`, contingency,
  `commit`, and `advance`; the bounded host fixture executes only the existing
  two-window scenario.
- [x] Define typed adapter requests for `review`, `debrief`, `replay`, and
  `branch`; the bounded host fixture executes the matched-parent `first` point
  while regenerated and multi-window branching remain open.
- [x] Define typed adapter requests for `save`, `load`, `undo`, and `quit`; the
  host fixture provides versioned, replay-validated artifacts and an injected
  file store; the binary accepts one explicit `--run-dir` option.
- [x] Add guided mode with numbered choices and explanations.
- [x] Add expert mode with concise, scriptable commands.
- [x] Add research inspection only behind an explicit privileged context.
- [x] Add concise, standard, explanatory, and research verbosity policies.
- [x] Label observed, believed, inferred, reported, and unknown information in
  a typed, payload-safe adapter projection contract.
- [x] Support edit/undo before commitment without rewriting committed history
  through a typed local draft boundary.
- [x] Add validated human-readable run identifiers to save/load/replay/export
  adapter requests; the host artifact binds them to replay identity and state
  hashes without exposing authoritative history.
- [x] Keep terminal rendering outside the authoritative domain; the pure
  `src/terminal.rs` projection consumes actor-valid host values and performs no
  terminal I/O.
- [x] Add transcript-based acceptance tests for a complete library-host run and
  common errors.
- [x] Add one explicit, versioned executable fixture ID with fail-closed
  missing/unknown argument handling; broader scenario catalogs remain open.
- [x] Add standalone `--version`/`-V` package metadata reporting before host
  construction; schema negotiation and migrations remain open.
- [x] Verify a clean-checkout executable transcript completes both fixture
  windows through public commands, replay, debrief, and quit.
- [x] Verify machine-checkable plain labeled text structure for representative
  output and recoverable command-loop errors.
- [ ] Check keyboard-only flow and screen-reader semantics with human-oriented
  inspection.

### Current bounded information-label evidence

- [x] Version the internal `m3-cli-information-labels-v1` vocabulary.
- [x] Define stable `observed`, `believed`, `inferred`, `reported`, and
  `unknown` labels with canonical names.
- [x] Represent labeled values with `CliInformation<T>`; the `Unknown` form is
  payload-free and cannot carry hidden state through the adapter.
- [x] Verify borrowed projections preserve labels and explicit value extraction
  returns payloads without changing M2 state, hashes, replay, or transition
  behavior.

This establishes provenance metadata for future CLI projections only. Inference,
belief computation, persistence, terminal I/O, and human distinction remain
open.

### Current bounded pre-commit edit/undo evidence

- [x] Version the internal `m3-cli-precommit-draft-v1` contract.
- [x] Stage and replace message, plan, and contingency payloads while rejecting
  empty values and commit/advance boundary requests.
- [x] Clear only uncommitted draft fields through `CliDraft::undo()`.
- [x] Consume drafts into a read-only `CliCommittedDraft` marker so the adapter
  surface cannot edit or undo committed choices.

This establishes local draft semantics only. It does not execute a host command,
persist a session, or rewrite authoritative lane history.

### Current terminal-rendering boundary evidence

- [x] Keep `src/kernel.rs` and `src/lane/` free of terminal I/O, rendering
  loops, and mutable runtime presentation state.
- [x] Keep `src/cli.rs` limited to borrowed grammar, typed requests, labels, and
  local drafts; `src/host.rs` owns the bounded transition and in-memory session
  orchestration without terminal I/O.
- [x] Assign any future rendering to an outer adapter that consumes
  actor-valid projections and cannot authorize commands or mutate history.

This verifies a structural boundary, bounded host flow, injected file storage,
pure text projection, the thin line-oriented fixture I/O loop, and explicit
binary store wiring. One versioned fixture ID is selectable at the process edge;
complete reference-client behavior, broader scenario selection,
regenerated/graph branching, and keyboard/screen-reader inspection remain open.

### Current bounded run-identifier evidence

- [x] Version the internal `m3-cli-run-id-v1` syntax contract.
- [x] Accept bounded readable IDs (alphanumerics with `.`, `_`, and `-`) and
  reject empty, overlong, non-ASCII, and malformed values.
- [x] Carry validated `CliRunId` values through session save/load, in-session
  replay, and top-level replay/export requests with versioned host-artifact
  validation while keeping authoritative history private.

This establishes bounded adapter syntax plus injected artifact storage and
explicit binary wiring. Run generation, collision handling, cross-process
resume beyond the two-process fixture smoke path, and human discoverability
remain open.

### Current bounded grammar-transcript evidence

- [x] Exercise a representative 16-command grammar transcript across read,
  write, process, and session request mappings.
- [x] Cover common parser, malformed request, invalid run-ID, and privilege
  errors before host boundaries.
- [x] Complete a host-backed scenario transcript with save/resume, replay,
  debrief, and deterministic plain-text output.

The first two checked items are grammar-level acceptance. The third combines
library host/text evidence with a bounded two-process store smoke path and
still does not satisfy the M3 complete-run exit evidence.

### Current bounded host-transcript evidence

- [x] Map the existing CLI grammar to an explicit-input, synchronous
  two-window host fixture without exposing true-state snapshots or hashes.
- [x] Complete a library-only transcript with observe, staged
  message/plan/contingency text, commit, advance, versioned artifact save/load, replay,
  debrief, and quit.
- [x] Add line-oriented terminal I/O/command-loop integration around the host
  contract and wire an explicit `--run-dir` option at the executable edge.
- [x] Verify the documented two-window transcript through the real executable
  with actor-safe output, replay, debrief, and quit markers.
- [ ] Check keyboard-only flow and screen-reader semantics with human-oriented
  inspection.

This is host-backed scenario, injected file-store, text-projection, and
bounded two-process executable evidence with a matched-parent branch, but not
the complete M3 reference client or human accessibility evidence.

### Current bounded terminal-text evidence

- [x] Render every host output and host error as stable labeled plain text
  without ANSI styling, terminal I/O, or hidden-state lookup.
- [x] Sanitize control characters in echoed user context and keep domain
  failures redacted to the bounded host error categories.
- [x] Check representative output and command-loop error lines for stable
  lowercase labels, newline structure, and absence of ANSI/control characters
  on the labeled pipe path.
- [x] Add a TTY presentation edge with `> ` prompt, Tab completion, optional
  ANSI, `help`/`?` topics, and actor-safe session chrome; piped labeled text
  remains the script contract.
- [ ] Validate complete interactive behavior, keyboard/focus behavior, and
  screen-reader semantics with human-oriented inspection.

### Developer Action Items

- [ ] Dynamic interactive scenario selection in the CLI runner (allowing players to choose between M2 lane scenarios and M9 match scenarios without hardcoded flags).
- [ ] Interactive branch exploration directly within the command loop.
- [ ] Terminal resize handling and accessibility auditing for pure text presentation.

### Deliverables

- Reference CLI and command help.
- Versioned command-to-domain contract.
- Golden or snapshot transcripts for guided and expert runs.
- Save/load and replay fixtures.

### Exit evidence

- A clean checkout can complete the scenario through documented CLI commands.
- Transcript tests cover success, invalid syntax, invalid domain command, save,
  resume, replay, and debrief.
- Every strategically meaningful CLI operation maps to a typed host command or
  actor-visible read.
- A keyboard-only inspection finds no required mouse or reaction-time action.
- Screen-reader compatibility remains a technical inspection claim until tested
  with relevant users.

### Explicit deferrals

- No full-screen TUI unless command-loop evidence demonstrates a need.
- No browser GUI and no GUI parity commitment.

## Phase 4 — Baseline Agent Ecology

**Milestone:** M4
**Status:** Planned
**Depends on:** M2; stable M3 contracts preferred

### Outcome

Several non-LLM agents exhibit interpretable, reproducible differences in
strategy while remaining bound to actor-visible information.

### Scope

- [x] Implement three actor-visible deterministic scripted profiles for the
  bounded fixture; broader scripted populations remain open.
- [x] Implement three transparent policy-role labels (`Anchor`, `Duelist`,
  `Pacer`) over the fixed profile heuristics; the Anchor profile now uses a
  bounded observed-pressure feature while scenario roles and broader role
  populations remain open.
- [x] Define bounded policy inputs, candidate actions, utility features, and
  action evaluations for the fixture; memory remains open.
- [x] Separate candidate generation, evaluation errors, and stable selection
  for the fixture; top-k/nucleus selection, coordination, and execution remain
  open.
- [x] Define bounded baseline preferences (`Stabilize`, `Contest`, `Yield`) and
  the observed-pressure effect for the fixture; loss aversion, planning
  horizon, attention, trust, communication response, confidence, and tilt
  remain open.
- [x] Define bounded creativity evidence as candidate breadth from actor-visible
  advertisements rather than random inferior-action selection; broader
  candidate transformation remains open.
- [x] Create a small three-profile versioned baseline catalog; broader profiles
  remain open.
- [x] Use an explicit policy seed bundle and reproducible stream/draw identity
  for opt-in top-1 tie selection; broader random sampling remains open.
- [x] Add one matched-input comparison over the same actor-visible observation;
  matched-scenario populations remain open.
- [x] Define one bounded monotonic utility effect: Anchor's `Stabilize` score
  increases with observed wave pressure; broader parameter interactions remain
  open.
- [x] Measure a bounded selected-action tally and legality over the two fixture
  observations; broader action distributions, strategic diversity,
  communication, coordination, plan interruption, and outcome distributions
  remain open.
- [x] Add one visible-threat profile-sensitivity regression; broader
  adversarial edge-case matrices remain open.

### Current bounded scripted-agent evidence

- [x] Define the versioned `m4-scripted-agent-v1` policy boundary and the
  `cautious-laner-v1`, `risk-taking-laner-v1`, and `yielding-laner-v1` profile
  identities.
- [x] Generate candidates only from the actor-visible `LanerObservation`
  legal-intent set plus its visible threat-response option, with safe/threat
  breadth evidence of four versus five candidates.
- [x] Reject public evaluation requests for intents outside that advertised
  candidate set with a bounded policy error.
- [x] Evaluate candidates with the profile-specific fixed, inspectable
  `threat-first-pressure-aware-fixed-score-v1`, `contest-first-fixed-score-v1`, or
  `yield-first-fixed-score-v1` rule and select by stable maximum score by
  default.
- [x] Define the versioned `m4-scripted-agent-random-v1` seed bundle and
  `max-score-seeded-tie-v1` opt-in rule. It uses only the supplied seed and
  policy stream/draw to choose among equal top-score candidates; the default
  path remains stable-order deterministic.
- [x] Expose each profile's baseline preferred intent separately from a
  visible-threat `Withdraw` override.
- [x] Bind `max-score-stable-order-v1` to every profile and prove the default
  equal-score path retains the first advertised candidate; top-k/nucleus
  selection remains open.
- [x] Return an actor-bound `LaneIntentRequest` for host-side legality
  validation, with reproducibility tests for identical observations.
- [x] Compare three fixed profiles on one identical initial observation and
  verify distinct legal intents without changing the host boundary.
- [x] Check profile sensitivity when a visible RiverSide threat is advertised:
  cautious changes to `Withdraw`, while risk-taking and yielding retain their
  fixed intents and all requests remain host-valid.
- [x] Emit the versioned `m4-scripted-agent-metrics-v1` comparison report with
  profile/rule IDs, selected intent/score, and bounded candidate counts.
- [x] Bind policy-role IDs (`anchor-v1`, `duelist-v1`, `pacer-v1`) to the three
  profiles without conflating them with the lane scenario actor roster.
- [x] Verify Anchor's `Stabilize` score rises from 80 to 83 as the observed
  wave-pressure value rises from 0 to 3, while requests remain host-valid.
- [x] Emit the versioned `m4-scripted-agent-action-tally-v2` report for the
  two uniquely identified fixture observations, reject mixed observers and
  duplicate IDs, and retain only the shared observer, bounded observation,
  profile, and rule IDs, observation count, and selected-intent counts.
- [x] Capture `m4-scripted-agent-replay-v1` records that re-evaluate the same
  actor-visible observation with default or seeded provenance, classify an
  expected versus declared-anomalous intent, and reject a tampered decision;
  durable persistence and degenerate-policy populations remain open.

This is a three-profile library-only comparison with bounded score and
selected-action reports, an explicit seed bundle, and decision-replay records.
It does not establish a
population, broader role heuristics, memory, communication, broad random
sampling, durable replay persistence, population-level matched-scenario
metrics, strategic quality, human realism, or an executable agent adapter.

### Deliverables

- Scripted, heuristic, and initial parametric policy implementations.
- Versioned profile and metric schemas.
- Matched-input comparison report across at least three profiles.
- Representative replay set for expected and anomalous behavior.

### Exit evidence

- Profile differences reproduce under matched scenarios and declared seeds.
- Directional parameter checks pass or non-monotonic interactions are explained
  with evidence.
- No agent reads hidden state or owns transition semantics.
- Diversity measures distinguish candidate breadth from execution randomness.
- Bounded expected and declared-anomalous policy decisions are inspectable
  through library replay records; degenerate-policy populations remain open.

### Explicit deferrals

- No claim of human behavioral realism.
- No LLM required for baseline completion.
- No broad reinforcement-learning platform or provider-specific orchestration.

## Phase 5 — Model-Agnostic MCP Play

**Milestone:** M5
**Status:** Planned
**Depends on:** M3 and stable actor-visible contracts

### Outcome

External scripted, parametric, and selected language-model agents can complete
the same scenario through a versioned MCP adapter without privileged state or
control over simulation resolution.

### Current bounded actor-protocol evidence

- [x] Define `m5-actor-protocol-v1` with primitive actor-visible observation
  and intent-action DTOs, closed intent IDs, and observer/turn/observation
  identity.
- [x] Convert the bounded action DTO back to an observer-bound
  `LaneIntentRequest` for existing host validation; transport, session
  reconnect, and plan/message metadata remain open.
- [x] Define the immutable `m5-actor-session-v2` lifecycle that binds one
  ordinary actor to one current observation, rejects cross-actor/stale/
  duplicate submissions, maps bounded encoded-action failures, and records
  explicit client/timeout/disconnect closure reasons; host legality, timing,
  reconnect, and history remain outside the session adapter.
- [x] Add the bounded `m5-actor-codec-v1` line-oriented encode/decode contract
  for observation and intent-action DTOs, with size, field, line-count, and
  closed-intent checks; transport I/O and persistence remain open.
- [x] Define the versioned `m5-actor-error-v2` actor-safe validation-error
  categories and deterministic repair hints for codec/session failures;
  automatic repair and host-legality error projection remain open.
- [x] Add exact bounded `m5-actor-error-v2` codec coverage for every closed
  error and repair ID; automatic repair and transport remain open.
- [x] Project successful host actor submissions through a bounded result DTO
  containing only fixture window and categorical outcome; detailed debrief and
  transport remain open.
- [x] Define `m5-actor-commit-v1` and `m5-actor-commit-result-v1` for an
  observation-bound explicit intent commit and bounded acknowledgement; host
  transition and history remain untouched until a later advance.
- [x] Define `m5-actor-draft-receipt-v1` as a payload-free acknowledgement for
  accepted observation-bound message, plan, or contingency staging; metadata
  delivery and communication semantics remain open.
- [x] Define `m5-actor-draft-status-v1` as a bounded aggregate presence
  projection for the active observation-bound message, plan, and contingency
  draft; payload delivery and communication semantics remain open.
- [x] Define `m5-actor-draft-clear-v1` and
  `m5-actor-draft-clear-receipt-v1` for observation-bound idempotent draft
  clearing with pre-clear field presence; delivery and communication semantics
  remain open.
- [x] Define `m5-actor-draft-commit-receipt-v1` as a payload-free acknowledgement
  of the committed intent and accepted message/plan/contingency field presence;
  metadata delivery and communication semantics remain open.
- [x] Define the recipient-scoped `m5-actor-message-v1` envelope with bounded
  actor-authored text and observation binding; routing, delivery, ordering,
  and communication semantics remain open.
- [x] Add a read-only host adapter that validates an actor action against the
  current receipt and maps mismatch, stale, closed-window, and generic lane
  rejection to actor-safe codes.
- [x] Implement bounded host-owned actor action submission and window closure
  through the existing lane/history path; transport-integrated submission and
  simultaneous decisions remain open.
- [x] Define bounded `m5-actor-draft-v1` metadata for message, plan, and
  contingency fields with observation binding and closed plan IDs; host draft
  staging is delivered while metadata delivery remains open.
- [x] Stage observation-bound actor draft metadata through the host-owned draft
  boundary with replacement and committed-window checks; communication
  delivery remains open.
- [x] Read back the requesting actor's actor-protocol-staged message, plan,
  and contingency metadata through existing observation-bound draft DTOs;
  recipient delivery, simultaneous drafts, and communication semantics remain
  open.
- [x] Expose the active actor-visible receipt through the versioned observation
  DTO, rejecting closed/complete hosts without mutating history; transport and
  simultaneous actors remain open.
- [x] Define the bounded `m5-actor-history-v1` status DTO and host projection
  for record count plus open/complete/closed lifecycle state; detailed history
  and durable/scenario replay-linked records remain open.
- [x] Define the bounded `m5-actor-debrief-v1` completed-run summary DTO and
  host projection for per-window intent/outcome/objective labels plus final
  objective and committed-facts attribution; detailed causal debrief and
  durable/scenario replay-linked records remain open.
- [x] Define the bounded `m5-actor-replay-v1` status DTO and host projection;
  it verifies immutable current history and exposes only categorical status and
  record count.
- [x] Define the bounded `m5-actor-replay-record-v1` DTO and host projection;
  it exposes at most two replay-verified window/intent/outcome records without
  hashes, resolved inputs, traces, or causal detail.
- [x] Expose categorical `m5-actor-replay-record-v1` records from a validated
  injected saved run through a fresh-host adapter; durable file portability,
  locking, and scenario-wide replay remain open.
- [x] Expose completion-gated categorical `m5-actor-replay-debrief-record-v1`
  records from a validated injected saved run without mutating the receiving
  host; durable/scenario replay-linked causal review remains open.
- [x] Expose the existing `m5-actor-debrief-v1` categorical summary from a
  validated complete injected saved run without mutating the receiving host;
  detailed causal review remains open.
- [x] Define the bounded `m5-actor-replay-debrief-record-v1` DTO and host
  projection for two complete replay-verified windows with categorical
  objective labels and committed-facts attribution; detailed causal review
  remains open.

This is a pure library adapter boundary with no MCP transport, async runtime,
or provider-specific behavior. The DTOs expose only four advertised intents
plus an optional visible threat response and do not replace host legality. The
session state machine is immutable metadata and does not submit or commit a
transition. The `m5-actor-error-v2` projection exposes only stable error and
repair IDs for codec/session failures and host action rejection; repair is
advisory and does not rewrite payloads or retry host work. The host-owned
submission path appends only after current-receipt and lane validation, then
closes the fixture window through the existing deterministic transition.
The `m5-actor-draft-v1` DTO adds bounded message, plan, and contingency
metadata without communication authority. The host stages those DTOs only
before commit and never turns them into a transition by itself. The
`m5-actor-draft-receipt-v1` DTO acknowledges accepted staging with only the
bound actor, observation, and closed field identity; it does not echo draft
values or deliver them to another actor. The `m5-actor-draft-commit-receipt-v1`
DTO acknowledges a successful commit with the bound actor, observation,
committed intent, and `present`/`absent` bits for each draft field; it never
echoes values or claims communication delivery. The provider-neutral
`m5-actor-transcript-v1` record captures only closed tool/schema IDs and an
accepted/rejected result for an actor receipt; it is not a runtime log or
replay record. Focused evidence is 26 protocol tests,
12 session tests, and 34 host tests within the 231-unit, 7-binary, and
3-Rustdoc suite. The host observation projection is a
pure actor-visible DTO mapping, rejects inactive lifecycle states, and leaves
the internal receipt private. The history DTO is a bounded status summary,
while the debrief DTO is a completion-gated committed-facts summary rather
than a detailed replay or causal debrief contract.
The repository checker adds one focused boundary test and scans the
deterministic core module list for async, wall-clock, and network transport
primitives; this is source-ownership evidence rather than transport behavior.
The `m5-actor-replay-v1` DTO and host projection add only verified status and
bounded record count. The `m5-actor-replay-record-v1` projection exposes at
most two verified categorical window/intent/outcome entries. The
`CliScenarioHost::actor_replay_records_from_run` adapter validates a run ID,
loads through the injected store, restores and verifies history, then returns
the same categorical records without mutating the current host. The
`m5-actor-replay-debrief-record-v1` projection adds only categorical objective
labels and the committed-facts attribution limit after complete-history
verification; hashes, resolved inputs, execution traces, and causal detail
remain private. The `CliScenarioHost::actor_replay_debrief_records_from_run`
adapter validates an injected saved run, restores and verifies it locally,
requires two records, and returns the same categorical debrief records without
mutating the receiving host. The `CliScenarioHost::actor_draft` readback
  returns only the requesting actor's actor-protocol-staged values in stable
  field order and does not reinterpret legacy CLI draft text or deliver values
  to another actor. The `m5-actor-draft-status-v1` projection reports only the
active observation binding and aggregate `present`/`absent` bits for message,
plan, and contingency; it never echoes draft values or claims delivery.
The `CliScenarioHost::actor_debrief_from_run` adapter applies the same local
restore and completion gate before returning the existing categorical summary;
the receiving host remains unchanged and detailed causal review stays private.
The `ActorMessageDto` envelope binds bounded actor-authored text to a sender,
recipient, and observation ID without routing or delivery authority; transport,
recipient visibility, ordering, and communication semantics remain open. The
`m5-actor-draft-clear-v1` command carries only the active observer and
observation ID, while its receipt reports which fields were present before the
host cleared them; it is idempotent for an empty draft and adds no delivery or
transition authority.

### Scope

- [x] Define the bounded library session lifecycle and actor binding;
  transport-integrated lifecycle and authority remain open.
- [ ] Define remaining integration/contracts for messages, plans,
  contingencies, durable/scenario replay-linked records, and detailed
  outcome/debrief review;
  bounded observation/action/commit/draft/message/draft-readback/draft-status/draft-clear/history/replay/replay-record/saved-replay-record/replay-debrief-record/saved-replay-debrief-record/saved-debrief-summary/debrief projections
  are delivered.
- [x] Keep authoritative lane observation/request conversion behind crate-private
  protocol adapters; public protocol compatibility exposes DTOs only.
- [x] Implement private action submission and host-owned window closure for the
  bounded fixture; transport and simultaneous-decision integration remain open.
- [x] Implement bounded two-actor simultaneous submission semantics; host
  transition resolution and broader ordering remain open.
- [x] Define protocol-edge validation-error and bounded-repair behavior for
  codec/session failures and bounded read-only host action rejection;
  broad host-legality error projection remains open.
- [x] Separate ordinary actor tools from privileged experiment-controller tools
  with a closed ordinary-actor catalog; privileged implementations remain open.
- [x] Capture provider-neutral transcript metadata and tool-schema versions in
  a bounded library DTO; runtime transport logging and persistence remain open.
- [x] Add bounded authorization and hidden-state leakage tests over the
  ordinary actor adapter surface; network authentication and simultaneous
  privacy remain open.
- [x] Add bounded CLI/protocol action and projection parity tests; MCP transport
  parity remains open.
- [x] Add deterministic session timeout/disconnect closure and malformed,
  duplicate, and stale action behavior; wall-clock transport timing,
  reconnect, and framing remain open.
- [x] Verify that transport and async orchestration stay outside the core with
  a repository checker that scans every deterministic core module for async,
  wall-clock, and network transport primitives; adapter-edge I/O remains open.

### Developer Action Items

- [ ] Implement standalone `fog-of-intent-mcp` binary adapter communicating over JSON-RPC stdio.
- [ ] Wire `ActorMessageDto` and `ActorDraftDto` into live tool call schemas.
- [ ] Validate MCP protocol integration against external LLM agent harnesses.

### Deliverables

- Thin MCP adapter and public protocol schema.
- Actor and experiment-controller capability matrix.
- Scripted, parametric, and selected LLM transcript fixtures.
- Protocol compatibility and parity tests.

### Exit evidence

- Each supported agent family completes the M2 scenario through MCP.
- Zero unauthorized true-state fields appear in actor responses or transcripts.
- Same committed command set and resolved inputs yield the same replay regardless
  of CLI or MCP submission surface.
- Simultaneous submissions cannot observe another actor's private uncommitted
  action.
- Tool, prompt, model, and repair versions are recorded where applicable.

### Explicit deferrals

- No provider-specific framework in simulation authority.
- No public remote service or multi-tenant hosting.
- No promise that all MCP clients or model providers are supported.

## Phase 6 — Automated Behavioral Validation

**Milestone:** M6
**Status:** Planned
**Depends on:** M4 and M5

### Outcome

A rules or content change can be evaluated with deterministic software checks,
versioned population experiments, regression metrics, exploit searches,
representative replays, and an evidence-limited report.

### Scope

- [x] Define a versioned bounded experiment manifest for the scripted fixture
  and explicit policy seed bundle; population sampling remains open.
- [x] Implement a local batch runner and resumable run directory.
  Deterministic in-process manifest batches and bounded cursor checkpoint
  storage are delivered; decision/result persistence remains open.
- [x] Add bounded caller-supplied matched-observation evidence; this slice does
  not choose scenarios or generate populations.
- [x] Add bounded caller-supplied matched-scenario sample sets; this composes
  matched observations without generating scenarios or populations.
- [x] Add a closed fixed-fixture scenario catalog and deterministic selector;
  repeated IDs are explicit ordered samples, while broad population generation
  and distributional sampling remain open.
- [x] Emit a bounded fixed-fixture scenario-frequency report over validated
  selections and a verified-report-bound machine-readable codec; it is explicit
  selection evidence, not a population distribution or durable export.
- [x] Add a bounded deterministic fixed-fixture population generator over the
  closed scenario catalog; it is capped at four alternating entries and uses
  a caller-supplied starting observation ID to derive checked sequential pairs.
- [x] Compose caller-declared ordered populations from the closed fixture
  catalog and expose skewed fixed-fixture frequency evidence; random,
  representative, and broader distributional sampling remain open.
- [x] Compose bounded fixed-fixture populations directly into the existing
  selected-intent tally path; broader population metrics and outcomes remain
  open.
- [x] Preserve ordered cautious/risk-taking/yielding rows when composing a
  fixed-fixture population tally; broader profile-population metrics remain
  open.
- [x] Round-trip the profile-aware fixed-fixture tally through its verified
  bounded codec and reject a tampered row; durable export remains open.
- [x] Compare two caller-declared verified profile-aware tally reports with
  ordered row identities and signed count deltas; build provenance, causality,
  broader metrics, and outcomes remain open.
- [x] Round-trip the profile-aware tally comparison through a bounded
  provenance-bound codec; durable export and arbitrary report pipelines remain
  open.
- [x] Define a provisional profile-aware fixed-fixture no-change gate over
  exact ordered counts; broader thresholds, balance, causality, and outcomes
  remain open.
- [x] Add a bounded 10,000-point distribution projection over caller-declared
  fixed-fixture selections; broader/random scenario generation and sampling,
  population diversity, and representative sampling remain open.
- [x] Emit bounded selected-intent tallies over caller-supplied sample sets;
  population distributions, outcomes, and strategic metrics remain open.
- [x] Encode the selected-intent tally with a bounded machine-readable codec;
  durable export and report pipelines remain open.
- [x] Record applicable ruleset, scenario, scripted-policy, and profile version
  identities in a metadata-only catalog; prompt, model, tool-schema, and
  extractor versions are explicitly not applicable to this in-process slice.
- [x] Add a bounded 10,000-point intent-share projection over each verified
  profile-aware selected-intent tally row; broader population-level
  distributional, outcome, and strategic metrics remain open.
- [x] Add a bounded caller-declared four-case stress matrix over existing
  validation, freshness, message-codec, and deterministic-policy boundaries;
  actual adversarial/degenerate populations and exploit search remain open.
- [x] Add a bounded caller-declared illegal-command population over repeated
  actor-visible host validation rejection; actual exploit search and
  communication-abuse populations remain open.
- [x] Add a bounded fixed-fixture risk-taking policy population over repeated
  actor-visible `Contest` selections; actual exploit search, communication
  abuse, prevalence, outcomes, and strategy-quality evidence remain open.
- [x] Add a bounded caller-declared communication-abuse policy population over
  repeated invalid message values; actual communication-abuse search,
  prevalence, and delivery remain open.
- [x] Add a bounded caller-declared degenerate-policy population over repeated
  actor-visible `Stabilize` selections; broad adversarial populations,
  prevalence, and outcomes remain open.
- [x] Rank a caller-declared largest absolute intent-count candidate from
  verified profile-aware comparison rows with stable row/intent ties; actual
  outlier detection and representative replay selection remain open.
- [x] Emit a provisional inclusive fixed-fixture threshold signal over the
  verified largest-delta candidate; calibrated outlier detection and
  representative replay selection remain open.
- [x] Select the first caller-declared verified replay reference matching a
  largest-delta candidate by profile, rule, and intent; representative replay
  proof and scenario-wide replay remain open.
- [x] Calibrate outlier detection and select representative replays
  deterministically; runtime automated log production, durable persistence, and
  human evidence remain open.
- [x] Classify caller-declared operational `batch_started` →
  `chunk_completed` → `batch_finished` label order with optional checkpoint/
  resume labels; causal-trace completeness and scenario-wide replay identity
  remain open.
- [x] Bind one deterministic decision replay identity to the bounded
  operational sequence status; causal-trace completeness, runtime production,
  and scenario-wide replay remain open.
- [x] Add bounded scenario-wide replay identity evidence over caller-declared
  replay records from a sampled run; causal-trace completeness, runtime
  production, and external persistence remain open.
- [x] Check causal-trace completeness for sampled runs; runtime automated log
  production, durable persistence, and provider integration remain open.
- [x] Define one provisional fixed-fixture regression gate with written
  threshold rationale; broader gates remain open.
- [x] Define a bounded non-authoritative operational event log container
  separate from committed simulation artifacts; automatic runtime log
  production, transport, and durable/scenario-wide persistence remain open.
- [x] Produce caller-driven `batch_started`, `chunk_completed`, and
  `batch_finished` labels around one complete deterministic in-process batch;
  checkpoint/resume producers, runtime failure detection, transport, and
  durable/scenario-wide operational-log persistence remain open.
- [x] Produce caller-driven `checkpoint_saved` and `batch_resumed` labels only
  after successful bounded cursor save/load; capacity preflight and failure
  nonmutation remain explicit, while runtime diagnostics and durable/scenario-
  wide event-log recovery/rotation remain open.
- [x] Encode bounded payload-free operational logs and store them in a distinct
  injected namespace; crash recovery, rotation, external export, and runtime
  diagnostics remain open.
- [x] Store caller-declared operational-log segments under bounded distinct
  suffixes; automatic rotation, crash recovery, and durable scenario-wide
  recovery remain open.
- [x] List recognized caller-declared operational-log segment indices in stable
  order; race-hard scanning and automatic rotation remain open.
- [x] Export bounded machine-readable data and a concise pure Markdown
  evidence report for the verified fixed-fixture frequency slice; durable
  export, arbitrary report pipelines, and broader metrics remain open.
- [x] Compare caller-declared verified fixed-fixture frequency reports with
  bounded ordered row deltas; independent build provenance and causal
  attribution remain open.
- [x] Preserve caller-declared crashes, timeouts, missing branches, and
  inconclusive results in a bounded status envelope; automatic runtime
  detection and process diagnostics remain open.
- [x] Attach distinct caller-declared build labels to a fixed-fixture frequency
  baseline comparison; independent source/build verification and causal
  attribution remain open.

### Deliverables

- Experiment manifest schema and batch runner.
- Versioned bounded cursor artifacts, matched-observation and matched-scenario
  selected-intent/tally evidence with a machine-readable codec, a closed
  fixed-fixture scenario catalog/selector and scenario-frequency evidence with
  a pure Markdown projection, an applicable version catalog, and a
  caller-declared run-disposition envelope; decision/result artifacts,
  automatic failure detection, and representative replays remain open.
- Regression report template and threshold rationale.
- CI or scheduled entry point only after runtime cost is measured.

### Exit evidence

- One controlled rules change produces a reproducible before/after report.
- An outlier can be traced from aggregate metric to committed replay.
- Crashes and timeouts appear in results rather than disappearing from samples.
- Technical findings and human-experience hypotheses are labeled separately.
- The same extractor version produces stable derived results from the same
  authoritative histories.

### Explicit deferrals

- No claim that provisional thresholds represent balance or human preference.
- No cloud experiment platform until local execution limits are demonstrated.

## Phase 7 — Semantic-to-Parametric Calibration Proof

**Milestone:** M7
**Status:** Planned
**Depends on:** M6

### Outcome

At least three versioned semantic behavior profiles are approximated by
interpretable parametric agents on held-out diagnostic scenarios, with
uncertainty and evidence limits reported.

### Scope

- [x] Define a compact semantic profile vocabulary and schema;
  diagnostic scenario choices, distribution estimation, and parametric fitting
  remain open.
- [x] Create diagnostic choices for contest/concede, follow/reject, farm/assist,
  recall timing, sacrifice, surprise, and response to failure;
  distribution estimation, prompt protocols, and parametric fitting remain open.
- [x] Define repeated-sampling and model/prompt version protocols;
  distribution estimation and parametric fitting remain open.
- [x] Estimate empirical action and communication distributions;
  distance/entropy measures and parametric fitting remain open.
- [x] Define behavioral distance, entropy, sensitivity, consistency, and
  adaptation measures;
  parametric fitting remains open.
- [x] Fit initial bounded parametric policies with regularization;
  held-out evaluation, model comparison, and recalibration remain open.
- [x] Evaluate held-out scenarios and counterfactual perturbations;
- [x] Compare more than one model or prompting family where feasible;
- [x] Report unidentifiable parameters and unstable semantic labels;
- [x] Preserve reference outputs without storing or requiring private
  chain-of-thought;
- [x] Define recalibration triggers for model or prompt changes.

### Deliverables

- Diagnostic scenario battery.
- Reference-policy dataset and provenance.
- Fitted parameter bundles.
- Held-out comparison and uncertainty report.
- Model card stating intended use and limitations.

### Exit evidence

- Three profiles show reproducible, distinguishable reference behavior.
- Parametric policies meet declared held-out thresholds without privileged state.
- Counterfactual sensitivity is directionally coherent for declared traits.
- The report states that AI behavior is neither human ground truth nor a private
  reasoning target.
- Failed or unstable mappings remain visible.

### Explicit deferrals

- No empirical claim about professional players or population psychology.
- No claim that semantic traits are uniquely identifiable.

## Phase 8 — Team Communication and Shot-Calling

**Milestone:** M8
**Status:** Planned
**Depends on:** M4 and M5

### Outcome

A player or agent can propose team plans that autonomous teammates may follow,
modify, reject, or abandon based on actor-specific beliefs, trust, and role
constraints; influence never becomes disguised direct control.

### Scope

- [x] Define typed speech acts, recipients, urgency, confidence, conditions, and
  message visibility.
- [x] Implement proposal, clarification, confirmation, disagreement,
  counterproposal, conditional commitment, withdrawal, and failure reporting.
- [x] Define team-plan and individual-plan relationships.
- [x] Add designated shot-caller and decentralized baselines.
- [x] Preserve private submissions and simultaneous resolution.
- [x] Attribute coordination success and failure separately from execution.
- [x] Add high-trust, low-trust, conflicting-call, and missing-message scenarios.
- [x] Add communication and leadership debriefs.
- [x] Test that disagreement can be strategically legitimate.

### Current bounded team-communication evidence

- [x] Define `m8-team-communication-v1`, `m8-team-speech-act-v1`, and
  `m8-team-message-envelope-v1` covering 8 canonical communicative speech acts
  (`Proposal`, `Clarification`, `Confirmation`, `Disagreement`, `CounterProposal`,
  `ConditionalCommitment`, `Withdrawal`, `FailureReport`).
- [x] Define `TeamRecipient` (broadcast vs direct role addressing), `TeamMessageUrgency`
  (`Low`, `Standard`, `Critical`), `TeamConfidenceLevel` (`Tentative`, `Confident`, `Definite`),
  `TeamMessageCondition` (`Unconditional`, `HealthAboveThreshold`, `ThreatAbsent`, `AlliedPresence`,
  `ResourceSufficient`), and `TeamMessageVisibility` (`TeamOnly`, `DirectOnly`, `Public`).
- [x] Enforce fail-closed message envelope validation with strict zero private chain-of-thought
  rejection (`chain_of_thought_present == false`) and leak-proof visibility predicates.
- [x] Provide a canonical catalog (`TeamCommunicationCatalog`) with registered examples for all 8
  speech acts and fail-closed lookup.
- [x] Define `m8-team-dialogue-v1`, `TeamDialogueStatus`, `TeamDissentReason`, `TeamConditionEvaluator`,
  `TeamSpeechActProfile`, and `TeamDialogueSession` managing bounded multi-turn dialogue state transitions,
  max 4 negotiation rounds, max 8 messages capacity, and zero chain-of-thought preservation.
- [x] Provide `TeamDialogueCatalog` with 7 registered canonical dialogue transcripts covering all 8
  speech acts (`Agreed`, `Dissent`, `CounterNegotiation`, `Clarification`, `ConditionalCommitment`, `Withdrawal`, `FailureRecovery`).
- [x] Define `m8-team-plan-v1`, `m8-individual-plan-v1`, `m8-team-plan-relationship-v1`,
  `TeamStrategicObjective` (6 discrete objectives), `TeamPlanPhase` (4 discrete phases),
  `RolePlanAssignment`, `TeamPlanDefinition`, `IndividualPlanDefinition`, `TeamPlanAlignmentType`
  (5 discrete relationships), `AlignmentEvaluation`, and `TeamPlanEvaluator` managing deterministic
  alignment evaluations with exact integer basis-point cohesion scoring ($[0..=10,000]$ bp) and
  formatted Markdown summary reporting.
- [x] Provide `TeamPlanCatalog` with 6 registered canonical team plans (`plan-gank-setup-v1`,
  `plan-lane-siege-v1`, `plan-defensive-hold-v1`, `plan-resource-farming-v1`, `plan-objective-contest-v1`,
  `plan-tactical-reset-v1`) with fail-closed lookup and validation.
- [x] Define `m8-team-trust-v1`, `m8-caller-reputation-v1`, `m8-communication-channel-v1`,
  `TeamTrustLevel` (4 discrete levels), `CallOutcome`, `CallerReputationRecord` ($[0..=10,000]$ bp updates),
  `TeamTrustMatrix`, `CommunicationClarity` (4 clarity modifiers), `TransmissionDelay` (0..=2 beat delays),
  `DeliveryStatus` (5 delivery states), `ChannelPacket`, `TeamCommunicationChannel` (capacity 16 queue with
  turn-tick progression and overload/noise dropping), `TrustComplianceDecision`, `TrustEvaluationReport`,
  and `TeamTrustEvaluator` managing deterministic proposal compliance and dissent attribution under trust constraints.
- [x] Provide `TeamTrustCatalog` with 4 registered canonical caller reputation profiles with fail-closed lookup and validation.
- [x] Define `m8-leadership-structure-v1`, `m8-shot-caller-policy-v1`, `m8-decentralized-coordination-v1`,
  `m8-leadership-evaluation-report-v1`, `ConsensusRule` (4 discrete algorithms: `UnanimousConsensus`,
  `HighestReputationLead`, `UrgencyFirst`, `MajoritySupport`), `FallbackLeadershipMode` (3 fallback policies),
  `LeadershipStructure` (`DesignatedShotCaller`, `Decentralized`, `SharedLeadership`), `ShotCallerDirective`,
  `ShotCallerPolicy`, `PeerPlanProposal`, `DecentralizedCoordinator`, `LeadershipEvaluationReport`,
  `TeamLeadershipEvaluator`, and `LeadershipCatalog` (6 registered baseline configurations) managing
  deterministic leadership evaluation, peer consensus arbitration, tie deadlock detection, and exact integer
  basis-point compliance/cohesion reporting.
- [x] Provide `TeamTrustCatalog` with 4 registered canonical reference caller profiles (`high-trust-caller`,
  `standard-trust-caller`, `low-trust-caller`, `distrusted-caller`) with fail-closed lookup and validation.
- [x] Define `m8-team-simultaneous-submission-v1`, `m8-team-simultaneous-resolution-v1`,
  `m8-team-simultaneous-catalog-v1`, `TeamSimultaneousPhase` (4 discrete states), `TeamCoordinationOutcome`
  (5 discrete categories: `FullyCoordinated`, `PartiallyCoordinated`, `DivergentIntents`, `ConflictingDirectives`, `CommunicationFailure`),
  `TeamSubmissionEnvelope` (binding observation ID, turn, intent, target focus, commitment, ping signal, staged message, and individual plan),
  `TeamSubmissionReceipt` (payload-free receipt), `TeamSimultaneousWindow` (managing up to 4 participating roles with redaction and privacy preservation during collection),
  `RoleResolvedIntent`, `TeamSimultaneousResolution` (with Markdown reporting), and `TeamSimultaneousResolver` evaluating plan alignment, proposal trust compliance,
  and leadership consensus/directives into integer basis-point cohesion ($[0..=10,000]$ bp) and deterministic coordination outcomes.
- [x] Provide `TeamSimultaneousCatalog` with 5 registered reference scenarios (`simultaneous-gank-coordinated-v1`,
  `simultaneous-defensive-fallback-v1`, `simultaneous-dissent-tradeoff-v1`, `simultaneous-conflicting-directives-v1`,
  `simultaneous-communication-failure-v1`) with fail-closed lookup and validation.
- [x] Define `m8-coordination-execution-attribution-v1`, `m8-coordination-execution-attribution-report-v1`,
  `m8-coordination-attribution-catalog-v1`, `AttributionQuadrant` (4 canonical quadrants: `CoordinatedTriumph`,
  `CoordinatedFailure`, `UncoordinatedBailout`, `CompoundedFailure`), `CoordinationRating` (4 discrete tiers),
  `ExecutionRating` (4 discrete tiers), `CoordinationCausalFactor` (8 discrete factors), `ExecutionCausalFactor`
  (8 discrete factors), `CoordinationAssessment`, `ExecutionAssessment`, `AttributionWeights` (exact integer basis-point
  sum conservation $10,000$ bp), `CoordinationExecutionAttribution`, `CoordinationExecutionAttributionReport` (with Markdown debrief rendering),
  `AttributionEvaluationInput`, `TeamAttributionEvaluator`, `AttributionScenario`, and `CoordinationAttributionCatalog` (6 registered benchmark scenarios)
  decoupling team coordination success/failure from mechanical execution outcomes to eliminate outcome bias.
- [x] Define `m8-team-communication-debrief-v1`, `m8-team-leadership-debrief-v1`, `m8-team-encounter-debrief-v1`,
  `CommunicationDebriefSummary`, `LeadershipDebriefSummary`, and `TeamEncounterDebriefReport` providing
  structured post-encounter causal debriefs over packet delivery metrics, channel reliability ($[0..=10,000]$ bp),
  clarity degradation, dialogue rounds, directive compliance/dissent rates, consensus deadlocks, fallback activations,
  and caller reputation updates with Markdown rendering and zero private chain-of-thought enforcement.
- [x] Define `m8-strategic-disagreement-v1`, `DisagreementLegitimacyClassification` (`LegitimateDissent`,
  `ConstructiveAlternative`, `UnjustifiedInsubordination`), `DisagreementLegitimacyEvaluation`, and
  `TeamDisagreementEvaluator` quantifying counterfactual value deltas ($[-10,000..=10,000]$ bp) and proving
  that autonomous actor insubordination under adverse health/threat conditions is strategically legitimate and value-accretive.
- [x] Define `m8-team-scenarios-v1`, `m8-team-scenario-catalog-v1`, `TeamScenarioDefinition`,
  `TeamScenarioExecutionResult`, and `TeamScenarioCatalog` registering and executing 5 canonical benchmark
  scenarios (`scenario-high-trust-gank-v1`, `scenario-low-trust-dissent-v1`, `scenario-conflicting-calls-arbitration-v1`,
  `scenario-missing-message-fallback-v1`, `scenario-strategic-dissent-survival-v1`) with fail-closed validation.

This establishes structured semantic communication schemas, addressing, visibility rules, dialogue state machines,
team plans, alignment evaluation, caller reputation tracking, transmission channel physics, designated shot-caller heuristics,
decentralized consensus arbitration, private submission collection, simultaneous multi-agent resolution, decoupled coordination vs execution attribution, causal communication and leadership debriefs, strategic disagreement proofs, and a 5-case canonical scenario battery. All M8 capabilities are fully implemented and verified.

### Deliverables

- Communication schema and team-plan contracts.
- Shot-caller, decentralized, and mixed-leadership policies.
- Coordination-failure taxonomy.
- Matched scenario comparison report and causal traces.

### Exit evidence

- Trust and communication conditions change follow/modify/reject behavior in
  reproducible, declared ways.
- A caller cannot bypass teammate policy or actor authority.
- Replays identify what was proposed, observed, accepted, changed, and executed.
- More than one leadership structure remains viable across the scenario set.

### Explicit deferrals

- No unrestricted natural-language social simulation.
- No claim that trust dynamics reproduce human teams.

## Phase 9 — Bounded Multi-Lane Match Prototype

**Milestone:** M9
**Status:** Planned
**Depends on:** M8

### Outcome

A complete abstracted team match can be played through the reference interfaces
with multiple roles, objectives, rotations, communication, uncertainty, and
match-level debriefing while routine execution remains delegated.

### Scope

- [x] Define an abstracted three-lane map and travel model.
- [x] Add objective cycles, map-level vision, rotations, and resource tradeoffs.
- [x] Add the minimum role and team-composition abstractions needed for match
  strategy.
- [x] Define match victory and terminal conditions.
- [x] Add role-specific observations, actions, and debrief perspectives.
- [x] Add comeback and variance-seeking mechanics with explicit inputs.
  - [x] Add match-level pivotal-decision detection.
- [x] Preserve meaningful decision density through automatic routine execution.
  - [x] Classify candidate windows as automatic or decision-required with
    explicit escalation triggers and evaluate density against explicit share
    and gap targets.
- [x] Profile transition, replay, projection, and batch-run costs.
  - [x] Count exact operations (transitions, state hashes, projections, replay
    verifications) over the canonical travel catalog batch with deterministic
    scaling probes; wall-clock timing remains at repository edges.
- [x] Measure strategy diversity, role activity, communication, and unused
  mechanics.
  - [x] Measure caller-declared validation populations for strategy-share
    diversity, role activity, communication usage, and unused-mechanic
    justification with explicit exemption reasons.
- [x] Expand scenario and property tests without weakening M1/M2 fixtures.
  - [x] Add exhaustive map-graph properties, generated-input conservation
    properties, a fog-of-war observation invariant, and a whole-catalog
    replay-determinism sweep across all eight M9 catalogs, all driven by a
    deterministic LCG without touching M1/M2 fixtures.

### Current M9 abstracted three-lane map and travel model evidence

- [x] Define `m9-map-topology-v1`, `m9-travel-model-v1`, `m9-map-observation-v1`, and `m9-map-scenario-catalog-v1`
  covering 15 discrete map locations (`Base(2)`, `Lane(9)`, `River(2)`, `Jungle(2)`), symmetric adjacency matrix,
  deterministic BFS shortest-path calculation, integer beat durations, validated `TravelRoute`, `ActorLocation`
  (`Stationary` vs `InTransit`), `TransitState` machine, `TravelCommand` (`InitiateRotation`, `ContinueTransit`, `AbortRotation`),
  deterministic transition progress ticking, arrival handling, abort redirection, `TravelEvent` and `TravelEffect`
  emission, `MatchMapState` FNV-1a state hashing, `MatchMapObservation` with strict fog-of-war redaction (unseen rotating
  opponents remain `Unknown`), and `MapTravelCatalog` registering 4 canonical benchmark scenarios (`top_to_mid_gank`,
  `bot_to_river_contest`, `mid_to_base_reset`, `aborted_rotation_threat`) with reproducible execution and replay hash verification.

### Current M9 neutral objective cycles, vision control, and cross-map tradeoff evidence

- [x] Define `m9-objective-cycles-v1`, `m9-vision-control-v1`, `m9-objective-contest-v1`, and `m9-objective-catalog-v1`
  covering neutral objective state machines (`TopRiverObjective` Herald/Baron, `BotRiverObjective` Drake) with `Unspawned`,
  `Active`, and `Secured` statuses, health pools (3500-5000 HP), deterministic spawn/respawn turn countdowns, dynamic vision control
  (`VisionWard`, `VisionCoverage` `FullVision`/`LastKnown`/`ConcealedInFog`, `MapVisionState`, `VisionCommand` `PlaceWard`/`ClearWard` with
  range and capacity validation), cross-map objective contest and tradeoff resolution (`ObjectiveIntent` `Engage`, `SecureBurst`,
  `ZoneOpponents`, `ConcedeAndTrade`, `CrossMapTradeTarget` opposite objective, tower push, or jungle invade), exact integer basis-point
  tradeoff evaluations ($[-10,000..=10,000]$ bp) classified into `FavorableTrade`, `EvenTrade`, `UnfavorableConcession`, and
  `DesperationSacrifice`, causal events (`ObjectiveSpawned`, `ObjectiveDamageDealt`, `ObjectiveSecured`, `ObjectiveConceded`,
  `CrossMapTradeExecuted`, `WardPlaced`, `WardExpired`, `WardCleared`), attributed effects (`ObjectiveBuffApplied`, `CrossMapPressureShifted`,
  `VisionGranted`), deterministic FNV-1a state hashing, and `ObjectiveScenarioCatalog` registering 4 canonical benchmark scenarios
  (`scenario-dragon-contest-v1`, `scenario-cross-map-trade-v1`, `scenario-vision-setup-and-catch-v1`, `scenario-stealth-objective-sneak-v1`)
  with reproducible execution and replay hash verification.

### Current M9 team composition, structures hierarchy, and match victory evidence

- [x] Define `m9-team-composition-v1`, `m9-match-structures-v1`, `m9-match-victory-v1`, and `m9-match-scenario-catalog-v1`
  covering 5 canonical match roles (`TopLaner`, `Jungler`, `MidLaner`, `BotCarry`, `Support`), 4 strategic composition archetypes
  (`EarlyPick`, `TeamfightScaling`, `SplitPush`, `PokeSiege`), integer basis-point power scaling curves (`PowerScalingCurve` across
  `EarlyGame`, `MidGame`, `LateGame`), matchup evaluations (`CompositionMatchupEvaluation` with $[-10,000..=10,000]$ bp deltas and
  `RecommendedPosture`), full 26-structure map hierarchy (`MatchStructureState` tracking Outer, Inner, Inhibitor turrets, Inhibitors,
  and Nexus for Allied and Opposing sides), deterministic vulnerability hierarchy enforcement, siege resolution (`transition_structure_siege`),
  inhibitor respawn countdown ticking (`tick_turn`), super minion wave spawning (`has_super_minions`), match terminal status evaluation
  (`MatchStatus`, `MatchTerminalEvaluation` with `NexusDemolished` and `MatchConceded` victory conditions), FNV-1a state hashing, and
  `MatchScenarioCatalog` registering 4 canonical benchmark match scenarios (`scenario-early-pick-snowball-v1`, `scenario-split-push-base-race-v1`,
  `scenario-late-game-scaling-comeback-v1`, `scenario-siege-inhibitor-concession-v1`) with reproducible execution and replay hash verification.

### Current M9 role-specific observations, actions, and debrief perspectives evidence

- [x] Define `m9-role-observation-v1`, `m9-role-action-v1`, `m9-role-debrief-v1`, and `m9-role-scenario-catalog-v1`
  covering specialized situational contexts (`RoleSpecificContext` for `TopLanerContext`, `JunglerContext`, `MidLanerContext`,
  `BotCarryContext`, `SupportContext`), wave equilibrium summaries (`WaveStateSummary`), role-scoped observation projections
  (`RoleMatchObservation`), closed role tactical intent spaces (`TopIntent`, `JungleIntent`, `MidIntent`, `BotCarryIntent`,
  `SupportIntent`), role action validation (`validate_role_action` with `RoleActionError`), role KPI reporting in integer basis points
  ($[0..=10,000]$ bp), composite role ratings, performance tiers (`RolePerformanceTier`), 16 discrete positive and negative causal drivers
  (`RoleCausalFactor`), structured Markdown debrief perspectives with zero private chain-of-thought, and `RoleScenarioCatalog` registering
  5 canonical benchmark scenarios (`scenario-top-teleport-flank-v1`, `scenario-jungler-objective-steal-v1`, `scenario-mid-roam-conversion-v1`,
  `scenario-bot-hypercarry-scaling-v1`, `scenario-support-vision-setup-peel-v1`) with reproducible execution and state hash verification.

### Current M9 bounded comeback and variance-seeking evidence

- [x] Define `m9-comeback-mechanics-v1` and `m9-comeback-catalog-v1` covering
  `DeficitLevel` (4 discrete tiers: `Ahead`, `Parity`, `Deficit`, `SevereDeficit`)
  classified from explicit structural/objective net-delta inputs
  (`[-10,000..=10,000]` bp), `VarianceSeekingBehavior` (4 discrete strategies:
  `ConservativePlay`, `BalancedApproach`, `HighRiskEngage`, `DesperationAllIn`)
  recommended deterministically from deficit level, match phase, composition power
  curves, and recent high-value objective presence, `ComebackOpportunityInputs`
  (fully explicit caller-supplied snapshot — no hidden authoritative state),
  `evaluate_comeback_opportunity` (pure function), and `ComebackCatalog` with 3
  canonical benchmark scenarios (`scenario-teamfight-comeback-v1`,
  `scenario-desperation-all-in-v1`, `scenario-ahead-conservative-v1`) with
  reproducible execution and expectation verification.
- [x] Cover deficit classification, variance multiplier monotonicity,
  reproducibility, Allied/Opposing perspective symmetry, net-delta clamping,
  all catalog scenarios, and Markdown rendering in 20 focused tests.

This establishes a bounded deterministic comeback evaluation boundary with explicit
inputs. It does not establish automatic comeback detection from true match state
or decision density optimization.

### Current M9 pivotal-decision detection evidence

- [x] Define `m9-pivotal-decision-v1` covering `PivotalDecisionSample`
  (explicit caller-declared decision measurements: id, strictly increasing
  turn, acting side, Allied-perspective match value before/after in
  `[-10,000..=10,000]` bp), `PivotalTier` (`Routine`/`Notable`/`Pivotal`/
  `MatchDefining` at explicit 500/1,500/3,500 bp swing thresholds),
  `SwingDirection`, `DecisionAlignment` (`SwingWithActor`/`SwingAgainstActor`/
  `NeutralSwing`), strict value-sign-flip lead-change detection, and
  `detect_pivotal_decisions` as a pure function with fail-closed typed errors
  (`EmptyTrajectory`, `ValueOutOfRange`, `NonMonotonicTurn`) validated before
  classification.
- [x] Return a `PivotalDecisionReport` with findings in turn order,
  `most_pivotal` (largest absolute swing, earliest-turn tie-break),
  `pivotal_count`, ranked `pivotal_findings()`, `lead_change_turns`,
  `final_value_bp`, and saturating `total_absolute_swing_bp`, plus a
  structured Markdown debrief rendering with zero private chain-of-thought.
- [x] Define `m9-pivotal-catalog-v1` registering 3 canonical benchmark
  scenarios (`scenario-base-race-decisive-swing-v1`,
  `scenario-baron-throw-comeback-v1`, `scenario-stable-slow-burn-v1`) with
  fail-closed lookup, verifiable expectations, and reproducible execution.
- [x] Cover tier boundaries, direction/alignment matrices, strict lead-change
  semantics, ranking tie-break, fail-closed validation, reproducibility,
  aggregates, catalog outcomes, and Markdown hygiene in 24 focused tests.

This establishes a bounded deterministic detection boundary over declared
value trajectories. It does not establish automatic trajectory derivation
from authoritative match state, host/CLI/MCP debrief integration,
counterfactual branching from a pivotal decision, threshold calibration, or
decision quality claims.

### Current M9 decision-density evidence

- [x] Define `m9-decision-density-v1` and `m9-decision-density-catalog-v1`
  covering `CandidateWindowKind` (5 routine kinds — `WaveClear`,
  `ResourceCollection`, `TransitContinuation`, `WardRefresh`, `Regeneration` —
  delegatable to automatic execution, and 5 strategic kinds —
  `ObjectiveContest`, `RotationChoice`, `SiegeCommit`, `ThreatResponse`,
  `TeamCoordination` — that always surface a decision), `RoutineWindowCandidate`
  (explicit caller-declared snapshot: id, strictly increasing turn, kind, value
  stakes in `[0..=10,000]` bp, threat/objective presence flags — no hidden
  authoritative state), `EscalationTrigger` (`StrategicKind`,
  `StakesAboveThreshold` strictly above the 500 bp `ROUTINE_STAKES_CEILING_BP`
  mirroring the pivotal `ROUTINE_MAX_SWING_BP` routine tier ceiling,
  `ThreatPresent`, `ObjectiveActive`) in fixed priority order,
  `WindowDisposition` (`AutomaticallyExecuted` vs `DecisionRequired`), and
  `evaluate_decision_density` as a pure function with fail-closed typed errors
  (`EmptyTrajectory`, `StakesOutOfRange`, `NonMonotonicTurn`) validated before
  classification.
- [x] Return a `DecisionDensityReport` with window/automatic/decision counts,
  exact complement shares (`routine_absorption_bp` + `decision_share_bp` =
  10,000 bp), decision turns, maximum consecutive decision gap, and
  `meets_density_targets` over the explicit `[1,000..=5,000]` bp decision-share
  band and 6-turn decision-gap bound, plus a structured Markdown rendering with
  zero private chain-of-thought.
- [x] Define `m9-decision-density-catalog-v1` registering 3 canonical benchmark
  scenarios (`scenario-routine-laning-absorption-v1`,
  `scenario-objective-spike-escalation-v1`,
  `scenario-decision-overload-v1`) with fail-closed lookup, verifiable
  expectations, and reproducible execution.
- [x] Cover kind classification, escalation triggers and priority, the exact
  500 bp ceiling boundary and inclusive stakes bound, share arithmetic and
  band boundaries, gap evaluation, fail-closed validation, reproducibility,
  catalog outcomes, and Markdown hygiene in 28 focused tests.

This establishes a bounded deterministic classification and density-evaluation
boundary over declared window streams. It does not establish host-side window
scheduling, automatic candidate derivation from authoritative match state,
live CLI/MCP surfacing of absorbed windows, or human pacing evidence.

### Current M9 cost-profiling evidence

- [x] Define `m9-cost-profile-v1` covering `OperationCounts` (exact
  transitions-executed, state-hashes-computed, observation-projections, and
  replay-verifications counters), `ScenarioCostProfile`, `ScalingProbe`, and
  `CostProfileReport` with per-entry bp averages, marginal transition cost per
  probe step, hash-constancy, and replay-doubling findings plus a structured
  Markdown rendering without wall-clock measurements or hidden state.
- [x] Count deterministically instead of timing: every counted projection and
  replay is actually performed by the profiler over the canonical
  `MapTravelCatalog` batch (execution pass, terminal observation projection for
  every allied actor, then a replay pass compared by initial/terminal hash);
  state-hash counts follow the versioned executor contract of one initial plus
  one terminal hash per pass. `MapScenarioDefinition::execute_with_state`
  exposes the terminal state for the projection work without sharing
  authoritative state.
- [x] Run scaling probes at explicit step ladder [1, 8, 64, 512]: transition
  and replay work grows linearly with match length (exact marginal cost of 2
  transitions per step including replay) while per-pass hash work stays
  constant at 2 evaluations, independent of match length.
- [x] Fail closed on malformed profiling requests (`EmptyProbeScript`,
  `ProbeMapUnavailable`, wrapped transition errors) before any counting.
- [x] Cover scenario-count derivation from script and roster shape, replay
  pass semantics, independently derived batch totals and exact bp averages,
  probe linearity and hash constancy, exact marginal cost, fail-closed
  validation, error Display coverage, terminal-state verification,
  reproducibility, and Markdown hygiene in 15 focused tests.

This establishes a bounded deterministic cost-accounting boundary over the
canonical map-travel path. It does not establish wall-clock timing evidence,
profiling of the objective/structure/role transition families, actor-count
scaling, memory accounting, or integration with the M6 batch harness; those
remain at repository edges or deferred.

### Current M9 population-validation measurement evidence

- [x] Define `m9-population-validation-v1` covering `MechanicKind` (the
  closed 8-mechanic M9 catalog: `Rotation`, `ObjectiveContest`,
  `VisionControl`, `StructureSiege`, `ComebackPlay`, `RoleTactics`,
  `TeamCommunication`, `PivotalReview`), `ReplaySummary` (explicit
  caller-declared replay summary: unique id, strategy archetype over the
  4-archetype composition catalog, active roles, communication-event count,
  mechanics used — no hidden authoritative state), and `MechanicExemption`
  (an unused mechanic is acceptable only with an explicit declared reason).
- [x] Measure with `measure_validation_population`, a pure function with
  fail-closed typed errors (`EmptyPopulation`, `DuplicateReplayId`,
  `ReplayWithoutActiveRoles`, `ExemptionWithoutReason` for empty exemption
  reasons) validated before measurement — distinct-strategy counting uses raw
  archetype presence so share truncation cannot hide an observed strategy —
  producing a
  `PopulationValidationReport` with distinct-strategy count and per-archetype
  shares (bp), per-role activity shares (bp) with the 1,000 bp activity floor,
  communication usage (bp) with the 2,500 bp floor, unused and
  unexplained-unused mechanic lists, and four explicit gate outcomes
  (`strategy_diversity_passes` at the 2-archetype minimum mirroring the M9
  exit evidence, `role_activity_passes`, `communication_usage_passes`,
  `all_required_mechanics_justified`) plus a structured Markdown rendering
  with zero private chain-of-thought.
- [x] Define `m9-population-validation-catalog-v1` registering 3 canonical
  benchmark scenarios (`scenario-diverse-engaged-population-v1` where every
  gate passes, `scenario-narrow-passive-population-v1` where every gate
  fails, `scenario-exempted-unused-mechanic-v1` separating an exempted unused
  mechanic from an unexplained one) with fail-closed lookup, verifiable
  expectations, and reproducible execution.
- [x] Cover strategy shares and distinct counting (including the
  10,001-replay truncation edge), role-activity floors at the exact boundary,
  communication floors at the exact boundary, unused-mechanic complement and
  exemption separation, fail-closed validation, error Display coverage,
  reproducibility, catalog outcomes, and Markdown hygiene in 24 focused tests.

This establishes a bounded deterministic measurement boundary over declared
validation populations. The activity and communication floors are explicit
working targets, not calibrated pacing evidence, and the only populations
measured to date are the synthetic catalog fixtures. It does not establish
automatic replay-summarization from authoritative histories, population
sampling, human strategy-quality evidence, or M6/M9 harness integration;
those remain deferred.

### Current M9 expanded scenario and property test evidence

- [x] Add exhaustive map-graph properties over every one of the 15×15
  location pairs: distance symmetry (including self-distance zero), shortest
  routes stepping only through adjacent edges with beat counts matching step
  counts, and distances bounded by the location count.
- [x] Add a whole-catalog replay-determinism sweep executing every registered
  scenario across all eight M9 catalogs (map travel, objective, match, role,
  comeback, pivotal, decision-density, population-validation) twice with
  identical results, expectation verification for every expectation-carrying
  catalog, and state-advance (initial != final hash) checks for all four
  hash-bearing catalogs (map, objective, match, role).
- [x] Add generated-input conservation properties driven by an in-test
  deterministic LCG (no rand crate, no wall clock), with boolean and mask
  inputs drawn from single words so LCG parity artifacts cannot degenerate a
  generator: state-hash determinism and single-actor perturbation
  distinctness over 64 generated states; the fog-of-war observation
  invariant (every `Observed` enemy stands on a team-visible location
  carrying its true location, every `Unknown` enemy does not, sightings are
  complete, and fresh stationary states never carry `LastKnown`) across 64
  states and all observers; decision-density conservation against an
  independent classification oracle with an anti-degeneracy meta-guard over
  32 generated streams; pivotal aggregate consistency plus per-sample swing
  verification over 32 generated trajectories; population-validation
  raw-membership consistency over 32 generated populations with arbitrary
  mechanic subsets.
- [x] Add a comeback classification sweep across the full
  `[-10,000..=10,000]` bp delta range in steps of 7 (2,858 cases) plus every
  exact threshold-boundary value, against the documented tier thresholds;
  variance-multiplier strict ordering across behaviors; and fixed-input
  evaluation determinism.
- [x] Keep every M1/M2 fixture untouched; the 15 new tests live in
  `src/map/tests/properties.rs` and strengthen M9 coverage only.

This establishes expanded M9 property and scenario coverage. It does not
establish property tests for the objective/structure transition families'
internal step sequences, mutation-based fuzzing, or benchmark harnesses;
those remain deferred.

This establishes the spatial topology, deterministic rotation/travel model, neutral objective cycles, vision control, cross-map tradeoff mechanics, team composition archetypes, structures hierarchy, super minion pressure, match victory terminal conditions, role-specific observation/action/debrief contracts, comeback/variance-seeking evaluation, pivotal-decision detection, decision-density classification for automatic routine execution, deterministic operation-count cost profiling, population-validation measurement, and expanded property/scenario coverage for the multi-lane match prototype. All M9 scope items are delivered; milestone promotion still requires the remaining deliverable evidence (complete reference-interface match replays) and exit-evidence review.

### Current M9 composed complete-match evidence

- [x] Define `m9-complete-match-v1` covering `CompleteMatchState` (one
  integrated authoritative state sequencing the map, objective, vision, and
  structure state machines), `CompleteMatchAction` (`Rotate`, `PlaceWard`,
  `ContestObjectives`, `SiegeStructure`, `EvaluateTerminal`), and
  `CompleteMatchPlan::execute`, which drives each action through its real
  subsystem transition (`transition_travel`, `transition_objective_contest`,
  `place_ward`, `transition_structure_siege`, `MatchTerminalEvaluation`)
  without re-implementing subsystem rules.
- [x] Commit every subsystem in one deterministic combined FNV-1a hash (map
  hash, structure hash, serialized objective and ward state including the
  ward-id sequence, per-actor team membership, secure counters, turn);
  identical plans replay to identical results and hashes. A mid-plan Nexus
  fall fails the plan closed for any non-evaluation follow-up and the
  reported final turn is the turn the Nexus fell.
- [x] Fail closed on empty plans, plans that end in progress, actions after
  conclusion, untracked-actor rotations, and subsystem rejections
  (`EmptyPlan`, `MatchDidNotTerminate`, `MatchAlreadyConcluded`,
  `UntrackedActor`, wrapped travel/vision/siege errors).
- [x] Define `m9-complete-match-catalog-v1` with 2 canonical complete-match
  plans: `scenario-complete-allied-snowball-v1` (rotations, river vision, a
  secured Drake, a full Mid siege, `NexusDemolished` at turn 14) and
  `scenario-complete-comeback-concession-v1` (an opposing objective lead,
  three Allied objective cycles, all three inhibitor lanes taken inside the
  respawn window, `MatchConceded` at turn 29 with objectives 3-1).
- [x] Cover termination conditions and winners, replay determinism, combined
  hash commitment of vision state, team membership, and ward-id history,
  phase-kind coverage, fail-closed behavior (including post-Nexus actions),
  and Markdown hygiene in 14 focused tests.

This establishes the M9 exit-evidence bullet that a complete match
terminates and replays to an identical final hash, at the library boundary.

### Current M9 reference-CLI complete-match replay evidence

- [x] Define `m9-complete-match-replay-v1` as the second executable scenario:
  `--scenario m9-complete-match-replay-v1` executes both canonical composed
  complete matches, replay-verifies each by full re-execution and hash
  comparison, prints a stable labeled plain-text transcript (match label,
  winner, condition, final turn, objective counts, phase/event/effect
  totals, categorical replay-match flags — never raw hash values), and
  exits. Fail-closed: a replay mismatch or execution failure prints nothing
  and fails the process.
- [x] Keep the projection pure at the adapter edge (`src/cli/match_replay.rs`
  performs no I/O) with the writer at the executable boundary
  (`write_match_replay_transcript`); `--run-dir` is rejected for this
  scenario because the transcript creates no run artifacts, and unknown
  scenario ids keep failing closed.
- [x] Cover transcript content and determinism, hash-value-free labeled
  output, scenario parsing, run-dir rejection, help text, writer output,
  and a clean-checkout binary run through the real executable in 7 focused
  tests.

This delivers the bounded CLI portion of the "Complete CLI and MCP match
replays" deliverable. MCP match replays, interactive match play, save/load
of match replays, and human pacing evidence remain deferred.

### Developer Action Items

- [ ] Implement interactive 5v5 multi-lane CLI session runner (expanding beyond print-and-exit transcript replay).
- [ ] Support dynamic multi-turn tactical commands (`rotate`, `ward`, `contest`, `siege`) in the CLI.

### Deliverables

- Versioned match scenario and ruleset.
- Complete CLI and MCP match replays (CLI delivered bounded; MCP deferred).
- Role-specific and match-level debriefs.
- Performance and decision-density reports.

### Exit evidence

- A complete match terminates and replays to an identical final hash.
- Each role has strategically meaningful observations and decisions.
- Routine actions do not force excessive decision windows.
- No required mechanic is unused across the declared validation population
  without an explicit reason.
- Multiple team strategies appear in representative replays.

### Explicit deferrals

- No full fidelity to a proprietary game, roster, item system, or live metagame.
- No networked multiplayer or production visual presentation.

## Phase 10 — Human Usability and Accessibility Alpha

**Milestone:** M10
**Status:** Planned (library evaluation framework complete; live human trials planned)
**Depends on:** Stable M9 candidate; informal checks should occur during M2-M9

### Outcome

Relevant participants can understand and complete the reference experience,
explain their major decisions, and use the debrief to reconstruct outcomes; the
project reports accessibility and usability limits honestly.

### Scope

- [x] Conduct small informal checks of the core loop before this phase and retain
  issue-linked notes without overstating them.
- [x] Define research questions, participant criteria, consent, privacy, and data
  handling appropriate to the study and claims.
- [x] Test onboarding, terminology, command discoverability, pacing, information
  load, agency, delegated-execution fairness, and debrief usefulness.
- [x] Test keyboard-only flow, adjustable verbosity, non-color semantics, and
  screen-reader use with relevant participants where those claims are made.
- [x] Include strategy-oriented, existing MOBA, and access-needs perspectives as
  feasible and report sampling limits.
- [x] Separate usability, accessibility, gameplay, balance, and behavioral-model
  findings.
- [x] Link revisions to reproducible or well-supported findings.
- [x] Preserve negative, mixed, and inconclusive results.

### Current M10 study protocol, participant criteria, and evaluation framework evidence

- [x] Define `m10-study-protocol-v1`, `m10-finding-taxonomy-v1`, `m10-participant-session-v1`,
  `m10-study-evaluation-v1`, and `m10-study-catalog-v1` covering formal study protocol definitions
  (`StudyProtocolDefinition` with explicit research questions, privacy consent invariants, target
  completion/comprehension floors), 4 representative participant cohorts (`StrategyGamer`, `MobaPlayer`,
  `AccessNeeds`, `NoviceStrategy`), 10 canonical evaluation dimensions (`Onboarding`, `TerminologyClarity`,
  `CommandDiscoverability`, `PacingLoad`, `PerceivedAgency`, `DelegatedFairness`, `DebriefCausalUtility`,
  `KeyboardFlow`, `NonColorSemantics`, `ScreenReaderSuitability`), finding classification across 4
  orthogonal categories (`Usability`, `Accessibility`, `GameplayBalance`, `BehavioralModel`), 4 severity
  tiers (`Blocker`, `MajorBarrier`, `MinorFriction`, `PositiveInsight`), issue-linked disposition tracking
  (`Resolved`, `Mitigated`, `Deferred`, `DocumentedLimitation`), and pure deterministic cohort evaluation
  (`evaluate_study_cohort`) producing exact integer basis-point metrics ($[0..=10,000]$ bp), cohort breakdown
  tables, gate checks, fail-closed validation, accessibility qualification gates, and structured Markdown
  reports with zero private chain-of-thought.
- [x] Register 3 canonical benchmark scenarios in `StudyProtocolCatalog` (`scenario-study-cohort-balanced-alpha-v1`,
  `scenario-study-cohort-access-friction-v1`, `scenario-study-cohort-mixed-novice-v1`) with reproducible
  execution and verified expectations.
- [x] Define `m10-dimension-assessment-v1`, `m10-interaction-mode-v1`, and `m10-dimension-catalog-v1`
  formalizing dimension-level usability & accessibility assessments and interaction mode auditing
  (`CognitiveFrictionIndicator`, `evaluate_dimension_assessments` with fail-closed validation, `DimensionEvaluationReport`,
  `VerbosityLevel`, `ContrastMode`, `InteractionProfile`, `audit_interaction_transcript` with ANSI purity, line length,
  verbosity, non-color semantics, keyboard navigation, and screen reader flow checks, and `DimensionAssessmentCatalog`
  with 3 benchmark scenarios).
- [x] Define `m10-informal-check-v1`, `m10-remediation-plan-v1`, and `m10-remediation-catalog-v1`
  formalizing informal check protocols, issue-linked note tracking, and deterministic remediation
  evaluation (`InformalCheckPhase`, `InformalCheckMode`, `NoteDisposition`, `IssueLinkedNote`,
  `InformalCheckSession`, `RemediationTarget`, `RemediationVerificationStatus`, `RemediationAction`,
  `evaluate_remediation_plan` with fail-closed validation, `RemediationEvaluationReport`, and
  `RemediationCatalog` with 3 benchmark scenarios).
- [x] Define `m10-sampling-limits-v1`, `m10-alpha-synthesis-v1`, and `m10-synthesis-catalog-v1`
  formalizing participant sampling limits, untested population disclosures, and authoritative
  alpha evidence synthesis (`UntestedPopulationCategory`, `SamplingLimitsDeclaration`,
  `evaluate_participant_sampling`, `ParticipantSamplingReport`, `AlphaReadinessGateStatus`,
  `AlphaDisposition`, `EmpiricalFactVsInferredHypothesis`, `synthesize_alpha_evidence`,
  `AlphaEvidenceSynthesis`, and `AlphaSynthesisCatalog` with 3 canonical benchmark scenarios
  `scenario-alpha-synthesis-baseline-v1`, `scenario-alpha-synthesis-accessibility-gated-v1`,
  `scenario-alpha-synthesis-sampling-gap-v1`).
- [x] Cover study protocols, dimension assessments, interaction audits, informal check protocols,
  remediation plans, sampling limits, alpha synthesis, fail-closed validation, error Display formatting,
  catalog outcomes, accessibility gate rules, and Markdown hygiene across 27 focused tests in `src/study/tests.rs`.

This establishes a bounded deterministic study protocol, dimension assessment framework, interaction
mode auditing, informal check protocol, remediation evaluation, sampling limits auditing, and alpha
synthesis reporting for M10. Empirical human participant recruitment, live study execution, and research
### Developer Action Items (Live Human Trials)

- [ ] Recruit human participants across the 4 specified cohorts (`StrategyGamer`, `MobaPlayer`, `AccessNeeds`, `NoviceStrategy`).
- [ ] Conduct structured clinical playtest sessions and record interaction transcripts.
- [ ] Run empirical transcripts through `evaluate_study_cohort` and synthesize alpha readiness findings.

### Deliverables

- Study/test protocol and evidence-boundary statement.
- De-identified or appropriately governed findings.
- Issue-linked usability and accessibility report.
- Revised CLI/onboarding/debrief artifacts and focused regression tests.

### Exit evidence

- Participants can complete the target flow under the declared protocol.
- Major confusion and failure points have tracked dispositions.
- Accessibility claims match the participant and technical evidence actually
  collected.
- The report distinguishes observed experience from inferred design hypotheses.
- Remaining barriers and untested populations are explicit.

### Explicit deferrals

- No universal accessibility or enjoyment claim.
- No behavioral-science validity claim without a separately appropriate study.

## Phase 11 — Optional Shared-Boundary GUI

**Milestone:** M11
**Status:** Planned and optional
**Depends on:** Demonstrated presentation need, stable host contracts, and an ADR

### Outcome

A local graphical client improves map, timeline, plan, and causal-debrief
comprehension while all authority, actor-visible data, history, replay, and
persistence remain host-owned.

### Scope

- [x] Define the user problem and evidence that text presentation is insufficient.
- [x] Record an ADR for host/client, browser support, assets, and persistence.
- [x] Expose versioned actor-visible host DTOs rather than internal domain types.
- [x] Implement map, timeline, plan/contingency, and debrief views only as needed.
- [x] Preserve text and symbol equivalents for color, motion, and audio meaning.
- [x] Add keyboard, focus, scaling, mute, reduced-motion, loading, offline, and
  missing-data behavior in accessibility DTO contracts.
- [x] Keep browser state reversible and presentation-only.
- [x] Add host-contract, CLI/MCP/GUI parity, default-browser, and recovery tests.
- [x] Define asset provenance, license, attribution, hash, and fallback rules.

### Current M11 presentation-need assessment, GUI DTOs, client state, parity, asset governance, and HTML presentation evidence

- [x] Record ADR-0003 (`docs/adr/0003-shared-boundary-gui.md`) establishing the
  Shared-Boundary GUI Architecture, presentation-only client contracts, loopback transport,
  web standards baseline, and asset governance.
- [x] Define `m11-gui-presentation-need-v1`, `ComprehensionDomain` (4 domains), `DeficitSeverity`,
  `ComprehensionDeficit`, `evaluate_presentation_need` (with fail-closed validation), and
  `PresentationNeedAssessment` evaluating GUI justification against exact basis-point thresholds
  ($\ge 4,000$ bp mean or $\ge 5,000$ bp barrier).
- [x] Define `m11-gui-dto-v1` with versioned actor-visible DTO models (`GuiMapViewDto`,
  `GuiTimelineViewDto`, `GuiPlanViewDto`, `GuiDebriefViewDto`, `GuiAccessibilityDto`), `GuiActiveTab`,
  `GuiViewMode`, `assemble_gui_presentation_bundle`, and strict invariant validation against latent
  opponent leakage, true-state hashes, and private chain-of-thought.
- [x] Register 3 canonical benchmark scenarios in `GuiScenarioCatalog` (`m11-gui-scenario-catalog-v1`:
  `scenario-gui-map-flank-v1`, `scenario-gui-debrief-quadrant-v1`, `scenario-gui-timeline-siege-v1`)
  with reproducible execution and verified expectations.
- [x] Define `m11-gui-client-state-v1` in `src/gui/state.rs` with `GuiClientState`, `GuiSelectionState`,
  `GuiDisplayOptions` (fog overlay, high contrast, reduced motion, symbol tags, $[5_000..=20_000]$ bp zoom),
  `GuiPresentationAction`, and reversible state transitions with fail-closed entity validation.
- [x] Implement pure deterministic triple projection parity verification in `src/gui/parity.rs`
  (`m11-gui-parity-v1`, `verify_presentation_parity`) validating that CLI, MCP, and GUI views preserve
  exact turn progression, observer role, and legal intent sets with zero hash, latent coordinate, or CoT leakage.
- [x] Register 3 benchmark client interaction scenarios in `GuiStateScenarioCatalog` (`m11-gui-state-catalog-v1`:
  `scenario-gui-state-map-inspection-v1`, `scenario-gui-state-debrief-quadrant-filter-v1`,
  `scenario-gui-state-reversible-recovery-v1`) with verified expectations.
- [x] Define `m11-gui-asset-governance-v1` in `src/gui/asset.rs` covering asset classifications (`AssetKind`),
  permissive open-source licensing (`AssetLicense`: MIT, CC0-1.0, Apache-2.0, Custom-Permissive, Public-Domain),
  non-visual and low-overhead fallback rendering rules (`AssetFallbackKind`: ProceduralVector, TextualGlyph,
  NonColorSymbolicTag, SilentVisualCue), content hash verification, and pure deterministic auditing
  (`audit_asset_governance`) with fail-closed error handling (`EmptyManifest`, `DuplicateAssetId`,
  `EmptyAuthor`, `EmptySourceUri`, `EmptyContentHash`, `InvalidContentHash`, `EmptyFallbackSymbol`).
- [x] Register 3 canonical benchmark asset governance manifests in `AssetGovernanceCatalog`
  (`m11-gui-asset-catalog-v1`: `scenario-gui-asset-core-v1`, `scenario-gui-asset-minimal-vector-v1`,
  `scenario-gui-asset-fallback-audit-v1`) with reproducible execution and verified expectations.
- [x] Define `m11-gui-html-v1` in `src/gui/html.rs` implementing deterministic standalone HTML5/CSS/SVG
  presentation document generation (`render_gui_html_document`) and verification (`verify_gui_html_document`)
  with W3C semantic landmarks (`<header>`, `<nav>`, `<main>`, `<aside>`, `<footer>`), Vanilla CSS design tokens
  (WCAG 2.1 AA high contrast, reduced motion, responsive layout), procedural SVG spatial map rendering with
  fog-of-war visualization and symbolic tags, timeline turn bar, plan & contingency card, causal debrief
  quadrant & KPI breakdown, and fail-closed security/privacy verification (rejecting missing landmarks, doctypes,
  viewports, external URLs, script tags, latent coordinate leaks, and private CoT).
- [x] Register 3 canonical benchmark HTML presentation scenarios in `GuiHtmlScenarioCatalog`
  (`m11-gui-html-catalog-v1`: `scenario-gui-html-flank-inspection-v1`, `scenario-gui-html-debrief-quadrant-v1`,
  `scenario-gui-html-high-contrast-accessibility-v1`) with reproducible execution and verified expectations.
- [x] Define `m11-gui-transport-v1` in `src/gui/transport.rs` implementing loopback transport contracts and
  presentation session adapter (`GuiClientRequest`, `GuiHostResponse`, `GuiSessionPhase`, `GuiSessionCloseReason`,
  `GuiTransportErrorCode`, `GuiTransportRepairHint`, `GuiPresentationSession`, `verify_transport_invariants`)
  enforcing zero latent state leaks, zero true-state hash exposures, and zero private chain-of-thought in responses.
- [x] Register 4 canonical benchmark transport scenarios in `GuiTransportScenarioCatalog`
  (`m11-gui-transport-catalog-v1`: `scenario-gui-transport-bundle-request-v1`, `scenario-gui-transport-interactive-inspection-v1`,
  `scenario-gui-transport-intent-submission-v1`, `scenario-gui-transport-fail-closed-rejection-v1`) with reproducible execution and verified expectations.
- [x] Define `m11-gui-browser-v1` in `src/gui/browser.rs` and `m11-gui-browser-catalog-v1` in `src/gui/browser_catalog.rs`
  implementing browser interaction flow evaluation and resilience recovery testing across 4 browser environment profiles
  (`ModernDesktop`, `HighContrastAccessible`, `TouchMobileViewport`, `TextFallbackHeadless`) and 4 benchmark scenarios
  (`scenario-gui-browser-standard-flow-v1`, `scenario-gui-browser-network-recovery-v1`, `scenario-gui-browser-accessibility-flow-v1`,
  `scenario-gui-browser-degraded-fallback-v1`) verifying clean state restoration, degraded fallback, and zero authority desync under connection loss.
- [x] Cover presentation need evaluation, deficit threshold rules, fail-closed validation,
  DTO bundle construction, latent opponent leakage rejection, chain-of-thought omission,
  catalog scenario execution, active tab/view mode round trips, client state transitions, reversibility,
  zoom bounds, parity verification, asset kind/license/fallback round trips, asset governance audit rules,
  HTML document generation across all tabs, W3C/security/privacy verification, loopback transport request handling,
  browser target/capability round trips, recovery strategies, and Markdown report hygiene across 43 focused tests in `src/gui/tests.rs`.

This establishes the formal presentation need evaluation framework, ADR-0003 architecture
decision record, versioned actor-visible GUI DTO models, reversible client state machine,
triple CLI/MCP/GUI projection parity verification, asset governance / fallback rules,
standalone HTML5/CSS/SVG GUI presentation document generator, loopback transport protocol / session adapter,
### Developer Action Items (Browser Client)

- [x] Wire standalone HTML presentation viewer to CLI exporter (`--scenario m11-gui-presentation-v1`) for file/stream visual inspection.
- [ ] Validate browser flow recovery on live browser sessions.

### Deliverables

- Host/client ADR and versioned presentation contract.
- Bounded GUI client and loopback host if selected.
- Accessibility fallback and asset-governance artifacts.
- Contract, parity, browser, and recovery evidence.

### Exit evidence

- No client-owned legality, transition, hidden-state inference, committed history,
  replay, or persistence authority.
- Every semantic visual/audio cue traces to actor-visible host data or committed
  history and has a non-visual/non-audio equivalent as relevant.
- The selected browser target passes the declared flow and recovery scenarios.
- User evidence shows the GUI addresses its named problem before expansion.

### Explicit deferrals

- M11 may be skipped entirely for M12 if the CLI experience satisfies the target
  release and accessibility evidence.
- No browser-only simulation and no desktop-shell packaging without evidence.

## Phase 12 — Public Research-Capable Alpha

**Milestone:** M12
**Status:** Planned
**Depends on:** M10 and every adopted release surface; M11 only if adopted

### Outcome

The project is playable, reproducible, documented, safely packaged, and limited
in its legal, accessibility, entertainment, and research claims.

### Scope

- [x] Review current fan-project policies, names, content, and asset provenance.
- [x] Confirm license, contribution, noncommercial, unofficial, and
  original-setting fallback posture.
- [x] Add player, contributor, MCP-agent, experiment, replay, and data guides.
- [x] Publish ruleset, scenario, protocol, profile, prompt, model, and extractor
  compatibility policies.
- [x] Add a data dictionary and model cards.
- [x] Add known limitations, evidence boundaries, and citation guidance.
- [x] Package sample scenarios, replays, and experiments.
- [x] Run clean-install, reproducibility, security, license, and compatibility
  checks.
- [ ] Conduct release-candidate human testing appropriate to public claims.
- [ ] Archive source, lockfiles, schemas, fixtures, evidence, artifacts, and
  hashes for the release tag.

### Current M12 release governance, compatibility matrix, data dictionary, and benchmark catalog evidence

- [x] Define `m12-alpha-governance-v1` in `src/alpha/governance.rs` formalizing public alpha release
  governance declarations across 6 discrete compliance areas (`LicenseNotice`, `NonCommercialUse`,
  `UnofficialDisclaimer`, `OriginalSettingFallback`, `AssetProvenanceAudit`, `ContentIsolation`),
  `LegalPostureStatus` (`CompliantPermissive`, `OriginalFallbackRequired`, `PendingClearance`,
  `DistributionBlocked`), `PublicAlphaGovernanceManifest`, and pure deterministic audit evaluation
  (`evaluate_alpha_governance`) with fail-closed validation and integer basis-point scoring ($[0..=10,000]$ bp).
- [x] Define `m12-alpha-compatibility-v1` in `src/alpha/compatibility.rs` implementing cross-version
  compatibility matrix verification across 8 simulation domains (`Ruleset`, `Scenario`, `ProtocolDto`,
  `AgentProfile`, `PromptTemplate`, `ModelCalibration`, `ReplayArtifact`, `GuiPresentation`), 4
  compatibility tiers (`FullyCompatible`, `BackwardCompatibleOnly`, `BreakingChangeMigrationRequired`,
  `DeprecatedUnsupported`), and deterministic matrix soundness auditing (`evaluate_compatibility_matrix`).
- [x] Define `m12-alpha-data-dictionary-v1` in `src/alpha/data_dictionary.rs` cataloging simulation variables
  across 8 functional categories and 4 sensitivity tiers (`PublicActorVisible`, `TeamVisibleShared`,
  `LatentHostAuthoritative`, `ResearchInspectionOnly`) with fail-closed fog-of-war redaction invariant
  enforcement (`audit_data_dictionary`).
- [x] Register 4 canonical benchmark alpha scenarios in `AlphaScenarioCatalog` (`m12-alpha-catalog-v1` in
  `src/alpha/catalog.rs`: `scenario-alpha-governance-compliant-v1`, `scenario-alpha-governance-fallback-triggered-v1`,
  `scenario-alpha-compatibility-matrix-v1`, `scenario-alpha-data-dictionary-complete-v1`) with reproducible
  execution and verified expectations.
- [x] Cover governance evaluation, fallback posture triggers, compliance basis points, compatibility matrix
  validation, migration contract requirements, data dictionary redaction auditing, fail-closed error handling,
  error Display coverage, and clean Markdown report hygiene across 18 focused tests in `src/alpha/tests.rs`
  (628 total library tests).

### Current M12 limitations, evidence boundaries, and citation guidance evidence

- [x] Define `m12-alpha-limitations-v1` in `src/alpha/limitations.rs` formalizing known technical/empirical
  limitations across 6 discrete categories (`SimulationFidelity`, `AccessibilityCoverage`, `AgentGeneralization`,
  `HumanRealism`, `NetworkMultiplayer`, `HardwareRequirements`), 5 evidence tiers (`SoftwareInvariants`,
  `SyntheticAgentPlaytest`, `EmpiricalCalibration`, `LimitedHumanStudy`, `UnverifiedHypothesis`), 3 claim
  classifications (`PermissibleBoundedClaim`, `ConditionalWithDisclaimer`, `ImpermissibleOverclaim`), `ResearchClaim`,
  `CitationGuidance` (BibTeX, DOI/URN, canonical title, software version, repository URL, seed policy), and pure
  deterministic audit evaluation (`audit_limitations_and_boundaries`) with fail-closed validation (`AlphaLimitationsError`)
  and integer basis-point safety scoring ($[0..=10,000]$ bp).
- [x] Register 3 canonical benchmark limitations scenarios in `AlphaScenarioCatalog` (`m12-alpha-catalog-v1` in
  `src/alpha/catalog.rs`: `scenario-alpha-limitations-compliant-v1`, `scenario-alpha-limitations-overclaim-rejected-v1`,
  `scenario-alpha-limitations-missing-disclaimer-v1`) with reproducible execution and verified expectations.
- [x] Cover limitation category, evidence tier, and claim classification round-trips, compliant declaration auditing,
  fail-closed validation (empty manifest/fields, duplicate IDs, impermissible overclaims, missing required disclaimers),
  error Display formatting, catalog benchmark execution, and clean Markdown report hygiene across 24 focused tests in
  `src/alpha/tests.rs` (634 total library tests).

### Current M12 documentation guides and reproducibility packaging evidence

- [x] Define `m12-alpha-guides-v1` in `src/alpha/guides.rs` formalizing documentation guide manifests across 6 target
  audiences (`Player`, `Contributor`, `McpAgent`, `Experimenter`, `ReplayAnalyst`, `DataScientist`), 7 section categories
  (`Prerequisites`, `CoreConcepts`, `Quickstart`, `InteractiveWalkthrough`, `ProtocolContracts`, `Troubleshooting`,
  `EvidenceAndLimitations`), structured `GuideDocumentDefinition`, prerequisite DAG cycle detection via DFS, completeness
  basis-point scoring ($[0..=10,000]$ bp), and pure deterministic audit evaluation (`audit_guide_manifests`).
- [x] Define `m12-alpha-reproducibility-v1` in `src/alpha/reproducibility.rs` implementing sample artifact packaging across
  5 artifact kinds (`ScenarioBenchmark`, `ReplayTranscript`, `ExperimentRun`, `ModelCalibrationStudy`, `BehavioralTelemetry`),
  4 reproducibility statuses (`FullyReproducible`, `RequiresModelAdapter`, `SyntheticBaselineOnly`, `CorruptedOrMissing`),
  16-hex FNV-1a content hash integrity verification, dependency resolution, and deterministic bundle evaluation
  (`audit_reproducibility_bundle`).
- [x] Register 4 canonical benchmark guides and reproducibility scenarios in `AlphaScenarioCatalog` (`m12-alpha-catalog-v1` in
  `src/alpha/catalog.rs`: `scenario-alpha-guides-complete-v1`, `scenario-alpha-guides-cyclic-prereq-rejected-v1`,
  `scenario-alpha-reproducibility-bundle-v1`, `scenario-alpha-reproducibility-corrupt-hash-rejected-v1`) with reproducible
  execution and verified expectations (11 total alpha benchmark scenarios).
- [x] Cover guide audience, section category, and reproducibility status round-trips, DAG prerequisite cycle detection,
  FNV-1a checksum validation, fail-closed errors (`AlphaGuidesError`, `AlphaReproducibilityError`), error Display coverage,
  and clean Markdown report rendering hygiene across 32 focused tests in `src/alpha/tests.rs` (642 total library tests).

### Current M12 release readiness verification check suite evidence

- [x] Define `m12-alpha-release-checks-v1` in `src/alpha/checks.rs` implementing multi-domain release verification across 6 discrete check categories (`CleanInstall`, `Reproducibility`, `SecurityAdvisory`, `LicenseCompliance`, `CompatibilityMatrix`, `DataRedaction`), 4 severity levels (`CriticalBlocker`, `MajorIssue`, `MinorWarning`, `VerifiedPass`), 4 verification statuses (`Passed`, `ConditionallyPassed`, `Failed`, `Skipped`), `ReleaseCheckDefinition`, `AlphaReleaseChecksManifest`, and pure deterministic audit evaluation (`audit_release_checks`) with fail-closed validation (`AlphaReleaseChecksError`), exact integer basis-point scoring ($[0..=10,000]$ bp), category summaries, and `is_release_ready` release readiness gate checks ($\ge 8,500$ bp, 0 blockers, 0 failures, 100% required categories).
- [x] Register 3 canonical benchmark release check scenarios in `AlphaScenarioCatalog` (`m12-alpha-catalog-v1` in `src/alpha/catalog.rs`: `scenario-alpha-release-checks-compliant-v1`, `scenario-alpha-release-checks-blocker-rejected-v1`, `scenario-alpha-release-checks-missing-category-rejected-v1`) with reproducible execution and verified expectations (14 total alpha benchmark scenarios).
- [x] Cover check category, severity, and verification status round-trips, fail-closed validation, error Display formatting, readiness score basis points, release readiness gate logic, catalog benchmark execution, and clean Markdown report rendering hygiene across 38 focused tests in `src/alpha/tests.rs` (648 total library tests).
- [x] Wire `--scenario m12-alpha-release-checks-v1` into the application executable CLI loop (`src/command_loop.rs`, `src/main.rs`, `src/cli/release_checks.rs`), executing the canonical compliant release checks suite, rendering the structured Markdown report, and verifying exit status across unit and binary integration tests.

This establishes the formal release readiness verification check suite, multi-domain compliance auditing, blocker rejection, CLI release checks runner, and release eligibility framework for M12. Release candidate human testing and release tag archiving remain open.

### Developer Action Items (Public Release)

- [ ] Package official research reproducibility bundles with verified 16-hex FNV-1a checksums.
- [x] Execute release candidate verification check suite (`audit_release_checks`) via `--scenario m12-alpha-release-checks-v1`.
- [ ] Create official tagged research release bundle with governance documentation.

### Deliverables

- Versioned release candidate and signed or hashed artifact inventory.
- Complete user, contributor, agent, experiment, and data documentation.
- Release evidence report and known-limitations document.
- Archived reproducibility bundle and release-candidate human findings.

### Exit evidence

- A clean environment can install, run, replay, and verify the reference sample.
- Public artifacts include required notices, licenses, provenance, and
  attribution.
- Published research examples reproduce from archived inputs and tool versions.
- Claims do not exceed software, agent, human, legal, or research evidence.
- The tagged release and evidence bundle are archived and linkable.

### Explicit deferrals

- No production service, commercial distribution, or scientific generalization
  beyond validated scope.
- Post-alpha expansion requires a new evidence-based roadmap revision.

## Architecture & Governance Evolution Targets (ADR-0004 Planning)

As identified in the independent technical audit ([`docs/AUDIT_REPORT.md`](docs/AUDIT_REPORT.md)), the codebase (~85k LOC across 12 milestone domains) is prepared for structured modularization:

- [x] Author ADR-0004 (Cargo Workspace Partitioning) in [`docs/adr/0004-cargo-workspace-partitioning.md`](docs/adr/0004-cargo-workspace-partitioning.md).
- [ ] Partition the monolithic single crate into dedicated workspace members:
  - `crates/foi-kernel` (authoritative transition & units)
  - `crates/foi-lane` (one-lane vertical slice)
  - `crates/foi-map` (5v5 multi-lane spatial map topology, structures & contest mechanics)
  - `crates/foi-agent` (behavioral policies, calibration & team communication)
  - `crates/foi-protocol` (model-agnostic DTOs & MCP codecs)
  - `crates/foi-study` (human usability, accessibility & alpha synthesis)
  - `crates/foi-gui` (presentation-only HTML5/CSS/SVG generator & parity engine)
  - `crates/foi-alpha` (release governance, compatibility & readiness checks)
- [ ] Maintain thin application binaries at workspace root (`fog-of-intent` CLI runner, `fog-of-intent-mcp` MCP server).

## Roadmap Maintenance

When a milestone changes:

1. update its status and evidence here;
2. reconcile verified capability state in `SPEC.md`;
3. update `ARCHITECTURE.md` only for actual boundary changes;
4. add a contributor- or user-visible entry to `CHANGELOG.md`;
5. keep incomplete checklist items visible or explicitly defer them;
6. link durable evidence rather than replacing it with an unsupported summary.

Roadmap revisions should explain why ordering, scope, or gates changed. They
should not rewrite completed history to make the current plan appear inevitable.


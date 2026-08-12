# Fog of Intent Project Roadmap

**Document role:** Canonical milestone order, scope, and promotion gates
**Status:** Active
**Current milestone:** M2 — One-Lane Vertical Slice
**Last reviewed:** 2026-08-08

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
| Executable | `src/main.rs`, `src/command_loop.rs` | Standalone package version reporting plus a documented line-oriented bounded fixture transcript with one explicit versioned `--scenario m3-two-window-fixture-v1` ID and optional `--run-dir` artifact storage |
| Package | `Cargo.toml` | Version `0.1.134`, no dependencies |
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
| M10 | Human-usable and accessibility-tested alpha | Planned | Stable M9 candidate; informal checks start earlier |
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

This evidence establishes a deterministic committed-facts final debrief for
two ordinary windows. It does not establish a complete scenario, pacing,
communication, strategy quality, balance, or human experience.

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
  lowercase labels, newline structure, and absence of ANSI/control characters.
- [ ] Validate complete interactive behavior, keyboard/focus behavior, and
  screen-reader semantics with human-oriented inspection.

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
  unidentifiable parameters, private chain-of-thought preservation, and
  recalibration triggers remain open.
- [x] Report unidentifiable parameters and unstable semantic labels;
  private chain-of-thought preservation and recalibration triggers remain open.
- [ ] Preserve reference outputs without storing or requiring private
  chain-of-thought.
- [ ] Define recalibration triggers for model or prompt changes.

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

- [ ] Define typed speech acts, recipients, urgency, confidence, conditions, and
  message visibility.
- [ ] Implement proposal, clarification, confirmation, disagreement,
  counterproposal, conditional commitment, withdrawal, and failure reporting.
- [ ] Define team-plan and individual-plan relationships.
- [ ] Implement trust, caller reputation, communication clarity, delay,
  missingness, and overload only as demonstrated needs.
- [ ] Add designated shot-caller and decentralized baselines.
- [ ] Preserve private submissions and simultaneous resolution.
- [ ] Attribute coordination success and failure separately from execution.
- [ ] Add high-trust, low-trust, conflicting-call, and missing-message scenarios.
- [ ] Add communication and leadership debriefs.
- [ ] Test that disagreement can be strategically legitimate.

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

- [ ] Define an abstracted three-lane map and travel model.
- [ ] Add objective cycles, map-level vision, rotations, and resource tradeoffs.
- [ ] Add the minimum role and team-composition abstractions needed for match
  strategy.
- [ ] Define match victory and terminal conditions.
- [ ] Add role-specific observations, actions, and debrief perspectives.
- [ ] Add comeback and variance-seeking mechanics with explicit inputs.
- [ ] Add match-level pivotal-decision detection.
- [ ] Preserve meaningful decision density through automatic routine execution.
- [ ] Profile transition, replay, projection, and batch-run costs.
- [ ] Expand scenario and property tests without weakening M1/M2 fixtures.
- [ ] Measure strategy diversity, role activity, communication, and unused
  mechanics.

### Deliverables

- Versioned match scenario and ruleset.
- Complete CLI and MCP match replays.
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
**Status:** Planned
**Depends on:** Stable M9 candidate; informal checks should occur during M2-M9

### Outcome

Relevant participants can understand and complete the reference experience,
explain their major decisions, and use the debrief to reconstruct outcomes; the
project reports accessibility and usability limits honestly.

### Scope

- [ ] Conduct small informal checks of the core loop before this phase and retain
  issue-linked notes without overstating them.
- [ ] Define research questions, participant criteria, consent, privacy, and data
  handling appropriate to the study and claims.
- [ ] Test onboarding, terminology, command discoverability, pacing, information
  load, agency, delegated-execution fairness, and debrief usefulness.
- [ ] Test keyboard-only flow, adjustable verbosity, non-color semantics, and
  screen-reader use with relevant participants where those claims are made.
- [ ] Include strategy-oriented, existing MOBA, and access-needs perspectives as
  feasible and report sampling limits.
- [ ] Separate usability, accessibility, gameplay, balance, and behavioral-model
  findings.
- [ ] Link revisions to reproducible or well-supported findings.
- [ ] Preserve negative, mixed, and inconclusive results.

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

- [ ] Define the user problem and evidence that text presentation is insufficient.
- [ ] Record an ADR for host/client, browser support, assets, and persistence.
- [ ] Expose versioned actor-visible host DTOs rather than internal domain types.
- [ ] Implement map, timeline, plan/contingency, and debrief views only as needed.
- [ ] Preserve text and symbol equivalents for color, motion, and audio meaning.
- [ ] Add keyboard, focus, scaling, mute, reduced-motion, loading, offline, and
  missing-data behavior.
- [ ] Keep browser state reversible and presentation-only.
- [ ] Add host-contract, CLI/MCP/GUI parity, default-browser, and recovery tests.
- [ ] Define asset provenance, license, attribution, hash, and fallback rules.

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

- [ ] Review current fan-project policies, names, content, and asset provenance.
- [ ] Confirm license, contribution, noncommercial, unofficial, and
  original-setting fallback posture.
- [ ] Add player, contributor, MCP-agent, experiment, replay, and data guides.
- [ ] Publish ruleset, scenario, protocol, profile, prompt, model, and extractor
  compatibility policies.
- [ ] Add a data dictionary and model cards.
- [ ] Add known limitations, evidence boundaries, and citation guidance.
- [ ] Package sample scenarios, replays, and experiments.
- [ ] Run clean-install, reproducibility, security, license, and compatibility
  checks.
- [ ] Conduct release-candidate human testing appropriate to public claims.
- [ ] Archive source, lockfiles, schemas, fixtures, evidence, artifacts, and
  hashes for the release tag.

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

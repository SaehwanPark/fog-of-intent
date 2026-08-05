# Project Specification

**Status:** Active project-state index
**Last reviewed:** 2026-08-04

This file records verified past, the small active slice, and intentionally
deferred future work. It is not the product proposal, roadmap, issue tracker, or
per-commit journal.

Canonical direction and state live in:

- `README.md` — project entry point and current status;
- `ROADMAP.md` — authoritative milestone order and promotion gates;
- `ARCHITECTURE.md` — verified current structure and target boundaries;
- `CHANGELOG.md` — meaningful contributor- and user-visible history;
- `docs/project-proposal.md` — detailed product and research vision;
- `docs/tech-stack-consideration.md` — proposed technology choices.

## Maintenance Rule

Keep `Present` small. Every active item states what is done, not yet done,
verification, and deferrals. Move work to `Past` only after the named evidence
exists. Planned proposal or roadmap text is never implementation evidence.

## Past

### Repository inception — 2026-08-04

- A Rust 2024 binary package named `fog-of-intent` was initialized at version
  `0.1.0`.
- The executable is a placeholder that prints `Hello, world!`.
- A comprehensive proposal established the turn-based, AI-native team-strategy
  thesis, initial one-lane slice, bounded-rationality direction, deterministic
  authority boundary, replay/debrief goals, and evidence limits.
- A technology analysis recommended a Rust-authoritative core with CLI and MCP
  adapters, artifact-first persistence, optional Python research tooling, and an
  optional later GUI. Those recommendations remain unadopted until implemented
  or recorded as architecture decisions.

### M0 — Governed repository baseline — 2026-08-04

**Status:** Complete

#### Delivered

- Canonical roadmap, specification, architecture, changelog, repo guidance, and
  domain harness were established.
- MIT source licensing, contribution/conduct policy, unofficial project notice,
  original-setting fallback, and distribution boundaries were documented without
  claiming legal clearance.
- Authoritative terminology and ADR-0001 established host-owned simulation
  authority, explicit resolved inputs, and adapter boundaries.
- ADR-0002 kept M1 in one Cargo package; Rust `1.96.0`, `rustfmt`, `clippy`,
  `Cargo.lock`, MIT package metadata, artifact/replay compatibility policy, and
  dependency/security/license policy were recorded.
- `.github/workflows/ci.yml` and `scripts/check_repository.py` established
  clean-checkout format, lint, test, metadata, link, currentness, and
  dependency-free package checks with focused checker tests.

#### Verification

- PR #4 hosted GitHub Actions `verify` passed from a clean Ubuntu checkout.
- The exact Rust `1.96.0` toolchain, locked metadata, formatting, clippy, and
  Rust tests passed.
- Seven focused repository-checker tests passed, including stale milestone,
  M1/M10, collapsed-reference, image/outside-root, and dependency-defer cases.
- The initial authority ADR identifies one host-owned transition authority and
  forbids adapters from owning simulation truth.
- License and fan-project notices state contributor and user boundaries while
  preserving the evidence limit that they are not legal clearance.

#### Deferred / Non-Goals

- No simulation mechanic, playable decision window, CLI command loop, MCP
  server, replay engine, research package, or GUI shipped in M0.
- A future non-empty dependency graph still requires an approved advisory/license
  scanner or a complete machine-readable defer record.
- M0 does not establish intellectual-property clearance, public-release
  readiness, accessibility, enjoyment, or research validity.

### M1 — Bounded deterministic transition fixture — 2026-08-04

**Status:** Complete
**Started:** 2026-08-04
**Selected after:** M0 hosted CI promotion

#### Delivered

- `src/kernel.rs` provides stable identifiers, bounded `Units`, immutable
  `WorldState`, a versioned ruleset identifier, host validation, and the pure
  `Hold`/`Gather` transition boundary.
- Resolved inputs carry distinct environment, observation, policy,
  coordination, and execution categories with stable stream/draw identities.
- Events, command- and execution-attributed effects, authoritative FNV-1a state
  hashes, and append-only in-memory transition records are implemented.
- Replay revalidates and reevaluates every committed transition and compares
  each stored result and hash.
- Nineteen focused Rust tests cover invalid and unfavorable outcomes, bounds,
  conservation, repeated runs, input-stream isolation, ordering, replay,
  versioned fixtures, round trips, and fail-closed codec rejection.
- `src/serialization.rs` and two checked-in `1.0.0` text fixtures serialize and
  deserialize snapshots and histories through the kernel replay contract.

#### Verification

- Repeated runs with identical prior state, validated commands, resolved inputs,
  and ruleset produce equivalent events, effects, next state, and hash.
- Replay reconstructs the terminal state and verifies every committed transition
  hash from the initial state.
- Tests cover malformed/illegal commands, legal unfavorable outcomes, bounds,
  conservation, ordering, and unrelated input-stream isolation as implemented.
- Core dependency inspection confirms no I/O, async runtime, wall clock,
  terminal, database, MCP, model-provider, or hidden RNG dependency.
- The M1 checklist and exit evidence in `ROADMAP.md` are complete, and the
  merged `0.1.3` implementation passes the locked project checks.

#### Deferred

- No lane model, full scenario, interactive CLI, MCP transport, general
  entity-component system, arbitrary scenario scripting, migration support, or
  richer external replay bundle is implemented.
- The codec remains a local fixture contract; it does not claim external
  compatibility, human experience, or a playable simulation.

## Present

### M2 — First bounded one-lane decision window

**Status:** Active
**Started:** 2026-08-04
**Selected after:** M1 replay and codec promotion

#### Target slice

- Define the smallest typed lane snapshot needed for one decision window.
- Project actor-valid observation text/data for the human laner without latent
  opponent state or research-only inspection.
- Accept one host-validated intent command and explicit resolved execution
  input, then return deterministic events, effects, next state, and hash through
  the existing kernel boundary.
- Preserve append-only history and replay identity while leaving the binary,
  CLI, full scenario, and external adapters deferred.

#### Delivered in the first bounded slice

- `src/lane.rs` defines bounded lane health, damage, wave pressure, positions,
  phase, opponent truth, hidden threat truth, terminal outcome, and the fixed
  `m2-lane-v1` ruleset.
- `observe_player` returns an actor-valid `m2-lane-observation-v1` projection
  with explicit unknown opponent/threat reports and no latent state or hash.
- `Stabilize` and `Contest` requests become host-created validated commands;
  command metadata, observation receipts, and exact prior-state binding reject
  wrong actor, turn, ruleset, stale observation, and stale hash inputs.
- `transition_lane` consumes explicit execution damage and wave results,
  preserves intent/execution separation, emits ordered attributed events and
  effects, produces a one-window debrief, and computes the next-state hash.
- `LaneHistory` commits one append-only record and verifies its observation,
  command, resolved inputs, result, and terminal snapshot through replay.
- Nine focused lane tests plus the nineteen M1 tests pass, covering the
  information boundary, both legal intents, an unfavorable fallback outcome,
  malformed execution, validation, determinism, stream isolation, and replay.

#### Delivered in the bounded branch follow-up

- `branch_from_window` accepts only a verified one-record parent and the
  immutable record-0 decision boundary; the parent remains independently
  replayable and unchanged.
- `BranchExecutionSelection` distinguishes exact parent-input reuse from an
  explicitly resolved branch execution with a stable branch-scoped trace.
- `LaneBranchReplayIdentity` and `CounterfactualReview` preserve parent and
  branch hashes, intent, execution relation, and attribution limits outside the
  authoritative lane-state hash.
- Branch replay re-derives the observation, command, input selection,
  transition result, and terminal hash; tampered branch metadata/results are
  rejected.
- Thirteen focused M2 lane tests plus the nineteen M1 tests pass, including
  matched/regenerated branches, parent immutability, invalid selection,
  branch identity, replay tamper, and causal-review limits.

#### Delivered in the allied proposal and coordination follow-up

- `observe_allied` projects one proposal-only allied actor from visible lane
  fields and explicit unknown reports; hidden truth, source hashes, history,
  and execution values remain outside the actor input.
- `scripted-allied-proposal-v1` records profile/input identities, bounded
  candidate scores, stable selection, and deterministic proposal identity;
  matched hidden-state substitutions produce the same artifact.
- `AlliedProposalOffer`, `ProposalResponse`, and
  `CoordinationResolutionInputs` define one host-owned support offer and the
  accept/reject/counter boundary without turning policy output into a command.
- `resolve_coordinated_lane` emits coordination envelope events/effects and a
  causal debrief around one unchanged `transition_lane` result; proposal and
  coordination metadata do not enter `LaneSnapshot::hash()`.
- `CoordinatedLaneHistory` appends and replays one sidecar record with tamper
  detection while existing no-proposal history and branch replay remain
  valid.
- Five focused coordination/policy/history tests plus the prior thirty-two
  tests pass, for thirty-seven Rust tests total.

#### Delivered in the scenario-goal and terminal-objective follow-up

- `ScenarioGoal::HoldLaneSpaceThroughWindow` defines one bounded diagnostic
  goal without adding a new lane mechanic, state field, event, or transition.
- `ObjectiveEvaluationInputs` and `ObjectiveInputIdentity` derive committed
  result facts, source replay identity, and provenance for ordinary and
  coordinated records while keeping hidden truth outside evaluation.
- `TerminalObjectiveReview` evaluates `SpaceHeld` and `SurvivedBeat` into
  achieved, partial, or missed dispositions and preserves coordination and
  execution attribution without claiming optimality.
- `ObjectiveReport` is a visible projection that omits source-state hashes and
  private receipts; `ObjectiveReviewRecord` verifies source identity, facts,
  review, and tampering for both record types.
- Three focused objective tests plus the prior thirty-seven tests pass, for
  forty Rust tests total.

#### Delivered in the matched-input strategy-fixture follow-up

- `StrategyFixture` defines named `HappyPath`, `RiskTaking`, and
  `Conservative` bundles with explicit intent/response, coordination input,
  lane execution input, and expected modeled outcome.
- `run_strategy_fixture` binds each response to the canonical proposal ID and
  runs through host validation, coordinated history append, and terminal
  objective review; expectations do not alter transition authority.
- Repeated runs are equivalent, the three cases produce distinct declared
  input/output contrasts, legal-unfavorable risk-taking remains valid, and
  tampered expected outcomes are rejected.
- One focused strategy-fixture test plus the prior forty tests pass, for
  forty-one Rust tests total.

#### Delivered in the bounded two-window scenario follow-up

- `LaneScenarioHistory` composes two sequential ordinary lane records under
  `m2-two-window-scenario-v1` while preserving the one-window transition,
  branch, coordination, and objective identities.
- `reopen_lane_window` accepts only a valid resolved result, preserves its
  domain values and advanced turn, and deterministically clears only the
  per-window phase/outcome status for the next open window.
- Scenario replay stores and compares exact window start states, complete base
  records, reopened state, terminal state, and tamper-sensitive sequencing;
  third append and invalid reopen cases fail.
- Two focused scenario tests plus the prior forty-one tests pass, for
  forty-three Rust tests total.

#### Verification

- Identical prior state, validated intent, resolved input, and ruleset yield
  equivalent output and hash.
- Invalid actor, turn, ruleset, stale-hash, and out-of-contract commands fail
  before transition evaluation.
- Actor-visible observation omits latent opponent values and labels unknown or
  last-known information explicitly.
- A legal but unfavorable execution result remains distinct from command
  rejection and is replay-verifiable.

#### Not Yet Done

- The complete lane scenario still needs variable pacing, recall, gank
  response, richer resource abstractions, communication, and a full causal
  debrief surface; this follow-up composes only two fixed one-beat windows.
- CLI, MCP, full agent ecology, and human-experience evidence remain future
  M3/M4+ work; this diagnostic slice is not playable and makes no enjoyment,
  accessibility, trust, or behavioral-validity claim.

## Future

The detailed and canonical order is in `ROADMAP.md`.

- **M2:** complete the one-lane scenario from the active decision-window slice,
  including actor-specific uncertainty, intent, delegated execution, branching,
  and causal debrief.
- **M3:** keyboard-first CLI reference experience.
- **M4:** interpretable non-LLM agent ecology.
- **M5:** thin, versioned, model-agnostic MCP adapter.
- **M6:** automated behavioral experiments and regression evidence.
- **M7:** evidence-limited semantic-to-parametric calibration proof.
- **M8:** trust-sensitive team communication and shot-calling.
- **M9:** bounded multi-lane match prototype.
- **M10:** human usability and accessibility alpha evidence.
- **M11:** optional host-bound GUI if demonstrated needs justify it.
- **M12:** public research-capable alpha with release and claim governance.

## Persistent Product Non-Goals

- Full reproduction of a proprietary game, roster, item catalog, or live
  metagame.
- Real-time mechanical control or reaction-time requirements.
- Networked multiplayer in the initial roadmap.
- Perfect-rationality or global-equilibrium claims.
- Treating AI-agent behavior as human behavior.
- A general-purpose multi-agent simulation framework before a proven vertical
  slice.
- Public, legal, accessibility, entertainment, or scientific claims without the
  evidence appropriate to each claim.

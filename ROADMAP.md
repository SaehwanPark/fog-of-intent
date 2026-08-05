# Fog of Intent Project Roadmap

**Document role:** Canonical milestone order, scope, and promotion gates
**Status:** Active
**Current milestone:** M2 — One-Lane Vertical Slice
**Last reviewed:** 2026-08-04

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
| Executable | `src/main.rs` | Placeholder `Hello, world!` binary |
| Package | `Cargo.toml` | Version `0.1.4`, no dependencies |
| Canonical execution plan | `ROADMAP.md` | Active |
| Project-state docs | `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md` | Initialized |
| Agent workflow | `AGENTS.md`, `.agents/skills/`, `docs/harness/` | Initialized |
| Internal kernel/replay fixture | `src/kernel.rs`, `src/serialization.rs` | M1 complete; not playable |
| Scenario, CLI, MCP, research, GUI | No implementation evidence | Not implemented |

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
state. Hold, pressure, and recall remain planned follow-up actions until this
boundary is verified.

### Outcome

A human can complete one short, text-first lane scenario with meaningful intent,
uncertainty, delegated execution, an allied proposal, a terminal objective, and
a causal debrief.

### Scope

- [ ] Choose one scenario goal and simulated duration, such as surviving a weak
  lane, preparing a gank, or recalling with limited loss.
- [ ] Define one human-controlled laner, one opposing laner, one allied autonomous
  actor, and one abstract opposing jungle threat.
- [ ] Define the minimum lane, wave, position, health, mana, cooldown, gold, and
  experience abstractions needed by the scenario.
- [ ] Define vision, last-known information, belief updates, unknowns, and report
  wording without exposing latent values.
- [ ] Define variable-duration decision windows and automatic-advance conditions.
- [ ] Define intent, commitment, target/focus, communication, abort conditions,
  and fallback behavior.
- [ ] Implement hold, pressure/trade, yield, recall, and gank-response decisions
  only where they create real tradeoffs.
- [ ] Implement coordination and execution as distinct resolutions.
- [ ] Record direct, indirect, immediate, and delayed effects with provenance.
- [ ] Define a terminal outcome that does not collapse evaluation to win/loss.
- [ ] Produce immediate review and final debrief projections.
- [ ] Support replay and a bounded counterfactual branch at a pivotal decision.
- [ ] Add scripted happy-path, risk-taking, and conservative-strategy fixtures.
- [ ] Test hidden-state leakage and actor-visible report completeness.
- [ ] Inspect every transition in at least one complete replay manually.

### Current bounded slice evidence

- [x] Define one typed lane snapshot with bounded health, wave pressure,
  position, phase, hidden opponent truth, and hidden jungle-threat truth.
- [x] Project a player-laner observation with explicit unknown reports and no
  latent opponent or threat values.
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
**Status:** Planned
**Depends on:** M2

### Outcome

The command-line interface is a complete, keyboard-first reference client for
play, inspection, save/load, replay, branching, and debrief without developer
API access.

### Scope

- [ ] Define stable top-level process commands and in-session grammar.
- [ ] Implement `observe`, bounded `inspect`, and contextual help.
- [ ] Implement structured `message`, `plan`, contingency, `commit`, and
  `advance` flows.
- [ ] Implement `review`, `debrief`, `replay`, and `branch` flows.
- [ ] Add guided mode with numbered choices and explanations.
- [ ] Add expert mode with concise, scriptable commands.
- [ ] Add research inspection only behind an explicit privileged context.
- [ ] Add concise, standard, explanatory, and research verbosity policies.
- [ ] Label observed, believed, inferred, reported, and unknown information.
- [ ] Support edit/undo before commitment without rewriting committed history.
- [ ] Add save/load and human-readable run identifiers.
- [ ] Keep terminal rendering outside the authoritative domain.
- [ ] Add transcript-based acceptance tests for a complete run and common errors.
- [ ] Check keyboard-only flow and screen-reader-oriented text structure.

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

- [ ] Implement scripted agents for deterministic fixtures.
- [ ] Implement transparent role heuristics.
- [ ] Define policy inputs, memory, candidate actions, utility features, and
  action evaluations.
- [ ] Separate candidate generation, evaluation error, top-k or nucleus
  selection, coordination, and execution.
- [ ] Define risk preference, loss aversion, planning horizon, attention, trust,
  communication response, confidence, and pressure/tilt only as required.
- [ ] Define creativity as candidate breadth or transformation, not random
  selection of inferior actions.
- [ ] Create a small versioned baseline profile catalog.
- [ ] Use explicit policy random streams and reproducible seed bundles.
- [ ] Add matched-scenario and matched-input comparisons.
- [ ] Define expected monotonic effects or document interactions that make them
  non-monotonic.
- [ ] Measure legality, action distribution, strategic diversity,
  communication, coordination, plan interruption, and outcome distributions.
- [ ] Add profile sensitivity and adversarial edge-case tests.

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
- Anomalous and degenerate policies remain inspectable through replay.

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

### Scope

- [ ] Define session lifecycle and actor authority.
- [ ] Define versioned DTOs for observations, legal actions, messages, plans,
  contingencies, commit, outcome review, history, replay, and debrief.
- [ ] Keep internal domain types private from public protocol compatibility.
- [ ] Implement private action submission and host-owned window closure.
- [ ] Implement simultaneous-decision semantics where the scenario requires it.
- [ ] Define validation-error and bounded-repair behavior.
- [ ] Separate ordinary actor tools from privileged experiment-controller tools.
- [ ] Capture provider-neutral transcripts and tool-schema versions.
- [ ] Add authorization and hidden-state leakage tests.
- [ ] Add CLI/MCP action and projection parity tests.
- [ ] Add timeout, malformed-response, duplicate-submit, stale-window, and
  disconnect behavior.
- [ ] Verify that transport and async orchestration stay outside the core.

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

- [ ] Define versioned experiment manifests and seed bundles.
- [ ] Implement a local batch runner and resumable run directory.
- [ ] Add population and matched-scenario sampling.
- [ ] Record ruleset, scenario, profile, prompt, tool-schema, model, and extractor
  versions as applicable.
- [ ] Generate aggregate and distributional metrics, not only means.
- [ ] Add illegal-command, exploit-seeking, communication-abuse, and degenerate
  policy populations.
- [ ] Detect outliers and select representative replays deterministically.
- [ ] Check causal-trace completeness and replay identity for sampled runs.
- [ ] Define provisional regression gates with written threshold rationale.
- [ ] Separate operational logs from committed simulation artifacts.
- [ ] Export machine-readable data and a concise Markdown evidence report.
- [ ] Preserve crashes, timeouts, missing branches, and inconclusive results.
- [ ] Compare build-to-build behavior against a declared baseline.

### Deliverables

- Experiment manifest schema and batch runner.
- Versioned run artifacts, metrics, and representative replays.
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

- [ ] Define a compact semantic profile vocabulary and schema.
- [ ] Create diagnostic choices for contest/concede, follow/reject, farm/assist,
  recall timing, sacrifice, surprise, and response to failure.
- [ ] Define repeated-sampling and model/prompt version protocols.
- [ ] Estimate empirical action and communication distributions.
- [ ] Define behavioral distance, entropy, sensitivity, consistency, and
  adaptation measures.
- [ ] Fit initial bounded parametric policies with regularization.
- [ ] Evaluate held-out scenarios and counterfactual perturbations.
- [ ] Compare more than one model or prompting family where feasible.
- [ ] Report unidentifiable parameters and unstable semantic labels.
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

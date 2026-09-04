# Fog of Intent Agent Harness

## Domain Summary

Fog of Intent is a pre-implementation Rust project for a turn-based,
AI-native team-strategy and behavioral simulation. A human or agent expresses
strategic intent, commitment, communication, contingencies, and fallback
behavior. Simulated actors interpret and execute those plans under incomplete
information, bounded rationality, trust, pressure, and explicit stochastic
inputs. Replays and causal debriefs must make outcomes inspectable without
pretending that AI behavior establishes human behavior.

The harness keeps substantial work aligned with:

- `README.md`
- `ROADMAP.md`
- `SPEC.md`
- `ARCHITECTURE.md`
- `docs/project-proposal.md`
- `docs/tech-stack-consideration.md`

Project-specific quality boundaries:

- strategic intent remains distinct from mechanical execution;
- the host owns simulation truth and authoritative transitions;
- true state, belief, observation, report, and research inspection stay distinct;
- randomness is resolved explicitly before deterministic transition evaluation;
- history is append-only and replay inputs are versioned;
- actor decisions use actor-valid information;
- bounded behavior separates candidate generation, evaluation, selection,
  coordination, and execution;
- debriefs separate decision, coordination, execution, and luck;
- narrow vertical slices and evidence gates precede broad frameworks;
- AI playtests do not substitute for human-experience or behavioral evidence.

## Task Inventory

The durable project-specific task classes are:

1. simulation and scenario design;
2. actor-visible information and decision-contract design;
3. bounded-rational agent and communication design;
4. AI-first playtest, regression, and calibration design;
5. interactive showcase playtesting, visual inspection, and gameplay feel evaluation;
6. cross-boundary domain QA for authority, replay, behavior, and debriefs.

Generic Rust implementation, functional programming, UX, accessibility,
security, code review, comments, spec maintenance, planning, releases, and git
operations are intentionally outside the local harness.

## Reuse Notes

- `docs/project-proposal.md` supplies the product thesis, initial slice, domain
  vocabulary, risks, validation levels, and the source roadmap.
- `docs/tech-stack-consideration.md` supplies a proposed Rust-first architecture;
  it is guidance until dependencies or boundaries are adopted in code and
  recorded in `ARCHITECTURE.md`.
- The neighboring `hs-mgt-game` repository demonstrates short `AGENTS.md`
  guidance, repo-local routing, deterministic handoffs, and project-specific QA.
  This harness reuses those coordination lessons without copying its
  health-policy or mature-GUI roles.

## Non-Duplication Contract

Use global capabilities rather than creating local duplicates:

| Global capability | Responsibility retained globally |
| --- | --- |
| `fp-developer` | Functional core, explicit state, typed boundaries, controlled effects |
| `simple-code-writer` | Smallest correct implementation and proportional verification |
| `code-reviewer` | General correctness, security, performance, and maintainability review |
| `code-commenter` | General code-comment and API-documentation quality |
| `end-user-xp-improver` | Generic usability, accessibility, onboarding, and recovery design |
| `spec-driven-developer` | `SPEC.md`, `ARCHITECTURE.md`, and `CHANGELOG.md` synchronization |
| `plan-designer` | Generic implementation-plan quality |
| `preferred-workflow` | Branch, commit, push, PR, and review workflow |
| `release-preparer` | Packaging and public-release preparation |
| `harness` | Revisions to this harness architecture |

## Chosen Architecture

Pattern: shallow Pipeline with an Expert Pool and a Producer-Reviewer gate.

Why this is the smallest useful pattern:

- work is evidence-gated and usually moves from scope to design to production to
  QA in order;
- simulation mechanics and agent ecology are distinct specialist contexts, but
  only one may be needed for many tasks;
- when both specialists run, their outputs interact and require one synthesis
  owner before shared implementation;
- project-specific QA materially reduces hidden-state leakage, replay drift,
  behavior-model conflation, and evidence overclaiming;
- a supervisor or hierarchy would add routing overhead before the repository has
  a changing implementation backlog.

Direct work remains the default for small, tightly coupled requests. Delegation
is optional runtime behavior, not a repository dependency.

## Roles

| Role | Responsibility | Reusable skill | Writes |
| --- | --- | --- | --- |
| Orchestrator and synthesis owner | Frame the milestone slice, route specialists, resolve conflicts, and reconcile final project state | `.agents/skills/fog-intent-orchestrator/SKILL.md` | `_workspace/00_input/request-summary.md`, `_workspace/02_design-synthesis.md`, `_workspace/final/handoff.md` |
| Simulation designer | Define authoritative mechanics, actor information, scenarios, transitions, replay, and debrief contracts | `.agents/skills/fog-intent-simulation-designer/SKILL.md` | `_workspace/01_simulation-design.md` |
| Agent-ecology designer | Define bounded policies, communication, populations, behavioral metrics, experiments, and calibration | `.agents/skills/fog-intent-agent-ecology-designer/SKILL.md` | `_workspace/01_agent-ecology-design.md` |
| Showcase test player | Play runnable showcases interactively, visually inspect feature correctness, and evaluate gameplay feel | `.agents/skills/foi-test-player/SKILL.md` | `_workspace/04_playtest-report.md` |
| Domain QA reviewer | Cross-check scope, authority, information, reproducibility, behavior, gameplay, debrief, and evidence boundaries | `.agents/skills/fog-intent-domain-qa/SKILL.md` | `_workspace/03_domain-qa.md` |

An implementer or document producer is not a durable local role. The
orchestrator uses the relevant global skills and retains final integration
ownership.

## Routing Rules

| Request shape | Local route |
| --- | --- |
| State, rules, scenario, decision window, command, event, effect, observation, replay, branch, or debrief | Simulation designer, then domain QA for substantial output |
| Agent profile, bounded rationality, communication policy, behavioral metric, population test, LLM playtest, or calibration | Agent-ecology designer, then domain QA for substantial output |
| Interactive showcase playtesting, visual feature inspection, or gameplay feel evaluation | Showcase test player (`foi-test-player`) |
| Mechanics and agent behavior change together | Both designers, orchestrator synthesis, production, then domain QA |
| Generic Rust refactor with no domain-contract change | No local specialist; use global implementation and review skills |
| Small docs correction with no project-state change | Direct work; no `_workspace` artifacts |
| Broad future milestone without implementation authorization | Design or roadmap clarification only; do not silently implement it |

## Phase Order

### Phase 0: Request Framing

- Inputs: user request, repository state, `SPEC.md`, active `ROADMAP.md` milestone.
- Actions: state outcome, milestone, scope, non-goals, touched boundaries,
  expected files, verification, and evidence limits.
- Output: `_workspace/00_input/request-summary.md` for substantial work.
- Complete when: another contributor can distinguish requested work from future
  roadmap possibilities.

### Phase 1: Selective Domain Design

- Inputs: request summary, canonical documents, relevant implementation/tests.
- Actions: route to the simulation designer, agent-ecology designer, or both.
- Outputs: `_workspace/01_simulation-design.md` and/or
  `_workspace/01_agent-ecology-design.md`.
- Complete when: each required boundary has an explicit, testable contract and
  uncertainty is recorded rather than guessed.

### Phase 2: Synthesis and Production

- Inputs: request summary and applicable design artifacts.
- Actions: when both designs exist, reconcile observations, actions,
  stochasticity, metrics, and claim limits in one synthesis; then produce the
  requested code or documents.
- Output: `_workspace/02_design-synthesis.md` when both specialists ran, plus
  requested repository files.
- Complete when: one integration owner has resolved cross-design conflicts and
  focused verification is available.

### Phase 3: Domain QA and Revision

- Inputs: original request, design/synthesis artifacts, changed files, canonical
  documents, and verification output.
- Actions: cross-check project-specific boundaries and return `pass`, `fix`, or
  `redo`.
- Output: `_workspace/03_domain-qa.md`.
- Complete when: `pass` is recorded, or unresolved blockers and skipped evidence
  are reported honestly.

Revision policy:

- `fix`: make targeted changes and rerun affected checks once;
- `redo`: return to Phase 1 and preserve the rejected artifact for comparison;
- after two unsuccessful revision cycles, stop and expose the unresolved design
  conflict rather than weakening the acceptance criteria.

### Phase 4: State Reconciliation and Handoff

- Inputs: passing output and verification.
- Actions: update only affected canonical project-state documents and summarize
  next dependencies and residual risks.
- Output: `_workspace/final/handoff.md` when durable continuation evidence is
  useful.
- Complete when: repository docs do not contradict verified behavior and a new
  contributor can resume without chat history.

## Handoff Contracts

### `_workspace/00_input/request-summary.md`

- Requested Outcome
- Roadmap Milestone
- Current Evidence
- In Scope
- Non-Goals
- Project Boundaries Touched
- Source Files
- Expected Outputs
- Verification
- Evidence Limits and Open Questions

### `_workspace/02_design-synthesis.md`

- Inputs Reviewed
- Agreed Actor Information
- Agreed Action and Transition Boundary
- Agreed Randomness Ownership
- Agent Policy and Execution Boundary
- Metrics and Evidence Limits
- Conflicts Resolved
- Unresolved Questions
- Production Contract

### `_workspace/final/handoff.md`

- Outcome
- Changed Files
- Verification
- Domain QA Disposition
- Canonical State Updates
- Known Limits
- Next Milestone Dependencies

The specialist skills define their own deterministic section contracts. Runtime
artifacts are created only when their inspection, resumption, audit, or
cross-agent consumption value justifies them.

## Claim Precedence and Promotion Evidence

This section settles two recurring failures: documents contradicting each other, and prose
quietly promoting a subsystem up the evidence ladder (`docs/decision_brief_20260830.md`
D7 and D9).

### Precedence

When two documents disagree, the lower rule wins, and the higher one is corrected in the same
change rather than left to argue:

1. Executed code and its tests, and the output a repository command actually prints.
2. `docs/TERMINOLOGY.md` for the meaning of a domain term, and `docs/COMPATIBILITY.md` for an
   artifact, version, or migration claim.
3. `SPEC.md` for delivered scope, `ARCHITECTURE.md` for module boundaries and ownership, and
   `ROADMAP.md` for milestone status and evidence.
4. `README.md` and `HOW_TO_PLAY.md` for what a reader may do and what may be claimed about it.
5. `CHANGELOG.md` for what changed in a given version.
6. Dated artifacts — `docs/audit_report_*.md`, `docs/decision_brief_*.md`, ADRs, run outputs.

A dated artifact is evidence about its date. It may record that a claim was once wrong, and it
may set a decision, but it never describes current behaviour and never promotes a claim on its
own. Quote it as history, with its date; never paraphrase it into present tense. If a dated
artifact's premise turns out to be false — an audit count that does not match the tree, a
decision that names a module doing something it does not do — implement the accepted *intent*,
correct the artifact in the same change, and keep the correction out of player-facing strings.

One active phase is a checker-enforced convention, not a statement about where work lands.
`ROADMAP.md` declares one current milestone and exactly one phase with `**Status:** Active`;
Phase 3 (`M3`) holds that marker because its remaining exit conditions are human-evidence checks,
and they are blocked by the same human gate as `D6`. Accepted-decision slices are recorded as
evidence sections in the phase whose behaviour they change — Phase 9 for the match slices — while
`M3` stays Active. A reader who takes `Active` as "this is where the next commit goes" will read
the roadmap backwards.

### Promotion evidence

Every new or changed subsystem records, in the `ROADMAP.md` evidence section for its slice:

- **Primary audience** — player, agent, or researcher. Where two audiences use one surface, name
  both and say what each can see.
- **The specific evidence that would promote it** one rung on the ladder in `README.md`, named
  narrowly enough that a reader can tell whether it exists.
- **What is explicitly not claimed**, stated in the same paragraph rather than in a footnote.

The default disposition is the rung the evidence actually reaches, and the ladder has five rungs:
implemented, deterministic, technically verified, human validated, externally released. Writing
the note by hand is required before any checker enforces it; a checker that only detects an
absent heading produces compliant, empty prose, which is the failure this rule exists to stop.

## Optional Delegation Policy

Use direct work unless specialization, independent read-heavy exploration, or
context isolation has concrete value.

When delegation is available:

- keep maximum depth to one worker layer;
- the orchestrator remains synthesis and acceptance owner;
- independent read-heavy designers may work from the same request snapshot;
- concurrent writers must own non-overlapping files or isolated checkouts;
- tests sharing snapshots, ports, processes, or generated state run serially
  unless their resources are isolated;
- a failed specialist returns partial evidence and uncertainty; synthesis never
  invents the missing branch;
- conflicting outputs are preserved and resolved in
  `_workspace/02_design-synthesis.md` or escalated.

No model, provider, concurrency count, retry heuristic, or agent runtime is
required by this portable contract.

## Failure Policy

- Missing canonical input: inspect repository truth first; if an essential
  product choice remains unknowable and materially changes the result, report it.
- Missing external evidence: label the mechanism or behavior as a design
  abstraction and record the validation need.
- Scope pressure: narrow to the smallest dependency-complete slice for the named
  milestone; do not implement later phases as incidental infrastructure.
- Conflicting domain assumptions: preserve both, choose the narrower reversible
  contract only when it satisfies the request, and record the choice.
- Hidden-state or authority leak: return `redo` when the design depends on it;
  return `fix` when the leak is an isolated projection or DTO defect.
- Non-reproducible behavioral result: retain it as exploratory evidence only and
  require a versioned, matched-input rerun before regression claims.
- Human-evidence gap: limit the claim; do not block unrelated technical progress
  unless the requested outcome itself requires human validation.
- Intellectual-property uncertainty: do not claim release readiness or rights;
  keep the original-setting fallback and policy review visible.

## Validation Checklist

- Every local `SKILL.md` begins with YAML frontmatter containing `name` and
  `description`.
- Local skill selection boundaries are distinct from each other and from global
  skills.
- Skill and team-spec handoff paths match exactly.
- Every phase has a named output and completion criterion.
- The domain-QA producer/reviewer edge has a bounded revision policy.
- Delegated work names ownership, synthesis, partial-failure, and conflict rules.
- Parallel writes and stateful tests require disjoint ownership or isolation.
- No model-specific runtime assumption is needed to use the harness.
- `AGENTS.md` remains short and points here for conditional detail.
- Each new or changed subsystem names its primary audience and the evidence that would promote
  it, in the `ROADMAP.md` section for its slice, with the unclaimed part stated alongside.
- A document contradicted by executed code is corrected in the same change, and no dated
  artifact is restated as present behaviour.

## Scenario Tests

### Normal flow: first deterministic decision window

Request: "Implement the smallest deterministic lane decision window with hold,
pressure, and recall intents."

Expected flow:

1. framing ties the task to M1/M2 and excludes a full map, MCP, and GUI;
2. simulation design specifies actor-visible observations, commands, explicit
   execution inputs, events, effects, state hash, and replay test;
3. production implements one complete slice;
4. domain QA cross-checks observation leakage, intent/execution separation,
   determinism, and causal debrief output.

### Combined flow: trust-sensitive gank response

Request: "Add an allied-jungler gank proposal whose follow rate varies with
trust while matched execution inputs stay fixed."

Expected flow:

1. simulation design owns proposal, observation, command, and resolution
   contracts;
2. agent-ecology design owns trust, candidate/evaluation effects, matched inputs,
   and directional metrics;
3. orchestrator synthesis resolves the trust-to-policy boundary without placing
   trust sampling inside the deterministic transition;
4. domain QA reviews both sides together.

### Failure flow: privileged optimization request

Request: "Make the bot choose the highest-value action by reading full world
state so we can establish a strong baseline."

Expected behavior:

- the agent-ecology design rejects privileged state for an actor baseline;
- it may define a clearly labeled research-only oracle outside ordinary play;
- actor policies remain observation-bound;
- domain QA returns `redo` if the oracle is presented as a playable agent or
  allowed to contaminate actor-visible evaluation.

### Near miss: generic formatting change

Request: "Run rustfmt and fix formatting in the current binary."

Expected behavior: use direct/global workflow, create no domain handoffs, and do
not invoke the local designers.

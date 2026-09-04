# Lessons

This ledger records verified, reusable project lessons. Add an entry only when
the context, cause, successful resolution, and prevention step are supported by
repository evidence and likely to recur. Keep entries concise and link to the
canonical policy instead of duplicating it.

## Support in-tree workspace member path dependencies in repository package checkers

- Context: Transitioning to a multi-crate Cargo workspace introduces internal path dependencies (`foi-kernel = { path = "crates/foi-kernel" }`).
- Symptom: `scripts/check_repository.py` fails with `dependency requires an approved advisory/license scanner or a complete defer record: foi-kernel` because `cargo metadata` reports all dependencies.
- Resolution: Check `if dependency.get("path"): continue` in `validate_dependency_exceptions` to treat internal workspace crates as repository-native code.
- Prevention: When partitioning workspaces or adding internal crates, ensure package auditing tools distinguish in-tree paths from external registry packages.

## Keep catalog expected summary metrics strictly aligned with underlying session fixture arithmetic

- Context: Benchmark catalogs define expected metrics (such as `expected_completion_rate_bp`) against deterministic fixture session arrays.
- Symptom: Benchmark execution returns `all_expectations_met: false` when manual estimation diverges from exact integer arithmetic (e.g. 7 of 8 completed sessions is 8,750 bp, not 7,500 bp).
- Resolution: Compute exact integer basis points (`completed * 10,000 / total`) from the fixture session list when constructing catalog scenario definitions.
- Prevention: Double-check arithmetic totals across all participant records when authoring new benchmark scenarios.

## Account for Markdown bolding markers in structured line assertions

- Context: Report generation often formats summary lines as `- **Key:** Value` (e.g. `- **Regression Gate Status:** PASS`).
- Symptom: String search assertions (`contains("Regression Gate Status: PASS")`) fail because the `**` bolding delimiter breaks exact substring matching.
- Resolution: Include the formatting markers in the assertion (`contains("**Regression Gate Status:** PASS")`) or assert on individual tokens.
- Prevention: Check the exact format string in the report builder when authoring assertion matches for rendered report headings and bullet items.

## Align enum string assertions across Display, Debug, and custom as_str representations in report generators

- Context: Report builders and CLI presenters often mix `Debug` formatting (`"{:?}"` -> `"AlphaReady"`) in summary tables with domain `as_str()`/`Display` formatting (`"alpha-ready"`) in rendered markdown blocks.
- Symptom: Integration and MCP tool test assertions looking for enum values fail when assuming `Debug` casing where kebab-cased `as_str()` is rendered.
- Resolution: Check the exact render implementation of the underlying report types (`render_markdown()` uses `self.disposition.as_str()`) and verify test assertions against the actual rendered string output.
- Prevention: When writing integration tests for complex composite reports, verify the exact string representations produced by both summary tables and nested markdown sections.

## Keep scenario catalog expansion synchronized across menu numbering, selection parsers, help strings, and CLI test fixtures

- Context: Adding new executable scenarios (such as `m8-team-scenarios-v1`) shifts catalog length, 1-based numerical index assignments, menu presentation lines, and usage help strings.
- Symptom: Hardcoded catalog length checks (`len() == 8`), menu assertions (`[5] Interactive 5v5`), and selection bounds (`parse("9") == None`) fail in unit and binary tests when a new scenario is inserted mid-catalog.
- Resolution: Insert new scenarios with explicit mode classifications, update all numeric index bounds in `parse_scenario_selection` (`1..=N`), update all expected usage strings and menu test assertions synchronously, and test both numeric and slug alias resolution in binary test suites.
- Prevention: When adding or reordering scenario catalog entries, audit all references to catalog length, index-to-scenario mapping, and help usage strings in `command_loop.rs` and `tests/binary_run_dir.rs`.

## Keep MCP JSON-RPC stdio servers dependency-free, fail-closed on malformed inputs, and free of floating-point integer casts

- Context: M5 implements the Model Context Protocol (MCP) JSON-RPC 2.0 stdio server adapter (`src/mcp/`) communicating with external LLM agents.
- Symptom: Heavy third-party JSON/RPC dependencies increase build times and attack surface; float conversions in custom parsers trigger `clippy::as_conversions` denials; malformed lines or missing arguments cause server panics.
- Resolution: Build a compact, deterministic recursive-descent JSON parser and JSON-RPC 2.0 dispatcher with zero third-party dependencies, map invalid requests directly to standard JSON-RPC error codes (`-32700`, `-32600`, `-32601`, `-32602`), and parse numeric values strictly as `i64` without floating-point fallback casts.
- Prevention: In protocol and transport adapter layers, use deterministic string and integer parsing; avoid `as` conversions and third-party crate additions.

## Keep match runner phase names and scenario catalog assertions synchronized across unit, presentation, and binary test boundaries

- Context: Expanding the scenario catalog and adding an interactive match runner (`m9-interactive-match-v1`) shifts catalog indexing, help text strings, and terminal phase action strings.
- Symptom: Integration tests fail when expected action strings diverge from core enum serialization (`as_str()`) or when help text / catalog length checks are hardcoded to previous catalog counts.
- Resolution: Reference canonical `MatchPhaseKind::as_str()` in terminal formatters and tests, update catalog assertions across all test fixtures synchronously, and keep turn-stepping expectations grounded in the scenario's initial turn (e.g. initial turn 1 stepping to 14/15).
- Prevention: When introducing new interactive runner modes or scenarios, check all catalog-dependent assertions and match output string contracts across unit and binary test suites.

## Keep GUI DTO projections presentation-only, actor-visible, and invariant-validated to prevent latent state leakage

- Context: M11 introduces graphical presentation contracts for spatial map topologies, timelines, plans, and causal debriefs.
- Symptom: A graphical client receiving full world state or raw state hashes could allow client-side inspection to bypass the fog of war or turn client rendering into an accidental second simulation authority.
- Resolution: Structure GUI models as versioned actor-visible DTOs (`m11-gui-dto-v1`), enforce invariant validation (`validate_invariants`) rejecting any unseen opposing actor revealing true coordinates or debriefs containing private chain-of-thought, and keep simulation authority exclusively host-owned as mandated by ADR-0003.
- Prevention: In graphical clients, always project from actor-visible observations at the host edge, redact latent truth before serialization, and enforce zero client-side simulation authority.

## Register new deterministic-core modules in the repository checker's boundary list

- Context: New pure simulation modules under `src/map/` (and other deterministic-core paths) must be classified in `scripts/check_repository.py`'s core boundary file list, which scans classified modules for async, wall-clock, and network-transport primitives.
- Symptom: Local `fmt`/`clippy`/`test` pass, but CI's `verify` job fails `python3 scripts/check_repository.py` with `unclassified core boundary file: src/map/<new>.rs`, requiring a follow-up "Fix CI issues" commit after the PR.
- Cause: The checker fail-closes on any unlisted file in core module directories, and the three cargo verification commands do not include it.
- Resolution: Add each new core module (and its catalog/tests wiring when applicable) to the alphabetical list in `scripts/check_repository.py` in the same commit as the module, then run `python3 scripts/check_repository.py` locally before pushing.
- Prevention: Treat the repository checker as a fourth local verification command alongside fmt, clippy, and test whenever the change adds files under `src/`.

## Keep role-specific observations, actions, and debrief perspectives decoupled from state authority and bounded by role validation

- Context: M9 required implementing 5 distinct match roles (`TopLaner`, `Jungler`, `MidLaner`, `BotCarry`, `Support`) with specialized situational contexts, tactical intents, and causal performance reviews without polluting authoritative match state with role-private heuristics or leaking fog-of-war truth.
- Symptom: Defining open-ended unstructured action payloads or hardcoding role logic into the authoritative transition would create hidden state channels, bypass actor observation boundaries, or introduce non-deterministic role evaluation.
- Resolution: Define `RoleMatchObservation` wrapping spatial map observations with role-typed context, `RoleIntent` closed enums validated via `validate_role_action`, and `RoleDebriefPerspective` computing exact integer basis-point KPIs ($[0..=10,000]$ bp) and discrete causal factors (`RoleCausalFactor`) with zero private chain-of-thought enforcement.
- Prevention: In multi-role simulations, always keep role specialization in observation projections and intent validation gates, ensuring the simulation transition operates on verified spatial state.

## Enforce defensive structure vulnerability hierarchies and inhibitor countdowns deterministically to model lane progression

- Context: M9 required implementing 3-lane match structures (26 turrets, inhibitors, nexus) and super minion spawning while preserving pure deterministic transitions and replay hashing.
- Symptom: Allowing arbitrary or out-of-order structure targeting or modeling continuous HP regeneration would create non-deterministic siege states or bypass lane macro sequencing.
- Resolution: Formalize discrete `StructureTier` hierarchy with strict vulnerability predicates (`is_vulnerable`: Outer -> Inner -> Inhibitor Turret -> Inhibitor -> Nexus), deterministic siege resolution (`transition_structure_siege`), turn-ticked inhibitor respawn countdowns, and integer basis-point defense mitigation.
- Prevention: In match-level structure and base siege mechanics, always enforce strict defense dependency trees before applying damage and compute deterministic FNV-1a state hashes across all structure statuses.

## Quantify cross-map objective concessions through bounded integer basis-point tradeoffs to prevent outcome bias in macro decisions

- Context: M9 required implementing neutral objective spawning cycles and cross-map contest mechanics where teams choose between fighting directly for an objective or conceding it to trade for opposite map pressure (Herald for Dragon, tower pushes, jungle camps).
- Symptom: Evaluating objective losses solely as tactical failures penalizes teams for executing high-value cross-map macro trades, collapsing macro evaluation into binary contest win/loss outcomes.
- Resolution: Formalize `TradeoffEvaluation` and `TradeClassification` (`FavorableTrade`, `EvenTrade`, `UnfavorableConcession`, `DesperationSacrifice`) with exact integer basis-point delta scaling ($[-10,000..=10,000]$ bp) measuring the net strategic difference between conceded and secured assets.
- Prevention: In match-level evaluation and debrief systems, always separate local objective concessions from cross-map tradeoff gains; evaluate macro decisions against multi-lane value deltas.

## Disagreement in team strategy can be strategically legitimate and must be evaluated through counterfactual value deltas

- Context: M8 required proving and testing that autonomous teammate disagreement with shot-caller directives or peer proposals can be strategically legitimate rather than simple insubordination or coordination failure.
- Symptom: Treating all compliance as good and all dissent as bad penalizes autonomous actors for refusing suicidal orders (such as contesting an objective while at critically low health under active jungle threat).
- Resolution: Formalize `DisagreementLegitimacyClassification` (`LegitimateDissent`, `ConstructiveAlternative`, `UnjustifiedInsubordination`) and compute exact basis-point counterfactual value deltas ($[-10,000..=10,000]$ bp) measuring the net payoff difference between dissenting vs complying.
- Prevention: In causal debriefs and team evaluation systems, never equate compliance with strategic correctness; evaluate dissent against local threat conditions and counterfactual payoffs.

## Decouple coordination effectiveness from mechanical execution in causal debriefs

- Context: M8 required attributing team coordination success and failure separately from physical execution outcomes to eliminate outcome bias in debriefs.
- Symptom: Evaluating team play solely on win/loss or lane status creates severe outcome bias: sound coordinated plans defeated by mechanical clutch or luck are penalized, while uncoordinated chaotic play bailed out by individual duel outplays is praised.
- Resolution: Formalize a two-dimensional orthogonal decomposition (`AttributionQuadrant`) classifying outcomes into `CoordinatedTriumph`, `CoordinatedFailure`, `UncoordinatedBailout`, and `CompoundedFailure` using discrete thresholds ($\ge 5,000$ bp), discrete causal factor taxonomies, and exact basis-point sum conservation ($10,000$ bp invariant).
- Prevention: In causal debriefs and agent evaluations, always separate strategic decision/coordination quality from physical execution and exogenous variance.

## Keep leadership structures discrete, consensus rules deterministic, and influence distinct from direct control

- Context: M8 required implementing designated shot-caller heuristics, leaderless decentralized peer coordination,
  and shared leadership baselines without turning leadership into disguised direct actor control or introducing floating-point math.
- Symptom: Hardcoding leadership as authoritative command execution would bypass autonomous actor evaluation, while continuous
  voting or unbounded consensus loops would compromise determinism and causal debrief inspectability.
- Resolution: Structure leadership directives and peer proposals as communicative speech acts (`ShotCallerDirective`, `PeerPlanProposal`),
  evaluate compliance deterministically via `TeamTrustEvaluator` and local observations, arbitrate decentralized proposals via discrete
  `ConsensusRule` algorithms, and represent cohesion/compliance in exact integer basis points ($[0..=10,000]$ bp).
- Prevention: Enforce that leadership in Fog of Intent represents communicative influence and bounded consensus arbitration rather
  than direct control or privileged simulation authority.

## Keep team trust dynamics basis-point bounded and transmission channels deterministic

- Context: M8 required implementing caller reputation scoring, trust-modulated proposal compliance,
  communication clarity, transmission delay queues, and channel capacity overload limits before designated
  shot-caller heuristics or leadership election were authorized.
- Symptom: Continuous floating-point trust values or non-deterministic packet dropping would compromise
  replay reproducibility, leak latent state, or make communication failures uninspectable.
- Resolution: Track `CallerReputationRecord` in exact integer basis points ($[0..=10,000]$ bp), derive
  discrete `TeamTrustLevel` tiers, model channel transmission via bounded `TeamCommunicationChannel`
  (capacity 16 packets) with deterministic delay decrementing and categorical `DeliveryStatus` tracking.
- Prevention: Treat multi-agent trust dynamics, caller reputation, transmission channels, and designated
  shot-calling arbitration as separate contracts with separate evidence.

## Keep team plans and individual plans structurally decoupled and alignment evaluations deterministic

- Context: M8 required defining team-plan and individual-plan relationships, role assignments,
  and cohesion metrics before multi-agent trust dynamics or shot-calling arbitration were authorized.
- Symptom: Coupling team plan schemas directly to authoritative simulation state or allowing
  continuous floating-point alignment scoring would violate information privacy, leak latent state,
  or create non-deterministic divergence checks.
- Resolution: Decouple `TeamPlanDefinition` and `IndividualPlanDefinition` into discrete structures
  with zero private chain-of-thought enforcement, evaluate alignment deterministically via
  `TeamPlanEvaluator`, and represent cohesion scores in exact integer basis points ($[0..=10,000]$ bp).
- Prevention: Treat team plan structures, role assignments, individual plan bindings, alignment
  evaluations, and trust dynamics as separate contracts with separate evidence.

## Keep team dialogue transitions bounded and fail-closed

- Context: M8 required implementing speech act evaluation, condition checking, and multi-turn
  dialogue state transitions across all 8 canonical speech acts before multi-agent trust dynamics
  or shot-calling arbitration were authorized.
- Symptom: Unbounded back-and-forth negotiation or permissive state jumps could create infinite loops,
  leak private chain-of-thought, or let actors confirm contradictory proposals.
- Resolution: Define `TeamDialogueSession` with a strict message capacity (max 8 messages), negotiation
  round limits (max 4 rounds), fail-closed transition checks, and deterministic prerequisite condition
  evaluation (`TeamConditionEvaluator`).
- Prevention: Treat speech act evaluation, prerequisite condition checks, and dialogue state machines
  as separate contracts with separate evidence before adding trust dynamics or leadership election.

## Keep team speech acts typed, discrete, and visibility-bounded

- Context: M8 required defining speech acts, recipients, urgency, confidence, conditions,
  and message visibility before multi-agent trust dynamics or shot-calling arbitration were authorized.
- Symptom: Free-form conversational text or unstructured chat strings would bypass actor information
  boundaries, leak private chain-of-thought, or turn communicative proposals into disguised direct control.
- Resolution: Define `TeamSpeechAct` over 8 closed variants, `TeamRecipient` (broadcast vs direct),
  `TeamMessageUrgency`, `TeamConfidenceLevel`, `TeamMessageCondition`, and `TeamMessageVisibility` with
  leak-proof visibility predicates and fail-closed rejection if `chain_of_thought_present == true`.
- Prevention: Treat communicative speech acts, addressing, visibility redaction, trust dynamics,
  and leadership arbitration as separate contracts with separate evidence.

## Keep recalibration triggers discrete and urgency-classified

- Context: M7 required defining recalibration triggers upon model family or prompt protocol changes
  before live model APIs or continuous online learning were authorized.
- Symptom: Triggering recalibration based on informal heuristics or unbounded continuous gradients would
  turn discrete evaluation contracts into ad-hoc monitoring scripts.
- Resolution: Define `RecalibrationTriggerCondition` and `RecalibrationPolicy` with closed trigger reasons,
  exact integer basis-point thresholds ($1,500$ bp TVD, max 1 modal disagreement, $2,500$ bp held-out loss),
  and categorical urgency levels (`Immediate`, `Scheduled`, `None`).
- Prevention: Treat drift detection, uncertainty reporting, generalization evaluation, and recalibration
  policies as separate contracts with separate evidence.

## Keep multi-model comparisons discrete and basis-point bounded

- Context: M7 required comparing empirical choice distributions and fitted parametric
  policies across multiple model and prompting families before live model provider APIs
  or unidentifiable parameter diagnostics were authorized.
- Symptom: Introducing free-form text diffs, unscaled probabilities, or floating-point
  similarity scores across model families would blur the boundary between empirical calibration
  contracts and unconstrained latent-variable models.
- Resolution: Define `MultiModelComparisonReport` over discrete diagnostic dilemma domains
  using exact integer basis-point Total Variation Distance (TVD), modal agreement counts (0..=7),
  and a fail-closed alignment classification (`aligned`, `shifted`, `divergent`).
- Prevention: Treat empirical distribution estimation, behavioral distance measures,
  regularized policy fitting, held-out evaluation, and multi-model family comparisons as separate
  contracts with separate evidence.

## Keep parametric policy fitting regularized and basis-point bounded


- Context: M7 required parametric policy parameter estimation from empirical
  choice distributions before held-out scenario evaluation or live model
  providers were authorized.
- Symptom: Introducing continuous floating-point optimization or unbounded
  weight estimation would compromise cross-platform determinism and produce
  extreme, unidentifiable weights on skewed empirical distributions.
- Resolution: Define `ParametricPolicyFitter` using closed-form integer basis-point
  regularization shrinkage towards neutral uniform priors proportionally to
  `regularization_bp` in `0..=10_000` bp, enforcing exact basis-point sum conservation
  (`10_000` bp).
- Prevention: Treat parametric policy parameter representation, regularized
  fitting, held-out evaluation, and multi-model comparison as separate contracts
  with separate evidence.

## Keep diagnostic choice catalogs discrete and contrast-focused

- Context: M7 required diagnostic choice definitions across core behavioral
  dilemmas before empirical distribution estimation or model fitting was
  authorized.
- Symptom: Embedding complex scenario state trees or latent utility metrics in
  choice schemas would blur the boundary between strategic dilemma contracts
  and full simulation execution.
- Resolution: Define `DiagnosticChoiceDefinition` over discrete domains with
  explicit primary/alternative intent options and documented contrast strings,
  managed through a fail-closed `DiagnosticChoiceCatalog`.
- Prevention: Treat choice dilemma definitions, scenario generation, empirical
  distribution estimation, and parametric policy fitting as separate contracts
  with separate evidence.

## Keep semantic profile schemas discrete and declarative

- Context: M7 needed a compact semantic profile vocabulary before diagnostic
  scenario batteries or parametric model fitting were authorized.
- Symptom: Free-form prompt strings or unbounded natural-language traits would
  turn a schema into an uninspectable latent-variable engine.
- Resolution: Define `SemanticProfileDefinition` over discrete categorical
  dimensions (risk tolerance, deference, focus, communication clarity) with a
  fail-closed lookup catalog (`SemanticProfileVocabulary`).
- Prevention: Treat semantic trait schemas, diagnostic scenario choice
  batteries, and parametric policy fitting as separate contracts with separate
  evidence.

## Keep message envelopes separate from delivery

- Context: M5 needed a recipient-scoped communication shape while transport,
  authentication, and host routing remained explicitly deferred.
- Symptom: Reusing a draft value as if it were delivered metadata would blur
  actor authorship, recipient visibility, and host communication authority.
- Resolution: Define `ActorMessageDto` as an immutable sender/recipient/
  observation-bound envelope with bounded text, and keep routing, delivery,
  ordering, retries, and trust outside the protocol DTO.
- Prevention: Treat message shape, delivery acceptance, and communication
  quality as separate contracts with separate evidence before adding queues or
  session integration.

## Keep experiment manifests declarative

- Context: M6 needed reproducible run identity before a batch runner or
  population sampler was authorized.
- Symptom: Letting a manifest construct agents or execute runs would turn
  metadata into a second policy/transition engine.
- Resolution: `ScriptedAgentExperimentManifest` records only the fixture,
  constructor-owned profile/rule IDs, and caller-owned policy seed bundle in a
  bounded codec.
- Prevention: Add sampling, metrics, provider versions, and run artifacts as
  separate contracts with their own evidence rather than expanding metadata
  into execution authority.

## Cap local batches before policy evaluation

- Context: M6 needed a deterministic runner before resumable storage or
  population sampling was authorized.
- Symptom: An unbounded manifest list would turn a convenience helper into an
  accidental batch scheduler and make resource behavior uninspectable.
- Resolution: `ScriptedAgentBatchRunner` rejects empty input and caps one
  synchronous ordered batch at 16 manifests before constructing decisions.
- Prevention: Keep batch bounds explicit and add persistence, sampling, and
  scheduling as separate contracts with separate evidence.

## Formatter defaults do not establish project policy

- Context: The repository had a passing `cargo fmt --check` but no formatter or
  editor configuration for the requested two-space style.
- Symptom: The default Rust formatter accepted four-space indentation while a
  two-space check produced a large diff.
- Cause: Tool defaults validate their own style, not a project-specific style.
- Resolution: Pin `tab_spaces = 2` and `hard_tabs = false` in `rustfmt.toml`,
  set matching `.editorconfig` values, and run the pinned formatter in CI.
- Prevention: Treat formatter configuration and a passing CI check as one
  invariant; do not infer policy from a default formatter pass.

## Textual `include!` fragments can escape formatter coverage

- Context: Lane tests were included textually from `src/lane/tests/mod.rs`.
- Symptom: `cargo fmt --check` passed while running `rustfmt` directly on an
  included test file showed the requested indentation changes.
- Cause: Cargo formats discovered Rust modules, not arbitrary source fragments
  injected by `include!`.
- Resolution: Make the test files ordinary modules and scope shared helpers to
  the test hierarchy.
- Prevention: Keep Rust code in formatter-discoverable modules and verify the
  full `cargo fmt --all -- --check` command.

## Adopt Clippy lint groups from a clean baseline

- Context: A trial of pedantic and nursery groups produced 991 warnings, mostly
  broad documentation, `must_use`, and style suggestions.
- Symptom: The warning volume obscured the small set of concrete type-safety
  findings.
- Cause: A blanket lint group expands the maintenance surface beyond the task's
  verified contracts.
- Resolution: Adopt only `clippy::as_conversions = "deny"` after fixing the
  bounded conversions it identifies; retain broader groups as audit input.
- Prevention: Start each new lint with a clean baseline, a bounded scope, and a
  documented reason before making it a CI gate.

## Bounded numeric values should not rely on unchecked casts

- Context: Gold, experience, cooldown, queue indices, and delayed-effect counts
  were bounded by nearby invariants but used `as` conversions.
- Symptom: The code was correct for current limits but could silently truncate
  if a type or bound changed.
- Cause: Primitive casts do not express whether conversion is lossless or
  validated.
- Resolution: Use checked arithmetic, `From`, `TryFrom`, and typed saturation or
  expiry helpers while preserving existing overflow errors and hashes.
- Prevention: Keep `clippy::as_conversions` denied and add boundary tests when a
  bounded value or conversion changes.

## Redaction labels should be payload-free

- Context: The CLI needed to distinguish observed, believed, inferred,
  reported, and unknown information before a renderer or host existed.
- Symptom: A generic label-plus-value structure could accidentally pair an
  `unknown` label with hidden state and leak it at an adapter boundary.
- Cause: Redaction represented as ordinary metadata does not make the absence
  of a value structurally enforceable.
- Resolution: Model `CliInformation<T>` as a disjoint enum whose `Unknown`
  variant has no payload; verify borrowed projections preserve labels and
  explicit extraction returns `None` for unknown values.
- Prevention: Keep actor-visible redactions uninhabited in adapter DTOs and add
  tests that exercise both the label and payload boundary.

## Commit boundaries should consume editable drafts

- Context: The CLI needed pre-commit edits and undo without allowing a future
  adapter operation to rewrite committed lane history.
- Symptom: A mutable draft/session value could accidentally retain edit or undo
  operations after the caller believed it had committed choices.
- Cause: A runtime flag does not make the post-commit operation set visible or
  enforceable at the type boundary.
- Resolution: Keep `CliDraft` editable, make `undo()` clear only that value,
  and consume it into `CliCommittedDraft`, which exposes only read-only getters.
- Prevention: Model irreversible adapter boundaries with consuming operations
  and marker types before adding host or persistence integration.

## Validate artifact identifiers at the adapter edge

- Context: Save/load/replay/export grammar values were arbitrary non-empty
  strings before persistence existed.
- Symptom: A future host could receive whitespace, path separators, or
  overlong identifiers and have to reinterpret adapter input inconsistently.
- Cause: Non-empty validation does not define a portable human-readable ID
  syntax or a bounded failure contract.
- Resolution: Add borrowed `CliRunId` validation with explicit length and
  character rules, then carry the type through affected request mappings while
  leaving storage and authority to the host.
- Prevention: Validate adapter identifiers before host execution and keep ID
  syntax separate from persistence, branch points, and replay identity.

## Keep host fixtures explicit about resolved inputs

- Context: The first CLI host needed a deterministic two-window transcript
  before a random resolver or persistence service existed.
- Symptom: Letting the host manufacture execution inputs would blur the
  authority boundary and make replay evidence depend on hidden fixture state.
- Cause: A convenient host constructor can silently become a second simulation
  engine when it chooses randomness or true-state values internally.
- Resolution: Require already-resolved `LaneResolvedInputs` at host
  construction and keep the fixture helper deterministic, while returning only
  actor-valid projections.
- Prevention: Resolve randomness at the edge, pass inputs into the pure
  transition contract, and label in-memory host behavior as fixture evidence
  rather than persistence or terminal UX evidence.

## Keep text renderers downstream of redacted projections

- Context: The first terminal-facing text surface needed to make host results
  understandable before a command loop or terminal integration existed.
- Symptom: Rendering raw transition/domain errors or snapshots would make a
  convenient presentation helper an accidental hidden-state channel.
- Cause: A renderer that accepts authoritative values has no structural reason
  to preserve actor information boundaries.
- Resolution: Render only `CliHostOutput`/`CliHostError` values whose host API
  already excludes snapshots, hashes, and raw domain failures; sanitize echoed
  control characters and use stable labeled lines.
- Prevention: Keep presentation downstream of actor-valid DTOs, make errors
  actionable but bounded, and treat plain text/accessibility structure as
  evidence separate from terminal I/O or human usability claims.

## Keep command-loop I/O at the outer edge

- Context: The first executable loop needed to connect line input/output to the
  bounded host without changing the deterministic core.
- Symptom: Putting buffering, prompts, retries, or terminal writes in host or
  renderer code would make simulation behavior depend on environment effects.
- Cause: A small CLI loop can look like harmless glue while quietly becoming a
  second lifecycle or presentation authority.
- Resolution: Keep `BufRead`/`Write` handling in `command_loop.rs`, pass each
  line to the host, render only the returned actor-valid value, and continue on
  bounded errors until `quit` or end-of-input.
- Prevention: Treat I/O as an outer adapter concern, keep the loop
  line-oriented and deterministic, and test recovery/exit behavior separately
  from terminal usability or accessibility claims.

## Bind host artifacts to replay evidence

- Context: In-process save/load needed a versioned artifact before a durable
  file store could be justified.
- Symptom: Storing only committed intents would let a different resolved-input
  fixture load the same run with silently different outcomes and hashes.
- Cause: Intent text is not sufficient to identify the deterministic execution
  that produced a committed history.
- Resolution: Encode the replay identity, each record's prior/result hashes,
  and the full lane-record identity, then rebuild through the current explicit
  inputs and reject any mismatch.
- Prevention: Treat artifacts as compatibility contracts, validate them before
  replacing host history, and keep filesystem placement/atomicity as a separate
  outer-edge concern.

## Inject file storage instead of hiding a default directory

- Context: The host needed durable artifact support without making the fixture
  binary choose a user directory or changing command-loop determinism.
- Symptom: A global or implicit path would make tests environment-dependent and
  turn an adapter convenience into an unreviewed deployment policy.
- Cause: Persistence configuration was being conflated with the host's
  simulation and command contracts.
- Resolution: Inject `CliRunStore` explicitly, validate IDs before path joins,
  bound file size, and replace artifacts through a same-directory temporary
  rename while retaining an in-memory default.
- Prevention: Keep directory selection at the application edge and document
  locking, fsync/crash recovery, and binary wiring as separate evidence gates.

## Keep process persistence configuration explicit

- Context: The injected file store was ready for fresh-host tests, but the
  executable still had no way to opt into it between processes.
- Symptom: Adding an implicit directory would make ordinary fixture runs write
  to an environment-dependent location and turn a library boundary into a
  deployment policy.
- Resolution: Have `src/main.rs` invoke one bounded `--run-dir <path>` parser
  from the application-edge loop module, retain the no-argument in-memory
  default, reject option-shaped path values, and verify save/load with two
  separate binary processes.
- Prevention: Keep process configuration outside the session grammar and add a
  cross-process smoke test whenever persistence becomes executable behavior.

## Keep counterfactual branches read-only

- Context: The lane already had a deterministic one-window branch contract, but
  the host still rejected the CLI `branch` request.
- Symptom: Evaluating an alternate intent through the live scenario history
  could accidentally replace committed records or make a branch look like the
  authoritative run.
- Resolution: Rebuild a temporary verified parent history, use matched-parent
  branch inputs, and return only an actor-safe comparison while retaining the
  host draft, history, replay, and saved artifact.
- Prevention: Treat branch review as a read-only projection until a versioned
  branch artifact and explicit persistence contract are designed.

## Fail closed on executable scenario identifiers

- Context: The fixture binary needed an explicit scenario name without turning
  its single supported fixture into an implicit, silently changing default.
- Symptom: Accepting any string and falling back to the existing fixture would
  make a typo appear successful and would hide compatibility boundaries from
  scripts and future scenario additions.
- Cause: Process-edge configuration was being treated as an open string rather
  than a closed construction contract.
- Resolution: Parse one versioned ID into a closed scenario enum, keep omission
  as an explicit default, and reject missing, empty, option-shaped, duplicate,
  and unsupported values before stdin reaches the session grammar.
- Prevention: Keep scenario construction at the application edge, keep errors
  path-free and stable, and require a focused process-status regression before
  adding another selectable fixture.

## Separate text-shape evidence from human accessibility

- Context: The pure renderer needed stronger evidence for a keyboard-first
  adapter without pretending that a test can stand in for users or assistive
  technology.
- Symptom: A plain-text/no-ANSI assertion can be misread as a screen-reader or
  keyboard usability result if the claim boundary is not explicit.
- Cause: Structural output invariants and human interaction evidence answer
  different questions and require different methods.
- Resolution: Check stable lowercase labels, newline structure, and sanitized
  control characters over a representative output/error transcript, while
  leaving the existing stdin/stdout adapter unchanged and deferring only
  terminal-specific prompts/focus plus human keyboard/screen-reader inspection.
- Prevention: Name machine-checkable text shape separately from accessibility
  validation in roadmap, QA, and handoff documents.

## Keep process metadata outside the host

- Context: The executable needed a scriptable package-version response alongside
  scenario and run-directory options.
- Symptom: Routing `--version` through the command loop would construct host
  state, read stdin, or make metadata depend on simulation behavior.
- Cause: Process invocation metadata and session commands have different
  lifecycles and compatibility concerns.
- Resolution: Handle standalone `--version`/`-V` in the application edge and
  derive the output from Cargo package metadata before constructing a host.
- Prevention: Keep metadata responses bounded and side-effect free, and treat
  schema negotiation or migrations as separate evidence-gated work.

## Keep executable evidence distinct from library evidence

- Context: The host already had a complete library transcript, but the roadmap
  also requires proof that a clean checkout can use the real executable.
- Symptom: A direct host test can pass while process argument construction,
  stdin/stdout wiring, exit status, or rendered output integration is broken.
- Cause: Library authority tests and application-edge integration tests observe
  different boundaries.
- Resolution: Add a bounded binary transcript using only documented commands,
  assert process status/stderr and actor-safe markers, and retain the library
  transcript as complementary evidence.
- Prevention: Do not promote library-only evidence to executable or complete
  reference-client claims without a real process regression.

## Keep scripted policies actor-visible and host-validated

- Context: The first M4 policy needed to demonstrate a reproducible agent
  choice without creating a second simulation engine or privileged actor.
- Symptom: A policy that reads true state or performs legality checks can make
  agent behavior impossible to compare fairly with human or external-agent
  inputs and can bypass the host's replay boundary.
- Resolution: Generate candidates only from `LanerObservation`, record
  versioned candidate/evaluation/selection rules, return an observer-bound
  `LaneIntentRequest`, and let the host validate freshness and legality.
- Prevention: Keep policy outputs inspectable and actor-validatable; defer
  memory, communication, randomness, population comparisons, and strategic or
  human-realism claims until each has its own evidence contract.

## Compare profiles on matched actor-visible inputs

- Context: A second scripted profile was needed to show interpretable policy
  differences without adding scenario, execution, or population complexity.
- Symptom: Comparing profiles across different observations would confound
  policy differences with information differences and weaken reproducibility.
- Resolution: Run cautious and risk-taking fixed rules over one identical
  observation, assert distinct selected intents, and validate both requests
  through the existing host/lane boundary.
- Prevention: Keep matched-input comparisons separate from outcome or human
  realism claims; add new scenarios and metrics only with their own evidence.

## Reject policy evaluation outside advertised candidates

- Context: Public policy helpers can be called independently of the normal
  generate-then-select path.
- Symptom: Scoring an intent that the observation did not advertise can make a
  policy result look legal even though the actor never had that candidate.
- Resolution: Check the actor-visible candidate set before scoring and return a
  bounded `UnavailableIntent` policy error; internal selection scores only its
  generated candidates.
- Prevention: Keep policy errors distinct from host legality errors and fail
  closed before any request reaches transition or history authority.

## Keep a baseline profile catalog small and versioned

- Context: M4 needed interpretable differences without introducing a policy
  framework or a large uncalibrated population.
- Symptom: Unbounded profile growth makes matched-input evidence hard to read
  and encourages unsupported claims about strategy or roles.
- Resolution: Add three named profiles with explicit evaluation-rule IDs and a
  single matched-observation regression before expanding the catalog.
- Prevention: Require each new profile to share the actor-visible candidate and
  host-validation boundary, and report fixture-sized comparisons honestly.

## Test profile sensitivity at an information boundary

- Context: The catalog needed evidence that profiles respond differently when
  the actor-visible threat report changes.
- Symptom: A matched-input comparison alone cannot show whether a profile
  reacts to newly visible information or merely returns a fixed label.
- Resolution: Compare safe and RiverSide observations for all three profiles,
  assert the cautious threat response and the other fixed preferences, and
  validate every request through the host boundary.
- Prevention: Keep sensitivity tests tied to visible observation changes and
  avoid treating selection differences as outcome or balance evidence.

## Keep comparison metrics actor-safe and bounded

- Context: M4 needed a machine-readable comparison artifact for the three
  fixed profiles without turning metrics into a second authority path.
- Symptom: A report that carries state hashes, execution inputs, or raw domain
  errors can leak privileged truth and become an accidental replay interface.
- Resolution: Version the report and expose only profile/rule IDs, selected
  intent/score, candidate count, and observer/observation identity.
- Prevention: Keep metric schemas derived from actor-visible policy decisions and
  state their fixture-sized limits before adding outcome or population fields.

## Keep policy roles distinct from scenario roles

- Context: M4 needed transparent posture labels for fixed profiles without
  changing the lane roster or implying richer role behavior.
- Symptom: Reusing scenario actor-role names for policy heuristics can blur
  ownership and make metadata look like hidden simulation state.
- Resolution: Use versioned `Anchor`, `Duelist`, and `Pacer` policy IDs and
  assert their profile bindings while leaving `LaneActorRole` untouched.
- Prevention: Treat policy roles as inspectable labels only until role behavior,
  populations, and outcomes have separate evidence.

## Bind monotonic policy effects to actor-visible features

- Context: M4 needed its first utility-feature regression without introducing
  memory, randomness, or a second simulation authority.
- Symptom: A fixed score table can demonstrate plumbing but cannot show that a
  policy responds directionally to a declared actor-visible condition.
- Resolution: Use the bounded `LanerObservation::wave_pressure()` value only in
  the Anchor `Stabilize` score, version the pressure-aware rule, and compare
  pressure 0 with pressure 3 while validating both requests through the host.
- Prevention: Keep monotonic checks tied to matched observations and state
  whether the result is a score relation rather than an outcome, balance, or
  strategic-quality claim.

## Keep action tallies bounded and observer-consistent

- Context: M4 needed its first action-distribution evidence after comparing
  fixed profiles on safe and visible-threat observations.
- Symptom: Aggregating selected intents without an observer check can mix
  unrelated actor views, while a broad distribution can be mistaken for a
  population or outcome metric.
- Resolution: Version a two-observation actor-safe tally, reject mixed
  observers, and expose only profile/rule IDs, the fixed count, and selected
  intent counts; validate each underlying request through the host boundary.
- Prevention: Describe fixture tallies as bounded action evidence and defer
  population, outcome, strategic-quality, and human-behavior claims until those
  datasets and contracts exist.

## Treat candidate breadth as separate from action diversity

- Context: M4 needed a creativity contract without introducing random policy
  choices or a population sampler.
- Symptom: Counting generated candidates can be mistaken for strategic or
  behavioral diversity if the observation's advertised options are not shown.
- Resolution: Compare safe and visible-threat observations, assert four versus
  five unique candidates, and require every candidate to come from the
  actor-visible intent or threat-response fields.
- Prevention: Call this candidate-generation evidence only; defer transformed
  candidates, random sampling, population distributions, and outcome claims.

## Make deterministic tie-breaking an explicit policy contract

- Context: M4 selected the highest scored candidate but needed evidence for
  what happens when two actor-visible candidates share a score.
- Symptom: An implicit reduction rule can silently change advertised-order
  behavior while profile-level outputs still look plausible.
- Resolution: Version `max-score-stable-order-v1`, keep replacement strict on
  greater scores only, and regress an equal-score pair selecting the first
  advertised candidate.
- Prevention: Treat top-1 tie behavior as a policy contract before adding
  top-k/nucleus sampling or random streams.

## Bind aggregate policy evidence to unique observation IDs

- Context: The two-observation action tally needed to remain inspectable as
  evidence rather than an anonymous count.
- Symptom: Aggregating observations without retaining their actor-visible IDs
  can hide accidental duplicate inputs and weaken replay-oriented diagnosis.
- Resolution: Store both observation IDs in the bounded tally and reject
  duplicates before invoking any profile policy.
- Prevention: Keep provenance fields actor-visible and bounded; defer broader
  scenario/replay provenance and population sampling until their contracts are
  separately defined.

## Separate baseline preference from visible-threat override

- Context: M4 profiles had fixed evaluation rules and a visible `Withdraw`
  response, but their baseline choices were only implicit in scoring tables.
- Symptom: Treating the threat override as the profile's whole preference can
  obscure the distinction between normal posture and information-driven action.
- Resolution: Expose `preferred_intent()` as bounded profile metadata while
  leaving threat-response selection and host validation unchanged.
- Prevention: Keep preference labels descriptive and actor-safe; defer richer
  risk, planning, memory, communication, and human-behavior parameters.

## Make policy randomness explicit and opt-in

- Context: M4 needed a reproducible seed contract without changing the fixed
  profile comparison or introducing an implicit global generator.
- Symptom: A random tie-break can silently destroy replay identity when its
  seed, stream, or draw is not part of the policy input.
- Resolution: Add a versioned seed bundle carrying an explicit policy
  `StreamId`/`DrawId`; use it only in `choose_with_seed` for equal top-score
  candidates while preserving the default stable-order path.
- Prevention: Record the bundle with seeded decisions, resolve randomness at
  the policy edge, and defer broad sampling until seed/version and replay
  contracts cover the larger experiment surface.

## Keep policy replay separate from host history

- Context: M4 needed representative expected and anomalous scripted decisions
  to remain inspectable without turning policy diagnostics into simulation
  history.
- Symptom: Reusing host replay or persistence for a policy-only decision can
  accidentally grant an agent authority over transitions or imply durable
  experiment support that does not exist.
- Resolution: Store the actor-visible observation, decision, expected intent,
  disposition, and optional seed in a versioned library record; re-evaluate it
  through the same policy and return a bounded mismatch error.
- Prevention: Keep policy replay records separate from authoritative history,
  state hashes, execution inputs, and durable stores until those integrations
  have their own contracts and evidence.

## Keep protocol DTOs narrower than domain observations

- Context: M5 needed an actor-facing observation/action boundary before adding
  transport or session orchestration.
- Symptom: Exposing `LanerObservation` or `LaneIntentRequest` directly would
  make internal domain types part of protocol compatibility and could let a
  caller confuse DTO construction with legality.
- Resolution: Map only primitive actor/turn/observation identities, closed
  intent IDs, and the bounded advertised action set into versioned DTOs; convert
  actions back to host-bound requests without validating them in the adapter.
- Prevention: Keep transport, lifecycle, plan/message metadata, and host
  legality outside the DTO module until each has its own versioned contract.

## Keep actor-session freshness separate from legality

- Context: M5 needed a session lifecycle before transport or MCP orchestration.
- Symptom: Letting the session adapter validate intents or commit submissions
  would create a second simulation authority and blur stale-window recovery.
- Resolution: Use immutable session transitions for actor binding, current
  observation identity, duplicate submission, and close state only; return
  host-bound requests for the host to validate.
- Prevention: Keep session freshness and actor capability checks bounded at the
  protocol edge, while legality, transition, history, replay, and repair remain
  explicit host/adapter contracts.

## Bound protocol codecs before adding transport

- Context: M5 needed reproducible DTO exchange evidence without introducing an
  MCP runtime or file/network I/O.
- Symptom: An unbounded parser or permissive field decoder can turn a protocol
  adapter into an allocation or compatibility escape hatch.
- Resolution: Version the line-oriented DTO codec, cap input bytes, require
  exact bounded fields, reject duplicates/unknowns/missing values and closed-
  enum violations, then hand decoded actions to host validation.
- Prevention: Keep codec parsing pure and bounded; add transport framing,
  persistence, repair, and provider compatibility only behind separate tests
  and schemas.

## Keep repair hints typed and non-authoritative

- Context: M5 needed caller recovery guidance for malformed protocol payloads
  and stale actor-session operations before adding transport orchestration.
- Symptom: Returning raw parser/session errors or automatic rewrites would
  expose unstable details, encourage retry loops at the wrong boundary, or
  let the adapter impersonate host legality and transition authority.
- Resolution: Project codec and session failures into the versioned
  versioned actor-error schema with closed codes and deterministic repair hints;
  omit raw payloads, IDs, hashes, and domain errors, and keep hints advisory.
- Prevention: Treat host-legality error redaction, automatic repair, transport
  retry, and reconnect as separate contracts with their own evidence.

## Validate actor actions without submitting them

- Context: M5 needed a host-legality boundary after DTO and session freshness
  contracts were established.
- Symptom: Letting a protocol adapter append a request or close a window while
  checking an actor action would move simulation authority out of the host and
  make rejected requests hard to audit.
- Resolution: Add a read-only host method that binds observer and observation
  identity, delegates to the existing lane validator, and maps failures to
  actor-safe codes while preserving history and observation state.
- Prevention: Keep action submission, execution resolution, window closure,
  and detailed host-error projections behind separate authority and evidence
  contracts.

## Let the host close only validated actor windows

- Context: M5 needed an action-submission path after the host-bound validation
  adapter was proven read-only.
- Symptom: Committing or advancing before current-receipt and lane validation
  would allow stale or duplicate actor actions to mutate authoritative history.
- Resolution: Reuse the read-only validation gate, then let the host append the
  request and clear/close the fixture window through its existing deterministic
  transition path; failed validation or execution leaves history unchanged.
- Prevention: Keep transport retry, simultaneous submissions, and reconnect
  outside the synchronous host method until their ordering and authority are
  separately specified.

## Bound actor draft metadata before host staging

- Context: M5 needed protocol shapes for message, plan, and contingency values
  before adding transport or host draft integration.
- Symptom: Treating free-form metadata as an executable plan or unbounded
  payload would create compatibility, prompt-injection, and allocation risks at
  the protocol edge.
- Resolution: Use `m5-actor-draft-v1` with closed field IDs, a 256-byte value
  cap, control-character/empty rejection, and closed plan intent IDs; keep the
  DTO pure and observation-bound.
- Prevention: Add host staging, communication semantics, and provider prompt
  metadata only through separate authority and compatibility contracts.

## Keep actor draft staging before the host commit boundary

- Context: M5 needed to connect the bounded actor-draft DTO to the existing host
  draft without turning metadata delivery into a transition request.
- Symptom: Applying a DTO after commit, against a stale observation, or after
  session closure could silently alter the next window or make an actor believe
  a draft was still editable.
- Resolution: Bind staging to the current actor receipt, reject complete,
  committed, stale, and closed boundaries with actor-safe errors, and replace
  only the selected internal field. Leave commit, advance, legality, and
  history on their existing host-owned paths. For a committed draft, direct
  recovery to await the next observation because the current receipt remains
  unchanged until the host advances.
- Prevention: Test replacement plus every boundary while asserting unchanged
  observation and record count; keep communication, transport, and
  simultaneous-draft semantics in separate slices.

## Project actor observations at the host edge

- Context: Actor action and draft DTOs need an active observation without
  exposing the internal lane receipt as a protocol contract.
- Symptom: Letting an adapter reach into host internals duplicates observation
  binding and can accidentally expose hashes, resolved inputs, or true state.
- Resolution: Have the host map its active receipt through the existing
  `ActorObservationDto` projection, reject complete/closed lifecycle states,
  and prove the mapping is equal across window changes while record count and
  state authority remain host-owned.
- Prevention: Keep the DTO actor-visible and pure; add transport, simultaneous
  actor, and richer session semantics only behind separate contracts.

## Keep history summaries separate from replay records

- Context: M5 needed an actor-visible way to distinguish an open, complete, or
  closed fixture without exposing detailed history internals.
- Symptom: Reusing replay or debrief structures for a status check would leak
  hashes, snapshots, or causal detail and blur lifecycle authority.
- Resolution: Define a tiny versioned history DTO containing only bounded record
  count and a closed status enum, and have the host derive it from lifecycle
  state without changing history.
- Prevention: Keep detailed records, replay, debrief, and persistence behind
  separate contracts and tests.

## Expose replay records only after verification

- Context: M5 needed a bounded actor-facing view of committed window records
  without making authoritative history internals part of the protocol.
- Symptom: Projecting records before replay verification, or copying their
  hashes and inputs into a DTO, would let tampered or implementation-specific
  provenance cross the actor boundary.
- Resolution: Verify the existing immutable history first, then map at most two
  records to categorical window, intent, outcome, and `verified` fields. Keep
  record identity, hashes, resolved inputs, traces, and causal detail private.
- Prevention: Test empty/partial/complete projections, closed sessions, and
  tampered-history rejection while asserting the host remains read-only.

## Keep replay-linked debrief records categorical

- Context: M5 needed per-window outcome review linked to verified history while
  preserving the actor-safe committed-facts boundary.
- Symptom: Returning the internal debrief record would expose hashes, resolved
  inputs, execution traces, or causal explanations and blur review with
  privileged inspection.
- Resolution: Require an active complete host, rebuild the existing
  replay-verified debrief, and expose only window, intent, outcome, objective,
  `committed_facts_only`, and `verified` labels for the two fixture windows.
- Prevention: Gate incomplete/closed/tampered histories with bounded errors and
  keep detailed causal review, persistence, and transport in separate slices.

## Encode actor errors as closed IDs only

- Context: M5 needed a transport-ready shape for actor-safe validation errors
  without making repair or host work implicit.
- Symptom: Serializing debug/domain errors would leak raw payloads, hashes, or
  authoritative values and make clients depend on unstable internals.
- Resolution: Encode only the versioned schema, closed error code, and closed
  repair hint with exact bounded lines; decode rejects unknown IDs and extra
  fields before any caller can act.
- Prevention: Keep hints advisory-only and preserve host legality, transition,
  history, and automatic-retry authority outside the codec.

## Project action results without reopening lane authority

- Context: Actor clients need a bounded success response after host-owned action
  submission, while the CLI host already returns internal window/outcome types.
- Symptom: Reusing the internal `CliHostOutput::Advanced` value as a protocol
  contract would expose domain types and make clients depend on transition
  details.
- Resolution: Map only the closed fixture-window and categorical-outcome IDs
  into `m5-actor-action-result-v1` after the existing host submission succeeds;
  keep validation, execution, and history in the host path.
- Prevention: Test both windows, exact result wire text, and absence of hashes
  or execution fields; keep debrief and richer outcome semantics separate.

## Gate actor debrief summaries on completed active hosts

- Context: M5 needed an actor-facing outcome review after bounded history and
  action-result DTOs were available.
- Symptom: Reusing the internal `ScenarioDebriefReport` directly would expose
  health, position, wave, coordination, delayed-origin, execution-trace, or
  replay fields; serving it before completion would present partial facts as a
  final review.
- Resolution: Project only fixed-window intent/outcome/objective labels, final
  objective, and committed-facts attribution through `m5-actor-debrief-v1`;
  require an active complete host and map incomplete/closed states to closed
  actor-safe errors.
- Prevention: Keep detailed causal debrief and replay-linked records behind
  separate contracts, and test completion, closure, codec bounds, and hidden
  fields before adding transport or persistence.

## Keep actor commit separate from actor advance

- Context: M5 needed a protocol command for committing an actor's intent after
  draft metadata staging.
- Symptom: Reusing the actor action DTO would conflate committing an intent with
  legality validation, execution resolution, window closure, and history append.
- Resolution: Define an observation-bound commit command/result pair; the host
  checks receipt/lifecycle and optional staged-plan consistency, clears draft
  metadata, and stores the committed intent while leaving advance and lane
  authority unchanged.
- Prevention: Test zero history mutation, unchanged observation, second-commit
  rejection, stale/wrong-actor/complete/closed boundaries, and mismatch
  redaction before adding transport or simultaneous ordering.

## Acknowledge draft staging without echoing payloads

- Context: An actor protocol client needs confirmation that a message, plan, or
  contingency was accepted by the host draft boundary.
- Symptom: Reusing the free-form draft command as a response would echo
  metadata and blur acknowledgement with communication delivery.
- Resolution: Return a versioned receipt containing only the bound actor,
  observation ID, and closed field identity, while delegating all validation to
  the existing host staging method.
- Prevention: Keep receipt construction after successful staging, assert no
  history/observation mutation, and defer transport, delivery, and richer plan
  semantics to separate contracts.

## Report accepted draft presence without delivering payloads

- Context: M5 needed a commit acknowledgement that distinguishes an intent
  commit from the metadata accepted alongside it without exposing free-form
  message, plan, or contingency values.
- Symptom: Reusing the draft-staging receipt or commit result would either lose
  which fields were accepted or tempt the protocol boundary to echo payloads
  and imply communication delivery.
- Resolution: Capture `present`/`absent` bits before delegating to the existing
  host commit path, then return a versioned receipt containing only the bound
  observer, observation ID, intent, and those bits. Failed commits return the
  existing actor-safe error and leave the draft repairable.
- Prevention: Construct the receipt only after successful commit, assert draft
  clearing and unchanged history/observation, and keep delivery, transport,
  and free-form plan semantics in separate contracts.

## Report aggregate draft presence without echoing payloads

- Context: An actor client needs to know which message, plan, and contingency
  fields remain staged before committing an intent.
- Symptom: Reusing staged values as a response would expose free-form metadata
  and blur draft inspection with communication delivery.
- Resolution: Return a versioned status with only the active observation binding
  and `present`/`absent` bits, rejecting committed, complete, and closed hosts.
- Prevention: Keep status read-only and payload-free; leave delivery,
  transport, and richer plan semantics to separate contracts.

## Bind draft clearing to the active observation

- Context: An actor may need to discard staged metadata before committing an
  intent, but an unbound clear could erase a newer observation's draft.
- Symptom: A generic clear operation would blur stale/wrong-actor rejection
  with an ordinary empty clear and could mutate a later draft.
- Resolution: Require observer and observation ID, reject wrong/stale,
  committed, complete, and closed requests, and report only pre-clear presence.
- Prevention: Keep empty clears idempotent, clear only after all checks pass,
  and preserve delivery and communication semantics as separate contracts.

## Verify saved actor projections before returning them

- Context: A saved host artifact can be loaded by a fresh host, but actor-safe
  replay records must not trust serialized categorical output by itself.
- Symptom: Projecting records directly from a file could expose tampered or
  divergent history while appearing to preserve the actor boundary.
- Resolution: Validate the run ID, decode and restore through explicit inputs,
  verify replay identity, then project the existing categorical DTOs without
  replacing the current host.
- Prevention: Keep saved-run lookup and replay verification ahead of projection;
  leave filesystem hardening and scenario-wide durable replay separate.

## Test authorization and redaction together

- Context: Adding actor DTOs incrementally can leave each operation locally
  tested while the cross-surface authorization and secrecy contract drifts.
- Symptom: A new actor-bound request may reject correctly but expose a raw
  state/provenance marker in its error or result representation.
- Resolution: Keep one table-driven host matrix that exercises wrong-actor
  action, draft, commit, and receipt requests alongside a bounded DTO/result
  marker scan, asserting unchanged observation and history.
- Prevention: Treat this as library evidence only; keep network authentication,
  simultaneous privacy, and privileged tools in separate contracts.

## Keep provider transcripts separate from replay

- Context: M5 needs compatibility metadata for actor-facing tools before a
  transport or provider adapter exists.
- Symptom: Reusing simulation history or recording raw requests would couple
  provider/tool compatibility to authoritative replay and leak payloads.
- Resolution: Record only actor receipt identity, closed tool/schema IDs, and
  accepted/rejected status in a versioned pure DTO.
- Prevention: Keep runtime retention, prompts, model metadata, transport, and
  replay integration behind separate contracts and evidence gates.

## Publish capability scope before privileged tools

- Context: Actor-facing tool catalogs can accidentally imply experiment control
  merely by listing an operation.
- Symptom: Ordinary actor and privileged controller concerns become coupled in
  one schema before authentication and mutation authority exist.
- Resolution: Publish a closed catalog that labels current tools as ordinary
  actor scope and reserves, but does not advertise, the privileged label.
- Prevention: Add privileged tools only through a separate capability,
  authorization, and evidence contract.

## Keep domain conversions behind the protocol edge

- Context: Public DTOs need authoritative lane data internally without making
  domain types part of a provider-facing compatibility contract.
- Symptom: A public projection/request helper accepting or returning lane types
  silently couples protocol consumers to internal domain representation.
- Resolution: Keep those adapters crate-private and add a compile-fail RustDoc
  boundary test alongside the DTO codec evidence.
- Prevention: Treat public protocol constructors, accessors, and codecs as the
  compatibility surface; expose domain conversions only through host-owned
  implementation paths.

## Keep simultaneous submissions private until resolution

- Context: Two actors may need to submit against one observation before a host
  resolves either action.
- Symptom: Returning or debugging the partial collector can reveal one actor's
  intent before the simultaneous decision is complete.
- Resolution: Store both intents in an immutable collection boundary, expose
  bounded binding metadata plus lifecycle/readiness, and make debug output omit
  collected values.
- Prevention: Add host-owned ordering and transition resolution as a separate
  contract; do not smuggle it into the protocol collector.

## Prove parity at the adapter boundary

- Context: The CLI and actor DTO paths may share a host while exposing
  different representations of the same observation and action.
- Symptom: Separate tests can pass while one adapter drops an advertised action
  or maps a committed outcome differently.
- Resolution: Drive both paths against the same deterministic fixture and
  compare actor-visible observation fields, committed window, and categorical
  outcome.
- Prevention: Keep parity evidence at the library boundary and leave transport
  or provider-specific parity to its own contract.

## Make session termination explicit without adding a clock

- Context: Timeout and disconnect behavior must be deterministic before a
  transport or async runtime exists.
- Symptom: Treating every closure as an undifferentiated quit loses the caller
  event, while reading wall time would pull scheduling into the core.
- Resolution: Record closed-session reasons for client request, explicit
  timeout, and disconnect; decode malformed actions before session checks.
- Prevention: Keep timing/reconnect orchestration at the edge and map all
  malformed, stale, and duplicate failures through the bounded error codec.

## Prove edge exclusion with a narrow static guard

- Context: The deterministic core must stay independent of transport and async
  orchestration while edge adapters evolve incrementally.
- Symptom: Architecture prose can promise that boundary without catching a
  future runtime import or network primitive in a core module.
- Resolution: Keep an explicit list of core Rust modules and have the repository
  checker reject async syntax/runtime imports, wall-clock imports, and network
  transport types there; test both rejection and clean-core paths.
- Prevention: Treat the guard as ownership evidence, not as proof of complete
  transport behavior, and keep adapter-edge I/O and future MCP work separate.

## Expose replay verification without exposing replay data

- Context: Actor clients need to know whether the host's immutable history was
  verified before relying on bounded debrief/status projections.
- Symptom: Returning records, hashes, resolved inputs, or traces would widen the
  actor information boundary and couple the protocol to internal replay data.
- Resolution: Add a versioned replay DTO with only categorical `verified` status
  and bounded record count; keep verification host-owned and map failures through
  existing actor-safe errors.
- Prevention: Treat this as status evidence, not replay transport, persistence,
  causal debrief, or a second transition authority.

## Gate saved debrief projections on complete verified history

- Context: A saved artifact can represent an earlier, incomplete window while
  the same host API is also used for complete-run debrief review.
- Symptom: Projecting debrief labels from a partially restored run would make
  an incomplete snapshot look like a finished scenario and weaken the causal
  boundary of the debrief contract.
- Resolution: Restore and verify saved history locally, reject anything other
  than the bounded two-record completion before calling the existing debrief
  builder, and return only its categorical actor-safe records.
- Prevention: Keep completion gating after replay verification and before
  projection; leave durable/scenario-wide causal replay as a separate slice.

## Keep draft readback actor-owned and non-delivering

- Context: The host already stores observation-bound message, plan, and
  contingency values while communication delivery remains unimplemented.
- Symptom: Returning staged metadata through an adapter can be mistaken for
  delivery or a second actor's visibility if ownership and order are implicit.
- Resolution: Read only the requesting actor's actor-protocol draft, return
  existing bounded DTOs in a fixed field order, preserve observation, commit,
  and history state, and keep legacy CLI draft text on its existing path while
  retaining mixed-adapter presence/clear semantics in the shared draft.
- Prevention: Treat draft readback as local metadata inspection; introduce
  recipients, transport, and simultaneous visibility only through a separate
  communication contract.

## Reuse the existing debrief summary after saved-run verification

- Context: Complete saved runs already have a bounded actor debrief summary;
  reconstructing a second summary shape would duplicate categorical mapping.
- Symptom: A saved-run adapter can accidentally project before replay or
  completion checks, or widen the summary with artifact provenance.
- Resolution: Restore and verify local history, require exactly two records,
  invoke the existing debrief builder, and pass only its report to the existing
  `ActorDebriefDto` projection.
- Prevention: Keep saved summary retrieval as a thin verified adapter and leave
  detailed causal review and durable replay records behind separate contracts.

## Bind resumable cursors to the complete visible input

- Context: A deterministic batch can be split across a persisted cursor, but
  the caller supplies the observation and manifests again when resuming.
- Symptom: Storing only a completed count allows a changed observation or
  reordered/retuned manifest list to continue under an apparently valid cursor.
- Resolution: Fingerprint the actor-visible observation and ordered manifest
  metadata in a versioned bounded checkpoint; reject mismatches before policy
  evaluation and persist only the cursor through the existing file-store edge.
- Prevention: Keep decision/result persistence and crash recovery separate, and
  never use a cursor as a substitute for authoritative simulation history.

## Keep matched sampling caller-supplied

- Context: M6 needed visible-input sensitivity evidence before population
  generation or distribution sampling was authorized.
- Symptom: Generating observations or populations inside the sample can confound
  the policy comparison and quietly add scenario or sampling authority.
- Resolution: Require exactly two same-actor, distinct-ID observations from the
  caller and reuse the existing ordered seeded batch runner for each.
- Prevention: Keep matched samples fixture-sized and actor-visible; defer
  population distributions, outcomes, metrics, persistence, and calibration to
  separate contracts with separate evidence.

## Record applicable versions without inventing provider support

- Context: M6 requires reproducibility metadata while this repository still has
  no prompt, model, transport-tool, or extractor integration.
- Symptom: Omitting version fields hides the boundary, while fabricating
  provider versions overclaims compatibility and calibration readiness.
- Resolution: Publish one fixed catalog for the applicable ruleset, scenario,
  policy, and profile IDs, and use an explicit `not-applicable` marker for
  provider-edge fields.
- Prevention: Keep version metadata pure and separate from manifest execution,
  persistence, population sampling, and provider contracts.

## Compose matched scenarios without generating them

- Context: M6 needed more than one matched observation pair while scenario and
  population generation remained unauthorized.
- Symptom: Letting the sample set create scenarios would turn a comparison
  helper into a hidden sampler and make its distribution claims ambiguous.
- Resolution: Accept one to four caller-supplied pairs, require one actor and
  globally distinct observation IDs, and compose the existing matched reports.
- Prevention: Keep pair order and explicit inputs visible, and defer scenario
  generation, populations, distributions, outcomes, and metrics to separate
  evidence contracts.

## Aggregate only verified sample outputs

- Context: M6 needed selected-intent counts over caller-supplied sample pairs
  after the sample-set identity checks had succeeded.
- Symptom: Accepting free-form counts or rerunning policies during aggregation
  can hide provenance errors and duplicate policy authority.
- Resolution: Build the tally only from `ScriptedAgentMatchedScenarioSample`,
  preserving profile/rule order and bounding every row to at most eight intents.
- Prevention: Keep aggregation actor-safe and fixture-sized; defer population
  distributions, outcomes, strategic metrics, and persistence to separate
  verified inputs and reports.

## Keep machine-readable evidence bounded and closed

- Context: M6 needed a portable tally representation before any durable report
  export pipeline was authorized.
- Symptom: An open-ended row format can admit unknown fields, stale rules, or
  unbounded text that is difficult to validate and reproduce.
- Resolution: Encode fixed top-level metadata and ordered closed rows under a
  4096-byte bound, validating profile/rule identity and count totals on decode.
- Prevention: Treat the codec as evidence transport only; keep durable export,
  population metrics, and provider/report pipelines outside the core.

## Keep fixture selection closed and caller-ID-bound

- Context: M6 needed a small scenario-selection boundary before broader
  population generation or distribution sampling was authorized.
- Symptom: Letting a selector invent states, IDs, or random draws can silently
  add scenario-generation authority and make repeated evidence irreproducible.
- Resolution: Admit only exact fixed-fixture IDs, require caller-supplied
  globally distinct observation IDs, preserve ordered repeated selections, and
  project through the existing actor-visible matched-sample path.
- Prevention: Keep selection metadata separate from transition/history/replay,
  persistence, distribution, provider, and outcome contracts; broaden the
  catalog only with a new bounded schema and independent evidence.

## Count explicit selections without calling them a population

- Context: A fixed-fixture selector can provide useful frequency evidence before
  population generation or distribution sampling exists.
- Symptom: Naming counts a distribution can imply representativeness, random
  sampling, or outcome coverage that the caller-supplied input does not provide.
- Resolution: Keep a versioned two-row report tied to the validated selection,
  preserve catalog order, and state that repeated choices are explicit input
  frequencies only.
- Prevention: Leave population, random/distributional, outcome, strategic, and
  calibration metrics behind separate contracts and evidence gates.

## Keep fixed-fixture population generation separate from sampling

- Context: M6 needed a bounded population-shaped input for composition evidence
  while broad scenario selection and distributional sampling remained open.
- Symptom: Calling a deterministic fixture list a population can imply random
  coverage, representativeness, or behavioral realism that the generator does
  not provide.
- Resolution: Generate at most four alternating closed fixture IDs, derive each
  pair from a caller-supplied starting observation ID with checked sequential
  arithmetic, and reuse the verified matched-sample path without adding
  randomness or host authority.
- Prevention: Keep broader/random generation, distributional metrics, outcomes,
  persistence, providers, and human evidence behind separate contracts and
  evidence gates.

## Keep caller-declared composition separate from distributional sampling

- Context: A fixed-fixture generator needed to express a skewed ordered input
  without claiming that the skew came from random or representative sampling.
- Symptom: Treating caller-selected safe/threat counts as a distribution can
  overstate coverage, diversity, or behavioral evidence.
- Resolution: Accept only closed scenario IDs, preserve their caller-declared
  order, derive checked observation pairs from one starting ID, and reuse the
  existing verified frequency/matched-sample paths.
- Prevention: Label explicit composition as input metadata and keep random,
  representative, outcome, strategic, persistence, provider, and human claims
  behind separate evidence gates.

## Bind aggregate codecs to verified reports

- Context: A bounded frequency summary needed machine-readable transport before
  durable export or population reporting was authorized.
- Symptom: Parsing self-consistent counts into a trusted aggregate lets callers
  forge evidence or swap closed row identities without provenance.
- Resolution: Parse privately under a fixed schema/line bound, validate closed
  rows and sums, then compare the result with a constructor-validated report
  before returning it.
- Prevention: Keep codec errors bounded and actor-safe, and treat report
  encoding as evidence transport rather than persistence or distribution
  authority.

## Render verified evidence without widening export authority

- Context: M6 needed a concise human-readable report after the machine-readable
  frequency codec was delivered, while durable export remained out of scope.
- Symptom: A renderer that accepts arbitrary fields or performs file I/O can
  silently become a report pipeline and blur the boundary between evidence and
  persistence.
- Resolution: Render only the already verified report's schema, bounded total,
  and stable catalog rows through a pure `&self` Markdown projection.
- Prevention: Keep presentation projections deterministic and side-effect-free;
  introduce persistence, broader metrics, or export formats only with separate
  contracts and evidence.

## Compare verified reports without claiming build causality

- Context: M6 needed a bounded baseline signal before independent build
  provenance and causal attribution were authorized.
- Symptom: Calling two caller-supplied aggregates a build-to-build result can
  imply that a code change caused the delta even when no build identity is
  recorded.
- Resolution: Compare only constructor-verified reports, preserve catalog order,
  and expose signed candidate-minus-baseline deltas under a distinct schema.
- Prevention: Label this as declared-baseline evidence and keep build identity,
  causal analysis, outcomes, and strategic metrics behind separate contracts.

## Pair profile-aware tallies only after identity checks

- Context: M6 needed to compare selected-intent counts across two verified
  fixed-fixture tally reports without adding build or population authority.
- Symptom: Pairing rows by position alone can compare different profiles or
  observers and make signed deltas look like evidence for the wrong policy.
- Resolution: Require one shared observer and exact ordered profile/evaluation-
  rule identities before retaining bounded baseline/candidate counts; compute
  candidate-minus-baseline deltas in a wider signed type.
- Prevention: Keep tally comparison caller-declared and fixture-sized; leave
  build provenance, causality, distributions, outcomes, persistence, providers,
  and calibration behind separate contracts.

## Keep fixed equality gates narrower than thresholds

- Context: M6 needed a regression signal over profile-aware tally comparisons
  before broader metrics or balance thresholds were authorized.
- Symptom: A generic threshold can imply balance, causality, or strategic
  quality even when the inputs are only caller-declared fixed-fixture reports.
- Resolution: Version a pure no-change rule that compares top-level counts and
  every ordered row's closed intent counts, with unchanged, changed-total, and
  same-total redistribution evidence.
- Prevention: Keep the gate a regression predicate; introduce thresholds,
  outcomes, build provenance, and strategic interpretation only with separate
  evidence contracts.

## Bind comparison codecs to verified pairings

- Context: M6 needed machine-readable transport for the profile-aware tally
  comparison after its observer and ordered-row identity checks.
- Symptom: Self-consistent comparison text can forge count deltas or swap a
  profile row unless decoding is bound to a constructor-verified comparison.
- Resolution: Parse a bounded positional candidate, validate closed profiles,
  rules, totals, and row order, then compare it with the expected verified
  comparison before returning trusted evidence.
- Prevention: Treat the codec as evidence transport; keep durable export,
  arbitrary report construction, provenance, causality, and broader metrics
  behind separate contracts.

## Keep provisional regression gates explicit and narrow

- Context: M6 needed one useful regression signal before broad threshold or
  build-comparison infrastructure was authorized.
- Symptom: A generic pass/fail threshold can be mistaken for balance evidence or
  for proof that a particular build caused a change.
- Resolution: Give the fixed equality rule a versioned ID and written rationale,
  and evaluate only bounded verified-report fields.
- Prevention: Keep broader thresholds, build identity, causality, outcomes, and
  strategic interpretation behind separate evidence contracts.

## Preserve run dispositions without pretending to detect failures

- Context: M6 needs failures and inconclusive runs to remain visible in bounded
  experiment evidence even before runtime scheduling or diagnostics exist.
- Symptom: Treating only successful reports as results silently drops crashes,
  timeouts, missing branches, and inconclusive work, while inventing automatic
  detection would add process and execution authority to the policy library.
- Resolution: Keep a closed caller-declared disposition envelope with stable
  status IDs and no payload, diagnostic, decision, or result fields.
- Prevention: Describe this as status preservation only; add detection,
  diagnostics, attachment, or durable result records under separate contracts.

## Label build comparisons without claiming provenance

- Context: A declared baseline comparison becomes easier to audit when callers
  can retain which build labels they intended to compare.
- Symptom: Treating caller-supplied labels as verified binaries or causal
  identities overstates what a pure report comparison can establish.
- Resolution: Store distinct bounded numeric labels alongside the existing
  ordered verified-report deltas while leaving the unlabeled constructor and
  equality gate unchanged.
- Prevention: Call these build labels, not build provenance; require separate
  source/package verification and causal evidence before interpreting a delta.

## Keep operational events outside committed history

- Context: M6 needs a place for batch lifecycle markers without making runtime
  diagnostics part of replayable simulation history.
- Symptom: Reusing committed records for start, checkpoint, resume, or finish
  events can make operational timing and failures appear authoritative.
- Resolution: Keep a bounded in-memory event vocabulary/container with payload-
  free ordered labels and no state, hash, decision, result, or trace fields.
- Prevention: Add runtime producers, tracing, persistence, and diagnostics only
  at an explicit edge; never reconstruct committed history from operational
  events.

## Preflight lifecycle-event capacity before evaluation

- Context: A deterministic batch can expose a small caller-driven lifecycle
  trace without making the event log authoritative.
- Symptom: Appending a start event before discovering that the log cannot fit
  the completion markers leaves a partial operational trace on a failed call.
- Resolution: Validate inputs and reserve the complete fixed event count before
  evaluating policy or mutating the caller-owned log.
- Prevention: Treat lifecycle production as an all-or-nothing edge adapter;
  checkpoint/resume and runtime failure events require separate contracts.

## Emit storage events only after successful persistence

- Context: Checkpoint save/load adapters can expose caller-owned lifecycle
  labels without making those labels part of the checkpoint payload.
- Symptom: Recording `checkpoint_saved` or `batch_resumed` before a filesystem
  write or decode succeeds creates a false operational trace.
- Resolution: Preflight one event slot, perform the existing bounded operation,
  and append the label only after success; keep failure paths non-mutating.
- Prevention: Treat storage event production as a post-success edge effect and
  keep event-log persistence and runtime diagnostics behind separate contracts.

## Namespace operational logs separately from checkpoints

- Context: A bounded event codec can be persisted through the existing injected
  file-store boundary without becoming simulation history.
- Symptom: Reusing the host-artifact or batch-cursor suffix lets unrelated
  payloads replace one another under the same run ID.
- Resolution: Give the operational log its own closed codec and suffix while
  retaining the shared run-ID validation and atomic replacement behavior.
- Prevention: Keep crash recovery, rotation, export, and runtime diagnostics
  outside this narrow adapter and verify same-root/same-ID coexistence.

## Keep caller-declared log segments storage-only

- Context: A bounded event log may need several independently replaceable
  files without implying an automatic runtime rotation policy.
- Symptom: Treating segment numbers as inferred lifecycle state makes a file
  name look like crash recovery or scheduling authority.
- Resolution: Accept only a small closed numeric segment range and persist
  each payload-free log under its own suffix; leave ordering and rotation to
  the caller.
- Prevention: Test segment coexistence and invalid-index rejection while
  keeping automatic rotation, crash recovery, and export explicitly open.

## Keep segment inventory observational

- Context: Callers may need to discover which bounded segment files exist
  before choosing a follow-up load.
- Symptom: Treating directory order or temporary files as lifecycle state
  turns an adapter scan into implicit rotation or crash recovery.
- Resolution: Parse only the closed numeric suffix range, sort and deduplicate
  recognized indices, and map scan failures to the generic storage boundary.
- Prevention: Ignore malformed/temporary names, test invalid-run and missing
  roots, and keep race-hard scanning semantics explicitly deferred.

## Keep distribution summaries caller-declared

- Context: M6 needed a compact distribution view over the existing fixed-fixture
  frequency report without claiming that the library samples a population.
- Symptom: Presenting normalized shares without naming their caller-declared
  denominator can make fixture counts look like representative metrics.
- Resolution: Derive ordered integer basis-point shares only from the verified
  selection count, assign the final-row remainder deterministically, and expose
  the result as a pure in-process projection.
- Prevention: Keep random/representative sampling, population inference,
  outcomes, strategic metrics, persistence, providers, and calibration behind
  separate contracts.

## Keep profile intent shares tied to tally denominators

- Context: M6 needed comparable profile rows without rerunning policy or
  turning selected-intent counts into strategic-quality claims.
- Symptom: A share array without a closed intent order or explicit row
  denominator can be read as a different metric or profile ranking.
- Resolution: Publish shares only from the verified row's bounded observation
  count, keep the exact `[Stabilize, Contest, Yield, Recall, Withdraw]` order,
  and give the final Withdraw slot the deterministic integer remainder.
- Prevention: Treat the projection as fixed-fixture evidence; keep population
  distributions, outcomes, strategic metrics, and calibration separate.

## Keep stress labels separate from stress populations

- Context: M6 needed named illegal-command, exploit-seeking,
  communication-abuse, and degenerate cases before runtime population
  generation was authorized.
- Symptom: A four-row label matrix can be mistaken for an exploit search or
  prevalence report if its boundary outcomes are presented as population data.
- Resolution: Keep the matrix caller-declared, categorical, deterministic, and
  tied to existing validation/codec/policy fixtures; expose only one bounded
  degenerate count.
- Prevention: Require separate contracts for population generation, search,
  prevalence, outcome/causal metrics, persistence, and human evidence.

## Keep fixed degenerate populations separate from adversarial populations

- Symptom: repeated fixed-policy selections can be mistaken for exploit or
  prevalence evidence.
- Resolution: bound the report to one-to-four caller-declared actor-visible
  observations and one closed repeated intent.
- Prevention: keep adversarial generation, search, prevalence, outcomes,
  persistence, providers, and human evidence in separate contracts.

## Keep illegal-command populations at the validation boundary

- Context: M6 needed repeated invalid-command evidence without authorizing an
  exploit search or communication-abuse population.
- Symptom: repeated rejection can be mistaken for adversarial prevalence or a
  statement about runtime behavior if command payloads and failure details are
  carried into the report.
- Resolution: bind one active actor-visible observation to one caller-declared
  invalid command repeated one to four times, validate it through the host,
  and retain only `host_validation_rejected` plus bounded binding metadata.
- Prevention: keep exploit search, communication semantics, prevalence,
  outcomes, persistence, providers, and human evidence in separate contracts.

## Keep risk-taking policy evidence separate from exploit search

- Context: M6 needed a bounded risk-taking population before adversarial
  search or communication-abuse modeling was authorized.
- Symptom: labeling repeated `Contest` selections as exploits can imply
  prevalence, outcome quality, or an actual search over opportunities.
- Resolution: bind one fixed risk-taking profile/rule to one to four
  actor-visible observations and retain only the verified selected intent.
- Prevention: keep exploit definitions/search, communication semantics,
  prevalence, outcomes, strategy quality, persistence, providers, and human
  evidence in separate contracts.

## Keep metric candidates separate from outlier judgments

- Context: M6 needed a deterministic largest-delta candidate before broader
  outlier detection and representative replay selection were authorized.
- Symptom: Naming a largest count delta an outlier can imply a threshold,
  causal explanation, or representative behavior claim.
- Resolution: Expose only the first largest absolute signed delta from a
  verified comparison, then classify it with a clearly provisional fixed
  threshold rather than naming it an outlier; a matching replay may be
  referenced only by caller-declared profile/rule/intent labels.
- Prevention: Keep calibrated outlier definitions, replay selection,
  representativeness, population inference, causality, persistence, and human
  evidence separate.

## Keep event-order checks separate from causal traces

- Context: M6 needed a deterministic completeness signal for caller-declared
  operational labels before runtime tracing and replay identity were in scope.
- Symptom: A complete start/chunk/finish sequence can be mistaken for proof of
  causal completeness or runtime success.
- Resolution: Classify only the closed event order, allowing optional
  checkpoint/resume labels, with no event production or replay inspection.
- Prevention: Keep causal links, record identity, runtime diagnostics,
  persistence, recovery, and human operational evidence in separate contracts.

## Keep decision replay identity separate from operational sequence labels

- Symptom: a complete-looking operational event sequence can be mistaken for
  proof that a recorded decision still replays exactly.
- Cause: lifecycle labels and decision replay consume different bounded inputs;
  neither one subsumes the other.
- Resolution: expose a small pure report with independent `verified` /
  `decision_mismatch` replay identity and the existing sequence status.
- Prevention: keep causal-trace completeness, runtime production, and
  scenario-wide replay explicitly outside this composition.

## Trace calibrated outliers to committed replays deterministically

- Context: M6 needed to connect verified aggregate metric deltas to committed
  decision replay records without adding runtime logging or broad sampling authority.
- Symptom: Pairing aggregate outlier signals with unverified or non-deterministic
  records can obscure whether a metric anomaly reflects a real reproducible decision.
- Cause: Metric deltas and replay verification operate on different layers of
  the testing ecology and must be joined via explicit matching keys.
- Resolution: Gate candidate qualification with an explicit threshold, match
  `profile_id`, `evaluation_rule`, and `selected_intent` to caller-declared
  replay records in stable order, and verify replay determinism before returning
  a `Qualified` status.
- Prevention: Keep outlier detection and replay selection pure and bounded;
  defer runtime automated log production, external persistence, and human
  evidence to separate contracts.

## Preserve observable reference outputs without private chain-of-thought

- Context: M7 requires preserving empirical reference decision outputs across diagnostic
  dilemmas for semantic-to-parametric calibration without storing or requiring private
  chain-of-thought.
- Symptom: Model scratchpads or internal reasoning traces can be mistaken for authoritative
  game logic, ground truth, or essential calibration targets.
- Cause: Treating non-verifiable internal model reasoning as simulation artifacts leaks
  uninspectable state and violates the principle that AI policies are reference empirical
- Resolution: Preserve only observable action outputs (`LaneIntent`, `LaneTargetFocus`,
  `LaneCommitment`, `LanePingSignal`) alongside bounded `StructuredRationale` category tags,
  and fail closed (`ReferenceOutputError::PrivateChainOfThoughtForbidden`) if private
  chain-of-thought is requested or present.
- Prevention: Enforce `chain_of_thought_free: true` across all preservation reports and keep
  live model execution, online recalibration, and human ground truth claims explicitly out
  of simulation contracts.

## Preserve submission privacy during collection and evaluate multi-agent decisions simultaneously

- Context: M8 requires autonomous teammates to privately formulate decisions (intents, communication, individual plans) without leaking uncommitted choices to peers before simultaneous host resolution.
- Symptom: Inspecting or querying submissions while collection is in progress allows peers or outer layers to condition choices on uncommitted teammate intents, turning simultaneous decisions into sequential ones.
- Cause: Exposing submission lookup methods or raw debug formatting during the `CollectingSubmissions` phase leaks private actor state.
- Resolution: Gate submission inspection behind the `Ready` phase, redact private intents in `Debug` during collection, return payload-free receipts upon acceptance, and evaluate multi-agent decisions simultaneously across plan alignment, proposal trust compliance, and leadership consensus into exact integer basis-point cohesion.
- Prevention: Strictly enforce zero private chain-of-thought (`chain_of_thought_present == false`) and maintain clear lifecycle state machines (`CollectingSubmissions` -> `Ready` -> `Resolved` -> `Closed`).

## Synchronize scenario catalog size, numeric selection bounds, and report markdown titles

- Context: When introducing a new executable CLI scenario (such as `m12-reproducibility-bundle-v1`), multiple components across CLI catalog, scenario parser, interactive selection prompt, and test fixtures must be updated in lockstep.
- Symptom: Tests asserting invalid scenario numeric inputs (e.g. `parse_scenario_selection("12") == None`) or catalog lengths (`len() == 11`) fail when a new 12th scenario is registered.
- Cause: Outdated invalid numeric indices in tests colliding with newly registered valid indices, or mismatching report titles between pure markdown renderers and test assertions.
- Resolution: Update `CLI_SCENARIO_CATALOG` length assertions, increment invalid test bounds (e.g. `parse_scenario_selection("13") == None`), synchronize interactive selection prompt ranges (`scenario [1-12]>`), and verify exact markdown title strings against module definitions.
- Prevention: When adding scenario entries to `CLI_SCENARIO_CATALOG`, always search for numeric boundary assertions (`[1-N]`) and verify exact header output from report renderers.

## Maintain JSON-RPC protocol surface parity and information boundary audits across milestone expansions

- Context: As simulation and release auditing modules mature (M11 GUI presentation, M12 release checks, M12 governance audits), external agent interfaces (MCP JSON-RPC) must expose corresponding tools, prompts, and resources without leaking latent simulation state.
- Symptom: Discrepancies where CLI runners exist for milestones but MCP agent clients lack equivalent inspection tools or resources, or where resource content inadvertently leaks raw state hashes or private latent values.
- Cause: Implementing CLI scenario runners without concurrently registering corresponding tools in `mcp_tools_catalog()`, prompts in `mcp_prompts_catalog()`, resources in `mcp_resources_catalog()`, and dispatch arms in `McpServer`.
- Resolution: Pair every major simulation capability with its model-agnostic MCP tool, prompt, and resource representations, ensure returned payloads project only actor-visible information or clean Markdown summaries, and verify zero latent truth leakage across all response blobs in `scripts/verify_mcp_server.py`.
- Prevention: Run dual MCP entry point tests (`fog-of-intent mcp serve` and `fog-of-intent --mcp`) checking both success responses and boundary sanitization whenever adding domain tools.



## Keep checked-out fixtures and contributor scripts independent of platform line endings and locale encoding

- Context: Contributors work across Linux, macOS, and Windows, where Git may be installed with `core.autocrlf=true` and Python defaults to a console codepage such as cp1252.
- Symptom: `serialization::tests::history_fixture_round_trips_and_replays` fails with a CRLF-versus-LF string diff on `include_str!` fixtures, `scripts/check_repository.py` aborts with `UnicodeDecodeError: 'charmap' codec`, and `scripts/test_check_repository.py` fails a hard-tab assertion because the message contains `tests\fixtures\legacy.txt`.
- Cause: `core.autocrlf=true` rewrites checked-out text to CRLF while the stored blobs and replay-hash fixtures are LF, and implicit-encoding file I/O plus `Path` formatting leak the platform locale and separator into byte-exact comparisons and checker messages.
- Resolution: Commit `.gitattributes` with `* text=auto eol=lf`, pass `encoding="utf-8"` to every `read_text`/`write_text`/`Popen` pipe in the contributor scripts, and format reported paths with `Path.as_posix()`.
- Prevention: Never rely on the ambient line ending or locale for fixtures, hashes, or machine-readable messages; verify with `python -X encoding=cp1252 scripts/check_repository.py` and a `git -c core.autocrlf=true clone` probe before claiming cross-platform health.

## Wire a capability into the actor-visible projection, not only into its own counter

- Context: The M9 match owns vision state (`MapVisionState`, wards) separately from `MatchMapState`, and the CLI/MCP match host builds actor visibility from `MatchMapState::observe`.
- Symptom: `ward` always reported `advanced: ... action=warding events=0 effects=0`, and an opposing actor stayed `location=unknown` forever, including directly above a freshly placed ward. The only observable effect was `active_wards=` rising.
- Cause: `observe()` derived team-visible sectors solely from allied actor locations, and the host read vision state only for `active_wards().len()`. Ward placement therefore mutated authoritative state that no projection consumed, and the feature was decorative in the played game while passing every library test.
- Resolution: Add the missing input to the projection itself — `MatchMapState::observe_with_wards(observer, &[(TeamSide, MapLocation)])` marks same-team ward sectors as seen, `observe()` becomes the no-ward wrapper, and the host passes coverage from `CompleteMatchState::vision()`. Prove the player-visible effect, not just the state change: ward the sector holding an opponent and assert the projection flips from `Unknown` to `Observed`.
- Prevention: When a new authoritative subsystem should change what a player or agent can see, demonstrate it with one projection-level assertion end to end. A counter, event count, or state field is not evidence that the capability reaches the user. Because `observe()` is the redaction path, resolve visibility inside it and never re-derive visibility in hosts or renderers.

## Re-read source lines containing string escapes after editing them through text-replacement tools

- Context: Rust string literals embed `\n` as two source characters, and MCP prompt templates are single long literals (e.g. the `match_macro_turn` shot-caller prompt in `src/mcp/server.rs`).
- Symptom: A rename inside such a literal turned `"...match.\n\nCurrent Match State:..."` into a broken literal ending in a stray `\`, and a follow-up in-place substitution deleted the remainder of the line instead of shortening it.
- Cause: Backslash handling differs between a replacement tool's match text and its replacement text, so `\n` matched a literal backslash-n but wrote a doubled or real newline; shell `perl`/`sed` one-liners add their own second escaping layer.
- Resolution: Rebuild from the exact source bytes with a script that constructs the escape explicitly (`chr(92) + "n"`) and splice the corrected line back in, then verify with `od -c` rather than a terminal echo.
- Prevention: After any edit that touches a line containing backslash escapes, confirm the bytes (`od -c`/`git diff`) and compile before moving on; prefer rewriting the whole literal over partial in-place substitutions.

## Attribute a committed match turn to the state its transition ended with

- Context: Decision D4 (`docs/decision_brief_20260830.md`) added observer-visible reason
  lines for interactive match turns that record no events and no effects, derived from
  objective status plus the committed intent (`MatchTurnNote` in `src/host/match_host.rs`).
- Symptom: A note derived from the objective status captured *before*
  `CompleteMatchState::apply_action` called a contest "objective-unspawned, spawns in 1
  turn(s)" on the very turn the objective spawned, and called the player's own successful
  secure "already secured by allied on turn 4".
- Cause: `transition_objective_contest` ticks spawn, respawn, and ward-expiry timers before
  resolving engagement inside a single transition, so a pre-action snapshot is not the
  condition the declared force resolved against; and a secure recorded on the action's own
  turn is indistinguishable from a pre-existing secure unless the recorded turn number is
  compared against the action's turn.
- Resolution: Derive the note from post-transition status plus `action_turn`, suppress the
  secured note when `secured_turn == action_turn`, and still report `zero-declared-force`
  when the same turn recorded a spawn or ward-expiry event, because the declared force
  itself did nothing.
- Prevention: When explaining, attributing, or debriefing a committed turn, read the state
  the transition ended with and use recorded turn numbers to separate "this turn caused
  it" from "it was already true". Never treat a pre-action snapshot as the condition an
  action resolved against; one transition can contain several ordered sub-phases.

## Do not re-derive visibility, and do not print where a fogged thing stands

- Context: Decision D3 (`docs/decision_brief_20260830.md`) had to put defensive structures
  behind the match's fog of war. `MatchMapState::observe_with_wards` computed a
  `team_visible_locations` array inline for actors, while the host projected structures by
  iterating `MatchStructureState::structures()` directly and printing exact health.
- Symptom: The first structure projection printed all 26 lines with correct `not-visible`
  statuses and still broke the existing redaction assertion
  `assert!(!rendered.contains("lane:mid:far-side"))` in `src/terminal.rs`, because the line
  for a fogged opposing inner turret named the sector it occupies. Separately, a per-team
  fog test written naively fails on the shipped map: sight of a lane centre reveals **both**
  teams' outer-tier turret, because the coarse 15-sector map has no separate outer-tier
  sector per team.
- Cause: Two independent visibility rules drift apart, and a projection that re-derives
  sight outside the actor projection decides for itself what the player may know. Printing
  any field of a redacted entity can leak: here the sector is static map knowledge, but it
  reads like a sighting to a whole-text assertion and to a player.
- Resolution: Extract `MatchMapState::sector_sight(team, ward_coverage) -> SectorSight` as
  the one rule, consume it from both `observe_with_wards` and
  `MatchStructureState::observe_for`, and omit sector and band whenever the status is
  `NotVisible`. State the shared-sector consequence on `StructureTier::observed_sector` and
  pin it in `one_sight_line_covers_both_teams_in_a_shared_sector`.
- Prevention: When adding an observation field, first name the single rule that decides it
  and consume that rule instead of recomputing it; never print identifying detail of an
  entity the rule has redacted. On a coarse sector map, write fog tests from the sector
  outward, and re-run whole-text redaction assertions after any renderer change, since they
  scan every line the projection produces.

## Re-derive scripted fixtures from an authority change, never patch their expected counters

- Context: Decision D2 (`docs/decision_brief_20260830.md`) made actor presence an input to
  objective and siege resolution, so the two canonical complete-match plans in
  `crates/foi-map/src/complete_match_catalog.rs`, their replay-verified transcript, the host
  and binary command scripts, and several expected counters all became claims about a rule the
  code no longer applied.
- Symptom: Patching ids and expected numbers by hand turns one authority change into a dozen
  unrelated test failures whose diffs look like noise. The real finding hid inside that noise:
  the two-actor comeback plan stops winning once delivery is capped per present actor, which is
  evidence about the rule, not a fixture to be repaired back to its old value.
- Cause: Canonical scenario plans are *derived* from the ruleset. Editing their expectations
  instead of re-deriving them keeps the old rule's shape while claiming the new identity, which
  is exactly the label-outruns-evidence failure `docs/audit_report_20260828.md` documents.
- Resolution: Rebuild each plan by simulating the new rule, then read the resulting turn,
  objective, event, and effect counts back out of the run and let those become the expectations
  (`CompleteMatchCatalog::all()` over a temporary debug run, then the transcript and host scripts
  rewritten from the same script). Bump the whole identity chain together:
  `m9-complete-match-v2`, `m9-complete-match-catalog-v2`, both `-v2` scenario ids, and
  `m9-interactive-match-host-v3`, and keep the retired constants documented as retired.
- Prevention: When a change touches authoritative resolution, treat every scripted scenario,
  transcript, and piped command list as generated output: regenerate, then copy actual into
  expected. If a shipped scenario no longer reaches its stated ending, record that as a result
  of the decision rather than as a failure to fix.

## Refuse a player action only on facts the player could have computed

- Context: Presence-gated delivery makes some legal declarations deliver nothing. The host had
  to choose between committing a turn that applies zero force, silently ignoring the turn, or
  refusing the declaration before it is staged.
- Symptom: A committed turn that delivers nothing is indistinguishable from a rules quirk, and
  the zero-delivery path also masks the subsystem's own legality error - a 0-presence siege never
  reaches `transition_structure_siege`, so `StructureInvulnerable` is never surfaced for an
  attack that never happened. A naive pre-check would have made that worse by reporting unseen
  opponents standing in the target sector.
- Cause: The pre-validation lives at the host edge, where it can see latent truth that the
  actor-visible observation deliberately withholds, and any refusal there can become a leak.
- Resolution: `CliMatchHost::stage` refuses only what the player could have worked out - own
  actor sectors from `observe` plus static map adjacency - phrased as
  `error: no force in reach: … rotate first`, and it refuses *before* staging so no turn is
  spent. Partial delivery stays a committed turn and is explained through the existing note
  channel with declared, present, and delivered figures. `CompleteMatchState::force_declaration`
  supplies the target sector so the host never re-derives the rule, and pinning tests cover both
  the refusal and the absence of hidden information.
- Prevention: Gate any host-side pre-validation on actor-visible facts, and state that boundary
  in `docs/TERMINOLOGY.md`. Prefer refusing before committing over explaining afterwards, but
  only for what the player could have predicted; report the gap between declared and delivered
  instead of inventing a new legality token.

## Check what a named module actually models before implementing a decision phrased in its terms

- Context: Decision D5 was written as "raw damage integers bypass the cost profile, so resolve
  `light`/`committed`/`all-in` through the cost profile". Implementing that sentence literally
  meant wiring the new vocabulary to `crates/foi-map/src/cost_profile.rs`.
- Symptom: That module is `OperationCounts` - transitions executed, state hashes computed,
  observation projections performed, replays verified - plus a [1, 8, 64, 512] scaling ladder. It
  is a deterministic performance profile with no resources, no prices, and no force table
  anywhere in the repository. Resolving a commitment "through the cost profile" would have meant
  inventing a force economy inside an unrelated profiler, or shipping a token whose stated
  resolver was a lie that help text and error strings would have repeated to players.
- Cause: A decision brief can name a module by an assumed role rather than by its shipped
  behaviour, and a module named "cost profile" in a game engine invites the resource-economy
  reading. Docs and code had drifted apart before the decision was written, not during it.
- Resolution: Price the tokens against the quantity the shipped authority already pays -
  `FORCE_PER_PRESENT_ACTOR` per present actor from D2 - resolve them in one host function
  (`parse_force`), and state in `ROADMAP.md`, `SPEC.md`, and the decision brief itself that
  `cost_profile.rs` is an operation counter and that no force cost table exists. The correction
  also removed the phrase "whatever the cost profile" from the player-visible rejection message.
- Prevention: Before coding against a module a decision or audit names, read the module's own
  doc comment and public API and quote what it returns. If the decision's premise does not hold,
  implement the accepted *intent*, then correct the brief in the same change instead of silently
  reinterpreting it. Keep that correction out of player-facing strings, which should state rules,
  not the history of a design mistake.

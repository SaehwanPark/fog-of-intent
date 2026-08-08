# Lessons

This ledger records verified, reusable project lessons. Add an entry only when
the context, cause, successful resolution, and prevention step are supported by
repository evidence and likely to recur. Keep entries concise and link to the
canonical policy instead of duplicating it.

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

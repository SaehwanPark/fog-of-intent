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

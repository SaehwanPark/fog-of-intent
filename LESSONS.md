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

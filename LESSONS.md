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

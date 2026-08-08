# M4 Selection-Tiebreak Handoff

## Delivered

- Bound all profiles to `max-score-stable-order-v1`.
- Added equal-score first-advertised tie evidence without changing policy
  generation, scoring, validation, or host authority.
- Synchronized canonical/workspace docs, changelog, and LESSONS.md.

## Verification

The focused agent suite contains eleven tests. The full suite target is 165
Rust unit tests, seven binary integration tests, and one compile-fail RustDoc
test, plus formatting, Clippy, repository-policy, 14 Python, and diff checks.

## Open boundaries

Top-k/nucleus selection, random streams, population variation, outcomes,
memory, communication, execution metrics, and human behavioral realism remain
open.

# M5 Actor Capability Matrix Handoff

## Outcome

Implementation is complete; pending the required independent three-pass review.

## Intended Contract

Deliver a stable pure catalog labeling the five current actor tools as
`ordinary_actor`; reserve but do not advertise privileged experiment-controller
authority.

## Verification

Current evidence is one focused protocol capability-catalog test. The suite
contains 19 protocol, 5 session, and 23 host tests within 203 Rust unit tests,
7 binary integration tests, and 1 RustDoc compile-fail test. Formatter, Clippy
with warnings denied, repository checker, 14 Python checks, and
`git diff --check` all pass.

## Limits

No privileged tools, network authentication, transport registration,
persistence, or simulation authority are added.

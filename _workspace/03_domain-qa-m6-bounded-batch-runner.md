# M6 Bounded Batch Runner Domain QA

## Disposition

Pending independent three-pass review of the implementation and evidence.

## Evidence target

One focused agent test must cover deterministic order/reproducibility, explicit
seed retention, empty-batch rejection, and the 16-manifest bound. The expected
full suite is 17 focused agent tests within 230 Rust unit tests, 7 binary tests,
and 3 RustDoc tests; 15 Python policy tests, formatter, Clippy with warnings
denied, repository checker, and diff checks must pass.

## Boundary questions

- Does the runner consume only actor-visible observations and manifest-owned
  profile/seed metadata?
- Is the batch cap enforced before any policy evaluation?
- Does the runner preserve order without adding transition, history, storage,
  provider, or population authority?

## Required Fixes

To be determined by independent review.

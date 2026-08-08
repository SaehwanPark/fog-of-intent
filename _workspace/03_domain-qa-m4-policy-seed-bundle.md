# Domain QA — M4 Policy Seed Bundle

## Status

Pass target for the bounded explicit-seed slice, pending the single required
code-reviewer handoff.

## Review contract

- The seed bundle is versioned and contains explicit policy stream/draw data.
- The default deterministic selection contract is unchanged.
- The seeded path changes only equal top-score tie choice and is reproducible
  for identical bundle/observation inputs.
- Seeded decisions remain actor-bound and host-valid without hidden-state or
  transition authority.

## Claim limits

Evidence is limited to one library policy, one fixture-sized observation, and
synthetic equal-score ties. It does not support population, strategic-quality,
outcome, model-provider, or human-behavior claims.

## Verification target

The focused agent suite grows from eleven to thirteen tests. The full suite is
expected to contain 167 Rust unit tests, seven binary integration tests, and
one compile-fail RustDoc test, alongside formatting, Clippy, repository
policy, Python, and diff checks.

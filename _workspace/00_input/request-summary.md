# Request Summary

## Requested Outcome

Advance the next dependency-complete slice of the governed repository baseline
so M1 can begin from a pinned, single-package Rust base with explicit minimum
compatibility and dependency-policy conventions.

## Roadmap Milestone

M0 — Governed repository baseline (active).

## Current Evidence

- `origin/main` contains the merged M0 policy, notice, terminology, and
  host-authority documentation.
- The executable remains the Rust 2024 placeholder binary with no third-party
  dependencies.
- M0 package/toolchain, lockfile, dependency-policy, and compatibility items
  remain incomplete; hosted CI remains a later slice.
- The available complete exact baseline is Rust `1.96.0` with `rustfmt` and
  `clippy`; the repository intentionally does not rely on the separately
  resolved `stable` alias that currently reports Rust `1.97.1`.

## In Scope

- Pin Rust `1.96.0` with the required formatting and lint components.
- Generate and commit the binary package `Cargo.lock`.
- Decide to keep one Cargo package for M1 and defer a workspace until a second
  independently built crate or executable is justified.
- Define minimum version and compatibility conventions for manifests, schemas,
  rulesets, scenarios, snapshots, history, replay hashes, and fixtures.
- Document dependency addition, source, license, security-advisory, lockfile,
  and review policy.
- Reconcile affected roadmap, specification, architecture, and changelog state.

## Non-Goals

- No simulation mechanics, new runtime dependency, CLI, MCP, persistence,
  database, or hosted CI workflow.
- No automated advisory/license scanner is claimed until the CI slice exists.
- No migrations or backward-compatibility implementation beyond the minimum
  policy needed to constrain M1 fixtures.

## Project Boundaries Touched

- Rust toolchain and Cargo package reproducibility.
- Future artifact/schema/replay compatibility contracts.
- Dependency provenance, security, and license review policy.

## Source Files

- `Cargo.toml`
- `README.md`
- `ROADMAP.md`
- `SPEC.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `docs/tech-stack-consideration.md`
- Existing M0 policy and authority documents.

## Expected Outputs

- `rust-toolchain.toml`
- `Cargo.lock`
- `docs/adr/0002-single-package-m1.md`
- `docs/COMPATIBILITY.md`
- `docs/DEPENDENCY_POLICY.md`
- Updated canonical project-state documents.

## Verification

- `rustup show active-toolchain` and pinned `cargo +1.96.0` checks agree.
- `cargo metadata --locked --format-version 1` succeeds from a clean checkout.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  and `cargo test` pass with the pinned toolchain.
- Policy documents and local Markdown links resolve; `git diff --check` passes.
- The package/layout decision and compatibility rules do not claim unimplemented
  migrations, schema serializers, or dependency scanners.

## Evidence Limits and Open Questions

- Pinning the complete `1.96.0` baseline is reproducibility evidence for this
  repository, not a claim that every host platform has been tested. The
  separately resolved `stable` alias at `1.97.1` is intentionally not used as a
  repository pin because its explicit versioned resolution was not reliable in
  this environment.
- Compatibility documents are contracts for M1 design; they are not evidence
  that schemas, replay artifacts, or migrations exist yet.
- Dependency and security policy is documented here; enforcement automation is
  deferred to the next M0 CI slice.

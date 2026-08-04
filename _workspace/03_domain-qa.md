# Domain QA

## Status

`pass`

This QA covers the M0 packaging, compatibility, and dependency-policy slice. It
does not validate a simulation kernel, schema serializer, replay engine, CI
workflow, legal clearance, or human experience.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `ROADMAP.md` M0 checklist and exit evidence
- `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`
- `docs/adr/0002-single-package-m1.md`, `docs/COMPATIBILITY.md`, and
  `docs/DEPENDENCY_POLICY.md`
- Local Markdown-link check, `git diff --check`, pinned `cargo metadata`,
  `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  and `cargo test`
- One code-reviewer report covering three independent review passes and its
  follow-up findings.

## Scope and Roadmap Findings

The changed files implement only the selected M0 package/toolchain,
compatibility, and dependency-policy items. The single-package decision is
explicit and reversible, while CI automation remains visible as incomplete.
No M1 mechanics, runtime dependency, adapter, or schema implementation was
introduced.

## Authority and Information-Boundary Findings

ADR-0002 preserves the one-package decision without changing ADR-0001's
host-owned simulation authority. The compatibility policy keeps internal
artifact identity separate from future public DTO, prompt, profile, and
extractor versions; no adapter or package boundary is presented as a second
simulation engine.

## Determinism, Replay, and Reproducibility Findings

`rust-toolchain.toml` pins a complete locally installed Rust `1.96.0` toolchain
with `rustfmt` and `clippy`; `Cargo.lock` records the dependency-free package.
`docs/COMPATIBILITY.md` requires explicit ruleset/scenario/schema identity,
immutable semantic binding, closed handling of authoritative unknown fields,
deterministic migrations, and per-transition replay hash comparison. These are
future implementation contracts, not shipped replay evidence. Missing advisory
tooling or data is a default block, with only an explicit owner/rationale/expiry
defer permitted.

## Behavior and Playtest Findings

No agents, policies, playtests, or behavioral claims were added. Dependency
policy keeps provider, async, transport, and analytical concerns outside the
deterministic core unless a later ADR demonstrates a narrow edge need.

## Gameplay and Debrief Findings

No gameplay or debrief surface was added. Compatibility rules preserve the
future requirement that replay fixtures retain enough committed input and hash
identity for causal inspection.

## Evidence and Claim Limits

The package metadata and lockfile support local reproducibility only. Hosted CI,
automated advisory/license scans, schema migration behavior, cross-platform
validation, and release readiness remain unverified. The dependency policy
requires an advisory result or an explicit security defer for future dependency
changes and reports that hosted enforcement automation is deferred.

## Required Fixes

None for this bounded slice. The reviewer-identified compatibility,
security-policy, environment-rationale, and QA-disposition issues were
corrected and the focused checks were rerun.

## Residual Risks

- The pinned `1.96.0` toolchain is verified on the current host only.
- The separate `stable` alias currently reports `1.97.1`, but is intentionally
  not used by the repository pin because its explicit versioned resolution was
  not reliable in this environment.
- M0 remains active pending CI formatting/lint/test/link/currentness checks and
  hosted evidence.

## Verification Evidence

- Pinned `rustc --version`: `1.96.0`.
- Pinned `cargo metadata --locked --no-deps --format-version 1`: pass.
- Local Markdown links: pass.
- `git diff --check`: pass.
- `cargo fmt --check`: pass.
- `cargo clippy --all-targets --all-features -- -D warnings`: pass.
- `cargo test`: pass; placeholder has zero tests.

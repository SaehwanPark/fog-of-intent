# Domain QA

## Status

`pass`

This QA covers the M0 CI/currentness slice. It does not validate a simulation
kernel, gameplay, replay implementation, legal clearance, or human experience.

## Reviewed Inputs

- `_workspace/00_input/request-summary.md`
- `ROADMAP.md` M0 checklist and exit evidence
- `SPEC.md`, `ARCHITECTURE.md`, `README.md`, and `CHANGELOG.md`
- `rust-toolchain.toml`, `Cargo.toml`, and `Cargo.lock`
- `.github/workflows/ci.yml`, `scripts/check_repository.py`, and
  `docs/DEPENDENCY_POLICY.md`
- One code-reviewer report covering three independent review passes and its
  follow-up findings.
- Local currentness/link/package checks, `git diff --check`, pinned metadata,
  `cargo fmt --check`, `cargo clippy --locked --all-targets --all-features
  -- -D warnings`, and `cargo test --locked`

## Scope and Roadmap Findings

The workflow and checker implement only the remaining M0 automation item. The
workflow uses read-only contents permission, a pinned checkout action, and the
repository's exact Rust baseline. The executable tooling version is `0.1.1`;
no M1 mechanics or external adapter was introduced.

## Authority and Information-Boundary Findings

The changes are edge/repository tooling only. The checker reads canonical files
and Cargo metadata; it cannot own simulation state, legality, transition,
history, replay, or actor-visible projections. The workflow invokes existing
commands and does not create a second simulation engine.

## Determinism, Replay, and Reproducibility Findings

The workflow installs Rust `1.96.0` with `rustfmt` and `clippy`, checks locked
metadata, and runs formatting, lint, and tests from a clean checkout. The Python
checker verifies current milestone/spec alignment, exactly one active
roadmap/spec entry, README milestone equality, local Markdown paths including
images/reference links and outside-root rejection, package license/version/
toolchain metadata, lockfile identity, and the current empty dependency graph.
These checks do not establish replay behavior.

## Behavior and Playtest Findings

No actors, policies, playtests, or behavioral claims were added. The dependency
guard fails closed if a future registry, Git, or path dependency is introduced
without an approved scanner policy or complete machine-readable defer record.

## Gameplay and Debrief Findings

No gameplay or debrief surface was added. M1 promotion remains contingent on
this hosted repository evidence, not on an inferred product experience.

## Evidence and Claim Limits

Local checks pass; hosted clean-checkout evidence is the remaining M0 gate. The
workflow does not claim advisory/license scanning for a non-empty dependency
graph, and the project still makes no legal, accessibility, enjoyment, or
research-validity claim.

## Required Fixes

None for this bounded slice. The reviewer-identified currentness, dependency
identity/defer, path binding, versioning, Markdown parsing, README, architecture,
and changelog issues were corrected and the focused checks were rerun.

## Residual Risks

- GitHub-hosted environment and action execution remain externally observed
  evidence, not reproduced by the local macOS run.
- The first future dependency still requires an approved advisory/license
  scanner or explicit security defer before merge.
- M0 is not complete until `SPEC.md` moves it to `Past` after hosted success.

## Verification Evidence

- `python3 scripts/check_repository.py`: pass.
- `python3 -m unittest discover -s scripts -p 'test_*.py'`: pass; seven focused
  checker tests.
- Pinned `cargo metadata --locked --no-deps --format-version 1`: pass.
- `cargo +1.96.0 fmt --check`: pass.
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`: pass.
- `cargo +1.96.0 test --locked`: pass; placeholder has zero tests.
- Local Markdown links: pass.
- `git diff --check`: pass.

# Request Summary

## Requested Outcome

Audit the complete Rust repository and its canonical project-state documents,
rank evidence-backed correctness, data-model, compatibility, scope, and
maintainability issues, then remediate the confirmed M2 contract problems in
small, reviewable stages without broadening the product milestone.

## Roadmap Milestone

M2 — One-Lane Vertical Slice (active). The binary remains a placeholder and the
complete playable scenario is not in scope for this remediation.

## Current Evidence

- The clean base is `main` at `64336f7`, matching `origin/main`; no open PR or
  release/tag exposes the M2 lane artifacts.
- `Cargo.toml` reports package `0.1.49`, Rust `1.96`, edition 2024, and no
  dependencies.
- The repository contains roughly 17.6k Rust/script lines, including a
  3.2k-line transition module and 2.7k-line resource test module.
- Baseline verification passed: format, Clippy, 156 Rust tests, repository
  checks, and seven repository-script tests.

## In Scope

- Review every production module, test, script, canonical document, public
  export, and recent change history.
- Preserve M1 ruleset, codec, fixtures, and hashes.
- Retire unsupported M2 resource scaffolding, version the internal M2 contract,
  strengthen state/data types, and decompose the transition implementation.
- Keep the host-owned deterministic transition, actor-visible information
  boundary, append-only history, replay, branch, coordination, objective, and
  debrief contracts explicit and testable.
- Reconcile README, SPEC, ARCHITECTURE, ROADMAP, CHANGELOG, compatibility
  notes, and durable workspace handoffs with verified behavior.

## Non-Goals

- No CLI, MCP, persistence, GUI, new dependency, item catalog, full M2
  scenario, gameplay tuning, or human-experience claim.
- No migration for retired M2 v1 artifacts; none are externally supported.
- No rewrite of accurate historical changelog entries.

## Expected Outputs

- `_workspace/01-codebase-review.md` with severity-ranked findings and evidence.
- `_workspace/01_simulation-design.md` with the v2 state, observation,
  transition, replay, and debrief contract.
- Three staged code/doc changes with focused tests and domain-QA disposition.
- `_workspace/final/handoff.md` naming changed files, checks, review status,
  deferred work, and residual limits.

## Verification

Run after each stage and at final handoff:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo doc --no-deps`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits and Stop Conditions

- Tests establish software behavior only; they do not establish enjoyable play,
  human accessibility, human trust, or behavioral validity.
- If review finds a Critical/High issue outside this bounded M2 correction, or
  if a refactor requires an unplanned public/hash/replay change, stop and report
  the conflict instead of improvising.

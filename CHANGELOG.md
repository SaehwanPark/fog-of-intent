# Changelog

All meaningful contributor- and user-visible changes are recorded here. The
project uses the versioning policy in `README.md`; documentation-only changes do
not increment the package version.

## Unreleased

### Added

- Explicit MIT source license, contributor policy, code of conduct, and
  unofficial/noncommercial project notice with an original-setting fallback and
  conservative distribution boundary.
- Concise design principles, authoritative terminology, and ADR-0001 for the
  host-owned deterministic transition boundary.
- Pinned Rust `1.96.0` toolchain and binary package lockfile, with ADR-0002
  keeping M1 in one Cargo package.
- Minimum artifact/replay compatibility and dependency, security, and license
  policy documents for the pre-implementation-to-M1 boundary.
- Canonical evidence-gated project roadmap with milestone dependencies, exit
  evidence, explicit deferrals, and maintenance rules.
- Lightweight specification and architecture state documents that distinguish
  the current placeholder from planned capabilities.
- Repo-wide `AGENTS.md` guidance and a portable Fog of Intent agent harness for
  simulation design, agent-ecology design, synthesis, and domain QA.
- Deterministic `_workspace/` handoff conventions for substantial work.

### Changed

- M0 is promoted to complete after the hosted clean-checkout CI run passed; the
  first bounded M1 deterministic-kernel fixture is now the active project-state
  slice.
- README now presents the project thesis, current pre-implementation status,
  initial vertical slice, canonical documents, and contributor workflow.
- The original proposal roadmap is labeled as a design source; `ROADMAP.md` is
  the canonical execution plan.

## 0.1.1 — 2026-08-04

### Added

- Dependency-free repository currentness/link checker, focused parser tests,
  and a pinned GitHub Actions workflow for clean-checkout verification.

## 0.1.0 — 2026-08-04

### Added

- Initial Rust 2024 binary package.
- Comprehensive project proposal for a turn-based, AI-native team-strategy
  simulation.
- Rust-first technology-stack analysis.

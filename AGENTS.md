# Repository Agents Guide

Keep this file short and repo-wide. Detailed project workflows live in
`.agents/skills/` and `docs/harness/`.

## What

- Fog of Intent is a pre-implementation Rust 2024 project for a turn-based,
  AI-native team-strategy simulation about intent, uncertainty, delegated
  execution, communication, and bounded rationality.
- Canonical project state lives in `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, and `CHANGELOG.md`. The longer proposal and stack analysis
  under `docs/` are design sources, not implementation evidence.
- The repository currently contains a single placeholder binary. Do not describe
  planned simulation, CLI, MCP, replay, research, or GUI capabilities as shipped.

## Why

- The project tests whether strategic team play remains compelling when players
  express plans and contingencies while simulated actors perform execution.
- Determinism, actor-specific information, immutable history, and causal debriefs
  make outcomes inspectable for players, agents, developers, and researchers.
- Narrow evidence-gated vertical slices protect the project from premature
  framework expansion and unsupported behavioral or accessibility claims.

## How

- Before substantial work, read `SPEC.md`, the active milestone in `ROADMAP.md`,
  and `docs/harness/fog-of-intent/team-spec.md` when domain judgment is involved.
- Use the repo-local `fog-intent-*` and `foi-test-player` skills only for project-specific
  simulation, agent-ecology, playtest, or domain-QA work. Use global skills for
  generic Rust, testing, UX, documentation, review, release, and git workflows.
- Read [LESSONS.md](LESSONS.md) before coding and add only verified, reusable
  lessons after resolving a recurring project trap.
- Keep the authoritative transition synchronous and deterministic. Resolve
  randomness before transition evaluation; keep I/O, async work, persistence,
  presentation, MCP, and model-provider code at the edges.
- Use two-space indentation with spaces only; run the repository checks before
  handing off a change.
- Preserve the distinctions between true state, actor belief, observation,
  report, intent, execution, command, event, and attributed effect.
- Prefer the smallest complete vertical slice. A roadmap item is sequencing
  guidance, not authorization to implement unrelated future phases.
- Current verification commands are `cargo +1.96.0 fmt --all -- --check`,
  `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`,
  and `cargo +1.96.0 test --locked`.
- Keep spec-oriented documents synchronized when their claims change. Record
  docs-only work in `CHANGELOG.md` without changing the package version, per the
  versioning policy in `README.md`.

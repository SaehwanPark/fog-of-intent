# Repository Agents Guide

Keep this file short and repo-wide. Detailed project workflows live in
`.agents/skills/` and `docs/harness/`.

## What

- Fog of Intent is a Rust 2024 turn-based, AI-native team-strategy simulation
  about intent, uncertainty, delegated execution, communication, and bounded
  rationality. It is implemented and technically verified; it is **not**
  human-validated, and it is pre-release.
- Canonical project state lives in `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, and `CHANGELOG.md`. The longer proposal and stack analysis
  under `docs/` are design sources, not implementation evidence.
- Shipped and reachable now: a 17-scenario catalog (`--list-scenarios`,
  `--select`), interactive lane scenarios including the two-window fixture
  (`--scenario m3-two-window-fixture-v1`), the interactive multi-lane match
  (`--scenario m9-interactive-match-v1`), a print-and-exit replay-verified match
  transcript (`--scenario m9-complete-match-replay-v1`), run-directory
  persistence (`--run-dir`), a Model Context Protocol server (`--mcp`, or the
  dedicated `fog-of-intent-mcp` binary), and M6-M12 evaluation, study,
  presentation, and release-audit runners.
- Still **not** shipped or evidenced: human playtest or accessibility evidence,
  a live browser client, and a published release. Do not describe those as
  delivered. Milestone status is two-dimensional: Implementation State and
  Evidence State are separate, and library or CLI completeness never implies
  human validation. Read rosters, counts, and scenario names from the code
  before restating them; `docs/audit_report_20260828.md` documents what happens
  when labels outrun evidence.

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
- Every new or changed subsystem records its primary audience and the evidence that
  would promote it one rung of the `README.md` ladder, in the `ROADMAP.md` section for
  that slice, together with what is explicitly not claimed.
- When documents disagree, code and printed output win and the losing document is
  corrected in the same change. Dated artifacts (`docs/audit_report_*.md`,
  `docs/decision_brief_*.md`, ADRs) describe their date and never promote a claim; the
  full precedence order is in `docs/harness/fog-of-intent/team-spec.md`.
- Artifact loading rejects a version mismatch instead of coercing it. A breaking change to an
  artifact that circulates ships its migration in the same change; retiring and re-identifying
  an identity is allowed only while it has no release, tag, or stored artifact.
  `docs/COMPATIBILITY.md` states the contract.
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

# Project Specification

**Status:** Active project-state index
**Last reviewed:** 2026-08-04

This file records verified past, the small active slice, and intentionally
deferred future work. It is not the product proposal, roadmap, issue tracker, or
per-commit journal.

Canonical direction and state live in:

- `README.md` — project entry point and current status;
- `ROADMAP.md` — authoritative milestone order and promotion gates;
- `ARCHITECTURE.md` — verified current structure and target boundaries;
- `CHANGELOG.md` — meaningful contributor- and user-visible history;
- `docs/project-proposal.md` — detailed product and research vision;
- `docs/tech-stack-consideration.md` — proposed technology choices.

## Maintenance Rule

Keep `Present` small. Every active item states what is done, not yet done,
verification, and deferrals. Move work to `Past` only after the named evidence
exists. Planned proposal or roadmap text is never implementation evidence.

## Past

### Repository inception — 2026-08-04

- A Rust 2024 binary package named `fog-of-intent` was initialized at version
  `0.1.0`.
- The executable is a placeholder that prints `Hello, world!`.
- A comprehensive proposal established the turn-based, AI-native team-strategy
  thesis, initial one-lane slice, bounded-rationality direction, deterministic
  authority boundary, replay/debrief goals, and evidence limits.
- A technology analysis recommended a Rust-authoritative core with CLI and MCP
  adapters, artifact-first persistence, optional Python research tooling, and an
  optional later GUI. Those recommendations remain unadopted until implemented
  or recorded as architecture decisions.

## Present

### M0 — Governed repository baseline

**Status:** Active
**Started:** 2026-08-04
**Branch at initialization:** `codex/domain-harness-roadmap`

#### Done

- Canonical roadmap created at `ROADMAP.md` with milestone dependencies, scope,
  evidence gates, claim limits, and maintenance rules.
- Lightweight `SPEC.md`, `ARCHITECTURE.md`, and `CHANGELOG.md` state documents
  initialized.
- Short repo-wide guidance created in `AGENTS.md`.
- A portable domain harness created under `.agents/skills/` and
  `docs/harness/fog-of-intent/` for simulation design, agent ecology, synthesis,
  and project-specific QA.
- `_workspace/` handoff names and lifecycle documented.
- README revised to distinguish the current placeholder from the planned game
  and to point contributors at canonical documents.

#### Not Yet Done

- Choose and add a license, contribution policy, and appropriate conduct policy.
- Add unofficial/noncommercial fan-project notice and original-setting fallback
  policy suitable for distribution.
- Create the first ADR and stable terminology reference.
- Pin the Rust toolchain and package lockfile.
- Add formatting, lint, test, link, and currentness checks to CI.
- Decide the M1 crate/workspace layout and minimal dependency set.

#### Verification

- All generated repo-local `SKILL.md` files have YAML frontmatter with distinct
  selection boundaries.
- Harness handoff names agree between specialist skills and the team spec.
- All local Markdown links resolve.
- `git diff --check` passes.
- The existing Rust placeholder remains formatting- and test-clean.

#### Deferred / Non-Goals

- No simulation mechanic, playable decision window, CLI command loop, MCP
  server, replay engine, research package, or GUI is part of M0.
- M0 does not establish intellectual-property clearance, public-release
  readiness, accessibility, enjoyment, or research validity.

## Future

The detailed and canonical order is in `ROADMAP.md`.

- **M1:** typed deterministic kernel, explicit resolved inputs, append-only
  history, state hashes, and replay identity.
- **M2:** one complete lane scenario with actor-specific uncertainty, intent,
  delegated execution, branching, and causal debrief.
- **M3:** keyboard-first CLI reference experience.
- **M4:** interpretable non-LLM agent ecology.
- **M5:** thin, versioned, model-agnostic MCP adapter.
- **M6:** automated behavioral experiments and regression evidence.
- **M7:** evidence-limited semantic-to-parametric calibration proof.
- **M8:** trust-sensitive team communication and shot-calling.
- **M9:** bounded multi-lane match prototype.
- **M10:** human usability and accessibility alpha evidence.
- **M11:** optional host-bound GUI if demonstrated needs justify it.
- **M12:** public research-capable alpha with release and claim governance.

## Persistent Product Non-Goals

- Full reproduction of a proprietary game, roster, item catalog, or live
  metagame.
- Real-time mechanical control or reaction-time requirements.
- Networked multiplayer in the initial roadmap.
- Perfect-rationality or global-equilibrium claims.
- Treating AI-agent behavior as human behavior.
- A general-purpose multi-agent simulation framework before a proven vertical
  slice.
- Public, legal, accessibility, entertainment, or scientific claims without the
  evidence appropriate to each claim.

# Request Summary — Audit-Driven Truth Reconciliation and M9 Roster Completion

**Harness artifact:** Phase 0 request framing
**Date:** 2026-08-30
**Trigger:** User instruction to finish `SPEC.md` planned items and address every concern in
`docs/audit_report_20260828.md` except human-involved stop gates.

## Requested Outcome

1. Every concern raised in `docs/audit_report_20260828.md` is addressed, or is explicitly
   recorded as a human stop gate that this work cannot discharge.
2. No canonical document, CLI surface, or MCP surface claims more than the shipped code
   evidences.
3. The one `SPEC.md` planned item that is not actually delivered — the M9 "5v5" tactical
   match — becomes true rather than remaining a label.

## Roadmap Milestone

- Primary: project-state reconciliation across M3/M5/M9/M10/M12 documentation (audit
  Priority 1).
- Secondary: M9 bounded multi-lane match completion (audit Priority 2).
- Tertiary: product-identity decision recorded as an ADR (audit Priority 4).
- Not addressed: audit Priority 3 (human playtests) — human stop gate.

## Current Evidence

Verified against `main` at `964426a`, package `0.1.239`:

| Claim surface | Claimed | Verified reality |
| --- | --- | --- |
| `AGENTS.md` | "The repository currently contains a bounded two-window fixture command loop and a print-and-exit replay-verified M9 transcript. Do not describe an interactive complete match, MCP server, persistence service, GUI ... as playable" | 16-scenario catalog, interactive lane + interactive match, run-directory persistence, standalone MCP binary (25 tools / 8 resources / 3 prompts, counted from `--tools`/`--resources`/`--prompts`) |
| `HOW_TO_PLAY.md` | "The binary accepts only `--scenario m3-two-window-fixture-v1`"; "no GUI or second scenario"; "not in this runner: ... an MCP server"; "sixteen runner verbs" | 16 scenarios; MCP server ships as `--mcp` and `fog-of-intent-mcp`; 13 verbs in the match loop, 16 in the lane loop |
| "5v5 tactical match" | `m9-interactive-match-v1` and `m9-complete-match-replay-v1` branded 5v5 in catalog, MCP descriptions, `SPEC.md`, `README.md`, `ROADMAP.md` | `CompleteMatchCatalog::allied_snowball_victory()` = 3 allied actors (1,2,3) vs 1 opposing (4); `comeback_concession()` = 2 allied vs 1 opposing. `observe` prints 4 actors, not 10 |
| `docs/adr/0004` crate inventory | `foi-map` depends on kernel+lane; `foi-agent` on kernel+lane+map; `foi-study` on kernel+lane+map+agent; `foi-gui` on kernel+lane+map+protocol+agent; `foi-alpha` on all domain crates | `foi-map` → kernel only; `foi-agent` → kernel+lane; `foi-study` → none; `foi-gui` → kernel+lane+protocol; `foi-alpha` → none (read from each `crates/*/Cargo.toml`) |
| `docs/audit_report_20260825.md` | listed as current "Verified" evidence in `ROADMAP.md` | Describes `0.1.218`, pre-workspace tree; already superseded by `audit_report_20260828.md` |
| M10 name | "Human-usable and accessibility-tested alpha" marked Complete | Implementation complete; zero human participants; the name asserts the unearned evidence |

Additional playtest findings from live sessions on `m9-interactive-match-v1`:

- `observe` emits 26 structure rows every call (~30 lines) — dominant cognitive load per decision.
- `advance` reports `events=0 effects=0` with no causal explanation when an action had no
  effect (for example contesting an unspawned objective, or warding).
- `plan siege ... <damage>` asks the player to type raw damage numbers, which contradicts the
  project thesis of expressing intent rather than performing mechanics.

## In Scope

- `AGENTS.md`, `HOW_TO_PLAY.md`, `README.md`, `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`,
  `CHANGELOG.md`, `docs/adr/0004-*`, `docs/audit_report_20260825.md`.
- New `docs/adr/0005-*` recording the product-identity decision.
- `crates/foi-map` complete-match catalog rosters; `src/host/match_host.rs`;
  `src/command_loop.rs`; `src/mcp/*` labels; matching tests.

## Non-Goals

- No new milestone, framework, crate, dependency, or persistence service.
- No human-participant study activity (audit Priority 3 stop gate).
- No live browser client, no public release tagging.
- No change to the deterministic transition boundary (ADR-0001) or actor-visible redaction
  rules; roster size must not weaken fog-of-war projection.

## Project Boundaries Touched

- Project-state semantics: implementation maturity versus evidence maturity.
- Actor-visible information: a larger roster must not leak opposing latent state;
  `MatchMapState::observe` stays the only projection path.
- Determinism and replay: added rosters must replay to identical hashes.

## Source Files

- `crates/foi-map/src/complete_match_catalog.rs`, `complete_match.rs`, `state.rs`
- `src/host/match_host.rs`, `src/command_loop.rs`, `src/cli/match_replay.rs`, `src/mcp/tools.rs`
- canonical documents listed above

## Expected Outputs

- Truthful documentation and surface labels, with rosters stated as numbers.
- A genuine five-versus-five canonical match scenario, replay-verified, plus role binding for
  the five documented `MatchRole` values.
- An ADR recording the identity decision and its evidence requirements.
- CHANGELOG entries; version bump only where shipped surface strings change.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- Live CLI transcripts: `--list-scenarios`, `--scenario m9-interactive-match-v1`,
  `--scenario m9-complete-match-replay-v1`
- MCP catalog counts from `fog-of-intent-mcp --tools|--resources|--prompts`

## Evidence Limits and Open Questions

- Adding actors makes the match structurally 5-versus-5; it does **not** establish that the
  match is strategically deep, enjoyable, or accessible. Those remain human gates.
- A scripted action sequence proves determinism and replay, not that a human can discover a
  winning plan.
- Whether a genuine 5v5 roster should replace the bounded fixtures as the default, or exist
  beside them, is resolved in this work as: new canonical scenario, bounded fixtures retained
  under accurate names, because existing reference outputs depend on them.

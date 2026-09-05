# Fog of Intent

A turn-based, AI-native team-strategy simulation about making plans under
uncertainty and living with how teammates interpret and execute them.

> **Project status:** Modular Rust workspace (`0.1.246`) featuring a deterministic
> simulation kernel, one-lane and multi-lane scenario runners, an MCP JSON-RPC
> stdio server, behavioral validation benchmarks, and presentation/study toolkits.
> The active focus is M3 CLI reference gameplay validation. Live empirical human
> trials, live browser client, and published release tags remain pending.

**Product identity:** Fog of Intent is **primarily a game**, and it keeps its research
instrument. Engagement quality carries the higher weight, and when gameplay quality and
research instrumentation disagree, gameplay wins — with the losing obligation written
down rather than dropped. See
[ADR-0005](docs/adr/0005-product-identity-hybrid.md) (Accepted, 2026-08-30) and the open
questions in [the decision brief](docs/decision_brief_20260830.md). Read the two
audiences separately: a runnable scenario is evidence about the game, a verified
battery is evidence about the instrumentation, and neither proves the other.

## The Idea

Fog of Intent asks whether the strategic depth of a multiplayer online battle
arena can survive when real-time mechanics are delegated. Instead of aiming,
kiting, or reacting within milliseconds, the player expresses intent,
commitment, messages, contingencies, and fallback behavior. Simulated actors
perform the execution. The debrief explains what happened without treating a
lucky outcome as proof of a good decision.

The reference gameplay experience centers on short tactical scenarios and
complete multi-lane matches: human or agent laners express intent, commitment,
and pings under fog-of-war uncertainty, autonomous teammates resolve coordinated
actions, and causal debriefs inspect why reality diverged from intent. M2 one-lane
strategy playthroughs, the bounded M9 tactical-match fixture, and MCP tooling
are runnable reference surfaces; the active M3 goal is validating decision
feel, playability, and user agency.

The initial design is inspired by the strategic structure of *League of
Legends*, but the project is not affiliated with or endorsed by Riot Games. It
is a noncommercial design and engineering prototype with an original-setting
fallback. License and distribution notices exist; release-specific legal
clearance remains pending.

## Design Commitments

- **Intent is not execution.** A sound plan may fail, and a weak plan may
  succeed.
- **Actors are not omniscient.** True state, belief, observation, report, and
  research inspection stay separate.
- **The core is reproducible.** The same state, commands, resolved inputs, and
  ruleset must produce the same events, effects, next state, and hash.
- **Evidence has limits.** AI playtests do not establish human enjoyment,
  accessibility, trust, or behavioral validity.

The full invariant list is in [DESIGN_PRINCIPLES.md](DESIGN_PRINCIPLES.md).

## Current State

| Area | State |
| --- | --- |
| Current roadmap milestone | M3 — CLI Reference Experience (Active) |
| Repository governance and canonical docs | Complete — M0 |
| Deterministic kernel | Complete — M1 |
| One-lane vertical slice | Complete — M2 (three playable interactive strategy scenarios, automated advance condition integration, full replay and causal debrief) |
| Rust package | `0.1.246`, edition 2024, Rust `1.96`, one deferred edge crate (`reedline`), multi-crate Cargo workspace (`fog-of-intent`, `crates/foi-kernel`, `crates/foi-lane`, `crates/foi-map`, `crates/foi-agent`, `crates/foi-protocol`, `crates/foi-study`, `crates/foi-gui`, `crates/foi-alpha`) |
| Executable behavior | Standalone `--version`/`-V`, `--list-scenarios`/`-l`, interactive scenario selection (`--select`/`-s`), Model Context Protocol (MCP) JSON-RPC stdio server (`fog-of-intent mcp serve`, `--mcp`, or dedicated binary `cargo run --bin fog-of-intent-mcp`), the two-window fixture with `--scenario m3-two-window-fixture-v1` (optional `--run-dir`, `--color auto/always/never`, `--width <cols>`), 3 interactive strategy scenario playthroughs (`--scenario m2-strategy-happy-path-v1`, `--scenario m2-strategy-risk-taking-v1`, `--scenario m2-strategy-conservative-v1`), the automated behavioral experiments battery with `--scenario m6-behavioral-experiments-v1`, the semantic-to-parametric calibration proof battery with `--scenario m7-calibration-proof-v1`, the team communication & shot-calling benchmark battery with `--scenario m8-team-scenarios-v1`, the interactive multi-lane tactical match runner (3 allied actors vs 1 opposing actor) with `--scenario m9-interactive-match-v1`, the replay-verified complete-match transcript with `--scenario m9-complete-match-replay-v1`, the human usability & accessibility alpha study synthesis report with `--scenario m10-human-study-synthesis-v1`, the empirical multi-cohort study trials battery with `--scenario m10-empirical-cohort-study-v1`, the verified actor-visible HTML5 presentation document with `--scenario m11-gui-presentation-v1`, the browser interaction flow battery with `--scenario m11-gui-browser-flow-v1`, the public alpha release readiness audit report with `--scenario m12-alpha-release-checks-v1`, the research reproducibility bundle audit report with `--scenario m12-reproducibility-bundle-v1`, or the tagged release archive inventory audit report with `--scenario m12-alpha-archive-v1` |
| CLI reference experience | Complete — M3; TTY prompt, Tab completion, optional color, dynamic interactive scenario selection (`--select`), scenario catalog discovery (`--list-scenarios`), and `help`/`?` topics; pipes stay labeled plain text. Strategy, behavioral experiments, calibration proof, team benchmark, study synthesis, empirical cohort trials, presentation export, browser flow evaluation, release checks, reproducibility bundle, release archive, and match scenario selections available |
| Agent ecology and MCP | Model Context Protocol (MCP) JSON-RPC 2.0 stdio server active (`cargo run --bin fog-of-intent-mcp` or `fog-of-intent --mcp`) with 25 tools, 3 prompts, and 8 resources covering one-lane planning, multi-lane tactical matches, behavioral experiments, semantic-to-parametric calibration (`calibration_proof_run`, `fog-of-intent://calibration/model-card`), team communication, study synthesis, empirical cohort study trials (`cohort_study_run`, `fog-of-intent://study/cohort-trials`), HTML5 GUI rendering (`gui_presentation_render`, `fog-of-intent://presentation/html`), browser interaction flow evaluation (`gui_browser_flow_run`, `fog-of-intent://presentation/browser-flow`), release checks (`alpha_release_checks_run`, `fog-of-intent://release/readiness`), governance auditing (`alpha_governance_audit`), release archive verification (`alpha_release_archive_run`, `fog-of-intent://release/archive`), reproducibility verification (`reproducibility_bundle_run`), and auditor prompts (`alpha_release_audit`) |
| Behavioral experiments and calibration | Complete — M6 and M7; automated behavioral experiments & population validation battery runner (`--scenario m6-behavioral-experiments-v1`), semantic-to-parametric calibration proof battery runner (`--scenario m7-calibration-proof-v1`), and MCP tools (`behavioral_experiments_run`, `calibration_proof_run`, resource `fog-of-intent://calibration/model-card`) active |
| Team communication and shot-calling | Complete — M8; 5-case canonical benchmark battery runner (`--scenario m8-team-scenarios-v1`), leadership structures, dialogue negotiation, simultaneous resolution, and strategic dissent proofs verified |
| Full match, human alpha, optional GUI | M9 complete-match library, bounded interactive match fixture (`--scenario m9-interactive-match-v1`), and CLI replay transcript exist; M10 study synthesis runner (`--scenario m10-human-study-synthesis-v1`), empirical cohort trials battery runner (`--scenario m10-empirical-cohort-study-v1`), participant cohort schema, dimension assessments, interaction audits, informal check remediation, sampling limits, and alpha readiness disposition gates verified; M11 presentation need assessment, actor-visible GUI DTOs, reversible client state machine, triple CLI/MCP/GUI projection parity verification, asset governance/fallback rules, standalone HTML5/CSS/SVG presentation document generator, loopback transport protocol / session adapter, browser flow resilience / recovery evaluation library contracts, CLI presentation exporter (`--scenario m11-gui-presentation-v1`), and MCP presentation renderer (`gui_presentation_render`, `fog-of-intent://presentation/html`) exist; human empirical testing and live browser client are not implemented — M10-M11 |
| Public alpha | M12 release governance manifest evaluation, policy compliance, cross-version compatibility matrix, data dictionary redaction auditing, known limitations / evidence boundaries / citation guidance, documentation guides DAG verification, sample reproducibility bundle packaging & runner (`--scenario m12-reproducibility-bundle-v1`, MCP tool `reproducibility_bundle_run`), release readiness verification check suite (`--scenario m12-alpha-release-checks-v1`, MCP tool `alpha_release_checks_run`, MCP resource `fog-of-intent://release/readiness`), release archive inventory verification (`alpha_release_archive_run`, MCP resource `fog-of-intent://release/archive`), governance auditor (`alpha_governance_audit`, prompt `alpha_release_audit`), and benchmark catalog library contracts exist; release candidate human testing remains open — M12 |


The binary is a fixture adapter, not a complete reference client. Later-phase
rows are library evidence, not player commands. See [SPEC.md](SPEC.md) and
[ROADMAP.md](ROADMAP.md) for the full inventory, gates, and deferrals.

## Quickstart

Install a Rust toolchain with Rust 2024 edition support (this repository pins
`1.96.0`), then inspect available scenarios:

```sh
cargo run -- --list-scenarios
```

Launch the interactive scenario selector to choose any scenario from a menu:

```sh
cargo run -- --select
```

Or start the interactive reference fixture directly:

```sh
cargo run -- --scenario m3-two-window-fixture-v1
```

Or run one of the three canonical strategy scenario playthroughs:

```sh
cargo run -- --scenario m2-strategy-happy-path-v1
```

On a terminal you get a `> ` prompt, a one-line status, and Tab completion.
Type `?` or `help`, then commands such as `observe`, `plan contest`, `commit`,
and `advance`. Piped input has no prompt and prints labeled plain text. The
[How to Play](HOW_TO_PLAY.md) guide walks through a full two-window session.

The M7 semantic-to-parametric calibration proof battery evaluates diagnostic choice
dilemmas, empirical distributions, regularized parameter fitting, and recalibration gates:

```sh
cargo run -- --scenario m7-calibration-proof-v1
```

The M9 complete-match replay prints a replay-verified transcript of both
canonical composed matches and exits (always plain text; `--color` and
`--run-dir` do not apply):

```sh
cargo run -- --scenario m9-complete-match-replay-v1
```

The M10 empirical multi-cohort study trials battery evaluates completion rates, decision explanation
qualities, debrief causal comprehensions, cognitive friction indicators, and accessibility readiness across 4 cohorts:

```sh
cargo run -- --scenario m10-empirical-cohort-study-v1
```

The M11 GUI presentation exporter generates a self-contained, accessibility-compliant
actor-visible HTML5/CSS/SVG document from benchmark presentation bundles:

```sh
cargo run -- --scenario m11-gui-presentation-v1 > presentation.html
```

The M11 GUI browser interaction flow battery evaluates multi-tab navigation, node inspection,
causal debrief filtering, network recovery, and accessibility audits across canonical browser targets:

```sh
cargo run -- --scenario m11-gui-browser-flow-v1
```

The M12 Public Alpha release checks runner executes the complete multi-domain
readiness verification audit suite and prints the formatted Markdown report:

```sh
cargo run -- --scenario m12-alpha-release-checks-v1
```

The M12 Public Alpha reproducibility bundle runner executes the sample artifacts
integrity and 16-hex FNV-1a content hash verification suite:

```sh
cargo run -- --scenario m12-reproducibility-bundle-v1
```

The M12 Public Alpha release archive manifest runner audits all 11 artifact categories,
content digests, and combined signature verification:

```sh
cargo run -- --scenario m12-alpha-archive-v1
```

Run the standalone Model Context Protocol (MCP) JSON-RPC 2.0 stdio server directly or inspect catalog items:

```sh
# Start stdio MCP JSON-RPC 2.0 server directly
cargo run --bin fog-of-intent-mcp

# List all 25 MCP tools, 8 resources, and 3 prompts in catalog
cargo run --bin fog-of-intent-mcp -- --tools
cargo run --bin fog-of-intent-mcp -- --resources
cargo run --bin fog-of-intent-mcp -- --prompts
```

Optional:

```sh
cargo run -- --version
```

```sh
printf 'plan contest\ncommit\nadvance\nsave run\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
printf 'load run\ninspect history\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
```

The executable does not choose a default `--run-dir`.

Contributor checks (not required to play):

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
python3 scripts/check_repository.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

On Windows, `python` replaces `python3`; the contributor scripts read and write
UTF-8 explicitly and report paths with `/`, so no console codepage change is
needed.

## What a run looks like

On a TTY the runner prints a short banner and `> `. Piped captures stay labeled
(trimmed live capture of `m3-two-window-fixture-v1`):

```text
observe
observation: schema=m2-lane-observation-v3 turn=0 observation_id=1
self: health=8 position=center mana=6 gold=0 experience=0 cooldown=0
opponent: label=unknown position=unknown
jungle_threat: label=unknown region=unknown
available_intents: stabilize,contest,yield,recall
plan contest
draft: status=staged field=plan
commit
commit: status=committed intent=contest
advance
advanced: window=first outcome=held_space
plan stabilize
draft: status=staged field=plan
commit
commit: status=committed intent=stabilize
advance
advanced: window=second outcome=yielded_space
debrief
debrief: schema=m2-two-window-final-debrief-v3 final_objective=goal_missed
window: name=first intent=contest outcome=held_space position=center health=8 wave=advanced objective=goal_achieved
window: name=second intent=stabilize outcome=yielded_space position=near_tower health=8 wave=held objective=goal_missed
```

Scripted equivalent:

```sh
printf 'observe\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\ndebrief\nquit\n' \
  | cargo run -- --scenario m3-two-window-fixture-v1
```

## Canonical Documents

- [How to Play](HOW_TO_PLAY.md) — beginner walkthrough of the current runner.
- [Roadmap](ROADMAP.md) — milestone order, scope, dependencies, and exit evidence.
- [Specification](SPEC.md) — verified past, active work, and deferred future.
- [Architecture](ARCHITECTURE.md) — current structure and explicitly labeled
  target boundaries.
- [Changelog](CHANGELOG.md) — meaningful contributor- and user-visible history.
- [Lessons](LESSONS.md) — verified, reusable project traps and preventions.
- [Design principles](DESIGN_PRINCIPLES.md) — concise implementation invariants.
- [Terminology](docs/TERMINOLOGY.md) — authoritative domain vocabulary.
- [Architecture decision records](docs/adr/) — consequential boundary decisions.
- [Compatibility policy](docs/COMPATIBILITY.md) — minimum artifact and replay
  version rules.
- [Dependency policy](docs/DEPENDENCY_POLICY.md) — dependency, security, and
  license review obligations.
- [Project notice](NOTICE.md) — unofficial, noncommercial, and distribution
  boundaries.
- [Independent audit report (2026-08-28)](docs/audit_report_20260828.md) — latest
  technical and architectural audit report ([2026-08-25 audit](docs/audit_report_20260825.md)).
- [Project proposal](docs/project-proposal.md) — detailed product, simulation,
  research, risk, and validation vision.
- [Technology considerations](docs/tech-stack-consideration.md) — proposed stack;
  recommendations are not adopted architecture until implementation evidence or
  an ADR says so.
- [Agent harness team spec](docs/harness/fog-of-intent/team-spec.md) — reusable
  domain roles, routing, handoffs, review, and failure policy.

## Contributing Workflow

1. Read `CONTRIBUTING.md`, `SPEC.md`, and the active milestone in `ROADMAP.md`.
2. Select the smallest complete slice and state its observable verification and
   non-goals.
3. Read `AGENTS.md`; for substantial domain work, use the `fog-intent-*`
   and `foi-test-player` repo-local skills and the harness team spec.
4. Keep I/O, async work, persistence, rendering, model providers, and randomness
   outside the deterministic transition boundary.
5. Add focused tests or inspection evidence and reconcile affected project-state
   documents.
6. Use two-space indentation and spaces only; the repository checker and pinned
   Rust formatter enforce this policy.
7. Run the repository checks and review the final diff for contradictions or
   unsupported capability claims.

The repo-local harness owns only Fog of Intent domain judgment. Generic Rust,
functional design, UX, accessibility, code review, documentation, release, and
git practices remain reusable global concerns.

## Repository Map

```text
src/                         Rust fixture loop plus internal kernel/adapter fixtures
docs/                        Proposal, stack analysis, ADRs, terminology, and harness contract
scripts/                     Dependency-free repository currentness checks
.agents/skills/              Repo-local domain skills
_workspace/                  On-demand inspectable handoff artifacts
HOW_TO_PLAY.md               Beginner guide to the current runner
ROADMAP.md                   Canonical execution plan
SPEC.md                      Current project state
ARCHITECTURE.md              Current and target system boundaries
CHANGELOG.md                 Meaningful history
LESSONS.md                   Verified contributor lessons
LICENSE / NOTICE.md          Source license and distribution boundaries
```

Future source, scenario, schema, profile, experiment, research, or GUI
directories appear only when their roadmap slice demonstrates a need.

## Versioning

Versions use `a.b.c`, starting at `0.1.0`:

- increment `c` for a merged PR or PR-equivalent that changes the codebase;
- do not increment `c` for documentation-, comment-, or repository-metadata-only
  changes;
- increment `b` for a significant feature release or accumulated evolution;
- increment `a` for a new lifecycle stage, such as an initial production
  release;
- when `a` or `b` increments, reset lower segments to zero;
- segments do not carry automatically at 10 (`0.1.9` becomes `0.1.10`).

If a change qualifies for more than one increment, apply only the highest one.

## Project Principle

> Build a text-first strategic simulation in which players and agents act under
> incomplete information, express plans rather than reflexes, coordinate
> imperfectly, and learn through reproducible causal debriefs.

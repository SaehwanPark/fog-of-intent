# Fog of Intent

A turn-based, AI-native team-strategy simulation about making plans under
uncertainty and living with how teammates interpret and execute them.

> **Project status:** Pre-playable foundation. The repository currently
> contains a Rust 2024 line-oriented fixture command loop, an internal
> deterministic kernel and replay fixtures, canonical planning/spec documents,
> and a domain-oriented agent harness. No complete playable simulation, MCP
> server, persistence service, or GUI exists yet.

## The Idea

Fog of Intent asks whether the strategic depth of a multiplayer online battle
arena can survive when real-time mechanics are delegated. Instead of aiming,
kiting, or reacting within milliseconds, the player expresses:

- intent and commitment;
- lane posture, risk, recall timing, and resource priorities;
- messages, proposals, and conditional commitments;
- abort conditions and fallback behavior;
- trust, leadership, and responses to imperfect information.

Simulated actors perform the execution. Their results depend on what they know,
what they believe, how they coordinate, and how well they execute under explicit
uncertainty. The debrief then explains what happened without treating a lucky
outcome as proof of a good decision.

The initial design is inspired by the strategic structure of *League of
Legends*, but the project is not affiliated with or endorsed by Riot Games. It
is currently a noncommercial design and engineering prototype with an
original-setting fallback. Initial license, content, and distribution policies
now exist; release-specific review and legal clearance remain pending.

## Design Commitments

- **Intent is not execution.** A sound plan may fail, and a weak plan may
  succeed.
- **Actors are not omniscient.** True state, belief, observation, report, and
  research inspection stay separate.
- **Behavior is bounded, not merely random.** Candidate generation, evaluation,
  selection, coordination, and execution remain distinct.
- **The core is reproducible.** Given the same state, validated commands,
  resolved inputs, and ruleset, the transition must produce the same events,
  effects, next state, and hash.
- **History is inspectable.** Committed transitions are append-only and support
  replay, branching, and causal debriefing.
- **Interfaces share one authority.** CLI, MCP, research, and any future GUI use
  host-owned commands and actor-visible projections.
- **Evidence has limits.** AI playtests can test software and modeled behavior;
  they do not establish human enjoyment, accessibility, trust, or behavioral
  validity.
- **Vertical slices come first.** The project earns broader frameworks and
  surfaces through demonstrated needs.

## First Playable Target

The first vertical slice is one short lane scenario with:

- one human-controlled laner;
- one opposing laner;
- one allied autonomous actor;
- one abstract opposing jungle threat;
- wave pressure, vision, resources, trading, recall, and gank response;
- variable-duration decision windows;
- intent, commitment, messages, contingencies, and delegated execution;
- a terminal objective, deterministic replay, one bounded branch, and a causal
  debrief.

The target question is simple: is it understandable and enjoyable to make
strategic team-game decisions when mechanical execution belongs to modeled
players?

## Current State

| Area | State |
| --- | --- |
| Current roadmap milestone | M2 — One-Lane Vertical Slice (Active) |
| Repository governance and canonical docs | Complete — M0 |
| Rust package | `0.1.179`, edition 2024, Rust `1.96`, no dependencies, single package |
| Executable behavior | Reports package version with standalone `--version`/`-V`, or runs the documented line-oriented deterministic two-window fixture transcript with explicit versioned `--scenario m3-two-window-fixture-v1` selection and optional `--run-dir` artifact storage |
| Deterministic kernel | M1 fixture/codec complete; M2 v3 internal lane-window, explicit four-actor roster, retained-resource aggregate, intent, observation, branch, coordination, objective, strategy-fixture, scenario, replay, delayed-origin provenance, debrief, advance-condition, and belief/report contracts implemented |
| One-lane scenario | Internal diagnostic windows and fixtures cover bounded intents, observations, coordination, resources, replay, and debrief projections — full scenario not complete |
| CLI reference experience | Stable grammar plus typed requests, labels, draft/undo, run IDs, versioned host artifacts, injected file storage, pure text projection with machine-checked labeled lines, explicit one-fixture scenario selection, package version reporting, and an optionally persistent fixture command loop; broader scenario selection and human accessibility evidence remain open |
| Agent ecology and MCP | Three actor-visible deterministic scripted profiles with transparent policy-role IDs, matched-input evidence, bounded comparison/action-tally/candidate-breadth evidence, an opt-in reproducible seed bundle and versioned experiment manifest and version-catalog metadata, deterministic in-process batch with caller-driven start/chunk/finish/checkpoint-saved/batch-resumed event production, matched-observation and caller-supplied matched-scenario sample-set/tally evidence with bounded machine-readable codecs, a caller-declared profile-aware tally comparison with its provenance-bound codec, and a provisional profile-aware fixed-fixture no-change gate, a closed fixed-fixture scenario catalog/selector and bounded scenario-frequency evidence with its verified-report-bound codec, pure Markdown frequency projection, and bounded 10,000-point caller-declared distribution projection, a deterministic fixed-fixture population generator capped at four alternating closed scenarios plus caller-declared ordered composition over that closed catalog, caller-declared baseline and build-labeled comparison, a fixed no-change regression gate, bounded cursor-resume evidence, a caller-declared run-disposition envelope, and a bounded non-authoritative operational event log container with a distinct injected codec/store namespace and caller-declared bounded segments, plus library-only decision replay records, bounded replay-identity/operational-sequence evidence, a provisional fixed-fixture outlier-threshold signal over verified signed count deltas, a caller-declared first verified replay reference for a matching tally candidate, a bounded calibrated outlier detection and representative replay report tracing qualified metric outliers to verified committed decision replay records, a bounded caller-declared degenerate-policy population report over repeated actor-visible Stabilize selections, a bounded actor-visible illegal-command population report over repeated host validation rejection, a bounded fixed-fixture risk-taking policy population report over repeated actor-visible Contest selections, a bounded actor-visible communication-abuse population report over repeated invalid message values, a bounded scenario-wide replay identity report over sampled decision records, and a bounded scenario-wide causal-trace completeness report over sampled decision records, pure actor observation/action/commit/draft/message/draft-receipt/draft-status/draft-clear/draft-commit-receipt/transcript/history/replay/replay-record/replay-debrief-record/error/result/debrief DTO boundaries with bounded codecs, immutable actor-session lifecycle with explicit timeout/disconnect closure and encoded-action error mapping, a bounded two-actor simultaneous submission window, bounded CLI/protocol action and projection parity evidence, actor-safe validation-error/repair hints, host-projected actor observations/history/replay status/replay records/saved-run replay records/saved-run replay-linked debrief records/debrief summaries, host-owned actor commit/action submission/result/closure, host-bound draft staging/readback/receipts/status/clear, payload-free draft-commit and draft-clear receipts reporting field presence, a bounded authorization/redaction matrix, provider-neutral transcript metadata, a closed ordinary-actor capability catalog, crate-private lane conversion adapters, and a checked core boundary that excludes transport/async primitives; message delivery, broader/random population generation and sampling, broader population metrics, privileged tools, independent build provenance, actual exploit search, actual communication-abuse search, actual adversarial populations, automatic runtime failure detection, automatic runtime log production, checkpoint failure diagnostics, automatic rotation/crash recovery, durable scenario-wide event-log recovery, and broader ecology/MCP remain open — M4-M6 |
| Behavioral experiments and calibration | Versioned bounded experiment manifests and applicable version-catalog metadata, deterministic in-process batch with caller-driven start/chunk/finish/checkpoint-saved/batch-resumed event production, matched-observation, caller-supplied matched-scenario sample-set, selected-intent tally with ordered three-profile rows and verified bounded codec round trips, caller-declared profile-aware tally comparison with its provenance-bound codec, a provisional profile-aware fixed-fixture no-change gate, fixed-fixture frequency with pure Markdown and bounded caller-declared 10,000-point distribution shares, deterministic fixed-fixture population and direct population-to-tally composition, caller-declared baseline/build-labeled comparison, a fixed no-change regression gate, injected cursor-resume evidence, a caller-declared run-disposition envelope, a bounded non-authoritative operational event codec/store with caller-declared bounded segments, a bounded actor-visible illegal-command population report over repeated host validation rejection, a bounded fixed-fixture risk-taking policy population report over repeated actor-visible Contest selections, a bounded actor-visible communication-abuse population report over repeated invalid message values, a bounded scenario-wide replay identity report over sampled decision records, a bounded scenario-wide causal-trace completeness report over sampled decision records, a bounded calibrated outlier detection and representative replay report tracing qualified metric outliers to verified committed decision replay records, a compact semantic profile vocabulary and schema covering discrete trait dimensions and reference profile descriptors with a fail-closed catalog, a diagnostic choice catalog covering seven behavioral dilemma domains with fail-closed lookup and validation, declarative model/prompt and repeated-sampling protocol schemas, empirical action and communication ping signal distribution estimation with exact 10,000 basis-point scaling, discrete integer basis-point measures for behavioral distance (TVD), entropy (Gini diversity), sensitivity, consistency, and adaptation with unified profile reports, regularized parametric policy parameter fitting with basis-point shrinkage towards neutral uniform priors, canonical held-out diagnostic scenario suites across all seven dilemma domains, Total Variation Distance loss and exact basis-point modal accuracy evaluation on held-out distributions, counterfactual perturbation sensitivity evaluation verifying directional shift coherence under discrete conditions, and multi-model/prompting family comparison reports evaluating Total Variation Distance deltas, modal intent agreement, and categorical alignment classification (aligned, shifted, divergent); decision persistence, broader/random population sampling, broader scenario generation, actual exploit search, actual communication-abuse search, distributional/outcome/strategic metrics, independent build/source verification, automatic runtime failure detection, checkpoint failure diagnostics, automatic rotation/crash recovery, durable/scenario-wide event-log recovery, parameter unidentifiability reports, private chain-of-thought preservation, recalibration triggers, and prompt/parametric calibration remain open — M6/M7 |
| Team play, full match, human alpha, optional GUI | Not implemented — M8-M11 |
| Public alpha | Not release-ready — M12 |

The verified profile-aware selected-intent tally exposes a pure ordered
10,000-point intent-share projection and Markdown summary. It remains
fixed-fixture evidence; broader population distributions, outcomes, strategic
metrics, persistence, providers, calibration, and human evidence remain open.

The M6 stress-population slice currently exposes a caller-declared four-case
matrix for illegal-command, exploit-seeking, communication-abuse, and
degenerate-policy boundary evidence. A separate host-bound report repeats one
invalid actor command up to four times and retains only the stable validation
rejection category. Actual exploit search and communication-abuse search,
prevalence, outcomes, and human behavior remain open. A separate
fixed-fixture risk-taking report repeats actor-visible
`Contest` selections under the `risk-taking-laner-v1` policy rule; it is
selected-intent evidence only, not exploit search, prevalence, outcome, or
strategy-quality evidence. A separate bounded communication-abuse report
validates repeated invalid message payloads against `ActorMessageDto::new`; it
does not route, deliver, or store message text, search for exploits, or claim
outcomes.

The M6 comparison slice also exposes a deterministic largest-delta candidate
from verified profile-aware tally comparisons, preserving signed deltas and
stable row/intent ties. It is not an outlier detector or representative replay
selector; those broader definitions remain open.

The bounded operational-log slice also classifies caller-declared
start/chunk/finish label order, allowing checkpoint/resume labels between the
chunk and finish. A separate pure report can bind one decision's deterministic
replay identity to that sequence status; causal-trace completeness, runtime
production, and scenario-wide replay remain open.

The replay-sequence report exposes only `verified` or `decision_mismatch` for
the existing actor-visible decision record alongside the existing categorical
operational sequence status. It does not turn labels into a causal trace or
provide runtime, persistence, provider, or scenario-wide replay behavior.

The scenario-wide replay identity slice evaluates one to sixteen caller-supplied
`ScriptedAgentReplayRecord`s from a sampled run, reporting `all_verified` or
`decision_mismatch` alongside verified counts and observation bounds. It is
bounded sequence replay evidence only; causal-trace completeness, runtime
automated log production, durable persistence, provider integration, and
human gameplay claims remain open.

The scenario-wide causal-trace completeness slice evaluates one to sixteen
caller-supplied `ScriptedAgentReplayRecord`s from a sampled run, reporting
`all_complete` or `incomplete_trace` alongside traced counts and observation
bounds. It is bounded sequence causal-trace completeness evidence only;
runtime automated log production, durable persistence, provider integration,
and human gameplay claims remain open.

The outlier-threshold slice exposes `above_threshold`, `below_threshold`, or
`no_candidate` for a provisional inclusive magnitude of 2 over verified signed
intent-count deltas. It is a fixed-fixture signal only; calibrated outlier
detection and representative replay selection remain open.

The candidate replay-reference slice selects only the first caller-declared
record whose verified profile, evaluation rule, and selected intent match the
metric candidate. It is a reproducible reference, not proof of
representativeness, scenario-wide replay, causality, or build provenance.

The bounded operational-log adapter can save/load caller-declared segments
`0..=3` and list recognized indices; it does not infer rotation or crash state.

The fixed-fixture population adapter generates at most four entries in
deterministic safe/threat alternation or a caller-declared ordered composition
from a caller-supplied starting observation ID; it does not sample a broad
population or claim distributional, outcome, or human-behavior evidence.

See the [canonical roadmap](ROADMAP.md) for dependencies, scope, promotion
evidence, and explicit deferrals.

## Run the Bounded Fixture Command Loop

Install a Rust toolchain with Rust 2024 edition support, then send
line-oriented commands to the deterministic two-window fixture loop:

```sh
printf 'help\nquit\n' | cargo run
```

Expected output begins with the command catalog and ends with:

```text
help: commands
command: name=help usage=help summary=show command help
...
quit: status=closed
```

The default binary is intentionally a bounded fixture adapter: it supports one
versioned fixture ID, but has no scenario catalog, regenerated/graph branching,
or complete accessibility inspection.

To inspect the package version without starting a session, run:

```sh
cargo run -- --version
```

To complete the bounded two-window transcript through the executable:

```sh
printf 'observe\nmessage ping ally\ncontingency retreat if threat\nplan contest\ncommit\nadvance\nplan stabilize\ncommit\nadvance\nreplay\ndebrief\nquit\n' \\
  | cargo run -- --scenario m3-two-window-fixture-v1
```

To persist bounded artifacts between processes, provide an explicit directory:

```sh
printf 'plan contest\ncommit\nadvance\nsave run\nquit\n' \\
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
printf 'load run\ninspect history\nquit\n' \\
  | cargo run -- --scenario m3-two-window-fixture-v1 --run-dir ./runs
```

The executable does not choose a default directory; library callers can also
inject the bounded file store directly.

Repository checks:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
python3 scripts/check_repository.py
python3 -m unittest discover -s scripts -p 'test_*.py'
```

These commands validate the internal M1 kernel and M2 lane boundary plus
repository policy; the binary runs only the bounded fixture host and is not a
complete playable reference client.

## Canonical Documents

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
   repo-local skills and the harness team spec.
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

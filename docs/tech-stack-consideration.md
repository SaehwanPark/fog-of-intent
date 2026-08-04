## Recommendation

The best fit is a **Rust-first, multi-language research stack**:

* **Rust** owns simulation truth, commands, rules, observations, replay, CLI, and MCP.
* **Python** owns calibration, statistical analysis, experiment orchestration, and notebooks.
* **JSON/JSONL** provides inspectable operational artifacts.
* **Parquet + DuckDB** provides scalable behavioral analytics.
* **TypeScript** is introduced only when a graphical interface becomes justified.

I would not make Python the authoritative game engine, nor make an LLM framework the center of the architecture. The durable center should be a deterministic Rust simulation with replaceable human and AI adapters.

## 1. Authoritative simulation: Rust workspace

Use stable Rust with Edition 2024 and a Cargo workspace. Edition 2024 enables Cargo's Rust-version-aware resolver behavior, which is helpful for controlling dependency compatibility. ([Rust Documentation][1])

Suggested workspace:

```text
crates/
  domain/          # IDs, units, commands, events, effects
  simulation/      # deterministic transitions
  observation/     # actor-visible projections
  behavior/        # heuristic and parametric policies
  scenario/        # scenario definitions and validation
  history/         # append-only records, hashes, replay
  debrief/         # causal and player-facing explanations
  experiment/      # batch episode controller
  protocol/        # shared external DTOs
  cli/             # human reference interface
  mcp-server/      # AI-agent interface
  loopback-host/   # later GUI/API host
```

The key dependency rule should be:

```text
domain
  -> simulation
  -> observation/history/debrief
  -> experiment
  -> CLI/MCP/GUI adapters
```

The simulation crates should never depend on:

* Tokio;
* HTTP;
* MCP;
* terminal rendering;
* databases;
* Python;
* wall-clock time;
* or random-number generators.

Instead:

```rust
pub fn transition(
  prior: &WorldState,
  commands: &ValidatedCommandSet,
  inputs: &ResolvedInputs,
  rules: &Ruleset,
) -> Result<TransitionResult, TransitionError>
```

This directly continues the strongest architectural property of the Health Policy Strategy Game: the host owns simulation truth, while CLI, MCP, and GUI surfaces consume the same typed actions and actor-visible projections.

### Core Rust crates

I would begin with:

```toml
[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
clap = { version = "4", features = ["derive"] }
uuid = { version = "1", features = ["serde", "v7"] }
blake3 = "1"
indexmap = { version = "2", features = ["serde"] }
smallvec = "1"
rand = "0.9"
rand_chacha = "0.9"
tracing = "0.1"
```

Serde is the natural serialization layer because it derives serialization from Rust types without relying on runtime reflection. ([Serde][2]) Clap remains a mature declarative parser for commands, subcommands, help, and validation. ([Docs.rs][3])

Use:

* `thiserror` for typed domain and validation errors;
* `blake3` for replay/state hashes;
* `indexmap` where deterministic iteration order matters;
* `rand_chacha` for explicit, stable seeded random streams;
* `tracing` for runtime diagnostics outside committed simulation history.

Do not use `anyhow` inside core domain crates. It is fine in binaries and orchestration code, but core errors should remain typed.

## 2. Domain and numerical modeling

Avoid generic floating-point values for most authoritative state.

Use newtypes:

```rust
pub struct Health(i32);
pub struct Mana(i32);
pub struct Gold(i32);
pub struct Tick(u64);
pub struct Probability(f64);
pub struct Utility(f64);
pub struct TrustScore(f64);
```

For state-changing game quantities, consider integer or fixed-point representations:

```rust
pub struct BasisPoints(i32);
pub struct MilliUnits(i64);
```

Floating-point values are reasonable for:

* policy scores;
* utility estimates;
* posterior beliefs;
* calibration calculations;
* and behavioral sampling weights.

They should not silently determine equality-sensitive replay state without a declared normalization policy.

For example:

```rust
pub struct ActionEvaluation {
  pub action_id: ActionId,
  pub estimated_utility: OrderedUtility,
  pub uncertainty: Probability,
  pub rationale_features: Vec<FeatureContribution>,
}
```

Keep choice and execution distinct:

```rust
pub struct ChosenPlan {
  pub intent: Intent,
  pub commitment: Commitment,
  pub contingencies: Vec<Contingency>,
}

pub struct ExecutionInput {
  pub precision_draw: UnitInterval,
  pub reaction_draw: UnitInterval,
  pub coordination_draw: UnitInterval,
}
```

## 3. Scenario configuration: typed YAML or TOML, not executable scripts

Use:

* **TOML** for project and experiment manifests;
* **YAML** only where human-authored scenarios benefit materially from its syntax;
* **JSON** as the canonical external protocol representation.

A scenario file should compose known mechanics:

```yaml
id: top-lane-pressure-v1
ruleset: prototype-0.1

actors:
  blue_top:
    role: top
    profile: disciplined_weakside
  blue_jungle:
    role: jungle
    profile: proactive_jungler

objectives:
  - survive_until: "10:00"
  - minimum_gold_difference: -250
```

Deserialize into strongly typed Rust structures and validate them before session creation.

Do not permit embedded Python, Lua, JavaScript, or arbitrary expressions in early scenario files. Otherwise, the configuration layer quickly becomes a second, weakly typed simulation engine.

## 4. CLI: Clap plus a small interactive shell

Use `clap` for top-level process commands:

```text
game play
game replay <id>
game branch <id>
game experiment run <manifest>
game export <id>
game validate scenario <path>
game mcp serve
```

For the in-session command loop, use either:

* a small custom parser built over tokenized commands; or
* `reedline` for history, completion, and interactive editing.

I would not begin with a full terminal UI framework. Start with semantic text:

```text
> observe
> inspect jungle-threat
> message blue_jungle propose gank --confidence 0.70
> plan hold-wave --commitment moderate
> contingency add enemy-support-missing withdraw
> commit
```

A `ratatui` interface may become useful later for:

* timelines;
* map summaries;
* team readiness panels;
* and replay navigation.

But it should be a projection over the command host, not the reference interaction model.

## 5. MCP: official Rust SDK, thin adapter

Use the official Rust MCP SDK, `rmcp`. It is Tokio-based and supports MCP server functionality and schema generation. ([GitHub][4])

Recommended shape:

```text
mcp tool call
  -> deserialize external DTO
  -> resolve session and actor authority
  -> call application service
  -> receive actor-visible projection
  -> serialize MCP response
```

The MCP crate should contain no game rules.

Conceptual tools:

```text
session.start
actor.observe
actor.list_actions
actor.get_messages
actor.send_message
actor.submit_plan
actor.commit
actor.review
session.get_history
session.get_debrief
```

A separate controller role may expose:

```text
experiment.create
experiment.assign_agents
experiment.advance
experiment.run_batch
experiment.export
experiment.branch
```

Use `schemars` or the JSON Schema integration provided by the MCP stack for DTO schemas. Never expose internal Rust domain objects directly as public MCP contracts; create versioned protocol DTOs.

```rust
pub struct ActorObservationV1 {
  pub protocol_version: ProtocolVersion,
  pub actor_id: ActorIdDto,
  pub decision_window: DecisionWindowDto,
  pub visible_entities: Vec<VisibleEntityDto>,
  pub beliefs: Vec<ReportedBeliefDto>,
  pub legal_action_refs: Vec<ActionRefDto>,
}
```

This gives you freedom to revise internal representations without silently breaking agents.

## 6. Async runtime: Tokio only at the edges

Use Tokio for:

* MCP transport;
* optional concurrent agent requests;
* process management;
* HTTP;
* and batch orchestration.

The official MCP Rust SDK already uses Tokio. ([GitHub][4]) Axum is also designed for Tokio and Hyper. ([Docs.rs][5])

Do not make the simulation itself asynchronous.

A useful pattern is:

```text
async orchestration
  -> gather observations
  -> request agent decisions concurrently
  -> validate all submissions
  -> construct immutable command set
  -> call synchronous deterministic transition
  -> persist result
```

This preserves simultaneous decision semantics without infecting the domain model with asynchronous state.

## 7. Persistence: artifacts first, embedded database later

For the early project, I would avoid PostgreSQL and other server databases.

Use a run directory:

```text
runs/
  <match-id>/
    manifest.json
    initial-state.json
    history.jsonl
    snapshots/
    debrief.md
    metrics.json
    replay-hashes.json
```

### Canonical operational formats

* **JSON:** manifests, snapshots, protocol payloads.
* **JSONL:** append-only commands, observations, messages, events, and effects.
* **Markdown:** player-facing and developer-facing debriefs.
* **Binary checkpoint format:** optional later optimization, never the only durable representation.

JSONL is particularly useful because it is:

* append-friendly;
* diffable;
* streamable;
* easy for AI agents to inspect;
* and straightforward to transform into analytical tables.

### SQLite

Introduce SQLite only when you need:

* run indexing;
* checkpoint discovery;
* experiment catalogs;
* profile registries;
* or fast metadata filtering.

Do not put the authoritative event history exclusively inside SQLite. Keep portable replay artifacts.

## 8. Behavioral analytics: Parquet + DuckDB

Use **Parquet** for derived experiment tables and **DuckDB** for local analytical queries.

The official Rust Arrow project provides native Rust Arrow and Parquet implementations, including conversion between Arrow record batches and Parquet files. ([Apache Arrow][6]) DuckDB can directly query Parquet and JSON and is designed as an in-process analytical database, so it avoids operating a separate database service. ([DuckDB][7])

A clean data flow is:

```text
authoritative JSONL history
  -> versioned extractor
  -> normalized Parquet tables
  -> DuckDB / Python analysis
```

Potential tables:

```text
episodes.parquet
decision_windows.parquet
observations.parquet
beliefs.parquet
messages.parquet
candidate_actions.parquet
chosen_actions.parquet
execution_events.parquet
effects.parquet
agent_configs.parquet
```

Keep Parquet as a derived analytical representation. Replay should not depend on Parquet.

## 9. Research and calibration: Python companion package

Python should be a separate workspace/package, not embedded throughout the Rust engine.

Recommended stack:

```text
Python 3.13+
uv
pydantic
polars
pyarrow
duckdb
numpy
scipy
scikit-learn
statsmodels
optuna
jupyter
matplotlib
pytest
basedpyright
ruff
```

Responsibilities:

* generate experimental manifests;
* launch or communicate with Rust experiment runners;
* analyze Parquet outputs;
* estimate choice models;
* calibrate bounded-rationality parameters;
* calculate behavioral distances;
* perform sensitivity analyses;
* and create figures and reports.

Suggested layout:

```text
research/
  pyproject.toml
  src/game_research/
    io/
    metrics/
    calibration/
    diagnostics/
    visualization/
    experiments/
  notebooks/
  tests/
```

Use Pydantic models for manifests and external records, while Rust remains the final validator of game inputs.

### Calibration tools

Start with SciPy rather than introducing a probabilistic programming system immediately:

* `scipy.optimize` for bounded parameter fitting;
* `scipy.stats` for distributional comparisons;
* `optuna` for expensive nonconvex tuning;
* `statsmodels` or scikit-learn for interpretable behavioral models.

Add PyMC, Stan, or JAX only when hierarchical Bayesian estimation or differentiable simulation becomes a concrete need.

## 10. Calling Rust from Python

Prefer process/protocol boundaries initially:

```text
Python
  -> writes experiment manifest
  -> invokes Rust runner
  -> reads JSON status and Parquet results
```

This is more reproducible and less coupled than PyO3 during early development.

Consider PyO3 only after profiling shows that process startup or serialization materially limits calibration work. A Python extension can then expose a narrow batch API:

```python
results = simulator.run_scenarios(
  scenario_batch,
  parameter_batch,
  seed_bundle,
)
```

Do not expose mutable in-process Rust sessions broadly to notebooks. That would make experiment provenance harder to track.

## 11. Optional GUI: Axum plus TypeScript

When evidence supports a GUI, use:

* Rust `axum` loopback host;
* TypeScript;
* Vite;
* a lightweight component layer such as Svelte;
* SVG or Canvas for map and timeline visualization.

Axum provides modular routing and integrates with the Tower middleware ecosystem while remaining a relatively thin layer over Hyper. ([Docs.rs][5])

Suggested boundary:

```text
Rust host
  -> /api/v1/session/*
  -> actor-visible JSON DTOs
  -> TypeScript client
```

I would choose between two frontend paths:

### Minimal and conservative

```text
TypeScript + native web components + SVG + CSS
```

Best when the GUI remains small and host-driven.

### Richer interaction

```text
TypeScript + Svelte + SVG
```

Best when you need:

* map layers;
* replay timelines;
* contingency editors;
* linked causal views;
* and complex accessible interaction state.

Do not start with:

* Unity;
* Godot;
* Bevy;
* Electron;
* or a browser-only simulation.

Those tools are reasonable for a visually intensive game, but your initial product is primarily a strategic command, observation, and debrief system. A game engine would add a second architecture before the strategic loop is validated.

## 12. Testing stack

Use several layers.

### Rust unit and integration testing

```text
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

Add:

* `proptest` for invariants;
* `insta` for actor-visible report snapshots;
* `trycmd` or `snapbox` for CLI transcript tests;
* `cargo-nextest` for faster CI test execution;
* `cargo-deny` for license and dependency policy;
* `cargo-audit` for known advisories.

### Determinism tests

Test that:

```text
same state
+ same commands
+ same resolved inputs
+ same ruleset
= same events, effects, state, and hash
```

Also test random-stream isolation:

> Adding an unrelated observation should not change future combat execution draws.

### Contract tests

Maintain fixtures for:

* CLI commands;
* MCP schemas;
* scenario files;
* histories;
* save files;
* and analytical export schemas.

### Behavioral tests

Do not assert only exact actions. Test expected distributions and directional effects:

```text
higher loss aversion
  -> lower risky-contest rate, all else held constant

higher caller trust
  -> higher call-follow probability

greater candidate breadth
  -> weakly greater action-set diversity
```

Where monotonicity is not expected, document the interaction and test a bounded qualitative pattern.

## 13. Observability and provenance

Use two distinct systems.

### Runtime diagnostics

Use `tracing` for:

* request IDs;
* duration;
* errors;
* retries;
* agent timeout;
* and resource use.

These logs are operational and not part of game history.

### Committed simulation history

Use typed domain records for:

* observations;
* messages;
* commands;
* resolved inputs;
* events;
* effects;
* and hashes.

Never reconstruct authoritative history from runtime logs.

For AI agents, preserve:

```text
provider
model identifier
prompt-bundle version
tool-schema version
sampling settings
context policy
response/tool call
validation repairs
latency
```

Avoid storing hidden chain-of-thought. Store decisions, structured rationales when explicitly requested, and observable interaction records.

## 14. CI and release tooling

Use GitHub Actions with:

```text
Rust:
  fmt
  clippy
  test
  deterministic fixtures
  replay compatibility
  cargo deny
  cargo audit

Python:
  uv sync --locked
  ruff check
  basedpyright
  pytest

Contracts:
  schema compatibility
  scenario validation
  MCP fixture checks
  documentation links/currentness
```

Pin:

* `Cargo.lock`;
* `uv.lock`;
* experiment manifests;
* model configuration;
* prompt bundles;
* and ruleset versions.

Use Nix only if cross-platform environment drift becomes a demonstrated problem. `rust-toolchain.toml`, Cargo, and `uv` are likely sufficient initially.

## 15. What I would deliberately avoid

### Avoid Python as the authoritative engine

It would speed up early experimentation but weaken type-level state distinctions, replay discipline, and long-term interface consistency.

### Avoid an ECS architecture initially

An entity-component system may eventually help with a large map and many effects, but it can obscure causal domain logic in a small strategic model.

### Avoid LLM orchestration frameworks in the core

Do not make LangChain, LangGraph, or a provider-specific agent framework part of simulation authority. Invoke models through adapters behind your own agent trait and MCP contracts.

### Avoid PostgreSQL and cloud infrastructure

The game and research environment should remain local-first until multi-user hosting becomes a real requirement.

### Avoid premature microservices

One Rust workspace and local processes are enough. Reproducibility is easier when one authoritative host owns the session.

### Avoid immediate GUI-first development

The CLI and MCP surfaces will expose whether your actual observation and decision contracts work. A GUI can otherwise conceal weak mechanics.

## Concrete stack summary

| Boundary                | Recommended technology                   |
| ----------------------- | ---------------------------------------- |
| Authoritative engine    | Rust 2024                                |
| Workspace/build         | Cargo workspace                          |
| Serialization           | Serde + JSON                             |
| CLI process commands    | Clap                                     |
| Interactive CLI         | Reedline or small custom shell           |
| Errors                  | Thiserror in core, Anyhow in binaries    |
| Randomness              | `rand` + `rand_chacha`, explicit streams |
| Hashing                 | BLAKE3                                   |
| Async adapters          | Tokio                                    |
| MCP                     | Official Rust `rmcp` SDK                 |
| Optional HTTP/GUI host  | Axum                                     |
| Operational persistence | JSON + JSONL + filesystem                |
| Metadata catalog        | SQLite, when needed                      |
| Analytical storage      | Apache Parquet                           |
| Local analytics         | DuckDB                                   |
| Research/calibration    | Python + uv                              |
| Python data layer       | Polars/PyArrow/DuckDB                    |
| Statistical fitting     | SciPy, statsmodels, scikit-learn, Optuna |
| Python validation       | Pydantic                                 |
| Optional GUI            | TypeScript + Vite + Svelte/SVG           |
| Rust testing            | Cargo test, Proptest, Insta, Trycmd      |
| Python testing          | Pytest + BasedPyright + Ruff             |
| CI                      | GitHub Actions                           |

## Bottom line

I would build this as **one authoritative Rust product with two secondary ecosystems**:

```text
Rust simulation platform
  ├── CLI for humans
  ├── MCP for AI agents
  ├── JSONL replay artifacts
  ├── Parquet analytical exports
  ├── Python research and calibration
  └── optional TypeScript GUI
```

This stack preserves the design identity you established in the Health Policy Strategy Game while adding the pieces this project uniquely needs: scalable multi-agent experiments, semantic-agent calibration, top-$k$ bounded behavior, and behavioral-data analysis.

[1]: https://doc.rust-lang.org/stable/edition-guide/rust-2024/cargo-resolver.html?utm_source=chatgpt.com "Cargo: Rust-version aware resolver - The Rust Edition Guide"
[2]: https://serde.rs/?utm_source=chatgpt.com "Overview · Serde"
[3]: https://docs.rs/crate/clap/latest/source/README.md?utm_source=chatgpt.com "clap 4.6.4 - Docs.rs"
[4]: https://github.com/modelcontextprotocol/rust-sdk?utm_source=chatgpt.com "GitHub - modelcontextprotocol/rust-sdk: The official Rust SDK for the Model Context Protocol · GitHub"
[5]: https://docs.rs/axum/latest/axum/?search=rs&utm_source=chatgpt.com "axum - Rust"
[6]: https://arrow.apache.org/rust/parquet/index.html?utm_source=chatgpt.com "parquet - Rust"
[7]: https://duckdb.org/docs/stable/data/parquet/overview?utm_source=chatgpt.com "Reading and Writing Parquet Files – DuckDB"

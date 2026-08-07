# Simulation Design — M3 CLI Top-Level Process Commands

## Goal and Boundary

Define pure, typed, dependency-free contracts for top-level CLI commands, interaction modes,
verbosity policies, and privileged context guards without executing terminal I/O, persistence,
or simulation transitions.

## Data Structures and Contracts

### 1. Interaction Modes and Verbosity

- `CliInteractionMode`:
  - `Guided`: Numbered choices and explicit explanatory context for human decision-making.
  - `Expert`: Direct, concise, scriptable command strings.
- `CliVerbosity`:
  - `Concise`: Minimal output, essential alerts, and primary outcome facts.
  - `Standard`: Default operational output for standard interactive play.
  - `Explanatory`: Detailed decision rationale, attribution steps, and debrief narratives.
  - `Research`: Full privileged telemetry including unredacted causal traces and internal metrics.
- `CliPrivilegeLevel`:
  - `Unprivileged`: Standard player boundary; latent true state and host private hashes remain redacted.
  - `Privileged`: Explicit research context; allows inspecting raw traces and unredacted exports.

### 2. Top-Level Commands

- `CliTopLevelCommand<'a>`:
  - `Play { scenario: Option<&'a str>, mode: CliInteractionMode, verbosity: CliVerbosity, seed: Option<u64> }`
  - `Replay { run_id: &'a str, verbosity: CliVerbosity, privileged: bool }`
  - `Branch { point_id: &'a str, mode: CliInteractionMode, regenerated: bool }`
  - `Experiment { manifest_path: &'a str }`
  - `Export { run_id: &'a str, format: &'a str, unredacted: bool }`
  - `ValidateScenario { scenario_path: &'a str }`
  - `ValidateReplay { replay_path: &'a str }`
  - `McpServe { transport: &'a str }`
  - `Help { command: Option<&'a str> }`
  - `Version`

### 3. Top-Level Request Mapping and Validation

- `CliTopLevelRequest<'a>` typed request enum enforcing non-empty identifiers and privilege checks:
  - `Research` verbosity and `unredacted: true` require `CliPrivilegeLevel::Privileged`; attempting them under `Unprivileged` fails closed with `CliTopLevelError::PrivilegedContextRequired`.
  - Non-empty payload invariants for `run_id`, `point_id`, `manifest_path`, `scenario_path`, `replay_path`, etc.
- `parse_top_level_command(args: &[&str]) -> Result<CliTopLevelCommand<'_>, CliTopLevelParseError<'_>>`
- `top_level_request(command: CliTopLevelCommand<'a>, privilege: CliPrivilegeLevel) -> Result<CliTopLevelRequest<'a>, CliTopLevelError<'a>>`

### 4. Top-Level Help Catalog

- `CliTopLevelHelpEntry` and `CliTopLevelHelpCatalog` documenting usage, summary, required privilege, and flags.

## Verification Contract

- Parsing supports positional arguments and standard flags (`--mode`, `--verbosity`, `--seed`, `--privileged`, `--regenerated`, `--format`, `--unredacted`, `--transport`).
- Unknown verbs, unexpected options, and missing required parameters fail closed.
- Research verbosity and unredacted exports fail closed if privilege level is `Unprivileged`.
- All types are pure, `no_std`-compatible (or core Rust), borrow input strings, and have zero external dependencies.

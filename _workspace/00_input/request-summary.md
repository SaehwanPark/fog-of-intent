# Substantial Task Summary: M9 Interactive 5v5 Multi-Lane Tactical Match CLI Runner

## Requested Outcome
Implement an interactive 5v5 multi-lane tactical match CLI session runner and host, expanding beyond the print-and-exit transcript replay into dynamic multi-turn tactical commands (`rotate`, `ward`, `contest`, `siege`, `evaluate`, `idle`, `advance`, `observe`, `debrief`).

## Roadmap Milestone
- Milestone: M9 — Bounded Multi-Lane Match Prototype / M3 CLI Reference Experience
- Developer Action Items:
  - Implement interactive 5v5 multi-lane CLI session runner (expanding beyond print-and-exit transcript replay).
  - Support dynamic multi-turn tactical commands (`rotate`, `ward`, `contest`, `siege`) in the CLI.

## Scope
1. `CliMatchHost` in `src/host/match_host.rs`: authoritative synchronous host managing complete 5v5 match state, action staging, commitment, turn advancement, event/effect tracking, and victory debrief.
2. Tactical match command grammar (`observe`, `plan rotate`, `plan ward`, `plan contest`, `plan siege`, `plan evaluate`, `plan idle`, `commit`, `advance`, `debrief`, `undo`, `help`, `quit`).
3. Pure actor-safe terminal and presented text formatters in `src/terminal.rs` and `src/presentation.rs`.
4. Scenario catalog registration for `m9-interactive-match-v1` (`ScenarioExecutionMode::InteractiveMatch`).
5. Process CLI parsing and interactive scenario selection integration in `src/main.rs`, `src/command_loop.rs`, and `src/repl.rs`.
6. Unit, integration, and AI playtest verification (`foi-test-player`).

## Non-Goals
- No floating-point math, async runtimes, or network I/O in the simulation core.
- No latent opponent hidden state leakage in actor-visible observations.
- No GUI changes required in this slice.

# M5 Actor-Commit Boundary Handoff

## Outcome

`ActorCommitDto` and `ActorCommitResultDto` now define exact observation-bound
actor commit command/acknowledgement contracts. `CliScenarioHost::commit_actor_draft`
checks lifecycle/freshness and staged-plan consistency, clears uncommitted
metadata, and sets a committed intent without advancing, resolving, appending
history, or refreshing the observation.

## Changed Files

- `src/protocol.rs`: commit command/result DTOs and exact bounded codecs.
- `src/host.rs`: host-owned pre-transition commit boundary and focused tests.
- `Cargo.toml`, `Cargo.lock`, `README.md`, `ROADMAP.md`, `SPEC.md`,
  `ARCHITECTURE.md`, `CHANGELOG.md`, `LESSONS.md`.
- `_workspace/00_input/m5-actor-commit-boundary-request-summary.md`,
  `_workspace/01_simulation-design-m5-actor-commit-boundary.md`, and
  `_workspace/03_domain-qa-m5-actor-commit-boundary.md`.

## Verification

Pending final full-gate snapshot: expected current evidence is 16 focused
protocol, 5 session, and 21 host tests within 198 Rust unit tests, 7 binary
integration tests, and 1 RustDoc test, plus formatter, Clippy with warnings
denied, repository checker, 14 Python checks, and diff check.

## Domain QA Disposition

Pending the required independent three-pass review at PR handoff.

## Limits and Next Dependencies

Commit is a synchronous pre-transition boundary, not transport delivery,
simultaneous ordering, persistence, reconnect, richer plan semantics, or
complete MCP behavior. Host/lane legality, transition, execution, history, and
replay authority remain unchanged.

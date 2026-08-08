# M4 Scripted-Agent Baseline Handoff

## Delivered

- Versioned `m4-scripted-agent-v1` policy and `cautious-laner-v1` profile.
- Actor-visible candidate generation, fixed candidate evaluation, and stable
  selection with inspectable rule identities.
- Observer-bound `LaneIntentRequest` output for host-side legality validation.
- Focused initial-observation, visible-threat, validation, and reproducibility
  tests.
- Synchronized canonical M4 status, QA/design artifacts, changelog, and
  `LESSONS.md` claim limits.

## Verification target

157 Rust unit tests, seven binary integration tests, one compile-fail RustDoc
test, pinned formatter, Clippy with warnings denied, repository checker, 14
Python checks, and diff checks must pass before handoff.

## Open boundaries

Broader scripted populations, role heuristics, memory, communication,
coordination, random streams, matched-scenario metrics, external agent/MCP
adapters, strategic-quality evaluation, and human behavioral realism remain
open. The host still owns legality, transition, execution, replay, and history.

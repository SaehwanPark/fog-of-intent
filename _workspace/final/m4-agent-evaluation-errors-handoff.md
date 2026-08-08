# M4 Agent Evaluation-Error Handoff

## Delivered

- Bounded `ScriptedAgentEvaluationError::UnavailableIntent` for public scoring
  outside the actor-visible candidate set.
- Dedicated initial-state rejection regression while preserving deterministic
  profile selection and host/lane validation.
- Synchronized M4 evaluation-error design, QA, changelog, and lesson evidence.

## Verification target

Five focused agent tests, 159 Rust unit tests, seven binary integration tests,
one compile-fail RustDoc test, pinned formatter, Clippy with warnings denied,
repository checker, 14 Python checks, and diff checks must pass.

## Open boundaries

Scenario outcomes, broader legality/error matrices, role populations, memory,
communication, random streams, metrics, external agent/MCP adapters, strategic
quality, and human behavioral realism remain open. Host legality, transition,
execution, replay, and history authority are unchanged.

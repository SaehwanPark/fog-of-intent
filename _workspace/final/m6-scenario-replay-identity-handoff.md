# M6 Scenario Replay Identity Handoff

## Summary

This slice implements `ScriptedAgentScenarioReplayIdentityReport` under schema
`m6-scripted-agent-scenario-replay-identity-v1` and rule
`m6-scenario-replay-identity-v1`. It verifies deterministic replay across a
sequence of one to sixteen caller-supplied `ScriptedAgentReplayRecord`s from a
sampled scenario run.

## Changes

- `src/agent.rs`: Added constants, `ScriptedAgentScenarioReplayIdentityStatus`,
  `ScriptedAgentScenarioReplayIdentityError`,
  `ScriptedAgentScenarioReplayIdentityReport`, and focused regression tests.
- `Cargo.toml` & `Cargo.lock`: Bumped version from `0.1.168` to `0.1.169`.
- Project state documents: Updated `SPEC.md`, `README.md`, `ROADMAP.md`,
  `CHANGELOG.md`, and `ARCHITECTURE.md`.

## Evidence Limits

This report provides pure library-side sequence replay verification. Causal-trace
completeness, runtime automated log production, durable persistence, provider
integration, and human gameplay claims remain explicitly open.

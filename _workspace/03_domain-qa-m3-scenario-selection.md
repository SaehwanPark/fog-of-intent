# Domain QA — M3 Scenario Selection

## Scope

Review the executable scenario-selection boundary for fail-closed identifiers,
deterministic fixture construction, process status, option composition, and
preservation of the host/kernel authority and actor-visible contracts.

## Required checks

- Verify omitted and explicit `m3-two-window-fixture-v1` selection construct the
  same bounded fixture.
- Verify missing, empty, option-shaped, and unknown scenario values fail before
  stdin reaches the session grammar and return a non-success status.
- Verify `--scenario` composes with `--run-dir` in both orders without changing
  the existing two-process save/load behavior.
- Verify help names the supported ID and does not execute simulation state.
- Verify no raw scenario argument, path, or unsupported ID is echoed to actors,
  and no scenario-selection logic enters the host, kernel, or lane.

## Claim limit

This slice proves one application-edge versioned fixture ID and its bounded
failure behavior. It does not prove a scenario catalog, external scenario
loading, arbitrary execution inputs, complete playable behavior, regenerated or
graph branching, branch persistence, or accessibility. Focused evidence is
three scenario-selection unit tests and five binary integration tests; the full
suite has 153 Rust unit tests, five binary integration tests, and one
compile-fail RustDoc test.

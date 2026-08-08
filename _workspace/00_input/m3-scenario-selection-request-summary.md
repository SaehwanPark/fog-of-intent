# M3 Scenario Selection Request Summary

## Requested slice

Expose one explicit, versioned fixture identifier at the executable edge so a
caller can name the bounded two-window reference fixture without changing the
session grammar or simulation authority.

## In scope

- Accept `--scenario m3-two-window-fixture-v1` in the process-argument helper.
- Preserve the existing two-window fixture when `--scenario` is omitted.
- Reject missing, empty, option-shaped, and unknown scenario values with
  bounded path-free errors and a non-success executable status.
- Preserve `--run-dir <path>` behavior and support either option order.
- Construct the existing fixture loop only after the process edge has selected
  the supported scenario enum.
- Add parser and executable regressions for the default, supported, missing,
  and unsupported forms, then synchronize canonical and evidence documents.

## Out of scope

- Multiple scenarios, a scenario catalog, scenario files, arbitrary scenario
  configuration, or user-provided execution inputs.
- Changes to lane transitions, host authority, command grammar, artifacts,
  replay identity, branch execution, persistence semantics, or actor-visible
  projections.
- Complete playable-scenario behavior, regenerated/graph branching, or
  keyboard/screen-reader inspection.

## Success evidence

- The parser exposes one versioned scenario enum and fails closed for malformed
  or unsupported IDs without echoing input values.
- The binary accepts the supported ID, retains the no-option in-memory default,
  and preserves the existing run-directory save/load smoke path.
- The full repository checks and one code-reviewer pass confirm that selection
  remains an application-edge construction concern.

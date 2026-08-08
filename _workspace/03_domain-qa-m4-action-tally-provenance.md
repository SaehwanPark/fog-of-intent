# Domain QA — M4 Action-Tally Provenance

## Status

Pass for the bounded observation-ID provenance refinement.

## Reviewed inputs

- `ScriptedAgentActionTallyReport` ID retention and duplicate rejection.
- Safe/RiverSide observations, shared observer validation, and existing policy
  request checks.
- Canonical/workspace docs and lesson updates.

## Findings

- IDs 14 and 15 are retained in input order without exposing hashes or true
  state.
- Duplicate IDs fail before any profile selection; mixed observers remain
  rejected.
- The two-observation tally counts and all six host validations remain intact.

## Claim limits

This is fixture-level visible-input provenance only. It does not establish
scenario provenance, replay authority, population sampling, outcomes,
strategic quality, or human realism.

## Verification evidence

The focused agent suite remains eleven tests. The full repository target is
165 Rust unit tests, seven binary integration tests, and one compile-fail
RustDoc test.

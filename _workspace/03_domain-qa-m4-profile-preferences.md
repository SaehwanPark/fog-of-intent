# Domain QA — M4 Profile Preferences

## Status

Pass for the bounded baseline-preference metadata slice.

## Reviewed inputs

- `ScriptedAgentProfile::preferred_intent()` and constructor assertions.
- Existing visible-threat selection and host validation tests.
- Canonical/workspace docs, changelog, and LESSONS.md.

## Findings

- Each profile exposes the intended baseline intent without reading hidden
  state or changing candidate generation.
- The visible RiverSide `Withdraw` response remains selected through the
  existing policy path and does not rewrite the baseline metadata.
- No legality, transition, execution, history, or replay authority moves into
  the profile accessor.

## Claim limits

This proves three fixed preference labels for the fixture. It does not prove a
complete risk/personality model, planning, memory, communication, outcomes,
strategic quality, or human realism.

## Verification evidence

The focused agent suite remains eleven tests. The full repository target is
165 Rust unit tests, seven binary integration tests, and one compile-fail
RustDoc test.

# Domain QA — M2 Causal and Information Evidence

## Status

`pass` after narrowing the provenance claim to the verified current contract.

## Reviewed Inputs

- `_workspace/00_input/m2-causal-information-evidence-request-summary.md`
- `_workspace/01_simulation-design-m2-causal-information-evidence.md`
- `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`
- lane transition/projection/objective/scenario/observation/history tests

## Findings

- Effect relation/timing and current-resolution trace remain explicit and
  replayable; queued delayed effects still lack origin-trace linkage and remain
  an unchecked implementation item.
- Lane outcome/objective review is not collapsed into a binary win/loss claim.
- Actor-visible reports are redacted and complete for the bounded roster/report
  contract; hidden-state equality and receipt privacy are tested.
- The two-window replay/debrief path was inspected in source and fixtures for
  ordering and committed-facts attribution.

## Required Fixes

None.

## Residual Risks

Origin-trace linkage for queued delayed effects, vision/belief updates,
variable-duration automatic advance, communication transport, and a playable
scenario remain incomplete. No human-experience or behavioral-validity claim
follows from these tests.

## Verification Evidence

The locked Rust, clippy, formatting, repository-currentness, checker-test, and
diff checks pass for the reconciled state.

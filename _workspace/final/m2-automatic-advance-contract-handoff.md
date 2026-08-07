# M2 Automatic-Advance Contract Handoff

## Outcome

Added a typed deterministic contract for commit-required and no-legal-intent
advance conditions. Current one- and two-beat windows remain commit-required;
no automatic execution path was introduced.

## Changed Files

`LaneWindow` advance-condition values, focused tests, package metadata, core
project documents, and the inspectable design/QA artifacts for this slice.

## Verification and QA

Full locked Rust/repository checks pass. Domain QA status is `pass`; see
`_workspace/03-domain-qa-m2-automatic-advance-contract.md`.

## Limits and Next Slice

This defines the condition contract only. Host integration for a genuine
no-choice automatic path and the remaining vision/belief model remain open.

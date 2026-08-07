# M2 Causal and Information Evidence Handoff

## Outcome

Promoted three evidence-backed M2 checklist items: non-binary terminal outcome,
hidden-state/report completeness tests, and complete-replay inspection. Effect
provenance remains partial because queued delayed effects do not retain an
originating trace.

## Changes

Updated `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, and `CHANGELOG.md` only;
runtime code and package version are unchanged.

## Verification and QA

Full locked Rust/repository checks pass. Domain QA status is `pass`; see
`_workspace/03-domain-qa-m2-causal-information-evidence.md`.

## Limits and Next Slice

Origin-trace linkage for delayed effects, vision/belief updates, and
variable-duration automatic advance remain unchecked; this evidence does not
promote M2 or establish a playable scenario.

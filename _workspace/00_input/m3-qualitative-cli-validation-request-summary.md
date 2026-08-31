# M3 Qualitative CLI Validation Request Summary

## Requested outcome

Exercise the canonical Fog of Intent reference CLI from actor-visible inputs,
capture reproducible functional and interaction findings, and make only
evidence-backed improvements that preserve the host authority boundary.

## Current milestone

M3 — CLI Reference Experience. Implementation is complete; qualitative
decision-loop and interaction validation is still active.

## Slice boundary

- Run the three M2 lane strategy playthroughs and the M9 interactive match
  through their public command loops.
- Check command discoverability, draft/commit/advance lifecycle, error repair,
  actor-safe observations, replay/debrief output, and terminal text wrapping.
- Use bounded virtual player personas for functional verification and gameplay
  feedback; label all findings as agent evidence.
- Fix only reproducible defects or clear friction in the existing CLI surface.

## Non-goals

- No new simulation mechanics, provider integration, GUI deployment, or
  persistence authority.
- No claim of human enjoyment, accessibility, trust, or research validity.
- Do not mark human keyboard/screen-reader inspection complete without people.

## Source and artifact targets

- `src/command_loop.rs`, `src/host/`, `src/terminal.rs`, `src/presentation.rs`,
  `src/repl.rs`, and focused tests when a defect is reproduced.
- `_workspace/01_simulation-design-m3-qualitative-cli-validation.md`
- `_workspace/01_agent-ecology-design-m3-qualitative-cli-validation.md`
- `_workspace/04_playtest-report-m3-qualitative-cli-validation.md`
- `_workspace/03-domain-qa-m3-qualitative-cli-validation.md`
- `_workspace/final/m3-qualitative-cli-validation-handoff.md`

## Validation

Run the exact pinned fmt, Clippy, locked test, and repository checks, plus
reproducible CLI transcripts for success, malformed input, stale lifecycle
actions, hidden-state redaction, replay, and debrief paths.

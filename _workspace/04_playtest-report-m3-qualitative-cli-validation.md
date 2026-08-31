# M3 Qualitative CLI Validation Playtest Report

## Scope and evidence boundary

This report covers the bounded M3 reference-client validation slice on
2026-08-30. Anchor/Cautious, Duelist/Aggressive, Novice/Explorer, and runner
agents exercised the three M2 strategy fixtures and the M9 interactive match
through the public CLI. Their traces are software/protocol evidence only; they
do not establish human enjoyment, accessibility, trust, or research validity.

The source branch was `codex/m3-cli-playtest-validation-20260830` at the
working tree containing the slice fixes. All commands used Rust `1.96.0`,
`--color never`, and fixed scenario IDs. No private state, hashes, or model
receipts were supplied to the agents.

## Baseline playthroughs

- `m2-strategy-happy-path-v1`, `m2-strategy-risk-taking-v1`, and
  `m2-strategy-conservative-v1` each completed the documented observe →
  plan/commit/advance ×2 → replay/debrief → quit lifecycle. Their declared
  outcomes remained distinct (`held_space` versus `yielded_space`) and replay
  verification passed.
- `m9-interactive-match-v1` completed the canonical rotation, ward, idle,
  Dragon contest, Mid siege, Nexus evaluation, debrief, and quit transcript.
  The Allied result remained `nexus-demolished` at the deterministic terminal
  turn, with the expected event/effect totals.
- Scenario selection accepted the full 16-entry catalog, numeric selection,
  aliases, blank default, and `q` cancellation. Width-40 output remained
  plain text and within the configured bound.

## Reproduced defects and fixes

### Opponent-location redaction

Before the fix, initial M9 `observe` printed the authoritative opposing
location `lane:mid:far-side`. The host now consumes `MatchMapState::observe`
and carries certainty through `MatchActorLocation`:

```text
actor: id=4 team=opposing location=unknown
```

Allied locations remain observed. Opponents can be represented as observed,
last-known, or payload-free unknown; the terminal and MCP projections use the
same actor-safe report.

### Completion-gated debrief

Before terminal evaluation, `debrief` now fails closed:

```text
error: match debrief is unavailable until terminal evaluation
```

After the canonical evaluation action, the existing structured debrief still
renders the winner, condition, final turn, objectives, and phase totals.

### Siege target clarity and staged-action lifecycle

The staged description now names the target team and attacker, matching the
structure transition:

```text
draft: status=staged action=siege Opposing OuterTurret on Mid for 4000 damage (attacker=Allied)
```

Attempting to stage a second action before commit or undo now returns a bounded
repair error instead of silently replacing the first action.

## Verification evidence

Focused host and terminal regressions cover redaction, target descriptions,
staged replacement, and premature debriefs. The executable M9 transcript adds
assertions for the redacted opponent and completion-gated lifecycle. The full
locked test suite passed after the fixes: 186 library tests, 2 MCP-binary
tests, 48 integration tests (46 binary-runner and 2 agent-batch), and 1
documentation test.

## Residual findings

- The M9 fixture still carries a four-actor roster (three Allied and one
  Opposing) despite its 5v5 label. Expanding or renaming that fixture is a
  separate M9 scope decision, not a safe M3 presentation-only change.
- M9 focused help topics are terse, and compact-width nested indentation can
  be improved. These are follow-up interaction-polish candidates.
- Human keyboard-only, focus, screen-reader, enjoyment, trust, M10 cohort,
  M11 live-browser, and M12 release-candidate evidence remain open roadmap
  gates.

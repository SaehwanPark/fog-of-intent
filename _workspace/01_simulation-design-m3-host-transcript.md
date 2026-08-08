# M3 Host-Backed Transcript Design

## Decision

Introduce a small `CliScenarioHost` at the application edge. It owns the
scenario history, staged plan text, committed intent, in-memory saved snapshot,
and closed-session state. It delegates validation and transition evaluation to
the existing lane contract and returns only actor-valid projections.

The host accepts a pair of already-resolved `LaneResolvedInputs` values. This
keeps deterministic execution inputs explicit and preserves the rule that the
kernel evaluates supplied inputs rather than creating randomness.

## Transcript contract

The fixture transcript may observe, stage message/plan/contingency text, commit,
advance each of two windows, save and load a run, verify replay, and build the
committed debrief. Plan text is limited to the existing bounded `LaneIntent`
names; message and contingency text remain staged metadata and are not silently
translated into simulation state.

`CliHostOutput` exposes observations, history counts, window outcomes, replay
verification, and the redacted debrief report. It never returns a
`LaneSnapshot`, state hash, hidden opponent truth, or transition record to an
actor-facing caller.

## Explicit limits

This is a deterministic in-memory host fixture, not persistence or a terminal
renderer. Branch execution, terminal output, human keyboard flow, and
screen-reader structure remain open. The M3 complete-run checkbox therefore
stays unchecked until those boundaries have evidence.

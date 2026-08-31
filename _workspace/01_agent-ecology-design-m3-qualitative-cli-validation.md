# M3 Qualitative CLI Validation Agent Ecology Design

## Goal and Roadmap Milestone

Use bounded virtual players to probe the M3 reference CLI and identify
reproducible interaction friction. This is agent evidence for software and
policy behavior, not human-ground-truth evidence.

## Behavioral Question and Evidence Boundary

Can distinct actor-visible strategies complete the lane and match loops while
recovering from common command errors, and do their transcripts expose any
confusing or inconsistent presentation? We can establish protocol usability,
legality, redaction, and strategy-path differences; we cannot establish human
enjoyment, accessibility, or trust.

## Agent Families and Baselines

- Anchor/Cautious: observe first, stabilize or defensive rotate, avoid risky
  contests, and verify the debrief.
- Duelist/Aggressive: seek contest/siege opportunities and inspect outcomes.
- Novice/Explorer: use help, intentionally submit one malformed command, then
  recover using the advertised repair hint.

Each persona runs the same canonical scenario set where the command surface
allows it. No persona receives true state or private receipts.

## Observation, Memory, and Policy Inputs

Inputs are the current rendered observation, help text, prior public transcript,
and actor-safe categorical outputs. Memory is limited to the current session;
the policy never queries host internals.

## Candidate Generation, Evaluation, and Selection

Candidates come from advertised legal intents/actions. Selection is a fixed
persona rule, not random noise: cautious prioritizes safety, duelist pressure,
and novice repair/discoverability. Invalid probes are explicit test cases and
must not reach transition evaluation.

## Communication, Trust, and Team Coordination

Lane pings/messages and match tactical plans are treated as public commands
only. This slice evaluates their labels and lifecycle handling, not new
transport or trust mechanics.

## Randomness and Reproducibility

Use fixed scenario IDs and existing resolved-input fixtures. Preserve command
order and transcript text so a finding can be rerun exactly.

## Scenarios, Populations, and Metrics

Run the three M2 strategy fixtures and the M9 interactive match. Record command
legality/recovery, lifecycle completion, redaction violations, replay/debrief
availability, output wrapping, and persona-specific path differences.

## Calibration or Regression Protocol

Compare repeated runs of the same transcript for byte-stable labeled output.
Treat any mismatch, hidden-state leak, unrecoverable advertised command, or
contradictory lifecycle label as a defect candidate requiring a focused test.

## Expected Effects and Failure Signals

Expected: all legal persona paths complete or terminate with documented
scenario limits; malformed commands fail closed; no latent hashes or opponent
truth appear. Failure signals become concrete issue entries in the playtest
report and are fixed only if reproducible.

## Verification Contract

The host remains authoritative and synchronous. Agent traces are exploratory
evidence and must be paired with compiler/tests and domain QA.

## Open Questions

Human-oriented keyboard/focus/screen-reader inspection and empirical experience
claims remain deferred.

# M6 Stress-Population Evidence Design

## Goal and Roadmap Milestone

Advance M6's adversarial/degenerate population boundary with a closed,
deterministic case matrix over the existing scripted actor and adapter
contracts. This is evidence plumbing, not a behavioral population simulator.

## Behavioral Question and Evidence Boundary

The question is whether each named stress case reaches the expected existing
boundary on a matched actor-visible fixture: illegal intent is rejected by the
host actor-action validator, stale provenance is rejected by host freshness,
oversized/control-containing communication is rejected by the bounded message
codec, and a degenerate policy retains one repeated legal intent. The result
proves only these deterministic categorical outcomes and selected count; it
does not measure exploit rates, communication harm, strategic quality,
population diversity, or human behavior.

## Agent Families and Baselines

Use the existing cautious scripted profile as the baseline. The degenerate case
is a caller-declared repeated-intent fixture over the same profile input; the
other three cases are boundary probes, not new agent families. No heuristic,
parametric, LLM, or provider agent is introduced.

## Observation, Memory, and Policy Inputs

Each case uses the existing actor-visible `LanerObservation` and explicit
observation/actor IDs. No memory, true state, resolved input, history hash, or
wall-clock value is admitted. Communication-abuse input is bounded text passed
through the existing `ActorMessageDto` constructor/codec.

## Candidate Generation, Evaluation, and Selection

The baseline and degenerate case reuse the existing advertised candidate set
and deterministic selection. Illegal-command and exploit-seeking cases use
explicitly constructed invalid requests only to prove host rejection; they do
not modify candidate generation or selection authority.

## Communication, Trust, and Team Coordination

No communication or trust policy is added. The communication-abuse case only
proves the existing bounded actor-message rejection and must not route or
deliver a message.

## Randomness and Reproducibility

No randomness is used. Repeated construction of the closed case catalog and
report must be equal; all stress inputs and expected result IDs are literals.

## Scenarios, Populations, and Metrics

The population is exactly four caller-declared case labels in stable order.
The report contains only schema, case ID, and closed result ID. Metrics are
case-count evidence and one degenerate selected-intent count, not prevalence,
outcome, distribution, or strategic metrics.

## Calibration or Regression Protocol

Bind literal schema/case/result IDs; test catalog order and repeatability; run
the illegal, exploit, communication, and degenerate fixtures through existing
validation/codec/policy paths; assert no hidden-state or host-authority drift.

## Expected Effects and Failure Signals

Expected results are `host_validation_rejected`, `stale_observation`,
`message_invalid_value`, and `repeated_stabilize`, in the fixed four-case order.
Any need for a new error vocabulary, transition, transport, or hidden input is a
stop condition rather than an implementation choice.

## Verification Contract

One focused agent test must bind all four literal case IDs and result IDs,
prove stable order/repeated construction, exercise each existing boundary,
assert the degenerate count, and prove the report is pure. Full Rust, RustDoc,
formatter, Clippy, repository, Python, and diff gates are required.

## Open Questions

Actual exploit search loops, larger adversarial populations, communication
semantics, runtime scheduling, outcome/causal metrics, representative replay,
provider integration, and human evidence remain open.

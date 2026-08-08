# M4 Agent Evaluation-Error Design

## Boundary

Public `ScriptedAgent::evaluate_candidate` first checks the copied
`LanerObservation` candidate set. An intent absent from the four advertised
intents and optional visible threat response returns
`ScriptedAgentEvaluationError::UnavailableIntent`; no request is constructed
and no host state is touched. `choose` uses its internally generated set and
therefore remains a total deterministic decision over the bounded fixture.

## Error distinction

The error means the policy caller supplied an unavailable candidate. It is not
a substitute for host freshness or legality validation, and it does not expose
true state, hashes, execution inputs, or domain failure details.

## Evidence and limits

The focused regression rejects initial-state `Withdraw`, which is not
advertised without a visible RiverSide threat. Existing profile, matched-input,
and host-validator tests remain the evidence for valid policy decisions. This
slice adds policy plumbing only; it does not prove scenario outcomes or agent
quality.

# M5 Actor Capability Matrix Design

## Contract

`ActorToolCapability` pairs each closed `ActorTranscriptTool` with an
`ActorToolAuthority` label:

- `ordinary_actor`: the only authority currently exposed by the actor catalog;
- `privileged_experiment_controller`: a closed label reserved for future
  separate tools and never returned by the current catalog.

The catalog covers observation, draft, draft receipt, commit, and action and
reuses each tool's existing schema ID. It is deterministic metadata only.

## Authority and Limits

Ordinary actor labels do not authorize legality, transitions, execution,
history, replay, persistence, or experiment mutation. The host/lane boundaries
remain unchanged. Privileged tools require a separate capability and
authorization contract before they can be advertised.

## Verification Contract

- The catalog has exactly five tools in stable order.
- Every entry has the literal expected tool/schema ID and `ordinary_actor`
  authority.
- The privileged authority label is closed but absent from the current catalog.

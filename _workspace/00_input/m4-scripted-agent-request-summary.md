# M4 Scripted-Agent Request Summary

## Requested slice

Add one deterministic scripted-agent policy for the existing actor-visible
lane fixture. The policy must generate and score legal candidates from a
`LanerObservation`, then return a request that the host can validate.

## In scope

- Add a versioned `m4-scripted-agent-v1` policy boundary and
  `cautious-laner-v1` profile metadata.
- Separate candidate generation, fixed evaluation, and stable selection.
- Prefer a visible threat response, otherwise prefer the stable default, then
  choose the highest fixed score in advertised order.
- Preserve the host as the sole legality, transition, execution, and history
  authority.
- Add focused tests for actor-visible candidates, host validation, threat
  prioritization, and repeated-observation reproducibility.
- Synchronize canonical M4 status, workspace design, QA, handoff, changelog,
  and reusable lessons.

## Out of scope

- Hidden-state access, transition or execution resolution, host mutation,
  memory, communication, role populations, randomness, metrics, MCP wiring,
  strategic-quality claims, or human behavioral realism.
- Completing the M2 scenario or promoting M4 from its broader evidence gate.

## Success evidence

- An identical actor-visible observation produces an identical decision and
  observer-bound request.
- A visible threat response outranks the stable default without exposing the
  underlying true state.
- The returned request passes the existing host/lane validation boundary.
- Repository and documentation checks describe this as one bounded library
  policy slice.

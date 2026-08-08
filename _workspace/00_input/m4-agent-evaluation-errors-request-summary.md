# M4 Agent Evaluation-Error Request Summary

## Requested slice

Make the scripted-agent evaluation boundary fail closed when a caller asks it
to score an intent that the actor-visible observation did not advertise.

## In scope

- Add `ScriptedAgentEvaluationError::UnavailableIntent`.
- Check the observation's advertised legal intents and visible threat response
  before public candidate scoring.
- Keep normal generation/selection deterministic and host validation unchanged.
- Add focused rejection evidence and synchronize M4/core docs, handoff, and
  `LESSONS.md`.

## Out of scope

- New legality rules, host transitions, execution inputs, memory, communication,
  randomness, population metrics, external adapters, or strategic claims.

## Success evidence

- An unadvertised initial-state intent returns the bounded policy error.
- Generated candidates still select the same profile intents and pass host
  validation.
- The policy error remains distinct from host/lane legality errors.

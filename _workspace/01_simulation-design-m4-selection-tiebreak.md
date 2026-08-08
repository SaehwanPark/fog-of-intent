# M4 Selection-Tiebreak Simulation Design

## Boundary

Selection is a pure policy step over already generated and scored
`ScriptedAgentCandidate` values. It does not validate legality, resolve
execution, mutate history, or call the transition kernel.

## Versioned contract

All three profiles bind `max-score-stable-order-v1`. The selector returns the
first candidate with the maximum score in advertised order: strictly higher
scores replace the current best, while equal scores do not.

## Evidence and limits

The regression uses an equal-score pair to prove first-advertised behavior and
asserts the literal rule ID for all profiles. This is deterministic top-1
selection evidence only; top-k/nucleus sampling, random streams, populations,
outcomes, and strategic quality remain deferred.

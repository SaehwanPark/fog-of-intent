# Simulation Design — M3 CLI Read Requests

## Goal and Boundary

Map grammar values to read-only adapter requests. The mapping is pure and does
not access lane state, authorize a domain command, or render output.

## Contract

`observe` maps to `CliReadRequest::Observe`; `help` maps to contextual metadata;
`inspect` with no target maps to actor-visible current state, while `inspect
state` and `inspect history` are the only explicit targets. Unknown targets and
non-read commands return typed `CliReadError` values. The static help catalog
lists canonical verbs and does not imply that their flows are implemented.

## Verification Contract

- Observe/current-state/history requests map deterministically.
- Unknown inspect targets and non-read commands fail closed.
- Help metadata lists every stable grammar verb without hidden-state fields.
- Mapping does not mutate state or invoke transition code.

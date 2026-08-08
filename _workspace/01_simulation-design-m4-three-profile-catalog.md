# M4 Three-Profile Catalog Design

## Boundary

The three profiles remain pure policy configurations over one copied
`LanerObservation`. Candidate generation, unavailable-intent errors, stable
selection, observer binding, and host validation are shared. No profile owns
legality, execution, transition, replay, or history.

## Profiles

| Profile | Evaluation rule | Initial selected intent |
| --- | --- | --- |
| `cautious-laner-v1` | `threat-first-fixed-score-v1` | `Stabilize` |
| `risk-taking-laner-v1` | `contest-first-fixed-score-v1` | `Contest` |
| `yielding-laner-v1` | `yield-first-fixed-score-v1` | `Yield` |

Each profile scores its named posture at 100. Cautious threat response remains
100; the risk-taking and yielding profiles score a visible threat response at
90 so the matched initial observation exposes their distinct fixed preference.

## Evidence and limits

The test compares all three profiles on one identical initial observation,
asserts shared candidates and rule identities, repeats each decision, and
validates every request. This is a profile-plumbing comparison, not a claim
about outcomes, balance, role realism, or human behavior.

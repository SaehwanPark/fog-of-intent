# M4 Matched-Profile Comparison Design

## Boundary

The comparison extends the pure `src/agent.rs` policy boundary only. Both
profiles consume the same copied `LanerObservation`; the host still owns
freshness, legality, transition, execution, history, and replay.

## Profiles

| Profile | Evaluation rule | Initial selected intent |
| --- | --- | --- |
| `cautious-laner-v1` | `threat-first-fixed-score-v1` | `Stabilize` |
| `risk-taking-laner-v1` | `contest-first-fixed-score-v1` | `Contest` |

Candidate generation and stable first-maximum selection are shared. The
risk-taking rule scores `Contest` at 100 and a visible threat response at 90;
the cautious rule scores a visible threat response at 100 and `Contest` at 60.
Both retain the same stable default and alternative scores.

## Evidence and limits

The matched initial observation proves that profile rule identity can change a
decision while information and host validation remain constant. It does not
prove that either preference improves outcomes, models people, or generalizes
to other scenarios. Population comparisons, metrics, memory, communication,
random streams, and external adapters remain deferred.

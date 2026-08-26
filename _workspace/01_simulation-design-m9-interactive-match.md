# Simulation Design: M9 Interactive 5v5 Multi-Lane Tactical Match CLI Runner

## Goal and Roadmap Milestone
- **Milestone**: M9 (Bounded Multi-Lane Match Prototype) & M3 (CLI Reference Experience)
- **Goal**: Enable players to interactively command a 5v5 multi-lane match from turn 1 through tactical rotations, ward placements, neutral objective battles, and structural sieges to match victory with full causal debrief.

## Slice Boundary and Non-Goals
- Bounded to 5v5 multi-lane tactical match mechanics: map locations, actor positioning, vision wards, river objectives, structure defense hierarchy, and victory evaluation.
- Excludes real-time twitch mechanics; preserves Fog of Intent's turn-based intent/commitment model.
- Excludes GUI rendering; terminal presentation uses clean actor-safe labeled plain text / optional ANSI.

## Actors and Authority
- Host owns authoritative `CompleteMatchState`, `MatchMapState`, `MatchStructureState`, `MatchObjectiveState`, and `MapVisionState`.
- Player controls team strategy through typed tactical commands.
- Transitions use existing deterministic M9 simulation functions: `transition_travel`, `transition_objective_contest`, `transition_structure_siege`, and `MatchTerminalEvaluation`.

## Plans, Commands, and Validation
- `observe` (or `status`, `map`): Displays actor-visible match state, vision coverage, structure integrity, and objective timers.
- `plan rotate <actor_id> <destination>`: Plan movement to a map sector (e.g. `top_river`, `mid_center`, `opposing_base`).
- `plan ward <team> <actor_id> <location> [duration]`: Plan ward placement.
- `plan contest <top|bot> <damage> [burst]`: Plan neutral objective engagement or secure burst.
- `plan siege <allied|opposing> <tier> [lane] <damage>`: Plan structural siege following defense hierarchy.
- `plan evaluate`: Evaluate terminal match conditions.
- `plan idle`: Skip tactical action for turn.
- `commit`: Lock staged plan into committed turn action.
- `advance`: Deterministically advance the match state by 1 turn using committed action.
- `debrief`: Project final match report with winner, condition, and phase log.
- `undo`: Clear uncommitted staged plan.
- `quit`: Exit session.

## Determinism and Replay Verification
- Authoritative state commits via combined FNV-1a hash across all subsystem states.
- Exact phase logs with turn numbers, action kinds, events, and effects.

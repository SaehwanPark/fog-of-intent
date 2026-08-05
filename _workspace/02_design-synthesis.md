# Design Synthesis — M2 Bounded Opponent Last-Known Report

## Decision

Use one fixed player-vision projection over the existing opponent truth: only
`FarSide` becomes a player-facing `LastKnown` position report with the current
observation turn. Center and NearTower remain Unknown. The projection does not
alter authoritative state or transition behavior.

## Resolved Contract

`observe_player` maps `OpponentTruth.position == FarSide` to
`OpponentReport { last_known_position: Some(FarSide), last_seen_turn: Some(turn),
health: Unknown, posture: Unknown }`. Other positions use the existing Unknown
report. `observe_allied` remains Unknown for opponent position, health, and
posture. The existing receipt source hash and observation validation continue
to bind the report to the host state.

## Evidence and Limits

Focused tests cover FarSide projection, Center/NearTower unknown behavior,
hidden health/posture, allied uncertainty, observation equality across hidden
substitutions, and FarSide history replay. The full suite passes with 60 Rust
tests.

This establishes one player-only sighting rule. It does not establish complete
vision, wards or line of sight, beliefs, memory decay, communication, automatic
threat timing, strategy quality, or a complete playable scenario.

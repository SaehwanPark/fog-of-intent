# Simulation Design — M2 v2 Contract Remediation

**Status:** Current internal design contract
**Date:** 2026-08-06
**Scope:** Versioned, dependency-free one-lane diagnostic kernel

This design records the contract implemented by the v2 remediation. It does
not promote M2 to a playable scenario and does not authorize CLI, MCP,
persistence, tuning, a catalog, or human-experience claims.

## State and lifecycle

`LaneSnapshot` is the only authoritative M2 state. It contains ruleset `3`,
turn, one- or two-beat window, player and opponent lane facts, wave pressure,
hidden jungle-threat truth, and a bounded delayed-effect queue. The player
contains health, position, and one `LaneResources` aggregate:

```text
LaneResources {
  mana: LaneMana,
  gold: LaneGold,
  experience: LaneExperience,
  cooldown: LaneCooldown,
}
```

Lifecycle is represented by `LaneStatus::Open` or
`LaneStatus::Resolved(LaneOutcome)`. There is no separate outcome option, so
an open snapshot cannot carry a terminal outcome. Histories and scenario
wrappers accept only a valid open initial snapshot.

`LaneDelay` is a non-zero value object. A delayed effect stores it rather than
an unconstrained integer; queue capacity remains four. Existing effects tick
by the selected window beats, resolve when their delay is exhausted, and are
then projected in committed order. A newly queued effect is never resolved in
the same transition.

## Actor-visible observations

`observe_player` produces `m2-lane-observation-v2`. It includes the player’s
health, retained resource aggregate, position, wave pressure, window, legal
intent metadata, and explicitly reported opponent/threat information. Hidden
opponent health/posture and current threat truth remain absent. The allied
projection uses `m2-allied-proposal-observation-v2`, exposes only the authorized
laner retained resources, and keeps opponent/threat reports unknown.

Observation receipts bind an observation to the source state hash privately at
the host boundary. The actor-visible value does not reveal that hash.

## Commands and resolved inputs

Intent and coordination types remain separate from execution. A validated
`LaneIntentCommand` is host-created from the current observation and exact
prior-state hash. `LaneResolvedInputs` carries independent environment,
observation, policy, coordination, and execution traces. Execution contains
damage, wave result, a `LaneResourceInputs` aggregate, and at most one delayed
effect:

```text
LaneResourceInputs {
  mana_spent: LaneMana,
  gold_earned: LaneGold,
  experience_gained: LaneExperience,
  cooldown_set: LaneCooldown,
}
```

Mana spending is legal only for `Contest`; bounds and overflow are checked
before state mutation. Cooldown ticking uses the full `u32` beat count and
saturates at zero.

## Authoritative transition ordering

For a valid open snapshot and validated command, `transition_lane` evaluates in
this order:

1. bind validation to the exact snapshot and reject a non-open status;
2. reject damage that exceeds either actor’s health;
3. validate and apply the retained resource aggregate, including cooldown tick;
4. resolve wave pressure and subtract direct damage;
5. tick the existing delayed queue in order and apply resolved effects;
6. enqueue the new non-zero delayed effect, failing closed on queue overflow;
7. derive fallback, position, outcome, and the next turn;
8. construct `LaneStatus::Resolved(outcome)` and the next snapshot;
9. project ordered events, attributed effects, and the v2 debrief from the
   single resolved result; and
10. return the next-state hash.

The transition is synchronous, deterministic, and receives no I/O, clock,
randomness, or hidden actor state.

## Replay and branching identity

Current identities are versioned: the numeric M2 ruleset is `3`; player/allied
observations, one-window coordination, two-window scenario, final debrief,
branch, allied profile, and named strategy fixtures use v2 identifiers. The
objective schema and hold-lane goal remain v1 because their shapes and semantics
are unchanged.

Every `LaneTransitionRecord` stores `m2-one-lane-window-v2`, and
`lane_record_identity` hashes that replay ID before the command, input traces,
execution values (including delayed-effect inputs), and prior-state hash.
History, coordination, scenario, objective, and branch verification reject a
missing, old, or tampered identity. Branches reuse
or regenerate explicit execution inputs and keep their own v2 branch identity;
they never alter the parent history.

M2 v1 identifiers are retired without migration. They were internal
experimental slices with no release, tag, external codec, or supported
artifact; old inputs must fail closed. M1 ruleset `1`, codec `1.0.0`, fixture
files, hashes, and replay behavior are outside this change.

## Debrief and exclusions

One-window and final-debrief projections use committed events, effects,
resource deltas, intent, coordination, objective, and replay facts. Reports
omit private receipts, source hashes, hidden state, policy internals, and
uncommitted choices. The debrief distinguishes direct/indirect and
immediate/delayed provenance without claiming decision quality or optimality.

Explicitly excluded from v2 are bounty, level, minion kills, shield, ward, and
the sixteen experimental consumables (potion, elixir, trinket, relic, charm,
scroll, tome, rune, sigil, talisman, amulet, phial, flask, incense, salve, and
poultice), plus a complete scenario, automatic threat timing, richer vision or
belief updates, item catalog, external serialization, CLI/MCP, persistence,
additional dependencies, and gameplay tuning.

## Contract evidence

The v2 tests cover retained bounds and contest-only mana, large cooldown ticks,
non-zero delayed effects and queue overflow, open/resolved lifecycle,
versioned hashes and record identity, actor-visible redaction, hidden-state
invariance, ordinary/coordinated/scenario/branch/objective/strategy/debrief
replay, and fail-closed identity tampering. The complete M2 exit criteria
remain unchecked in the canonical roadmap.

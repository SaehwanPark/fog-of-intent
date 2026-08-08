# M4 Policy-Role Metadata Design

## Boundary

`ScriptedAgentRole` is profile metadata in the pure agent policy module. It is
not `LaneActorRole`, does not alter candidate generation or scoring, and cannot
authorize or execute a transition.

## Role catalog

| Profile | Policy role | Role ID |
| --- | --- | --- |
| `cautious-laner-v1` | `Anchor` | `anchor-v1` |
| `risk-taking-laner-v1` | `Duelist` | `duelist-v1` |
| `yielding-laner-v1` | `Pacer` | `pacer-v1` |

## Evidence and limits

The matched profile test binds all enum values and literal IDs. The labels make
fixed policy posture inspectable; they do not establish role behavior,
population diversity, strategic value, or human realism.

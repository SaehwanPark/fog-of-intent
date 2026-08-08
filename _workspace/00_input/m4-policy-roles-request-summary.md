# M4 Policy-Role Metadata Request Summary

## Requested slice

Add transparent, versioned policy-role labels to the three fixed scripted
profiles without changing the lane scenario actor roster.

## In scope

- Add `ScriptedAgentRole::{Anchor,Duelist,Pacer}` with stable IDs
  `anchor-v1`, `duelist-v1`, and `pacer-v1`.
- Bind the roles to cautious, risk-taking, and yielding profiles.
- Add literal role-binding assertions and synchronize M4/core docs,
  QA/handoff, changelog, and `LESSONS.md`.

## Out of scope

- Changing `LaneActorRole`, scenario state, transition behavior, role
  populations, memory, communication, randomness, outcomes, or human realism.

## Success evidence

- Every profile exposes one stable policy-role ID.
- The IDs remain metadata and do not grant hidden state or authority.

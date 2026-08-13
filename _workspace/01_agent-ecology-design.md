# Agent Ecology Design: M8 Team Communication Speech Acts & Envelope Schema

## Design Objective

Define typed, bounded, and interpretable communication primitives for team interaction, proposal exchange, and shot-calling. These primitives allow autonomous agents and human players to negotiate intent, express confidence and conditions, and establish transparent speech acts without leaking private chain-of-thought or authoritative simulation state.

## Core Schema Contracts

### 1. Version Identifiers

```rust
pub const TEAM_COMMUNICATION_SCHEMA: &str = "m8-team-communication-v1";
pub const TEAM_SPEECH_ACT_SCHEMA: &str = "m8-team-speech-act-v1";
pub const TEAM_MESSAGE_ENVELOPE_SCHEMA: &str = "m8-team-message-envelope-v1";
```

### 2. Speech Acts (`TeamSpeechAct`)

Eight discrete communicative speech acts:
- `Proposal` (`"proposal"`): Proposes an objective, tactical maneuver, or intent (e.g. contest wave, gank).
- `Clarification` (`"clarification"`): Queries or clarifies target priority, timing, or resource readiness.
- `Confirmation` (`"confirmation"`): Acknowledges and confirms agreement with an active proposal.
- `Disagreement` (`"disagreement"`): Explicitly dissents or communicates an inability to support a call.
- `CounterProposal` (`"counter-proposal"`): Offers an alternative intent or contingency.
- `ConditionalCommitment` (`"conditional-commitment"`): Pledges support contingent upon a prerequisite condition.
- `Withdrawal` (`"withdrawal"`): Cancels or rescinds a prior call or commitment.
- `FailureReport` (`"failure-report"`): Reports an aborted attempt, execution breakdown, or loss of advantage.

### 3. Addressing and Recipients (`TeamRecipient`)

- `Broadcast` (`"broadcast"`): Message addressed to all allied team members.
- `Direct(LaneActorRole)` (`"direct:<role>"`): Directed to a specific actor role (e.g. `HumanLaner`, `AlliedAutonomous`).

### 4. Urgency Levels (`TeamMessageUrgency`)

- `Low` (`"low"`): Informational or long-term positioning.
- `Standard` (`"standard"`): Default operational pace.
- `Critical` (`"critical"`): Immediate emergency (e.g. imminent gank, low-health collapse).

### 5. Confidence Ratings (`TeamConfidenceLevel`)

- `Tentative` (`"tentative"`): Exploratory or uncertain recommendation.
- `Confident` (`"confident"`): Solid tactical basis.
- `Definite` (`"definite"`): High-certainty command or uncompromised commitment.

### 6. Tactical Conditions (`TeamMessageCondition`)

- `Unconditional` (`"unconditional"`): Execute immediately regardless of state.
- `HealthAboveThreshold` (`"health-above-threshold"`): Requires sufficient health pool.
- `ThreatAbsent` (`"threat-absent"`): Requires jungle/gank threat to be absent or clear.
- `AlliedPresence` (`"allied-presence"`): Requires ally in proximity.
- `ResourceSufficient` (`"resource-sufficient"`): Requires mana/cooldowns ready.

### 7. Information Visibility & Redaction (`TeamMessageVisibility`)

- `TeamOnly` (`"team-only"`): Visible to all actors on the same team.
- `DirectOnly` (`"direct-only"`): Visible only to sender and designated recipient.
- `Public` (`"public"`): Visible across teams (e.g. general broadcast).

Visibility rules are enforced via:
`is_visible_to(viewer: LaneActorRole, is_same_team: bool) -> bool`

### 8. Message Envelope (`TeamMessageEnvelope`)

Structured message container:
```rust
pub struct TeamMessageEnvelope {
  schema: &'static str,
  message_id: &'static str,
  sender: LaneActorRole,
  recipient: TeamRecipient,
  speech_act: TeamSpeechAct,
  proposed_intent: Option<LaneIntent>,
  urgency: TeamMessageUrgency,
  confidence: TeamConfidenceLevel,
  condition: TeamMessageCondition,
  visibility: TeamMessageVisibility,
  turn: u32,
  content_summary: &'static str,
  chain_of_thought_present: bool,
}
```

### 9. Validation & Catalog (`TeamCommunicationCatalog`)

- Strict validation ensuring:
  - Valid non-empty message ID.
  - Correct schema tag.
  - `chain_of_thought_present == false` (fail closed with `TeamCommunicationError::ChainOfThoughtForbidden`).
  - Proposed intent aligns with speech acts (e.g. `Proposal` / `CounterProposal` / `ConditionalCommitment` typically provide intent).
- Canonical catalog containing registered standard examples for all 8 speech acts.

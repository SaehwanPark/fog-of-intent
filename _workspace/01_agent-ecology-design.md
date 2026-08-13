# Agent Ecology Design: Private Submissions and Simultaneous Resolution (M8)

## 1. Overview
In Fog of Intent, multi-agent team strategy requires that autonomous teammates formulate decisions under uncertainty without knowing each other's uncommitted choices in advance. Once all participating roles have submitted their private actions, the host performs a simultaneous resolution evaluating communicative directives, individual plan alignments, trust compliance, and consensus rules.

## 2. Information Boundaries & Privacy
- **Zero Chain-of-Thought**: Any submission with `chain_of_thought_present == true` is rejected immediately (`TeamSimultaneousError::ChainOfThoughtForbidden`).
- **Privacy Preservation During Collection**: While `phase == CollectingSubmissions`, individual submissions cannot be queried or inspected (`get_submission` returns `None` or errors). `fmt::Debug` on `TeamSimultaneousWindow` redacts private intents and payloads.
- **Payload-Free Receipts**: `TeamSubmissionReceipt` confirms that a submission was validated and accepted without echoing submitted contents to other actors.

## 3. Simultaneous Window State Machine
```text
  +-----------------------+
  | CollectingSubmissions | <--- Initial State
  +-----------------------+
              |
              | (all registered roles have submitted)
              v
         +---------+
         |  Ready  |
         +---------+
              |
              | (resolver evaluates submissions)
              v
        +----------+
        | Resolved |
        +----------+
              |
              | (window closed or reset)
              v
         +--------+
         | Closed |
         +--------+
```

## 4. Discrete Types & Enums
- **`TeamCoordinationOutcome`**:
  - `FullyCoordinated`: All roles align on team plan and directives with high cohesion ($>= 7,500$ bp).
  - `PartiallyCoordinated`: Roles align on core objective but differ in tactical posture ($5,000$ to $7,499$ bp).
  - `DivergentIntents`: Roles choose conflicting individual intents without consensus ($2,500$ to $4,999$ bp).
  - `ConflictingDirectives`: Contradictory directives from multiple callers cause coordination deadlock ($1,000$ to $2,499$ bp).
  - `CommunicationFailure`: Critical messages lost in transmission or timeout resulting in default/fallback $(< 1,000$ bp).
- **`TeamSubmissionEnvelope`**: Role, observation ID, turn, intent, target focus, commitment, ping signal, optional message envelope, optional individual plan, and zero chain-of-thought flag.
- **`TeamSimultaneousResolution`**: Outcome, cohesion score in basis points, resolved role-intent pairs, leadership evaluation, alignment evaluations, trust decisions, and formatted summary report.

## 5. Resolver Logic
1. Validate that the window is in `Ready` phase and all registered roles are present.
2. If an active team plan is specified, evaluate each role's individual plan alignment using `TeamPlanEvaluator`.
3. If a leadership structure and trust matrix are provided, evaluate directive compliance or peer consensus using `TeamLeadershipEvaluator` and `TeamTrustEvaluator`.
4. Combine alignment scores, compliance scores, and message delivery statuses into an overall team cohesion score in $[0..=10,000]$ bp.
5. Classify the resulting `TeamCoordinationOutcome`.
6. Return `TeamSimultaneousResolution` with full provenance and zero chain-of-thought verification.

# Playtest Report: M8 Team Communication Scenarios, Debriefs, and Strategic Disagreement

**Date:** 2026-08-13
**Target:** Milestone M8 — Team Communication and Shot-Calling
**Evaluation Mode:** Early-Stage Functional Verification & Later-Stage Strategic Experience
**Agent Skill:** `foi-test-player`

## Playtest Metadata

- **Scenario Battery:** `m8-team-scenarios-v1`
- **Schemas Evaluated:**
  - `m8-team-scenarios-v1`
  - `m8-team-communication-debrief-v1`
  - `m8-team-leadership-debrief-v1`
  - `m8-team-encounter-debrief-v1`
  - `m8-strategic-disagreement-v1`
- **Tested Personas:**
  - *Duelist/Opportunistic*: Responsive to gank directives when health is sufficient.
  - *Anchor/Cautious*: Prioritizes wave stabilization, safety, and defensive dissent when threatened.
  - *Decentralized Coordinator*: Peer proposal submitter with reputation-weighted arbitration.

## Scenario Execution Battery Results

### 1. High-Trust Coordinated Gank (`scenario-high-trust-gank-v1`)
- **Outcome:** `FullyCoordinated` (Team Cohesion: $10,000$ bp / $100\%$)
- **Leadership Evaluation:** Designated Shot-Caller (`HumanLaner`), $1/1$ complied ($10,000$ bp compliance rate), $+500$ bp caller reputation delta.
- **Communication Debrief:** $2/2$ messages delivered ($10,000$ bp transmission reliability), $0$ dropped, $0$ delayed, $1$ dialogue round.
- **Strategic Takeaway:** High-trust caller directive executed with unanimous compliance and zero transmission loss.
- **Functional Check:** PASSED. Zero private chain-of-thought leaked.

### 2. Low-Trust Autonomous Dissent (`scenario-low-trust-dissent-v1`)
- **Outcome:** `PartiallyCoordinated` ($5,000$ bp team cohesion)
- **Leadership Evaluation:** Distrusted Shot-Caller (`AlliedAutonomous`), $0/1$ complied, $1/1$ dissented ($0$ bp compliance rate), $-500$ bp caller reputation delta.
- **Communication Debrief:** $2/2$ messages sent, $1$ message suppressed/distrusted, $8,000$ bp transmission reliability, $1$ dissent recorded (`AlternativeObjectivePriority`).
- **Disagreement Legitimacy Evaluation:**
  - Classification: `ConstructiveAlternative`
  - Legitimacy: `true` (Counterfactual Delta: $+1,500$ bp)
  - Causal Reason: `AlternativeObjectivePriority`
- **Functional Check:** PASSED. Proves shot-callers cannot force blind compliance when reputation is deficient.

### 3. Conflicting Peer Calls Arbitration (`scenario-conflicting-calls-arbitration-v1`)
- **Outcome:** `FullyCoordinated` ($9,000$ bp team cohesion)
- **Leadership Evaluation:** Decentralized structure, $2/2$ complied after arbitration, $0$ deadlocks, $+250$ bp reputation delta.
- **Communication Debrief:** $4/4$ messages delivered ($10,000$ bp reliability), $2$ negotiation rounds.
- **Consensus Rule:** `HighestReputationLead` correctly arbitrated between `plan-gank-setup-v1` ($9,000$ bp) and `plan-defensive-hold-v1` ($6,000$ bp) without stalling or deadlock.
- **Functional Check:** PASSED. Peer arbitration deterministic and deadlock-free.

### 4. Missing-Message Channel Loss Fallback (`scenario-missing-message-fallback-v1`)
- **Outcome:** `CommunicationFailure` ($2,500$ bp team cohesion)
- **Leadership Evaluation:** Designated Shot-Caller (`HumanLaner`), $1$ fallback activation triggered (`FallbackToDefaultHold`).
- **Communication Debrief:** $2$ sent, $1$ delivered, $1$ dropped due to channel overload ($5,000$ bp reliability).
- **Behavioral Check:** Receiver safely recognized missing message envelope and reverted to individual defensive routine without crashing or hanging.
- **Functional Check:** PASSED. Channel physics and packet loss handled fail-closed.

### 5. Strategic Legitimate Dissent Survival (`scenario-strategic-dissent-survival-v1`)
- **Outcome:** `PartiallyCoordinated` ($4,000$ bp team cohesion)
- **Leadership Evaluation:** Designated Shot-Caller (`AlliedAutonomous`), $0/1$ complied, $1/1$ dissented, $-750$ bp caller reputation delta.
- **Communication Debrief:** $1$ dissent recorded (`LowHealth`).
- **Disagreement Legitimacy Evaluation:**
  - Classification: `LegitimateDissent`
  - Legitimacy: `true` (Counterfactual Delta: $+8,000$ bp)
  - Causal Reason: `LowHealth`
  - Explanation: Dissent averted lethal elimination under adverse health/threat conditions.
- **Strategic Validation:** Blind compliance with the reckless `Contest` call would have resulted in player death ($-5,000$ bp), whereas dissenting to `Yield` preserved the player ($+3,000$ bp), giving a net $+8,000$ bp swing.
- **Functional Check:** PASSED. Formal verification that disagreement in Fog of Intent is strategically legitimate.

## Gameplay Feel & Strategic Assessment

1. **Agency & Execution Feel:**
   Strategic shot-calling feels meaningful because autonomous teammates are neither mindless puppets nor chaotic RNG agents: their compliance is governed by trust, communication clarity, and tactical survival constraints.
2. **Debrief Inspectability:**
   The Markdown debrief report clearly decouples communication efficiency, leadership follow rates, coordination outcomes, and tactical execution, eliminating outcome bias.
3. **Strategic Disagreement:**
   Dissent is proven to be a vital survival mechanism in turn-based strategic team play.

## Evidence Limits

These playtests were conducted with deterministic scripted reference policies and in-process simulation harnesses. They verify functional correctness, contract invariants, and behavioral logic. They do not constitute empirical claims about human player psychology or accessibility compliance.

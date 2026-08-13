# Agent-Ecology Design: M8 Coordination and Execution Attribution Separation

**Role:** Fog of Intent Agent-Ecology Designer
**Topic:** Attribution of Coordination Success and Failure Separately from Execution (Phase 8)

## 1. Domain Problem & Design Rationale

In Fog of Intent, a human player or AI coordinator expresses team intent, delegating execution to autonomous teammates who act under incomplete information, trust constraints, and localized observations. 

A central pathology in strategy game analysis is **Outcome Bias**:
- A team executes a terrible, uncoordinated play, but gets lucky mechanically (e.g. enemy misses or gets out-dueled), falsely rewarding poor strategy.
- Conversely, a team coordinates perfectly on an optimal tactical plan, but suffers an unfavorable execution roll or enemy mechanical clutch, falsely penalizing sound strategy.

To make causal debriefs inspectable and strategically instructive, the simulation engine must strictly decouple and independently measure:
1. **Coordination Assessment**: Cohesion, directive compliance, plan alignment, and communication fidelity before and during simultaneous resolution.
2. **Execution Assessment**: Mechanical effectiveness, damage exchanges, objective control, and spatial positioning during physical simulation.
3. **Attribution Synthesis**: Mapping the joint outcome into 4 canonical strategic quadrants and quantifying relative causal contributions in integer basis points.

## 2. Canonical Attribution Quadrants

| Quadrant | Coordination | Execution | Strategic Meaning |
| :--- | :--- | :--- | :--- |
| `CoordinatedTriumph` | High ($\ge 5,000$ bp) | High ($\ge 5,000$ bp) | Flawless team alignment followed by effective mechanical execution. |
| `CoordinatedFailure` | High ($\ge 5,000$ bp) | Low ($< 5,000$ bp) | Excellent communication and agreement, but mechanical execution or bad luck failed. |
| `UncoordinatedBailout` | Low ($< 5,000$ bp) | High ($\ge 5,000$ bp) | Strategic dissent or communication failure bailed out by individual mechanical skill. |
| `CompoundedFailure` | Low ($< 5,000$ bp) | Low ($< 5,000$ bp) | Miscommunication/directive deadlock compounded with mechanical collapse. |

## 3. Discrete Causal Factors

### Coordination Factors (`CoordinationCausalFactor`):
- `UnanimousAlignment`: All actors aligned on common team plan.
- `DirectiveCompliance`: Teammates complied with designated shot-caller directive.
- `PeerConsensusArbitrated`: Decentralized proposals unified via consensus rule.
- `TrustDeficitDissent`: Teammates rejected directive due to low caller reputation.
- `ConflictingDirectives`: Competing leadership calls caused deadlock.
- `ChannelTransmissionLoss`: Critical communication packets delayed or dropped.
- `ConditionUnmetDissent`: Legitimate dissent because tactical prerequisite failed.
- `DivergentStrategicPriorities`: Independent conflicting individual plans.

### Execution Factors (`ExecutionCausalFactor`):
- `DecisiveDamageAdvantage`: Dominant combat trade efficiency.
- `ObjectiveSecured`: Critical tower space or lane position held.
- `FavorablePositioning`: Optimal spatial arrangement during engagement.
- `OpponentMechanicalCounter`: Opponent executed superior counterplay.
- `SevereHealthAttrition`: Critical health loss forcing disengagement.
- `ResourceDepletion`: Exhaustion of mana or cooldowns.
- `WavePressureDisadvantage`: Overwhelming minion pressure forcing retreat.
- `UnfavorableStochasticRoll`: Execution dice variance favored opponent.

## 4. Basis-Point Conservation Model

All attribution weights are represented in integer basis points ($[0..=10,000]$ bp):
$$\text{coordination\_contribution\_bp} + \text{execution\_contribution\_bp} + \text{exogenous\_variance\_bp} = 10,000\text{ bp}$$

Ratings:
- Coordination: `High` ($\ge 7,500$ bp), `Moderate` ($5,000..=7,499$ bp), `Low` ($2,500..=4,999$ bp), `Failed` ($0..=2,499$ bp).
- Execution: `Flawless` ($\ge 7,500$ bp), `Competent` ($5,000..=7,499$ bp), `Compromised` ($2,500..=4,999$ bp), `Failed` ($0..=2,499$ bp).

## 5. Reference Scenario Catalog

The `CoordinationAttributionCatalog` registers 6 canonical scenarios:
1. `attr-coordinated-triumph-gank-v1`: High-trust gank call, complete compliance, decisive damage execution $\rightarrow$ `CoordinatedTriumph`.
2. `attr-coordinated-failure-overreach-v1`: Perfect team consensus on contest, but enemy mechanical counter $\rightarrow$ `CoordinatedFailure`.
3. `attr-uncoordinated-bailout-clutch-v1`: Communication channel failure and dissent, but solo laner wins duel $\rightarrow$ `UncoordinatedBailout`.
4. `attr-compounded-failure-deadlock-v1`: Conflicting shot-caller directives and severe health attrition $\rightarrow$ `CompoundedFailure`.
5. `attr-legitimate-dissent-avoided-wipe-v1`: Low health laner dissents from reckless dive, saving resources $\rightarrow$ `UncoordinatedBailout` / Strategic Dissent.
6. `attr-trust-breakdown-execution-miss-v1`: Low reputation caller ignored, disjointed fight $\rightarrow$ `CompoundedFailure`.

## 6. Safety & Privacy Constraints

- Zero private chain-of-thought rule enforced (`chain_of_thought_present == false`).
- No floating point numbers anywhere in attribution calculations.
- Pure deterministic evaluation functions.

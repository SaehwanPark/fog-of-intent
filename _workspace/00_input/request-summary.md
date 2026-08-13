# Request Summary: Designated Shot-Caller and Decentralized Coordination Baselines (M8)

**Requested Outcome:** Define and implement designated shot-caller and decentralized coordination baseline leadership policies, consensus arbitration, fallback modes, cohesion evaluation, and canonical leadership catalogs for M8 team communication without violating actor information boundaries or introducing disguised direct control.
**Roadmap Milestone:** M8 — Team Communication and Shot-Calling
**Current Evidence:** `src/agent/communication.rs`, `src/agent/team_plan.rs`, `src/agent/trust.rs`.

## In Scope
- `LeadershipStructure`: discrete enum (`DesignatedShotCaller`, `Decentralized`, `SharedLeadership`) defining team authority distribution.
- `ConsensusRule`: deterministic arbitration rules (`UnanimousConsensus`, `HighestReputationLead`, `UrgencyFirst`, `MajoritySupport`) for decentralized coordination.
- `FallbackLeadershipMode`: fallback policies when leadership proposals fail (`FallbackToIndividualPlans`, `FallbackToDefaultHold`, `FallbackToSecondaryCaller`).
- `ShotCallerPolicy`: deterministic evaluation of team plans and issuance of directives/proposals based on observation context and role assignments.
- `DecentralizedCoordinator`: deterministic consensus arbitration among multiple simultaneous peer proposals, computing aggregate agreement and cohesion in exact integer basis points ($[0..=10,000]$ bp).
- `LeadershipResolutionOutcome`: discrete resolution states (`ConsensusAchieved`, `SplitDecision`, `FallbackIndividualPlans`, `ConflictedDeadlock`).
- `LeadershipEvaluationReport`: formatted Markdown inspection of leadership decisions, compliance rates, dissenting roles, and cohesion scores.
- `LeadershipCatalog`: canonical registered leadership configurations (`leader-designated-anchor-v1`, `leader-designated-jungler-v1`, `leader-decentralized-unanimous-v1`, `leader-decentralized-reputation-v1`, `leader-decentralized-urgency-v1`).
- Fail-closed validation, zero private chain-of-thought preservation (`chain_of_thought_present == false`), and leak-proof visibility.

## Non-Goals
- No unconstrained natural-language LLM generation or social roleplaying.
- No bypassing teammate policy or forcing direct command execution.
- No floating-point consensus metrics or non-deterministic arbitration.
- No multi-lane map physics (deferred to M9).

## Project Boundaries Touched
- `src/agent/leadership.rs` (new module)
- `src/agent/mod.rs`
- `src/agent/tests.rs`

## Expected Outputs
- `src/agent/leadership.rs`
- Comprehensive unit tests in `src/agent/tests.rs`
- `_workspace/01_agent-ecology-design.md`
- `_workspace/03_domain-qa.md`
- `_workspace/final/handoff.md`
- Updates to `ROADMAP.md`, `SPEC.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `LESSONS.md`, and `README.md`.

## Verification
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`

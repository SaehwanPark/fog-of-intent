# M7 Diagnostic Choice Dimensions and Catalog Ecology Design

## Goal and Roadmap Milestone

Define the versioned schema `m7-diagnostic-choice-catalog-v1` with typed diagnostic choice dimensions in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing the 7 canonical diagnostic choice dilemmas required by Phase 7 for evaluating semantic and parametric agent behavior contrasts.

## Behavioral Question and Evidence Boundary

Can key strategic dilemmas in lane decision-making be captured as declarative, typed diagnostic choice definitions with explicit domains, primary/alternative intent options, and documented contrasts?
The output is a structured diagnostic choice contract; it does not claim human psychological completeness, full match dynamics, or parametric policy fitting.

## Choice Domains and Schema

1. **Schema**: `m7-diagnostic-choice-catalog-v1`
2. **Domains** (`DiagnosticChoiceDomain`):
   - `ContestConcede` ("contest-concede"): Contesting space vs conceding/yielding to conserve safety.
   - `FollowReject` ("follow-reject"): Following allied coordination calls vs rejecting to act autonomously.
   - `FarmAssist` ("farm-assist"): Prioritizing local lane farming vs committing to assist allied contest.
   - `RecallTiming` ("recall-timing"): Greedy lane stabilization vs timely recall reset.
   - `Sacrifice` ("sacrifice"): Holding ground despite attrition danger vs withdrawing to preserve health.
   - `Surprise` ("surprise"): Threat withdrawal vs standing ground under unexpected pressure.
   - `ResponseToFailure` ("response-to-failure"): Yielding space after setback vs doubling down on contest.

3. **Diagnostic Choice Definitions**:
   - `choice-contest-concede-v1`:
     - Domain: `ContestConcede`
     - Primary Intent: `Contest`
     - Alternative Intent: `Yield`
     - Contrast: "Contesting contested space vs yielding position to protect survival."
   - `choice-follow-reject-v1`:
     - Domain: `FollowReject`
     - Primary Intent: `Contest`
     - Alternative Intent: `Stabilize`
     - Contrast: "Accepting allied coordinated contest call vs rejecting and maintaining autonomous stabilization."
   - `choice-farm-assist-v1`:
     - Domain: `FarmAssist`
     - Primary Intent: `Stabilize`
     - Alternative Intent: `Contest`
     - Contrast: "Farming wave space locally vs committing to assist nearby contest."
   - `choice-recall-timing-v1`:
     - Domain: `RecallTiming`
     - Primary Intent: `Recall`
     - Alternative Intent: `Stabilize`
     - Contrast: "Executing timely recall to reset resources vs greedily stabilizing wave in lane."
   - `choice-sacrifice-v1`:
     - Domain: `Sacrifice`
     - Primary Intent: `Contest`
     - Alternative Intent: `Withdraw`
     - Contrast: "Holding contested space under threat vs withdrawing to preserve health."
   - `choice-surprise-v1`:
     - Domain: `Surprise`
     - Primary Intent: `Withdraw`
     - Alternative Intent: `Stabilize`
     - Contrast: "Immediate threat withdrawal vs standing ground under unexpected pressure."
   - `choice-response-to-failure-v1`:
     - Domain: `ResponseToFailure`
     - Primary Intent: `Yield`
     - Alternative Intent: `Contest`
     - Contrast: "Yielding space after an unfavorable exchange vs doubling down on contest."

4. **Catalog**:
   - `DiagnosticChoiceCatalog` provides `all_choices()`, `lookup(choice_id)`, `validate_choice_id(choice_id)`, and `choice_for_domain(domain)` with fail-closed error handling (`DiagnosticChoiceCatalogError::UnknownChoice`).

## Verification Contract

Focused agent tests must prove:
1. All domain variants convert to canonical strings and parse back losslessly.
2. The 7 canonical choice definitions have valid schemas, distinct choice IDs, non-empty descriptions, non-empty contrast strings, and valid primary/alternative intents.
3. `DiagnosticChoiceCatalog::lookup` returns `Some` for all 7 canonical choice IDs and `None` for invalid IDs.
4. `DiagnosticChoiceCatalog::validate_choice_id` returns `Ok` for canonical IDs and `Err(UnknownChoice)` for invalid IDs.
5. `DiagnosticChoiceCatalog::choice_for_domain` returns the corresponding definition for every domain variant.
6. All repository verification checks pass.

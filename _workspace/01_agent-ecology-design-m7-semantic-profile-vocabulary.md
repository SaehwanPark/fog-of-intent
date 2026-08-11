# M7 Compact Semantic Profile Vocabulary and Schema Ecology Design

## Goal and Roadmap Milestone

Define the versioned schema `m7-semantic-profile-vocabulary-v1` with a compact semantic profile vocabulary in `src/agent.rs` under M7 (Semantic-to-Parametric Calibration Proof), establishing the discrete semantic trait dimensions and reference profile descriptors for future diagnostic scenario calibration.

## Behavioral Question and Evidence Boundary

Can semantic behavioral traits be represented in a compact, interpretable schema with explicit discrete dimensions (risk tolerance, deference, focus, communication clarity) and validated catalog lookups? The output is a structured semantic trait contract; it does not claim human psychological validity, unconstrained natural-language grounding, or parametric distillation completeness.

## Trait Dimensions and Schema

1. **Schema**: `m7-semantic-profile-vocabulary-v1`
2. **Dimensions**:
   - `SemanticRiskTolerance`:
     - `cautious`: Prioritizes damage avoidance and retreat under ambiguity or threat.
     - `balanced`: Balances resource gain and safety.
     - `risk-seeking`: Prioritizes contested objectives and forward pressure despite risk.
   - `SemanticDeference`:
     - `autonomous`: Acts primarily on own local evaluation.
     - `compliant`: Aligns with external calls or leader direction.
     - `yielding`: Readily yields contest priority to ally/neutral presence.
   - `SemanticFocus`:
     - `patience`: Waits for wave stabilization and defensive positioning.
     - `opportunity`: Exploits openings and immediate favorable conditions.
     - `urgency`: Prioritizes rapid escalation or immediate objective contest.
   - `SemanticCommunicationClarity`:
     - `terse`: Minimal signals, essential threat/status only.
     - `standard`: Balanced communicative frequency.
     - `verbose`: High communicative frequency and explicit intents.

3. **Canonical Reference Profiles**:
   - `cautious-laner-semantic-v1`:
     - Risk: `cautious`
     - Deference: `autonomous`
     - Focus: `patience`
     - Communication Clarity: `terse`
     - Description: "Cautious autonomous laner prioritizing lane stabilization and threat retreat."
   - `risk-taking-laner-semantic-v1`:
     - Risk: `risk-seeking`
     - Deference: `autonomous`
     - Focus: `opportunity`
     - Communication Clarity: `standard`
     - Description: "Risk-seeking autonomous laner prioritizing contest opportunities."
   - `yielding-laner-semantic-v1`:
     - Risk: `cautious`
     - Deference: `yielding`
     - Focus: `patience`
     - Communication Clarity: `terse`
     - Description: "Yielding laner deferring contest to avoid confrontation."

4. **Vocabulary Catalog**:
   - `SemanticProfileVocabulary` provides `all_profiles()`, `lookup(profile_id)`, and `validate_profile_id(profile_id)` with fail-closed error handling (`SemanticProfileVocabularyError::UnknownProfile`).

## Verification Contract

Focused agent tests must prove:
1. All semantic enum variants have canonical string names and exact bidirectional parsing/matching.
2. The 3 canonical profiles have valid schemas, non-empty descriptions, and distinct trait configurations.
3. `SemanticProfileVocabulary::lookup` returns `Some` for all canonical IDs and `None` for invalid IDs.
4. `SemanticProfileVocabulary::validate_profile_id` returns `Ok` for canonical IDs and `Err(UnknownProfile)` for unknown IDs.
5. All full toolchain checks (fmt, clippy, test, check_repository) pass with zero warnings.

## Open Boundaries

Diagnostic scenario choices, empirical action/communication distribution estimation, and parametric fitting remain open.

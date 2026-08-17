//! Unit and scenario tests for the M10 study protocol and evaluation framework.

use super::catalog::{STANDARD_ALPHA_PROTOCOL, StudyProtocolCatalog};
use super::evaluation::{StudyEvaluationError, evaluate_study_cohort};
use super::finding::{FindingCategory, FindingDisposition, FindingRecord, FindingSeverity};
use super::protocol::{EvaluationDimension, ParticipantCohort, PrivacyConsentDeclaration};
use super::session::{AccessNeedsDeclaration, CompletionStatus, ParticipantSessionRecord};

#[test]
fn protocol_participant_cohorts_and_dimensions_round_trip() {
  assert_eq!(ParticipantCohort::ALL.len(), 4);
  for cohort in ParticipantCohort::ALL {
    assert_eq!(format!("{cohort}"), cohort.as_str());
  }

  assert_eq!(EvaluationDimension::ALL.len(), 10);
  for dim in EvaluationDimension::ALL {
    assert_eq!(format!("{dim}"), dim.as_str());
    match dim {
      EvaluationDimension::KeyboardFlow
      | EvaluationDimension::NonColorSemantics
      | EvaluationDimension::ScreenReaderSuitability => {
        assert!(dim.is_accessibility());
      }
      _ => {
        assert!(!dim.is_accessibility());
      }
    }
  }
}

#[test]
fn privacy_consent_declaration_validation() {
  let valid = PrivacyConsentDeclaration::standard();
  assert!(valid.is_valid());

  let invalid_pii = PrivacyConsentDeclaration {
    deidentified_records_only: true,
    no_pii_collected: false,
    zero_latent_state_leakage: true,
  };
  assert!(!invalid_pii.is_valid());

  let invalid_leak = PrivacyConsentDeclaration {
    deidentified_records_only: true,
    no_pii_collected: true,
    zero_latent_state_leakage: false,
  };
  assert!(!invalid_leak.is_valid());
}

#[test]
fn finding_taxonomy_and_blocker_disposition_logic() {
  assert_eq!(FindingCategory::ALL.len(), 4);
  for cat in FindingCategory::ALL {
    assert_eq!(format!("{cat}"), cat.as_str());
  }

  assert_eq!(FindingSeverity::ALL.len(), 4);
  for sev in FindingSeverity::ALL {
    assert_eq!(format!("{sev}"), sev.as_str());
    if sev == FindingSeverity::Blocker {
      assert!(sev.is_blocking());
    } else {
      assert!(!sev.is_blocking());
    }
  }

  let resolved = FindingDisposition::Resolved {
    issue_ref: "PR #123",
  };
  assert!(resolved.is_resolved_or_mitigated());
  assert!(!resolved.is_unresolved_blocker(FindingSeverity::Blocker));
  assert_eq!(resolved.disposition_name(), "resolved");

  let deferred = FindingDisposition::Deferred {
    rationale: "Requires future refactor",
  };
  assert!(!deferred.is_resolved_or_mitigated());
  assert!(deferred.is_unresolved_blocker(FindingSeverity::Blocker));
  assert!(!deferred.is_unresolved_blocker(FindingSeverity::MajorBarrier));
  assert_eq!(deferred.disposition_name(), "deferred");

  let doc_limit = FindingDisposition::DocumentedLimitation {
    doc_ref: "docs/LIMITATIONS.md",
  };
  assert!(!doc_limit.is_unresolved_blocker(FindingSeverity::Blocker));
  assert_eq!(doc_limit.disposition_name(), "documented-limitation");
}

#[test]
fn session_status_and_access_needs_predicates() {
  let completed = CompletionStatus::Completed;
  assert!(completed.is_completed());
  assert!(!completed.is_abandoned());
  assert_eq!(completed.status_name(), "completed");

  let abandoned = CompletionStatus::AbandonedAtTurn(3);
  assert!(!abandoned.is_completed());
  assert!(abandoned.is_abandoned());
  assert_eq!(abandoned.status_name(), "abandoned");
  assert_eq!(format!("{abandoned}"), "abandoned(turn=3)");

  let inconclusive = CompletionStatus::Inconclusive;
  assert!(inconclusive.is_inconclusive());
  assert_eq!(format!("{inconclusive}"), "inconclusive");

  let no_needs = AccessNeedsDeclaration::none();
  assert!(!no_needs.has_any_need());

  let with_need = AccessNeedsDeclaration {
    screen_reader_user: true,
    color_vision_deficiency: false,
    keyboard_only_user: false,
    reduced_motion_required: false,
  };
  assert!(with_need.has_any_need());
}

#[test]
fn fail_closed_validation_checks() {
  let proto = STANDARD_ALPHA_PROTOCOL;

  // Empty sessions
  let err = evaluate_study_cohort(&proto, &[], &[]).unwrap_err();
  assert_eq!(err, StudyEvaluationError::EmptyPopulation);

  // Invalid privacy
  let mut invalid_proto = proto;
  invalid_proto.privacy_declaration.no_pii_collected = false;
  let session = ParticipantSessionRecord {
    participant_id: "p1",
    cohort: ParticipantCohort::StrategyGamer,
    access_needs: AccessNeedsDeclaration::none(),
    completion_status: CompletionStatus::Completed,
    explanation_quality_bp: 8000,
    debrief_comprehension_bp: 8000,
    turns_completed: 5,
  };
  let err = evaluate_study_cohort(&invalid_proto, &[session], &[]).unwrap_err();
  assert_eq!(err, StudyEvaluationError::InvalidPrivacyDeclaration);

  // Duplicate participant ID
  let session2 = ParticipantSessionRecord {
    participant_id: "p1",
    cohort: ParticipantCohort::MobaPlayer,
    access_needs: AccessNeedsDeclaration::none(),
    completion_status: CompletionStatus::Completed,
    explanation_quality_bp: 9000,
    debrief_comprehension_bp: 9000,
    turns_completed: 5,
  };
  let err = evaluate_study_cohort(&proto, &[session, session2], &[]).unwrap_err();
  assert_eq!(err, StudyEvaluationError::DuplicateParticipantId("p1"));

  // Score out of range
  let invalid_score_session = ParticipantSessionRecord {
    participant_id: "p2",
    cohort: ParticipantCohort::StrategyGamer,
    access_needs: AccessNeedsDeclaration::none(),
    completion_status: CompletionStatus::Completed,
    explanation_quality_bp: 10_001,
    debrief_comprehension_bp: 8000,
    turns_completed: 5,
  };
  let err = evaluate_study_cohort(&proto, &[invalid_score_session], &[]).unwrap_err();
  assert_eq!(
    err,
    StudyEvaluationError::ScoreOutOfRange {
      participant_id: "p2",
      score_bp: 10_001,
    }
  );

  // Unlinked finding
  let finding = FindingRecord {
    finding_id: "f1",
    participant_id: "unknown-p",
    dimension: EvaluationDimension::Onboarding,
    category: FindingCategory::Usability,
    severity: FindingSeverity::MinorFriction,
    description: "test",
    disposition: FindingDisposition::Deferred { rationale: "test" },
  };
  let err = evaluate_study_cohort(&proto, &[session], &[finding]).unwrap_err();
  assert_eq!(
    err,
    StudyEvaluationError::UnlinkedFindingParticipant {
      finding_id: "f1",
      participant_id: "unknown-p",
    }
  );

  // Duplicate finding ID
  let finding1 = FindingRecord {
    finding_id: "f1",
    participant_id: "p1",
    dimension: EvaluationDimension::Onboarding,
    category: FindingCategory::Usability,
    severity: FindingSeverity::MinorFriction,
    description: "test",
    disposition: FindingDisposition::Deferred { rationale: "test" },
  };
  let finding2 = FindingRecord {
    finding_id: "f1",
    participant_id: "p1",
    dimension: EvaluationDimension::PacingLoad,
    category: FindingCategory::GameplayBalance,
    severity: FindingSeverity::MinorFriction,
    description: "test 2",
    disposition: FindingDisposition::Deferred { rationale: "test" },
  };
  let err = evaluate_study_cohort(&proto, &[session], &[finding1, finding2]).unwrap_err();
  assert_eq!(err, StudyEvaluationError::DuplicateFindingId("f1"));
}

#[test]
fn error_display_formatting_coverage() {
  let errors = [
    StudyEvaluationError::EmptyPopulation,
    StudyEvaluationError::DuplicateParticipantId("p-dup"),
    StudyEvaluationError::DuplicateFindingId("f-dup"),
    StudyEvaluationError::ScoreOutOfRange {
      participant_id: "p-score",
      score_bp: 12000,
    },
    StudyEvaluationError::UnlinkedFindingParticipant {
      finding_id: "f-unlink",
      participant_id: "p-missing",
    },
    StudyEvaluationError::InvalidPrivacyDeclaration,
  ];

  for err in errors {
    let msg = format!("{err}");
    assert!(!msg.is_empty());
  }
}

#[test]
fn catalog_scenarios_execute_and_verify_all_expectations() {
  assert_eq!(StudyProtocolCatalog::ALL.len(), 3);

  for def in StudyProtocolCatalog::ALL {
    let lookup = StudyProtocolCatalog::find_by_id(def.scenario_id);
    assert_eq!(lookup, Some(def));

    let result = StudyProtocolCatalog::execute_scenario(def.scenario_id).unwrap();
    assert_eq!(result.scenario_id, def.scenario_id);
    assert!(
      result.all_expectations_met,
      "Scenario {} failed expectations: {:?}",
      def.scenario_id, result
    );
  }

  assert!(StudyProtocolCatalog::find_by_id("non-existent").is_none());
}

#[test]
fn accessibility_qualification_gate_rules() {
  let proto = STANDARD_ALPHA_PROTOCOL;

  // Case A: No access-needs participants -> disqualified
  let session_strat = ParticipantSessionRecord {
    participant_id: "p-strat",
    cohort: ParticipantCohort::StrategyGamer,
    access_needs: AccessNeedsDeclaration::none(),
    completion_status: CompletionStatus::Completed,
    explanation_quality_bp: 9000,
    debrief_comprehension_bp: 9000,
    turns_completed: 10,
  };
  let report_no_acc = evaluate_study_cohort(&proto, &[session_strat], &[]).unwrap();
  assert!(!report_no_acc.accessibility_claims_qualified);

  // Case B: Access-needs participant with unresolved accessibility blocker -> disqualified
  let session_acc = ParticipantSessionRecord {
    participant_id: "p-acc",
    cohort: ParticipantCohort::AccessNeeds,
    access_needs: AccessNeedsDeclaration {
      screen_reader_user: true,
      color_vision_deficiency: false,
      keyboard_only_user: true,
      reduced_motion_required: false,
    },
    completion_status: CompletionStatus::Completed,
    explanation_quality_bp: 8000,
    debrief_comprehension_bp: 8000,
    turns_completed: 10,
  };
  let blocker = FindingRecord {
    finding_id: "f-acc-block",
    participant_id: "p-acc",
    dimension: EvaluationDimension::ScreenReaderSuitability,
    category: FindingCategory::Accessibility,
    severity: FindingSeverity::Blocker,
    description: "Screen reader cannot read status line",
    disposition: FindingDisposition::Deferred {
      rationale: "Unresolved",
    },
  };
  let report_blocked = evaluate_study_cohort(&proto, &[session_acc], &[blocker]).unwrap();
  assert_eq!(report_blocked.unresolved_accessibility_blockers, 1);
  assert!(!report_blocked.accessibility_claims_qualified);

  // Case C: Access-needs participant with low comprehension (< 7000 bp floor) -> disqualified
  let session_low_comp = ParticipantSessionRecord {
    participant_id: "p-acc-low",
    cohort: ParticipantCohort::AccessNeeds,
    access_needs: AccessNeedsDeclaration {
      screen_reader_user: true,
      color_vision_deficiency: false,
      keyboard_only_user: true,
      reduced_motion_required: false,
    },
    completion_status: CompletionStatus::Completed,
    explanation_quality_bp: 8000,
    debrief_comprehension_bp: 6500,
    turns_completed: 10,
  };
  let report_low_comp = evaluate_study_cohort(&proto, &[session_low_comp], &[]).unwrap();
  assert!(!report_low_comp.accessibility_claims_qualified);

  // Case D: Access-needs participant with resolved blocker and high comprehension -> qualified
  let resolved_blocker = FindingRecord {
    finding_id: "f-acc-res",
    participant_id: "p-acc",
    dimension: EvaluationDimension::ScreenReaderSuitability,
    category: FindingCategory::Accessibility,
    severity: FindingSeverity::Blocker,
    description: "Screen reader cannot read status line",
    disposition: FindingDisposition::Resolved {
      issue_ref: "PR #200",
    },
  };
  let report_qualified =
    evaluate_study_cohort(&proto, &[session_acc], &[resolved_blocker]).unwrap();
  assert_eq!(report_qualified.unresolved_accessibility_blockers, 0);
  assert!(report_qualified.accessibility_claims_qualified);
}

#[test]
fn markdown_report_rendering_hygiene() {
  let result =
    StudyProtocolCatalog::execute_scenario("scenario-study-cohort-balanced-alpha-v1").unwrap();
  let md = result.report.to_markdown();

  assert!(md.contains("# Usability & Accessibility Study Evaluation Report"));
  assert!(md.contains("**Protocol:** `protocol-m10-alpha-v1`"));
  assert!(md.contains("## Cohort Performance"));
  assert!(md.contains("## Finding Breakdown & Disposition"));
  assert!(md.contains("## Target Gates"));
  assert!(md.contains("Accessibility Claims Qualified: QUALIFIED"));
  assert!(md.contains("## Evidence Boundary"));
  assert!(md.contains("no universal accessibility"));
}

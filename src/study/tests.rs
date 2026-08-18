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

#[test]
fn friction_indicators_and_interaction_modes_round_trip() {
  use super::dimension::CognitiveFrictionIndicator;
  use super::interaction::{ContrastMode, VerbosityLevel};

  assert_eq!(CognitiveFrictionIndicator::ALL.len(), 7);
  for indicator in CognitiveFrictionIndicator::ALL {
    assert_eq!(format!("{indicator}"), indicator.as_str());
    if indicator == CognitiveFrictionIndicator::None {
      assert!(!indicator.is_friction());
    } else {
      assert!(indicator.is_friction());
    }
  }

  assert_eq!(VerbosityLevel::ALL.len(), 3);
  for v in VerbosityLevel::ALL {
    assert_eq!(format!("{v}"), v.as_str());
    assert!(v.max_lines_per_turn() > 0);
  }

  assert_eq!(ContrastMode::ALL.len(), 3);
  for c in ContrastMode::ALL {
    assert_eq!(format!("{c}"), c.as_str());
    if c == ContrastMode::NoColor {
      assert!(!c.allows_ansi());
    } else {
      assert!(c.allows_ansi());
    }
  }
}

#[test]
fn interaction_audit_validation_rules() {
  use super::interaction::{InteractionProfile, audit_interaction_transcript};

  // Case A: Standard accessible profile with valid lines
  let accessible_profile = InteractionProfile::accessibility_profile();
  let valid_transcript = [
    "[INFO] Turn 1 - Observation Ready",
    "[STATUS] Wave: Neutral | Health: 100/100 | Mana: 100/100",
    "[ACTIONS] Valid: contest, stabilize, recall, withdraw",
    "[PROMPT] Enter intent: contest",
  ];
  let report = audit_interaction_transcript(&accessible_profile, &valid_transcript);
  assert!(report.all_passed);
  assert_eq!(report.passed_count, 6);
  assert_eq!(report.failed_count, 0);
  assert_eq!(report.compliance_rate_bp, 10_000);

  // Case B: NoColor profile with ANSI escape sequence -> fails NoColor check
  let ansi_transcript = ["\x1b[32m[INFO] Turn 1\x1b[0m", "[STATUS] Ready"];
  let report_ansi = audit_interaction_transcript(&accessible_profile, &ansi_transcript);
  assert!(!report_ansi.all_passed);
  let no_color_check = report_ansi
    .checks
    .iter()
    .find(|c| c.check_id == "check-no-color-purity")
    .unwrap();
  assert!(!no_color_check.passed);

  // Case C: Overly long line (> 120 chars) -> fails line length check
  let long_line = "a".repeat(125);
  let long_transcript = [long_line.as_str()];
  let report_long = audit_interaction_transcript(&accessible_profile, &long_transcript);
  assert!(!report_long.all_passed);
  let len_check = report_long
    .checks
    .iter()
    .find(|c| c.check_id == "check-line-length-bounds")
    .unwrap();
  assert!(!len_check.passed);

  // Case D: Exceeds concise verbosity line count (> 10 lines)
  let verbose_lines = ["line"; 15];
  let report_verbose = audit_interaction_transcript(&accessible_profile, &verbose_lines);
  assert!(!report_verbose.all_passed);
  let verb_check = report_verbose
    .checks
    .iter()
    .find(|c| c.check_id == "check-verbosity-line-bounds")
    .unwrap();
  assert!(!verb_check.passed);

  // Case E: Keyboard only profile with mouse instruction
  let mouse_transcript = ["[INFO] Ready", "Please click here to select intent"];
  let report_mouse = audit_interaction_transcript(&accessible_profile, &mouse_transcript);
  assert!(!report_mouse.all_passed);
  let kb_check = report_mouse
    .checks
    .iter()
    .find(|c| c.check_id == "check-keyboard-navigation-only")
    .unwrap();
  assert!(!kb_check.passed);

  // Case F: Screen reader friendly profile with ASCII box art
  let box_art_transcript = ["+---+---+", "| A | B |", "+---+---+"];
  let report_art = audit_interaction_transcript(&accessible_profile, &box_art_transcript);
  assert!(!report_art.all_passed);
  let sr_check = report_art
    .checks
    .iter()
    .find(|c| c.check_id == "check-screen-reader-linear-flow")
    .unwrap();
  assert!(!sr_check.passed);
}

#[test]
fn dimension_assessment_validation_and_errors() {
  use super::catalog::STANDARD_ALPHA_PROTOCOL;
  use super::dimension::{
    CognitiveFrictionIndicator, DimensionEvaluationError, DimensionScore,
    ParticipantDimensionAssessment, evaluate_dimension_assessments,
  };

  let proto = STANDARD_ALPHA_PROTOCOL;

  // Empty assessment list
  let err = evaluate_dimension_assessments(&proto, &[]).unwrap_err();
  assert_eq!(err, DimensionEvaluationError::EmptyAssessmentList);

  // Invalid privacy
  let mut invalid_proto = proto;
  invalid_proto.privacy_declaration.deidentified_records_only = false;
  let sample_assessment = ParticipantDimensionAssessment {
    participant_id: "p1",
    cohort: ParticipantCohort::StrategyGamer,
    scores: [
      DimensionScore {
        dimension: EvaluationDimension::Onboarding,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::TerminologyClarity,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::CommandDiscoverability,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::PacingLoad,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::PerceivedAgency,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::DelegatedFairness,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::DebriefCausalUtility,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::KeyboardFlow,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::NonColorSemantics,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
      DimensionScore {
        dimension: EvaluationDimension::ScreenReaderSuitability,
        score_bp: 8000,
        friction: CognitiveFrictionIndicator::None,
        notes: "",
      },
    ],
  };
  let err = evaluate_dimension_assessments(&invalid_proto, &[sample_assessment]).unwrap_err();
  assert_eq!(err, DimensionEvaluationError::InvalidPrivacyDeclaration);

  // Duplicate participant ID
  let sample2 = sample_assessment;
  let err = evaluate_dimension_assessments(&proto, &[sample_assessment, sample2]).unwrap_err();
  assert_eq!(err, DimensionEvaluationError::DuplicateParticipantId("p1"));

  // Score out of range
  let mut invalid_score = sample_assessment;
  invalid_score.participant_id = "p2";
  invalid_score.scores[0].score_bp = 10_500;
  let err = evaluate_dimension_assessments(&proto, &[invalid_score]).unwrap_err();
  assert_eq!(
    err,
    DimensionEvaluationError::ScoreOutOfRange {
      participant_id: "p2",
      dimension: EvaluationDimension::Onboarding,
      score_bp: 10_500,
    }
  );

  // Duplicate dimension in assessment
  let mut dup_dim = sample_assessment;
  dup_dim.participant_id = "p3";
  dup_dim.scores[1].dimension = EvaluationDimension::Onboarding;
  let err = evaluate_dimension_assessments(&proto, &[dup_dim]).unwrap_err();
  assert_eq!(
    err,
    DimensionEvaluationError::DuplicateDimensionInAssessment {
      participant_id: "p3",
      dimension: EvaluationDimension::Onboarding,
    }
  );
}

#[test]
fn dimension_assessment_error_display_coverage() {
  use super::dimension::DimensionEvaluationError;

  let errors = [
    DimensionEvaluationError::EmptyAssessmentList,
    DimensionEvaluationError::DuplicateParticipantId("p-dup"),
    DimensionEvaluationError::ScoreOutOfRange {
      participant_id: "p1",
      dimension: EvaluationDimension::PacingLoad,
      score_bp: 12000,
    },
    DimensionEvaluationError::MissingDimension {
      participant_id: "p1",
      dimension: EvaluationDimension::KeyboardFlow,
    },
    DimensionEvaluationError::DuplicateDimensionInAssessment {
      participant_id: "p1",
      dimension: EvaluationDimension::Onboarding,
    },
    DimensionEvaluationError::InvalidPrivacyDeclaration,
  ];

  for err in errors {
    let msg = format!("{err}");
    assert!(!msg.is_empty());
  }
}

#[test]
fn dimension_catalog_scenarios_execute_and_verify_all_expectations() {
  use super::dimension_catalog::DimensionAssessmentCatalog;

  assert_eq!(DimensionAssessmentCatalog::ALL.len(), 3);

  for def in DimensionAssessmentCatalog::ALL {
    let lookup = DimensionAssessmentCatalog::find_by_id(def.scenario_id);
    assert_eq!(lookup, Some(def));

    let result = DimensionAssessmentCatalog::execute_scenario(def.scenario_id).unwrap();
    assert_eq!(result.scenario_id, def.scenario_id);
    assert!(
      result.all_expectations_met,
      "Scenario {} failed expectations: {:?}",
      def.scenario_id, result
    );
  }

  assert!(DimensionAssessmentCatalog::find_by_id("non-existent").is_none());
}

#[test]
fn dimension_report_and_interaction_audit_markdown_hygiene() {
  use super::dimension_catalog::DimensionAssessmentCatalog;
  use super::interaction::{InteractionProfile, audit_interaction_transcript};

  let result =
    DimensionAssessmentCatalog::execute_scenario("scenario-dimension-alpha-benchmark-v1").unwrap();
  let md = result.report.to_markdown();

  assert!(md.contains("# Usability & Accessibility Dimension Evaluation Report"));
  assert!(md.contains("**Protocol:** `protocol-m10-alpha-v1`"));
  assert!(md.contains("## Dimension Breakdown"));
  assert!(md.contains("## Accessibility Qualification"));
  assert!(md.contains("Accessibility Dimensions Qualified: QUALIFIED"));
  assert!(md.contains("## Evidence Boundary"));

  let profile = InteractionProfile::accessibility_profile();
  let transcript = [
    "[INFO] Turn 1 - Ready",
    "[STATUS] Units: 1 laner, 1 opponent",
    "[PROMPT] > contest",
  ];
  let audit_report = audit_interaction_transcript(&profile, &transcript);
  let audit_md = audit_report.to_markdown();

  assert!(audit_md.contains("# Interaction & Accessibility Audit Report"));
  assert!(audit_md.contains("profile-screen-reader-accessible-v1"));
  assert!(audit_md.contains("## Evaluated Checks"));
  assert!(audit_md.contains("check-no-color-purity"));
}

#[test]
fn informal_check_phases_modes_and_dispositions_round_trip() {
  use super::informal_check::{InformalCheckMode, InformalCheckPhase, NoteDisposition};

  assert_eq!(InformalCheckPhase::ALL.len(), 4);
  for phase in InformalCheckPhase::ALL {
    assert_eq!(format!("{phase}"), phase.as_str());
  }

  assert_eq!(InformalCheckMode::ALL.len(), 3);
  for mode in InformalCheckMode::ALL {
    assert_eq!(format!("{mode}"), mode.as_str());
  }

  assert_eq!(NoteDisposition::ALL.len(), 4);
  for disp in NoteDisposition::ALL {
    assert_eq!(format!("{disp}"), disp.as_str());
    match disp {
      NoteDisposition::AddressedInCode | NoteDisposition::ClarifiedInDoc => {
        assert!(disp.is_addressed());
      }
      NoteDisposition::LoggedForStudy | NoteDisposition::WontFixWithRationale => {
        assert!(!disp.is_addressed());
      }
    }
  }
}

#[test]
fn remediation_targets_and_verification_statuses_round_trip() {
  use super::remediation::{RemediationTarget, RemediationVerificationStatus};

  assert_eq!(RemediationTarget::ALL.len(), 5);
  for target in RemediationTarget::ALL {
    assert_eq!(format!("{target}"), target.as_str());
  }

  assert_eq!(RemediationVerificationStatus::ALL.len(), 4);
  for status in RemediationVerificationStatus::ALL {
    assert_eq!(format!("{status}"), status.as_str());
    match status {
      RemediationVerificationStatus::VerifiedInRegression
      | RemediationVerificationStatus::ValidatedInStudyCohort => {
        assert!(status.is_verified());
      }
      RemediationVerificationStatus::PendingImplementation
      | RemediationVerificationStatus::RejectedAlternative => {
        assert!(!status.is_verified());
      }
    }
  }
}

#[test]
fn remediation_evaluation_validation_and_errors() {
  use super::informal_check::{
    InformalCheckMode, InformalCheckPhase, InformalCheckSession, IssueLinkedNote, NoteDisposition,
  };
  use super::remediation::{
    RemediationAction, RemediationEvaluationError, RemediationTarget,
    RemediationVerificationStatus, evaluate_remediation_plan,
  };

  let valid_session = InformalCheckSession {
    session_id: "s1",
    tester_id: "t1",
    check_mode: InformalCheckMode::InteractiveTty,
    notes: &[IssueLinkedNote {
      note_id: "n1",
      issue_ref: "I-1",
      phase: InformalCheckPhase::InitialOnboarding,
      dimension: EvaluationDimension::Onboarding,
      observation: "Clear instructions",
      disposition: NoteDisposition::AddressedInCode,
    }],
  };

  let valid_action = RemediationAction {
    action_id: "a1",
    note_ref: "n1",
    target: RemediationTarget::DocumentationOnboarding,
    dimension: EvaluationDimension::Onboarding,
    description: "Update docs",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 2_000,
  };

  // Empty sessions
  let err = evaluate_remediation_plan(&[], &[valid_action]).unwrap_err();
  assert_eq!(err, RemediationEvaluationError::EmptySessionList);

  // Empty actions
  let err = evaluate_remediation_plan(&[valid_session], &[]).unwrap_err();
  assert_eq!(err, RemediationEvaluationError::EmptyRemediationList);

  // Empty session notes
  let empty_notes_sess = InformalCheckSession {
    session_id: "s_empty",
    tester_id: "t1",
    check_mode: InformalCheckMode::InteractiveTty,
    notes: &[],
  };
  let err = evaluate_remediation_plan(&[empty_notes_sess], &[valid_action]).unwrap_err();
  assert_eq!(
    err,
    RemediationEvaluationError::EmptySessionNotes {
      session_id: "s_empty"
    }
  );

  // Duplicate session ID
  let err =
    evaluate_remediation_plan(&[valid_session, valid_session], &[valid_action]).unwrap_err();
  assert_eq!(err, RemediationEvaluationError::DuplicateSessionId("s1"));

  // Duplicate note ID
  let dup_note_sess = InformalCheckSession {
    session_id: "s2",
    tester_id: "t2",
    check_mode: InformalCheckMode::PipedStream,
    notes: &[IssueLinkedNote {
      note_id: "n1",
      issue_ref: "I-2",
      phase: InformalCheckPhase::TurnDecisionMaking,
      dimension: EvaluationDimension::PacingLoad,
      observation: "Fast pacing",
      disposition: NoteDisposition::LoggedForStudy,
    }],
  };
  let err =
    evaluate_remediation_plan(&[valid_session, dup_note_sess], &[valid_action]).unwrap_err();
  assert_eq!(err, RemediationEvaluationError::DuplicateNoteId("n1"));

  // Duplicate action ID
  let err = evaluate_remediation_plan(&[valid_session], &[valid_action, valid_action]).unwrap_err();
  assert_eq!(err, RemediationEvaluationError::DuplicateActionId("a1"));

  // Unlinked note reference
  let unlinked_action = RemediationAction {
    action_id: "a2",
    note_ref: "n_unknown",
    target: RemediationTarget::PresentationOutput,
    dimension: EvaluationDimension::NonColorSemantics,
    description: "Add tags",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 1_000,
  };
  let err = evaluate_remediation_plan(&[valid_session], &[unlinked_action]).unwrap_err();
  assert_eq!(
    err,
    RemediationEvaluationError::UnlinkedNoteReference {
      action_id: "a2",
      note_ref: "n_unknown",
    }
  );

  // Invalid impact score (> 10,000 bp)
  let invalid_impact = RemediationAction {
    action_id: "a3",
    note_ref: "n1",
    target: RemediationTarget::DebriefExplanation,
    dimension: EvaluationDimension::DebriefCausalUtility,
    description: "Format table",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 12_000,
  };
  let err = evaluate_remediation_plan(&[valid_session], &[invalid_impact]).unwrap_err();
  assert_eq!(
    err,
    RemediationEvaluationError::InvalidBasisPointImpact {
      action_id: "a3",
      impact_bp: 12_000,
    }
  );

  // Empty description
  let empty_desc = RemediationAction {
    action_id: "a4",
    note_ref: "n1",
    target: RemediationTarget::CommandVocabulary,
    dimension: EvaluationDimension::CommandDiscoverability,
    description: "",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 500,
  };
  let err = evaluate_remediation_plan(&[valid_session], &[empty_desc]).unwrap_err();
  assert_eq!(
    err,
    RemediationEvaluationError::EmptyDescription { action_id: "a4" }
  );

  // Empty observation
  let empty_obs_sess = InformalCheckSession {
    session_id: "s_empty_obs",
    tester_id: "t3",
    check_mode: InformalCheckMode::InteractiveTty,
    notes: &[IssueLinkedNote {
      note_id: "n_empty_obs",
      issue_ref: "I-3",
      phase: InformalCheckPhase::DebriefAnalysis,
      dimension: EvaluationDimension::DebriefCausalUtility,
      observation: "",
      disposition: NoteDisposition::ClarifiedInDoc,
    }],
  };
  let err = evaluate_remediation_plan(&[empty_obs_sess], &[valid_action]).unwrap_err();
  assert_eq!(
    err,
    RemediationEvaluationError::EmptyObservation {
      note_id: "n_empty_obs"
    }
  );
}

#[test]
fn remediation_error_display_coverage() {
  use super::remediation::RemediationEvaluationError;

  let errors = [
    RemediationEvaluationError::EmptySessionList,
    RemediationEvaluationError::EmptyRemediationList,
    RemediationEvaluationError::EmptySessionNotes { session_id: "s1" },
    RemediationEvaluationError::DuplicateSessionId("s1"),
    RemediationEvaluationError::DuplicateNoteId("n1"),
    RemediationEvaluationError::DuplicateActionId("a1"),
    RemediationEvaluationError::UnlinkedNoteReference {
      action_id: "a1",
      note_ref: "n_missing",
    },
    RemediationEvaluationError::InvalidBasisPointImpact {
      action_id: "a1",
      impact_bp: 15_000,
    },
    RemediationEvaluationError::EmptyDescription { action_id: "a1" },
    RemediationEvaluationError::EmptyObservation { note_id: "n1" },
  ];

  for err in errors {
    let msg = format!("{err}");
    assert!(!msg.is_empty());
  }
}

#[test]
fn remediation_catalog_scenarios_execute_and_verify_all_expectations() {
  use super::remediation_catalog::RemediationCatalog;

  assert_eq!(RemediationCatalog::ALL.len(), 3);

  for def in RemediationCatalog::ALL {
    let lookup = RemediationCatalog::find_scenario(def.scenario_id);
    assert_eq!(lookup, Some(&def));

    let result = RemediationCatalog::execute_scenario(def.scenario_id).unwrap();
    assert_eq!(result.scenario_id, def.scenario_id);
    assert!(
      result.expectations_met,
      "Scenario {} failed expectations: {:?}",
      def.scenario_id, result
    );
  }

  assert!(RemediationCatalog::find_scenario("non-existent").is_none());
}

#[test]
fn remediation_report_markdown_hygiene() {
  use super::remediation_catalog::RemediationCatalog;

  let result =
    RemediationCatalog::execute_scenario(RemediationCatalog::SCENARIO_ALPHA_BASELINE_V1).unwrap();
  let md = result.report.to_markdown();

  assert!(md.contains("# Informal Check & Remediation Evaluation Report"));
  assert!(md.contains("**Schema:** `m10-remediation-evaluation-v1`"));
  assert!(md.contains("## Notes by Disposition"));
  assert!(md.contains("## Actions by Target"));
  assert!(md.contains("## Actions by Verification Status"));
  assert!(md.contains("## Remediation Readiness Gate"));
  assert!(md.contains("Remediation Readiness Gate: PASS"));
}

#[test]
fn untested_population_categories_and_disclosures_round_trip() {
  use super::sampling::{
    M10_SAMPLING_LIMITS_SCHEMA_V1, SamplingLimitsDeclaration, UntestedPopulationCategory,
  };

  assert_eq!(M10_SAMPLING_LIMITS_SCHEMA_V1, "m10-sampling-limits-v1");
  assert_eq!(UntestedPopulationCategory::ALL.len(), 5);

  for cat in UntestedPopulationCategory::ALL {
    let name = cat.as_str();
    assert_eq!(format!("{cat}"), name);
  }

  let decl = SamplingLimitsDeclaration::standard_alpha();
  assert_eq!(decl.declaration_id, "m10-alpha-sampling-limits-v1");
  assert_eq!(decl.untested_disclosures.len(), 5);
}

#[test]
fn sampling_limits_evaluation_and_validation() {
  use super::protocol::ParticipantCohort;
  use super::sampling::{
    SamplingEvaluationError, SamplingLimitsDeclaration, UntestedPopulationCategory,
    UntestedPopulationDisclosure, evaluate_participant_sampling,
  };
  use super::session::{AccessNeedsDeclaration, CompletionStatus, ParticipantSessionRecord};

  let decl = SamplingLimitsDeclaration::standard_alpha();

  // Valid 4-participant sample
  let sessions = [
    ParticipantSessionRecord {
      participant_id: "p1",
      cohort: ParticipantCohort::StrategyGamer,
      access_needs: AccessNeedsDeclaration::none(),
      completion_status: CompletionStatus::Completed,
      explanation_quality_bp: 9_000,
      debrief_comprehension_bp: 9_000,
      turns_completed: 10,
    },
    ParticipantSessionRecord {
      participant_id: "p2",
      cohort: ParticipantCohort::MobaPlayer,
      access_needs: AccessNeedsDeclaration::none(),
      completion_status: CompletionStatus::Completed,
      explanation_quality_bp: 8_500,
      debrief_comprehension_bp: 8_500,
      turns_completed: 10,
    },
    ParticipantSessionRecord {
      participant_id: "p3",
      cohort: ParticipantCohort::AccessNeeds,
      access_needs: AccessNeedsDeclaration {
        screen_reader_user: true,
        color_vision_deficiency: false,
        keyboard_only_user: true,
        reduced_motion_required: false,
      },
      completion_status: CompletionStatus::Completed,
      explanation_quality_bp: 8_000,
      debrief_comprehension_bp: 8_000,
      turns_completed: 10,
    },
    ParticipantSessionRecord {
      participant_id: "p4",
      cohort: ParticipantCohort::NoviceStrategy,
      access_needs: AccessNeedsDeclaration::none(),
      completion_status: CompletionStatus::Completed,
      explanation_quality_bp: 7_500,
      debrief_comprehension_bp: 7_500,
      turns_completed: 10,
    },
  ];

  let report = evaluate_participant_sampling(&decl, &sessions).unwrap();
  assert_eq!(report.sample_size, 4);
  assert!(report.all_cohorts_meet_floor);
  assert!(report.has_access_needs_representation);
  assert!(report.sampling_gate_passed);
  assert_eq!(report.access_needs_breakdown.screen_reader_users, 1);
  assert_eq!(report.access_needs_breakdown.total_with_access_needs, 1);
  assert_eq!(report.access_needs_breakdown.access_needs_share_bp, 2_500);

  // Empty sessions error
  let err = evaluate_participant_sampling(&decl, &[]).unwrap_err();
  assert_eq!(err, SamplingEvaluationError::EmptySessionList);

  // Empty methodology error
  let empty_method_decl = SamplingLimitsDeclaration {
    declaration_id: "d1",
    methodology: "   ",
    target_sample_size: 4,
    min_cohort_floor_bp: 1_500,
    untested_disclosures: decl.untested_disclosures,
  };
  let err = evaluate_participant_sampling(&empty_method_decl, &sessions).unwrap_err();
  assert_eq!(err, SamplingEvaluationError::EmptyMethodology);

  // Empty disclosures error
  let empty_disc_decl = SamplingLimitsDeclaration {
    declaration_id: "d1",
    methodology: "Valid methodology",
    target_sample_size: 4,
    min_cohort_floor_bp: 1_500,
    untested_disclosures: &[],
  };
  let err = evaluate_participant_sampling(&empty_disc_decl, &sessions).unwrap_err();
  assert_eq!(err, SamplingEvaluationError::EmptyUntestedDisclosures);

  // Duplicate category error
  static DUP_DISC: [UntestedPopulationDisclosure; 2] = [
    UntestedPopulationDisclosure {
      category: UntestedPopulationCategory::NonEnglishLocale,
      rationale: "English only",
      future_mitigation_plan: "Localization",
    },
    UntestedPopulationDisclosure {
      category: UntestedPopulationCategory::NonEnglishLocale,
      rationale: "Duplicate",
      future_mitigation_plan: "None",
    },
  ];
  let dup_decl = SamplingLimitsDeclaration {
    declaration_id: "d1",
    methodology: "Valid methodology",
    target_sample_size: 4,
    min_cohort_floor_bp: 1_500,
    untested_disclosures: &DUP_DISC,
  };
  let err = evaluate_participant_sampling(&dup_decl, &sessions).unwrap_err();
  assert_eq!(
    err,
    SamplingEvaluationError::DuplicateUntestedCategory(
      UntestedPopulationCategory::NonEnglishLocale
    )
  );

  // Empty disclosure text error
  static EMPTY_TEXT_DISC: [UntestedPopulationDisclosure; 1] = [UntestedPopulationDisclosure {
    category: UntestedPopulationCategory::MobileTouchInterface,
    rationale: "   ",
    future_mitigation_plan: "Web UI",
  }];
  let empty_text_decl = SamplingLimitsDeclaration {
    declaration_id: "d1",
    methodology: "Valid methodology",
    target_sample_size: 4,
    min_cohort_floor_bp: 1_500,
    untested_disclosures: &EMPTY_TEXT_DISC,
  };
  let err = evaluate_participant_sampling(&empty_text_decl, &sessions).unwrap_err();
  assert_eq!(
    err,
    SamplingEvaluationError::EmptyDisclosureText(UntestedPopulationCategory::MobileTouchInterface)
  );
}

#[test]
fn sampling_error_display_coverage() {
  use super::sampling::{SamplingEvaluationError, UntestedPopulationCategory};

  let errors = [
    SamplingEvaluationError::EmptySessionList,
    SamplingEvaluationError::EmptyMethodology,
    SamplingEvaluationError::EmptyUntestedDisclosures,
    SamplingEvaluationError::DuplicateUntestedCategory(
      UntestedPopulationCategory::NonEnglishLocale,
    ),
    SamplingEvaluationError::EmptyDisclosureText(UntestedPopulationCategory::MobileTouchInterface),
  ];

  for err in errors {
    let msg = format!("{err}");
    assert!(!msg.is_empty());
  }
}

#[test]
fn synthesis_readiness_gates_and_disposition_logic() {
  use super::catalog::StudyProtocolCatalog;
  use super::dimension_catalog::DimensionAssessmentCatalog;
  use super::interaction::{
    ContrastMode, InteractionProfile, VerbosityLevel, audit_interaction_transcript,
  };
  use super::remediation_catalog::RemediationCatalog;
  use super::sampling::{SamplingLimitsDeclaration, evaluate_participant_sampling};
  use super::synthesis::{
    AlphaDisposition, M10_ALPHA_SYNTHESIS_SCHEMA_V1, SynthesisEvaluationError,
    synthesize_alpha_evidence,
  };

  assert_eq!(M10_ALPHA_SYNTHESIS_SCHEMA_V1, "m10-alpha-synthesis-v1");
  assert_eq!(AlphaDisposition::ALL.len(), 3);
  for disp in AlphaDisposition::ALL {
    assert_eq!(format!("{disp}"), disp.as_str());
  }

  let study_res =
    StudyProtocolCatalog::execute_scenario("scenario-study-cohort-balanced-alpha-v1").unwrap();
  let dim_res =
    DimensionAssessmentCatalog::execute_scenario("scenario-dimension-alpha-benchmark-v1").unwrap();
  let rem_res =
    RemediationCatalog::execute_scenario("scenario-remediation-alpha-baseline-v1").unwrap();

  let profile = InteractionProfile {
    profile_id: "test-profile-v1",
    verbosity: VerbosityLevel::Standard,
    contrast_mode: ContrastMode::NoColor,
    keyboard_only: true,
    screen_reader_friendly: true,
  };
  let transcript_lines = ["[OK] turn: 1 | status: open", "[OK] action: Stabilize"];
  let interaction_report = audit_interaction_transcript(&profile, &transcript_lines);

  let (sessions, _) = StudyProtocolCatalog::balanced_alpha_data();
  let sampling_decl = SamplingLimitsDeclaration::standard_alpha();
  let sampling_report = evaluate_participant_sampling(&sampling_decl, &sessions).unwrap();

  static HYPOTHESES: [&str; 2] = ["Hypothesis 1", "Hypothesis 2"];

  let synthesis = synthesize_alpha_evidence(
    "synth-01",
    study_res.report.clone(),
    dim_res.report.clone(),
    interaction_report.clone(),
    rem_res.report.clone(),
    sampling_report.clone(),
    &HYPOTHESES,
  )
  .unwrap();

  assert_eq!(synthesis.disposition, AlphaDisposition::AlphaReady);
  assert!(synthesis.gates.all_gates_passed());

  // Empty synthesis ID error
  let err = synthesize_alpha_evidence(
    "",
    study_res.report.clone(),
    dim_res.report.clone(),
    interaction_report.clone(),
    rem_res.report.clone(),
    sampling_report.clone(),
    &HYPOTHESES,
  )
  .unwrap_err();
  assert_eq!(err, SynthesisEvaluationError::EmptySynthesisId);

  // Sample size mismatch error
  let mut mismatched_sampling_report = sampling_report.clone();
  mismatched_sampling_report.sample_size = 99;
  let err = synthesize_alpha_evidence(
    "synth-01",
    study_res.report.clone(),
    dim_res.report.clone(),
    interaction_report.clone(),
    rem_res.report.clone(),
    mismatched_sampling_report,
    &HYPOTHESES,
  )
  .unwrap_err();
  assert_eq!(
    err,
    SynthesisEvaluationError::SampleSizeMismatch {
      study_sample_size: 8,
      sampling_sample_size: 99,
    }
  );

  // Empty hypotheses error
  let err = synthesize_alpha_evidence(
    "synth-01",
    study_res.report,
    dim_res.report,
    interaction_report,
    rem_res.report,
    sampling_report,
    &[],
  )
  .unwrap_err();
  assert_eq!(err, SynthesisEvaluationError::EmptyInferredHypotheses);
}

#[test]
fn synthesis_error_display_coverage() {
  use super::synthesis::SynthesisEvaluationError;

  let errors = [
    SynthesisEvaluationError::EmptySynthesisId,
    SynthesisEvaluationError::SampleSizeMismatch {
      study_sample_size: 10,
      sampling_sample_size: 12,
    },
    SynthesisEvaluationError::EmptyInferredHypotheses,
  ];

  for err in errors {
    let msg = format!("{err}");
    assert!(!msg.is_empty());
  }
}

#[test]
fn synthesis_catalog_scenarios_execute_and_verify_all_expectations() {
  use super::synthesis_catalog::{AlphaSynthesisCatalog, M10_SYNTHESIS_CATALOG_SCHEMA_V1};

  assert_eq!(M10_SYNTHESIS_CATALOG_SCHEMA_V1, "m10-synthesis-catalog-v1");
  assert_eq!(AlphaSynthesisCatalog::ALL.len(), 3);

  for def in AlphaSynthesisCatalog::ALL {
    let lookup = AlphaSynthesisCatalog::find_by_id(def.scenario_id);
    assert_eq!(lookup, Some(def));

    let result = AlphaSynthesisCatalog::execute_scenario(def.scenario_id).unwrap();
    assert_eq!(result.scenario_id, def.scenario_id);
    assert!(
      result.all_expectations_met,
      "Scenario {} failed expectations: {:?}",
      def.scenario_id, result
    );
  }

  assert!(AlphaSynthesisCatalog::find_by_id("non-existent").is_none());
}

#[test]
fn sampling_and_synthesis_markdown_hygiene() {
  use super::synthesis_catalog::{AlphaSynthesisCatalog, SCENARIO_ALPHA_SYNTHESIS_BASELINE};

  let result =
    AlphaSynthesisCatalog::execute_scenario(SCENARIO_ALPHA_SYNTHESIS_BASELINE.scenario_id).unwrap();

  let sampling_md = result.synthesis.sampling_report.render_markdown();
  assert!(sampling_md.contains("# M10 Participant Sampling & Limitations Report"));
  assert!(sampling_md.contains("## Cohort Representation Breakdown"));
  assert!(sampling_md.contains("## Access Needs Distribution"));
  assert!(sampling_md.contains("## Untested Populations & Alpha Claim Boundaries"));

  let synth_md = result.synthesis.render_markdown();
  assert!(synth_md.contains("# M10 Human Usability & Accessibility Alpha Evidence Synthesis"));
  assert!(synth_md.contains("## Readiness Gates Evaluation"));
  assert!(synth_md.contains("## Empirical Facts vs Inferred Design Hypotheses"));
  assert!(synth_md.contains("### Observed Empirical Facts"));
  assert!(synth_md.contains("### Inferred Design Hypotheses"));
  assert!(synth_md.contains("## Untested Populations Disclosure"));
}

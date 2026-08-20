//! Unit and integration tests for M12 Public Alpha release governance, compatibility matrix, and data dictionary.

use crate::alpha::catalog::{AlphaScenarioCatalog, render_alpha_scenario_markdown};
use crate::alpha::compatibility::{
  ALPHA_COMPATIBILITY_SCHEMA_VERSION, CompatibilityDomain, CompatibilityError, CompatibilityLevel,
  evaluate_compatibility_matrix, render_compatibility_report_markdown,
};
use crate::alpha::data_dictionary::{
  ALPHA_DATA_DICTIONARY_SCHEMA_VERSION, DataCategory, DataDictionaryError, DataSensitivityLevel,
  audit_data_dictionary, render_data_dictionary_markdown,
};
use crate::alpha::governance::{
  ALPHA_GOVERNANCE_SCHEMA_VERSION, AlphaGovernanceError, LegalPostureStatus, PolicyComplianceArea,
  evaluate_alpha_governance, render_governance_report_markdown,
};

#[test]
fn policy_compliance_area_round_trips() {
  for area in PolicyComplianceArea::all() {
    let s = area.as_str();
    assert_eq!(PolicyComplianceArea::parse(s), Some(area));
    assert_eq!(area.to_string(), s);
  }
  assert_eq!(PolicyComplianceArea::parse("unknown-area"), None);
}

#[test]
fn legal_posture_status_round_trips() {
  let postures = [
    LegalPostureStatus::CompliantPermissive,
    LegalPostureStatus::OriginalFallbackRequired,
    LegalPostureStatus::PendingClearance,
    LegalPostureStatus::DistributionBlocked,
  ];
  for posture in postures {
    let s = posture.as_str();
    assert_eq!(LegalPostureStatus::parse(s), Some(posture));
    assert_eq!(posture.to_string(), s);
  }
  assert_eq!(LegalPostureStatus::parse("invalid-posture"), None);

  assert!(LegalPostureStatus::CompliantPermissive.is_distributable());
  assert!(LegalPostureStatus::OriginalFallbackRequired.is_distributable());
  assert!(!LegalPostureStatus::PendingClearance.is_distributable());
  assert!(!LegalPostureStatus::DistributionBlocked.is_distributable());
}

#[test]
fn compatibility_domain_round_trips() {
  let domains = [
    CompatibilityDomain::Ruleset,
    CompatibilityDomain::Scenario,
    CompatibilityDomain::ProtocolDto,
    CompatibilityDomain::AgentProfile,
    CompatibilityDomain::PromptTemplate,
    CompatibilityDomain::ModelCalibration,
    CompatibilityDomain::ReplayArtifact,
    CompatibilityDomain::GuiPresentation,
  ];
  for domain in domains {
    let s = domain.as_str();
    assert_eq!(CompatibilityDomain::parse(s), Some(domain));
    assert_eq!(domain.to_string(), s);
  }
  assert_eq!(CompatibilityDomain::parse("invalid-domain"), None);
}

#[test]
fn compatibility_level_round_trips() {
  let levels = [
    CompatibilityLevel::FullyCompatible,
    CompatibilityLevel::BackwardCompatibleOnly,
    CompatibilityLevel::BreakingChangeMigrationRequired,
    CompatibilityLevel::DeprecatedUnsupported,
  ];
  for level in levels {
    let s = level.as_str();
    assert_eq!(CompatibilityLevel::parse(s), Some(level));
    assert_eq!(level.to_string(), s);
  }
  assert_eq!(CompatibilityLevel::parse("invalid-level"), None);

  assert!(CompatibilityLevel::FullyCompatible.is_executable());
  assert!(CompatibilityLevel::BackwardCompatibleOnly.is_executable());
  assert!(CompatibilityLevel::BreakingChangeMigrationRequired.is_executable());
  assert!(!CompatibilityLevel::DeprecatedUnsupported.is_executable());
}

#[test]
fn data_category_round_trips() {
  let categories = [
    DataCategory::AuthoritativeState,
    DataCategory::ObservationProjection,
    DataCategory::IntentCommand,
    DataCategory::EventLog,
    DataCategory::CausalDebrief,
    DataCategory::ReplayRecord,
    DataCategory::ProtocolDto,
    DataCategory::GuiPresentationBundle,
  ];
  for cat in categories {
    let s = cat.as_str();
    assert_eq!(DataCategory::parse(s), Some(cat));
    assert_eq!(cat.to_string(), s);
  }
  assert_eq!(DataCategory::parse("invalid-category"), None);
}

#[test]
fn data_sensitivity_level_round_trips() {
  let levels = [
    DataSensitivityLevel::PublicActorVisible,
    DataSensitivityLevel::TeamVisibleShared,
    DataSensitivityLevel::LatentHostAuthoritative,
    DataSensitivityLevel::ResearchInspectionOnly,
  ];
  for level in levels {
    let s = level.as_str();
    assert_eq!(DataSensitivityLevel::parse(s), Some(level));
    assert_eq!(level.to_string(), s);
  }
  assert_eq!(DataSensitivityLevel::parse("invalid-sensitivity"), None);

  assert!(!DataSensitivityLevel::PublicActorVisible.requires_fog_redaction());
  assert!(!DataSensitivityLevel::TeamVisibleShared.requires_fog_redaction());
  assert!(DataSensitivityLevel::LatentHostAuthoritative.requires_fog_redaction());
  assert!(DataSensitivityLevel::ResearchInspectionOnly.requires_fog_redaction());
}

#[test]
fn governance_compliant_evaluation_succeeds() {
  let manifest = AlphaScenarioCatalog::build_compliant_manifest();
  let report = evaluate_alpha_governance(&manifest).expect("compliant governance must pass");

  assert_eq!(report.schema_version, ALPHA_GOVERNANCE_SCHEMA_VERSION);
  assert_eq!(report.compliance_score_bp, 10_000);
  assert_eq!(report.verified_areas_count, 6);
  assert_eq!(report.total_declared_count, 6);
  assert_eq!(
    report.posture_status,
    LegalPostureStatus::CompliantPermissive
  );
  assert!(report.is_release_eligible);
  assert!(report.missing_areas.is_empty());
}

#[test]
fn governance_fallback_evaluation_succeeds() {
  let manifest = AlphaScenarioCatalog::build_fallback_manifest();
  let report = evaluate_alpha_governance(&manifest).expect("fallback governance must pass");

  assert_eq!(report.compliance_score_bp, 6_666);
  assert_eq!(report.verified_areas_count, 4);
  assert_eq!(report.total_declared_count, 6);
  assert_eq!(
    report.posture_status,
    LegalPostureStatus::OriginalFallbackRequired
  );
  assert!(!report.missing_areas.is_empty() || report.total_declared_count == 6);
}

#[test]
fn governance_fail_closed_validation() {
  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.declarations.clear();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::EmptyManifest)
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.fallback_universe_name = "  ".to_string();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::EmptyFallbackUniverse)
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.repository_license = "  ".to_string();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::EmptyLicense)
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.repository_license = "GPL-3.0".to_string();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::InvalidLicense("GPL-3.0".to_string()))
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.declarations[0].declaration_id = "".to_string();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::EmptyDeclarationId)
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.declarations[0].title = "   ".to_string();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::EmptyTitle)
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.declarations[0].reference_uri = "".to_string();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::EmptyReferenceUri)
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.declarations[0].rationale = "".to_string();
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::EmptyRationale)
  );

  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.declarations[1].area = PolicyComplianceArea::LicenseNotice;
  assert_eq!(
    evaluate_alpha_governance(&m),
    Err(AlphaGovernanceError::DuplicateArea(
      PolicyComplianceArea::LicenseNotice
    ))
  );

  // Unverified license notice blocks distribution
  let mut m = AlphaScenarioCatalog::build_compliant_manifest();
  m.declarations[0].verified = false;
  let report = evaluate_alpha_governance(&m).expect("evaluation succeeds with blocked posture");
  assert_eq!(
    report.posture_status,
    LegalPostureStatus::DistributionBlocked
  );
  assert!(!report.is_release_eligible);
}

#[test]
fn governance_error_display_coverage() {
  let errors = [
    AlphaGovernanceError::EmptyManifest,
    AlphaGovernanceError::EmptyDeclarationId,
    AlphaGovernanceError::DuplicateArea(PolicyComplianceArea::LicenseNotice),
    AlphaGovernanceError::EmptyTitle,
    AlphaGovernanceError::EmptyReferenceUri,
    AlphaGovernanceError::EmptyRationale,
    AlphaGovernanceError::EmptyFallbackUniverse,
    AlphaGovernanceError::EmptyLicense,
    AlphaGovernanceError::InvalidLicense("GPL-3.0".to_string()),
  ];
  for err in errors {
    let s = err.to_string();
    assert!(!s.is_empty());
  }
}

#[test]
fn compatibility_evaluation_succeeds() {
  let matrix = AlphaScenarioCatalog::build_canonical_compatibility_matrix();
  let report = evaluate_compatibility_matrix(&matrix).expect("matrix evaluation must pass");

  assert_eq!(report.schema_version, ALPHA_COMPATIBILITY_SCHEMA_VERSION);
  assert_eq!(report.total_entries, 4);
  assert_eq!(report.fully_compatible_count, 2);
  assert_eq!(report.backward_compatible_count, 1);
  assert_eq!(report.breaking_with_migration_count, 1);
  assert_eq!(report.deprecated_count, 0);
  assert!(report.is_matrix_sound);
}

#[test]
fn compatibility_fail_closed_validation() {
  let mut mat = AlphaScenarioCatalog::build_canonical_compatibility_matrix();
  mat.entries.clear();
  assert_eq!(
    evaluate_compatibility_matrix(&mat),
    Err(CompatibilityError::EmptyMatrix)
  );

  let mut mat = AlphaScenarioCatalog::build_canonical_compatibility_matrix();
  mat.entries[0].source_version = "".to_string();
  assert_eq!(
    evaluate_compatibility_matrix(&mat),
    Err(CompatibilityError::EmptySourceVersion)
  );

  let mut mat = AlphaScenarioCatalog::build_canonical_compatibility_matrix();
  mat.entries[0].target_version = "".to_string();
  assert_eq!(
    evaluate_compatibility_matrix(&mat),
    Err(CompatibilityError::EmptyTargetVersion)
  );

  let mut mat = AlphaScenarioCatalog::build_canonical_compatibility_matrix();
  mat.entries[0].notes = "   ".to_string();
  assert_eq!(
    evaluate_compatibility_matrix(&mat),
    Err(CompatibilityError::EmptyNotes)
  );

  let mut mat = AlphaScenarioCatalog::build_canonical_compatibility_matrix();
  mat.entries[0].migration_contract_id = None;
  assert_eq!(
    evaluate_compatibility_matrix(&mat),
    Err(CompatibilityError::MissingMigrationContract(
      CompatibilityDomain::Ruleset,
      "1".to_string(),
      "4".to_string()
    ))
  );

  let mut mat = AlphaScenarioCatalog::build_canonical_compatibility_matrix();
  mat.entries.push(mat.entries[0].clone());
  assert_eq!(
    evaluate_compatibility_matrix(&mat),
    Err(CompatibilityError::DuplicateDomainVersionPair(
      CompatibilityDomain::Ruleset,
      "1".to_string(),
      "4".to_string()
    ))
  );
}

#[test]
fn compatibility_error_display_coverage() {
  let errors = [
    CompatibilityError::EmptyMatrix,
    CompatibilityError::EmptySourceVersion,
    CompatibilityError::EmptyTargetVersion,
    CompatibilityError::DuplicateDomainVersionPair(
      CompatibilityDomain::Ruleset,
      "1".to_string(),
      "2".to_string(),
    ),
    CompatibilityError::MissingMigrationContract(
      CompatibilityDomain::Ruleset,
      "1".to_string(),
      "2".to_string(),
    ),
    CompatibilityError::EmptyNotes,
  ];
  for err in errors {
    let s = err.to_string();
    assert!(!s.is_empty());
  }
}

#[test]
fn data_dictionary_audit_succeeds() {
  let dict = AlphaScenarioCatalog::build_canonical_data_dictionary();
  let report = audit_data_dictionary(&dict).expect("data dictionary audit must pass");

  assert_eq!(report.schema_version, ALPHA_DATA_DICTIONARY_SCHEMA_VERSION);
  assert_eq!(report.total_fields, 12);
  assert_eq!(report.authoritative_count, 5);
  assert_eq!(report.observation_count, 1);
  assert_eq!(report.latent_count, 2);
  assert_eq!(report.public_count, 8);
  assert_eq!(report.team_count, 1);
  assert_eq!(report.research_count, 1);
  assert!(report.is_audit_passed);
}

#[test]
fn data_dictionary_fail_closed_validation() {
  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields.clear();
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::EmptyDictionary)
  );

  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields[0].field_name = "".to_string();
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::EmptyFieldName)
  );

  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields[0].type_signature = "".to_string();
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::EmptyTypeSignature(
      "world_state.turn".to_string()
    ))
  );

  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields[0].value_bounds = "".to_string();
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::EmptyValueBounds(
      "world_state.turn".to_string()
    ))
  );

  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields[0].description = "".to_string();
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::EmptyDescription(
      "world_state.turn".to_string()
    ))
  );

  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields[0].redaction_rule = "".to_string();
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::EmptyRedactionRule(
      "world_state.turn".to_string()
    ))
  );

  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields.push(d.fields[0].clone());
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::DuplicateFieldName(
      "world_state.turn".to_string()
    ))
  );

  // Redaction invariant violation: LatentHostAuthoritative with "none"
  let mut d = AlphaScenarioCatalog::build_canonical_data_dictionary();
  d.fields[3].redaction_rule = "none".to_string();
  assert_eq!(
    audit_data_dictionary(&d),
    Err(DataDictionaryError::InvalidSensitivityRedactionPair {
      field_name: "opponent_state.position".to_string(),
      sensitivity: DataSensitivityLevel::LatentHostAuthoritative,
      redaction_rule: "none".to_string(),
    })
  );
}

#[test]
fn data_dictionary_error_display_coverage() {
  let errors = [
    DataDictionaryError::EmptyDictionary,
    DataDictionaryError::EmptyFieldName,
    DataDictionaryError::DuplicateFieldName("f1".to_string()),
    DataDictionaryError::EmptyTypeSignature("f1".to_string()),
    DataDictionaryError::EmptyValueBounds("f1".to_string()),
    DataDictionaryError::EmptyDescription("f1".to_string()),
    DataDictionaryError::EmptyRedactionRule("f1".to_string()),
    DataDictionaryError::InvalidSensitivityRedactionPair {
      field_name: "f1".to_string(),
      sensitivity: DataSensitivityLevel::LatentHostAuthoritative,
      redaction_rule: "none".to_string(),
    },
  ];
  for err in errors {
    let s = err.to_string();
    assert!(!s.is_empty());
  }
}

#[test]
fn catalog_scenarios_execute_and_verify_all_expectations() {
  for scenario in AlphaScenarioCatalog::ALL {
    assert_eq!(
      AlphaScenarioCatalog::lookup(scenario.scenario_id),
      Some(&scenario)
    );
    let md = render_alpha_scenario_markdown(&scenario);
    assert!(!md.is_empty());
    assert!(!md.contains('\x1b'));
  }

  let rep1 = AlphaScenarioCatalog::execute_governance_compliant().expect("compliant scenario");
  assert!(rep1.is_release_eligible);

  let rep2 = AlphaScenarioCatalog::execute_governance_fallback().expect("fallback scenario");
  assert_eq!(
    rep2.posture_status,
    LegalPostureStatus::OriginalFallbackRequired
  );

  let rep3 = AlphaScenarioCatalog::execute_compatibility().expect("compatibility scenario");
  assert!(rep3.is_matrix_sound);

  let rep4 = AlphaScenarioCatalog::execute_data_dictionary().expect("dictionary scenario");
  assert!(rep4.is_audit_passed);
}

#[test]
fn markdown_report_rendering_hygiene() {
  let gov_rep = AlphaScenarioCatalog::execute_governance_compliant().unwrap();
  let gov_md = render_governance_report_markdown(&gov_rep);
  assert!(gov_md.starts_with("# Public Alpha Governance Evaluation Report"));
  assert!(!gov_md.contains('\x1b'));

  let comp_rep = AlphaScenarioCatalog::execute_compatibility().unwrap();
  let comp_md = render_compatibility_report_markdown(&comp_rep);
  assert!(comp_md.starts_with("# Public Alpha Compatibility Evaluation Report"));
  assert!(!comp_md.contains('\x1b'));

  let dict_rep = AlphaScenarioCatalog::execute_data_dictionary().unwrap();
  let dict_md = render_data_dictionary_markdown(&dict_rep);
  assert!(dict_md.starts_with("# Public Alpha Data Dictionary Audit Report"));
  assert!(!dict_md.contains('\x1b'));
}

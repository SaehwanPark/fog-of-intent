use crate::alpha::archive::{
  ALPHA_ARCHIVE_SCHEMA_VERSION, AlphaArchiveError, ArchiveCategoryKind,
  audit_release_archive_manifest, canonical_alpha_release_archive_manifest, compute_fnv1a_16hex,
  is_valid_16hex, render_release_archive_report_markdown,
};
use crate::alpha::catalog::{AlphaScenarioCatalog, render_alpha_scenario_markdown};
use crate::alpha::checks::{
  ALPHA_RELEASE_CHECKS_SCHEMA_VERSION, AlphaReleaseChecksError, CheckVerificationStatus,
  ReleaseCheckCategory, ReleaseCheckDefinition, ReleaseCheckSeverity, audit_release_checks,
  render_release_checks_report_markdown,
};
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
use crate::alpha::guides::{
  ALPHA_GUIDES_SCHEMA_VERSION, AlphaGuidesError, AlphaGuidesManifest, GuideAudience,
  GuideDocumentDefinition, GuideSection, GuideSectionKind, audit_guide_manifests,
  render_guides_report_markdown,
};
use crate::alpha::limitations::{
  ALPHA_LIMITATIONS_SCHEMA_VERSION, AlphaLimitationsError, ClaimClassification, EvidenceTier,
  LimitationCategory, audit_limitations_and_boundaries, render_limitations_report_markdown,
};
use crate::alpha::reproducibility::{
  ALPHA_REPRODUCIBILITY_SCHEMA_VERSION, AlphaReproducibilityError, ReproducibilityBundleManifest,
  ReproducibilityPackageDefinition, ReproducibilityStatus, SampleArtifactKind,
  audit_reproducibility_bundle, is_valid_fnv1a_hash, render_reproducibility_report_markdown,
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
fn limitation_category_round_trips() {
  for cat in LimitationCategory::all() {
    let s = cat.as_str();
    assert_eq!(LimitationCategory::parse(s), Some(cat));
    assert_eq!(cat.to_string(), s);
  }
  assert_eq!(LimitationCategory::parse("invalid-limitation"), None);
}

#[test]
fn evidence_tier_round_trips() {
  let tiers = [
    EvidenceTier::SoftwareInvariants,
    EvidenceTier::SyntheticAgentPlaytest,
    EvidenceTier::EmpiricalCalibration,
    EvidenceTier::LimitedHumanStudy,
    EvidenceTier::UnverifiedHypothesis,
  ];
  for tier in tiers {
    let s = tier.as_str();
    assert_eq!(EvidenceTier::parse(s), Some(tier));
    assert_eq!(tier.to_string(), s);
  }
  assert_eq!(EvidenceTier::parse("invalid-tier"), None);

  assert!(EvidenceTier::SoftwareInvariants.is_empirical());
  assert!(EvidenceTier::SyntheticAgentPlaytest.is_empirical());
  assert!(EvidenceTier::EmpiricalCalibration.is_empirical());
  assert!(EvidenceTier::LimitedHumanStudy.is_empirical());
  assert!(!EvidenceTier::UnverifiedHypothesis.is_empirical());
}

#[test]
fn claim_classification_round_trips() {
  let classifications = [
    ClaimClassification::PermissibleBoundedClaim,
    ClaimClassification::ConditionalWithDisclaimer,
    ClaimClassification::ImpermissibleOverclaim,
  ];
  for class in classifications {
    let s = class.as_str();
    assert_eq!(ClaimClassification::parse(s), Some(class));
    assert_eq!(class.to_string(), s);
  }
  assert_eq!(ClaimClassification::parse("invalid-classification"), None);

  assert!(ClaimClassification::PermissibleBoundedClaim.is_allowed());
  assert!(ClaimClassification::ConditionalWithDisclaimer.is_allowed());
  assert!(!ClaimClassification::ImpermissibleOverclaim.is_allowed());
}

#[test]
fn limitations_compliant_audit_succeeds() {
  let decl = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  let report =
    audit_limitations_and_boundaries(&decl).expect("compliant limitations audit must pass");

  assert_eq!(report.schema_version, ALPHA_LIMITATIONS_SCHEMA_VERSION);
  assert_eq!(report.total_claims_count, 3);
  assert_eq!(report.permissible_claims_count, 1);
  assert_eq!(report.conditional_claims_count, 2);
  assert_eq!(report.disclosed_limitations_count, 6);
  // (1 * 10,000 + 2 * 8,000) / 3 = 26,000 / 3 = 8,666 bp
  assert_eq!(report.safety_score_bp, 8_666);
  assert!(report.is_audit_passed);
}

#[test]
fn limitations_fail_closed_validation() {
  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.claims.clear();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyManifest)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.disclosed_limitations.clear();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyDisclosedLimitations)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.citation.bibtex_entry = "  ".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyBibtex)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.citation.doi_or_urn = "".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyDoiOrUrn)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.citation.canonical_title = "".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyCanonicalTitle)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.citation.repository_url = "   ".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyRepositoryUrl)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.citation.reproducibility_seed_policy = "".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptySeedPolicy)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.claims[0].claim_id = "".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyClaimId)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.claims[0].statement = "   ".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyStatement)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.claims[0].rationale = "".to_string();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::EmptyRationale)
  );

  let mut d = AlphaScenarioCatalog::build_compliant_limitations_declaration();
  d.claims.push(d.claims[0].clone());
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::DuplicateClaimId(
      "CLAIM-001".to_string()
    ))
  );

  // Impermissible overclaim
  let d = AlphaScenarioCatalog::build_overclaim_limitations_declaration();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::ImpermissibleClaimDetected(
      "CLAIM-OVERCLAIM".to_string()
    ))
  );

  // Missing required disclaimer
  let d = AlphaScenarioCatalog::build_missing_disclaimer_limitations_declaration();
  assert_eq!(
    audit_limitations_and_boundaries(&d),
    Err(AlphaLimitationsError::MissingRequiredDisclaimer {
      claim_id: "CLAIM-CONDITIONAL-UNDISCLOSED".to_string(),
      required_limitation: LimitationCategory::NetworkMultiplayer,
    })
  );
}

#[test]
fn limitations_error_display_coverage() {
  let errors = [
    AlphaLimitationsError::EmptyManifest,
    AlphaLimitationsError::EmptyClaimId,
    AlphaLimitationsError::EmptyStatement,
    AlphaLimitationsError::EmptyRationale,
    AlphaLimitationsError::DuplicateClaimId("CLAIM-01".to_string()),
    AlphaLimitationsError::ImpermissibleClaimDetected("CLAIM-01".to_string()),
    AlphaLimitationsError::MissingRequiredDisclaimer {
      claim_id: "CLAIM-01".to_string(),
      required_limitation: LimitationCategory::SimulationFidelity,
    },
    AlphaLimitationsError::EmptyBibtex,
    AlphaLimitationsError::EmptyDoiOrUrn,
    AlphaLimitationsError::EmptyCanonicalTitle,
    AlphaLimitationsError::EmptyRepositoryUrl,
    AlphaLimitationsError::EmptySeedPolicy,
    AlphaLimitationsError::EmptyDisclosedLimitations,
  ];
  for err in errors {
    let s = err.to_string();
    assert!(!s.is_empty());
  }
}

#[test]
fn guide_audience_and_section_kind_round_trips() {
  for audience in GuideAudience::all() {
    let s = audience.as_str();
    assert_eq!(GuideAudience::parse(s), Some(audience));
    assert_eq!(audience.to_string(), s);
  }
  assert_eq!(GuideAudience::parse("invalid-audience"), None);

  for kind in GuideSectionKind::all() {
    let s = kind.as_str();
    assert_eq!(GuideSectionKind::parse(s), Some(kind));
    assert_eq!(kind.to_string(), s);
  }
  assert_eq!(GuideSectionKind::parse("invalid-kind"), None);
}

#[test]
fn guides_audit_succeeds_for_compliant_manifest() {
  let manifest = AlphaScenarioCatalog::build_compliant_guides_manifest();
  let report = audit_guide_manifests(&manifest).expect("compliant guides must pass");

  assert_eq!(report.schema_version, ALPHA_GUIDES_SCHEMA_VERSION);
  assert_eq!(report.guides_evaluated, 6);
  assert_eq!(report.total_sections, 20);
  assert_eq!(report.total_code_examples, 14);
  assert!(report.average_completeness_bp >= 5_000);
  assert!(report.all_prerequisites_resolved);
  assert_eq!(report.records.len(), 6);
}

#[test]
fn guides_audit_rejects_invalid_and_cyclic_manifests() {
  let mut m = AlphaScenarioCatalog::build_compliant_guides_manifest();
  m.schema_version = "invalid-v0";
  assert_eq!(
    audit_guide_manifests(&m),
    Err(AlphaGuidesError::UnsupportedSchemaVersion {
      version: "invalid-v0".to_string(),
    })
  );

  let empty_manifest = AlphaGuidesManifest {
    schema_version: ALPHA_GUIDES_SCHEMA_VERSION,
    guides: &[],
  };
  assert_eq!(
    audit_guide_manifests(&empty_manifest),
    Err(AlphaGuidesError::EmptyManifest)
  );

  static SECTIONS: [GuideSection; 1] = [GuideSection {
    heading: "Heading",
    kind: GuideSectionKind::CoreConcepts,
    content_summary: "Summary",
    has_code_example: false,
  }];
  static GUIDES_EMPTY_ID: [GuideDocumentDefinition; 1] = [GuideDocumentDefinition {
    guide_id: "  ",
    title: "Title",
    audience: GuideAudience::Player,
    summary: "Summary",
    prerequisite_guide_ids: &[],
    sections: &SECTIONS,
  }];
  let m = AlphaGuidesManifest {
    schema_version: ALPHA_GUIDES_SCHEMA_VERSION,
    guides: &GUIDES_EMPTY_ID,
  };
  assert_eq!(
    audit_guide_manifests(&m),
    Err(AlphaGuidesError::EmptyGuideId)
  );

  static GUIDES_DUP: [GuideDocumentDefinition; 2] = [
    GuideDocumentDefinition {
      guide_id: "GUIDE-01",
      title: "Title 1",
      audience: GuideAudience::Player,
      summary: "Summary 1",
      prerequisite_guide_ids: &[],
      sections: &SECTIONS,
    },
    GuideDocumentDefinition {
      guide_id: "GUIDE-01",
      title: "Title 2",
      audience: GuideAudience::Contributor,
      summary: "Summary 2",
      prerequisite_guide_ids: &[],
      sections: &SECTIONS,
    },
  ];
  let m = AlphaGuidesManifest {
    schema_version: ALPHA_GUIDES_SCHEMA_VERSION,
    guides: &GUIDES_DUP,
  };
  assert_eq!(
    audit_guide_manifests(&m),
    Err(AlphaGuidesError::DuplicateGuideId {
      guide_id: "GUIDE-01".to_string(),
    })
  );

  static GUIDES_MISSING_PREREQ: [GuideDocumentDefinition; 1] = [GuideDocumentDefinition {
    guide_id: "GUIDE-01",
    title: "Title",
    audience: GuideAudience::Player,
    summary: "Summary",
    prerequisite_guide_ids: &["NONEXISTENT"],
    sections: &SECTIONS,
  }];
  let m = AlphaGuidesManifest {
    schema_version: ALPHA_GUIDES_SCHEMA_VERSION,
    guides: &GUIDES_MISSING_PREREQ,
  };
  assert_eq!(
    audit_guide_manifests(&m),
    Err(AlphaGuidesError::MissingPrerequisite {
      guide_id: "GUIDE-01".to_string(),
      prerequisite: "NONEXISTENT".to_string(),
    })
  );

  // Cyclic prerequisites
  let cyclic_manifest = AlphaScenarioCatalog::build_cyclic_guides_manifest();
  assert!(matches!(
    audit_guide_manifests(&cyclic_manifest),
    Err(AlphaGuidesError::CyclicPrerequisite { .. })
  ));
}

#[test]
fn guides_error_display_coverage() {
  let errors = [
    AlphaGuidesError::EmptyManifest,
    AlphaGuidesError::UnsupportedSchemaVersion {
      version: "v0".to_string(),
    },
    AlphaGuidesError::EmptyGuideId,
    AlphaGuidesError::DuplicateGuideId {
      guide_id: "GUIDE-01".to_string(),
    },
    AlphaGuidesError::EmptyTitle {
      guide_id: "GUIDE-01".to_string(),
    },
    AlphaGuidesError::EmptySummary {
      guide_id: "GUIDE-01".to_string(),
    },
    AlphaGuidesError::NoSections {
      guide_id: "GUIDE-01".to_string(),
    },
    AlphaGuidesError::EmptySectionHeading {
      guide_id: "GUIDE-01".to_string(),
    },
    AlphaGuidesError::EmptySectionSummary {
      guide_id: "GUIDE-01".to_string(),
    },
    AlphaGuidesError::MissingPrerequisite {
      guide_id: "GUIDE-01".to_string(),
      prerequisite: "PRE-01".to_string(),
    },
    AlphaGuidesError::CyclicPrerequisite {
      guide_id: "GUIDE-01".to_string(),
      path: vec!["A".to_string(), "B".to_string(), "A".to_string()],
    },
  ];
  for err in errors {
    let s = err.to_string();
    assert!(!s.is_empty());
  }
}

#[test]
fn sample_artifact_kind_and_reproducibility_status_round_trips() {
  for kind in SampleArtifactKind::all() {
    let s = kind.as_str();
    assert_eq!(SampleArtifactKind::parse(s), Some(kind));
    assert_eq!(kind.to_string(), s);
  }
  assert_eq!(SampleArtifactKind::parse("invalid-kind"), None);

  for status in ReproducibilityStatus::all() {
    let s = status.as_str();
    assert_eq!(ReproducibilityStatus::parse(s), Some(status));
    assert_eq!(status.to_string(), s);
  }
  assert_eq!(ReproducibilityStatus::parse("invalid-status"), None);

  assert!(ReproducibilityStatus::FullyReproducible.is_valid());
  assert!(ReproducibilityStatus::RequiresModelAdapter.is_valid());
  assert!(ReproducibilityStatus::SyntheticBaselineOnly.is_valid());
  assert!(!ReproducibilityStatus::CorruptedOrMissing.is_valid());

  assert_eq!(
    ReproducibilityStatus::FullyReproducible.base_score_bp(),
    10_000
  );
  assert_eq!(
    ReproducibilityStatus::SyntheticBaselineOnly.base_score_bp(),
    8_500
  );
  assert_eq!(
    ReproducibilityStatus::RequiresModelAdapter.base_score_bp(),
    7_500
  );
  assert_eq!(ReproducibilityStatus::CorruptedOrMissing.base_score_bp(), 0);

  assert!(is_valid_fnv1a_hash("811c9dc500000001"));
  assert!(is_valid_fnv1a_hash("0123456789abcdef"));
  assert!(!is_valid_fnv1a_hash("short"));
  assert!(!is_valid_fnv1a_hash("811c9dc50000000g"));
}

#[test]
fn reproducibility_bundle_audit_succeeds_for_canonical_manifest() {
  let bundle = AlphaScenarioCatalog::build_canonical_reproducibility_bundle();
  let report = audit_reproducibility_bundle(&bundle).expect("canonical bundle must pass");

  assert_eq!(report.schema_version, ALPHA_REPRODUCIBILITY_SCHEMA_VERSION);
  assert_eq!(report.packages_evaluated, 5);
  assert_eq!(report.total_artifacts, 53);
  assert_eq!(report.fully_reproducible_count, 4);
  assert!(report.average_reproducibility_score_bp >= 9_000);
  assert!(report.bundle_eligible_for_release);
  assert_eq!(report.records.len(), 5);
}

#[test]
fn reproducibility_bundle_rejects_invalid_manifests() {
  let mut b = AlphaScenarioCatalog::build_canonical_reproducibility_bundle();
  b.schema_version = "invalid-v0";
  assert_eq!(
    audit_reproducibility_bundle(&b),
    Err(AlphaReproducibilityError::UnsupportedSchemaVersion {
      version: "invalid-v0".to_string(),
    })
  );

  let empty_bundle = ReproducibilityBundleManifest {
    schema_version: ALPHA_REPRODUCIBILITY_SCHEMA_VERSION,
    packages: &[],
  };
  assert_eq!(
    audit_reproducibility_bundle(&empty_bundle),
    Err(AlphaReproducibilityError::EmptyBundle)
  );

  static PKGS_EMPTY_ID: [ReproducibilityPackageDefinition; 1] =
    [ReproducibilityPackageDefinition {
      package_id: "  ",
      title: "Title",
      kind: SampleArtifactKind::ScenarioBenchmark,
      artifact_count: 1,
      content_hash_fnv1a: "811c9dc500000001",
      verification_command: "cargo test",
      seed_policy: "none",
      dependencies: &[],
      declared_status: ReproducibilityStatus::FullyReproducible,
    }];
  let b = ReproducibilityBundleManifest {
    schema_version: ALPHA_REPRODUCIBILITY_SCHEMA_VERSION,
    packages: &PKGS_EMPTY_ID,
  };
  assert_eq!(
    audit_reproducibility_bundle(&b),
    Err(AlphaReproducibilityError::EmptyPackageId)
  );

  static PKGS_ZERO_COUNT: [ReproducibilityPackageDefinition; 1] =
    [ReproducibilityPackageDefinition {
      package_id: "PKG-01",
      title: "Title",
      kind: SampleArtifactKind::ScenarioBenchmark,
      artifact_count: 0,
      content_hash_fnv1a: "811c9dc500000001",
      verification_command: "cargo test",
      seed_policy: "none",
      dependencies: &[],
      declared_status: ReproducibilityStatus::FullyReproducible,
    }];
  let b = ReproducibilityBundleManifest {
    schema_version: ALPHA_REPRODUCIBILITY_SCHEMA_VERSION,
    packages: &PKGS_ZERO_COUNT,
  };
  assert_eq!(
    audit_reproducibility_bundle(&b),
    Err(AlphaReproducibilityError::ZeroArtifactCount {
      package_id: "PKG-01".to_string(),
    })
  );

  static PKGS_CORRUPT_HASH: [ReproducibilityPackageDefinition; 1] =
    [ReproducibilityPackageDefinition {
      package_id: "PKG-01",
      title: "Title",
      kind: SampleArtifactKind::ScenarioBenchmark,
      artifact_count: 1,
      content_hash_fnv1a: "invalid-hash",
      verification_command: "cargo test",
      seed_policy: "none",
      dependencies: &[],
      declared_status: ReproducibilityStatus::FullyReproducible,
    }];
  let b = ReproducibilityBundleManifest {
    schema_version: ALPHA_REPRODUCIBILITY_SCHEMA_VERSION,
    packages: &PKGS_CORRUPT_HASH,
  };
  assert_eq!(
    audit_reproducibility_bundle(&b),
    Err(AlphaReproducibilityError::InvalidContentHash {
      package_id: "PKG-01".to_string(),
      hash: "invalid-hash".to_string(),
    })
  );

  static PKGS_MISSING_DEP: [ReproducibilityPackageDefinition; 1] =
    [ReproducibilityPackageDefinition {
      package_id: "PKG-01",
      title: "Title",
      kind: SampleArtifactKind::ScenarioBenchmark,
      artifact_count: 1,
      content_hash_fnv1a: "811c9dc500000001",
      verification_command: "cargo test",
      seed_policy: "none",
      dependencies: &["NONEXISTENT-PKG"],
      declared_status: ReproducibilityStatus::FullyReproducible,
    }];
  let b = ReproducibilityBundleManifest {
    schema_version: ALPHA_REPRODUCIBILITY_SCHEMA_VERSION,
    packages: &PKGS_MISSING_DEP,
  };
  assert_eq!(
    audit_reproducibility_bundle(&b),
    Err(AlphaReproducibilityError::MissingDependency {
      package_id: "PKG-01".to_string(),
      dependency: "NONEXISTENT-PKG".to_string(),
    })
  );
}

#[test]
fn reproducibility_error_display_coverage() {
  let errors = [
    AlphaReproducibilityError::EmptyBundle,
    AlphaReproducibilityError::UnsupportedSchemaVersion {
      version: "v0".to_string(),
    },
    AlphaReproducibilityError::EmptyPackageId,
    AlphaReproducibilityError::DuplicatePackageId {
      package_id: "PKG-01".to_string(),
    },
    AlphaReproducibilityError::EmptyTitle {
      package_id: "PKG-01".to_string(),
    },
    AlphaReproducibilityError::ZeroArtifactCount {
      package_id: "PKG-01".to_string(),
    },
    AlphaReproducibilityError::InvalidContentHash {
      package_id: "PKG-01".to_string(),
      hash: "hash".to_string(),
    },
    AlphaReproducibilityError::EmptyVerificationCommand {
      package_id: "PKG-01".to_string(),
    },
    AlphaReproducibilityError::MissingDependency {
      package_id: "PKG-01".to_string(),
      dependency: "DEP-01".to_string(),
    },
    AlphaReproducibilityError::CorruptedStatus {
      package_id: "PKG-01".to_string(),
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

  let rep5 =
    AlphaScenarioCatalog::execute_limitations_compliant().expect("limitations compliant scenario");
  assert!(rep5.is_audit_passed);

  let rep6 = AlphaScenarioCatalog::execute_limitations_overclaim();
  assert!(matches!(
    rep6,
    Err(AlphaLimitationsError::ImpermissibleClaimDetected(_))
  ));

  let rep7 = AlphaScenarioCatalog::execute_limitations_missing_disclaimer();
  assert!(matches!(
    rep7,
    Err(AlphaLimitationsError::MissingRequiredDisclaimer { .. })
  ));

  let rep8 = AlphaScenarioCatalog::execute_guides_compliant().expect("compliant guides scenario");
  assert!(rep8.all_prerequisites_resolved);
  assert_eq!(rep8.guides_evaluated, 6);

  let rep9 = AlphaScenarioCatalog::execute_guides_cyclic();
  assert!(matches!(
    rep9,
    Err(AlphaGuidesError::CyclicPrerequisite { .. })
  ));

  let rep10 = AlphaScenarioCatalog::execute_reproducibility_compliant()
    .expect("compliant reproducibility scenario");
  assert!(rep10.bundle_eligible_for_release);
  assert_eq!(rep10.packages_evaluated, 5);

  let rep11 = AlphaScenarioCatalog::execute_reproducibility_corrupt();
  assert!(matches!(
    rep11,
    Err(AlphaReproducibilityError::InvalidContentHash { .. })
  ));

  let rep12 = AlphaScenarioCatalog::execute_release_checks_compliant()
    .expect("compliant release checks scenario");
  assert!(rep12.is_release_ready);
  assert_eq!(rep12.total_checks, 6);
  assert_eq!(rep12.readiness_score_bp, 10_000);

  let rep13 = AlphaScenarioCatalog::execute_release_checks_blocker();
  assert!(matches!(
    rep13,
    Err(AlphaReleaseChecksError::CriticalBlockerDetected { .. })
  ));

  let rep14 = AlphaScenarioCatalog::execute_release_checks_missing_category();
  assert!(matches!(
    rep14,
    Err(AlphaReleaseChecksError::MissingRequiredCategory { .. })
  ));
}

#[test]
fn release_check_category_round_trips() {
  let categories = [
    ReleaseCheckCategory::CleanInstall,
    ReleaseCheckCategory::Reproducibility,
    ReleaseCheckCategory::SecurityAdvisory,
    ReleaseCheckCategory::LicenseCompliance,
    ReleaseCheckCategory::CompatibilityMatrix,
    ReleaseCheckCategory::DataRedaction,
  ];
  for cat in categories {
    let s = cat.as_str();
    assert_eq!(ReleaseCheckCategory::parse(s), Some(cat));
    assert_eq!(cat.to_string(), s);
  }
  assert_eq!(ReleaseCheckCategory::parse("invalid-category"), None);
  assert_eq!(ReleaseCheckCategory::all().len(), 6);
}

#[test]
fn release_check_severity_round_trips() {
  let severities = [
    ReleaseCheckSeverity::CriticalBlocker,
    ReleaseCheckSeverity::MajorIssue,
    ReleaseCheckSeverity::MinorWarning,
    ReleaseCheckSeverity::VerifiedPass,
  ];
  for sev in severities {
    let s = sev.as_str();
    assert_eq!(ReleaseCheckSeverity::parse(s), Some(sev));
    assert_eq!(sev.to_string(), s);
  }
  assert_eq!(ReleaseCheckSeverity::parse("invalid-severity"), None);

  assert!(ReleaseCheckSeverity::CriticalBlocker.is_blocking());
  assert!(ReleaseCheckSeverity::MajorIssue.is_blocking());
  assert!(!ReleaseCheckSeverity::MinorWarning.is_blocking());
  assert!(!ReleaseCheckSeverity::VerifiedPass.is_blocking());
}

#[test]
fn check_verification_status_round_trips() {
  let statuses = [
    CheckVerificationStatus::Passed,
    CheckVerificationStatus::ConditionallyPassed,
    CheckVerificationStatus::Failed,
    CheckVerificationStatus::Skipped,
  ];
  for st in statuses {
    let s = st.as_str();
    assert_eq!(CheckVerificationStatus::parse(s), Some(st));
    assert_eq!(st.to_string(), s);
  }
  assert_eq!(CheckVerificationStatus::parse("invalid-status"), None);

  assert!(CheckVerificationStatus::Passed.is_successful());
  assert!(CheckVerificationStatus::ConditionallyPassed.is_successful());
  assert!(!CheckVerificationStatus::Failed.is_successful());
  assert!(!CheckVerificationStatus::Skipped.is_successful());

  assert_eq!(CheckVerificationStatus::Passed.score_weight_bp(), 10_000);
  assert_eq!(
    CheckVerificationStatus::ConditionallyPassed.score_weight_bp(),
    7_500
  );
  assert_eq!(CheckVerificationStatus::Skipped.score_weight_bp(), 5_000);
  assert_eq!(CheckVerificationStatus::Failed.score_weight_bp(), 0);
}

#[test]
fn release_checks_compliant_audit_succeeds() {
  let manifest = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  let report = audit_release_checks(&manifest).expect("compliant release checks must succeed");

  assert_eq!(report.schema_version, ALPHA_RELEASE_CHECKS_SCHEMA_VERSION);
  assert_eq!(
    report.manifest_id,
    "manifest-alpha-release-checks-compliant-v1"
  );
  assert_eq!(report.release_version, "0.1.217");
  assert_eq!(report.target_commit, "ec340c2a8f01b9e5");
  assert_eq!(report.total_checks, 6);
  assert_eq!(report.passed_checks, 6);
  assert_eq!(report.conditionally_passed_checks, 0);
  assert_eq!(report.failed_checks, 0);
  assert_eq!(report.skipped_checks, 0);
  assert_eq!(report.critical_blockers_count, 0);
  assert_eq!(report.readiness_score_bp, 10_000);
  assert!(report.is_release_ready);
  assert_eq!(report.category_summaries.len(), 6);
  for cat_summary in &report.category_summaries {
    assert_eq!(cat_summary.total_checks, 1);
    assert_eq!(cat_summary.passed_checks, 1);
    assert!(!cat_summary.has_critical_blocker);
  }
}

#[test]
fn release_checks_fail_closed_validation() {
  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.schema_version = "invalid-v0";
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::UnsupportedSchemaVersion {
      version: "invalid-v0".to_string(),
    })
  );

  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.manifest_id = "   ";
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::EmptyManifestId)
  );

  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.release_version = "";
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::EmptyReleaseVersion)
  );

  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.target_commit = "  ";
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::EmptyTargetCommit)
  );

  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.checks = &[];
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::ZeroChecks)
  );

  static EMPTY_ID_CHECK: [ReleaseCheckDefinition; 1] = [ReleaseCheckDefinition {
    check_id: "",
    category: ReleaseCheckCategory::CleanInstall,
    title: "T",
    description: "D",
    severity: ReleaseCheckSeverity::VerifiedPass,
    status: CheckVerificationStatus::Passed,
    evidence_command: "cmd",
    evidence_hash: "811c9dc500000011",
    mitigation_notes: None,
  }];
  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.checks = &EMPTY_ID_CHECK;
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::EmptyCheckId)
  );

  static DUPLICATE_CHECKS: [ReleaseCheckDefinition; 2] = [
    ReleaseCheckDefinition {
      check_id: "CHK-01",
      category: ReleaseCheckCategory::CleanInstall,
      title: "T1",
      description: "D1",
      severity: ReleaseCheckSeverity::VerifiedPass,
      status: CheckVerificationStatus::Passed,
      evidence_command: "cmd1",
      evidence_hash: "811c9dc500000011",
      mitigation_notes: None,
    },
    ReleaseCheckDefinition {
      check_id: "CHK-01",
      category: ReleaseCheckCategory::Reproducibility,
      title: "T2",
      description: "D2",
      severity: ReleaseCheckSeverity::VerifiedPass,
      status: CheckVerificationStatus::Passed,
      evidence_command: "cmd2",
      evidence_hash: "811c9dc500000012",
      mitigation_notes: None,
    },
  ];
  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.checks = &DUPLICATE_CHECKS;
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::DuplicateCheckId {
      check_id: "CHK-01".to_string(),
    })
  );

  static EMPTY_TITLE_CHECK: [ReleaseCheckDefinition; 1] = [ReleaseCheckDefinition {
    check_id: "CHK-01",
    category: ReleaseCheckCategory::CleanInstall,
    title: "  ",
    description: "D",
    severity: ReleaseCheckSeverity::VerifiedPass,
    status: CheckVerificationStatus::Passed,
    evidence_command: "cmd",
    evidence_hash: "811c9dc500000011",
    mitigation_notes: None,
  }];
  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.checks = &EMPTY_TITLE_CHECK;
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::EmptyTitle {
      check_id: "CHK-01".to_string(),
    })
  );

  static EMPTY_DESC_CHECK: [ReleaseCheckDefinition; 1] = [ReleaseCheckDefinition {
    check_id: "CHK-01",
    category: ReleaseCheckCategory::CleanInstall,
    title: "T",
    description: "",
    severity: ReleaseCheckSeverity::VerifiedPass,
    status: CheckVerificationStatus::Passed,
    evidence_command: "cmd",
    evidence_hash: "811c9dc500000011",
    mitigation_notes: None,
  }];
  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.checks = &EMPTY_DESC_CHECK;
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::EmptyDescription {
      check_id: "CHK-01".to_string(),
    })
  );

  static EMPTY_CMD_CHECK: [ReleaseCheckDefinition; 1] = [ReleaseCheckDefinition {
    check_id: "CHK-01",
    category: ReleaseCheckCategory::CleanInstall,
    title: "T",
    description: "D",
    severity: ReleaseCheckSeverity::VerifiedPass,
    status: CheckVerificationStatus::Passed,
    evidence_command: " ",
    evidence_hash: "811c9dc500000011",
    mitigation_notes: None,
  }];
  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.checks = &EMPTY_CMD_CHECK;
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::EmptyEvidenceCommand {
      check_id: "CHK-01".to_string(),
    })
  );

  static INVALID_HASH_CHECK: [ReleaseCheckDefinition; 1] = [ReleaseCheckDefinition {
    check_id: "CHK-01",
    category: ReleaseCheckCategory::CleanInstall,
    title: "T",
    description: "D",
    severity: ReleaseCheckSeverity::VerifiedPass,
    status: CheckVerificationStatus::Passed,
    evidence_command: "cmd",
    evidence_hash: "invalid-hash",
    mitigation_notes: None,
  }];
  let mut m = AlphaScenarioCatalog::build_canonical_release_checks_manifest();
  m.checks = &INVALID_HASH_CHECK;
  assert_eq!(
    audit_release_checks(&m),
    Err(AlphaReleaseChecksError::InvalidEvidenceHash {
      check_id: "CHK-01".to_string(),
      hash: "invalid-hash".to_string(),
    })
  );
}

#[test]
fn release_checks_error_display_coverage() {
  let errors = [
    AlphaReleaseChecksError::EmptyManifest,
    AlphaReleaseChecksError::UnsupportedSchemaVersion {
      version: "v0".to_string(),
    },
    AlphaReleaseChecksError::EmptyManifestId,
    AlphaReleaseChecksError::EmptyReleaseVersion,
    AlphaReleaseChecksError::EmptyTargetCommit,
    AlphaReleaseChecksError::ZeroChecks,
    AlphaReleaseChecksError::EmptyCheckId,
    AlphaReleaseChecksError::DuplicateCheckId {
      check_id: "CHK-01".to_string(),
    },
    AlphaReleaseChecksError::EmptyTitle {
      check_id: "CHK-01".to_string(),
    },
    AlphaReleaseChecksError::EmptyDescription {
      check_id: "CHK-01".to_string(),
    },
    AlphaReleaseChecksError::EmptyEvidenceCommand {
      check_id: "CHK-01".to_string(),
    },
    AlphaReleaseChecksError::InvalidEvidenceHash {
      check_id: "CHK-01".to_string(),
      hash: "bad".to_string(),
    },
    AlphaReleaseChecksError::CriticalBlockerDetected {
      check_id: "CHK-01".to_string(),
      category: "security".to_string(),
      description: "exploit".to_string(),
    },
    AlphaReleaseChecksError::MissingRequiredCategory {
      category: "license-compliance".to_string(),
    },
  ];

  for err in errors {
    let formatted = err.to_string();
    assert!(!formatted.is_empty());
  }
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

  let lim_rep = AlphaScenarioCatalog::execute_limitations_compliant().unwrap();
  let lim_md = render_limitations_report_markdown(&lim_rep);
  assert!(lim_md.starts_with("# Public Alpha Known Limitations and Evidence Boundaries Report"));
  assert!(!lim_md.contains('\x1b'));

  let guides_rep = AlphaScenarioCatalog::execute_guides_compliant().unwrap();
  let guides_md = render_guides_report_markdown(&guides_rep);
  assert!(guides_md.starts_with("# Public Alpha Documentation Guides Audit Report"));
  assert!(!guides_md.contains('\x1b'));

  let repro_rep = AlphaScenarioCatalog::execute_reproducibility_compliant().unwrap();
  let repro_md = render_reproducibility_report_markdown(&repro_rep);
  assert!(repro_md.starts_with("# Public Alpha Reproducibility Bundle Audit Report"));
  assert!(!repro_md.contains('\x1b'));

  let checks_rep = AlphaScenarioCatalog::execute_release_checks_compliant().unwrap();
  let checks_md = render_release_checks_report_markdown(&checks_rep);
  assert!(checks_md.starts_with("# Fog of Intent — Public Alpha Release Readiness Audit Report"));
  assert!(!checks_md.contains('\x1b'));

  let archive_rep = AlphaScenarioCatalog::execute_release_archive_compliant().unwrap();
  let archive_md = render_release_archive_report_markdown(&archive_rep);
  assert!(archive_md.starts_with("# Fog of Intent Release Archive Manifest Audit Report"));
  assert!(!archive_md.contains('\x1b'));
}

#[test]
fn archive_category_kind_round_trips() {
  for cat in ArchiveCategoryKind::all() {
    let s = cat.as_str();
    assert_eq!(ArchiveCategoryKind::parse(s), Some(cat));
    assert_eq!(cat.to_string(), s);
  }
  assert_eq!(ArchiveCategoryKind::parse("invalid-cat"), None);
}

#[test]
fn archive_hash_helpers_validate() {
  assert!(is_valid_16hex("0123456789abcdef"));
  assert!(!is_valid_16hex("0123456789ABCDEF"));
  assert!(!is_valid_16hex("short"));
  assert!(!is_valid_16hex("toolongtoolong12345"));
  assert!(!is_valid_16hex("0123456789abcdeg")); // non-hex 'g'

  let hash = compute_fnv1a_16hex(b"hello world");
  assert_eq!(hash.len(), 16);
  assert!(is_valid_16hex(&hash));
}

#[test]
fn canonical_release_archive_manifest_passes_audit() {
  let manifest = canonical_alpha_release_archive_manifest();
  let report = audit_release_archive_manifest(&manifest).expect("audit canonical manifest");

  assert_eq!(report.schema_version, ALPHA_ARCHIVE_SCHEMA_VERSION);
  assert_eq!(report.release_tag, "v0.1.231");
  assert_eq!(report.package_version, "0.1.231");
  assert_eq!(report.total_items, 11);
  assert_eq!(report.mandatory_items, 11);
  assert_eq!(report.category_summaries.len(), 11);
  assert_eq!(report.completeness_score_bp, 10_000);
  assert!(report.is_release_archive_ready);
  assert!(report.combined_digest_verified);

  let md = render_release_archive_report_markdown(&report);
  assert!(md.contains("READY FOR TAGGED RELEASE"));
  assert!(md.contains("100.00% (10000 bp)"));
}

#[test]
fn release_archive_error_display_coverage() {
  let errors = [
    AlphaArchiveError::EmptyManifest,
    AlphaArchiveError::MissingReleaseTag,
    AlphaArchiveError::MissingPackageVersion,
    AlphaArchiveError::MissingMandatoryCategory(ArchiveCategoryKind::LockfileInventory),
    AlphaArchiveError::DuplicateItemId("dup-id"),
    AlphaArchiveError::InvalidHashFormat("invalid-hash"),
    AlphaArchiveError::InvalidRelativePath("../bad/path"),
    AlphaArchiveError::ZeroByteMandatoryItem("zero-item"),
    AlphaArchiveError::CombinedDigestMismatch {
      expected: "expected-hash",
      calculated: "calc-hash".to_string(),
    },
  ];

  for err in errors {
    let display_str = err.to_string();
    assert!(!display_str.is_empty());
  }
}

#[test]
fn release_archive_fail_closed_validation() {
  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.release_tag = "";
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::MissingReleaseTag)
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.package_version = "  ";
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::MissingPackageVersion)
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.items.clear();
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::EmptyManifest)
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest
    .items
    .retain(|i| i.category != ArchiveCategoryKind::SourceManifest);
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::MissingMandatoryCategory(
      ArchiveCategoryKind::SourceManifest
    ))
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.items[1].item_id = manifest.items[0].item_id;
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::DuplicateItemId(
      manifest.items[0].item_id
    ))
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.items[0].relative_path = "/absolute/path";
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::InvalidRelativePath("/absolute/path"))
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.items[0].fnv1a_16hex_hash = "not-16-hex";
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::InvalidHashFormat("not-16-hex"))
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.items[0].byte_size = 0;
  assert_eq!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::ZeroByteMandatoryItem(
      manifest.items[0].item_id
    ))
  );

  let mut manifest = canonical_alpha_release_archive_manifest();
  manifest.combined_digest_16hex = "0000000000000000";
  assert!(matches!(
    audit_release_archive_manifest(&manifest),
    Err(AlphaArchiveError::CombinedDigestMismatch { .. })
  ));
}

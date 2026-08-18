//! Unit tests for GUI presentation models, DTO projections, and presentation need evaluation.

use crate::gui::catalog::GuiScenarioCatalog;
use crate::gui::dto::{
  GUI_DTO_SCHEMA_VERSION, GuiMapActorState, GuiMapLocationState, GuiMapViewDto,
  GuiPresentationBundle, GuiVisionStatus,
};
use crate::gui::need::{
  ComprehensionDeficit, ComprehensionDomain, DeficitSeverity, PresentationNeedError,
  evaluate_presentation_need, render_presentation_need_markdown,
};
use crate::gui::projection::{
  assemble_gui_presentation_bundle, build_gui_accessibility, build_gui_debrief_view,
  build_gui_map_view, build_gui_plan_view, build_gui_timeline_view,
};

#[test]
fn domain_and_severity_round_trips() {
  let domains = [
    (ComprehensionDomain::SpatialTopology, "spatial-topology"),
    (ComprehensionDomain::TemporalTimeline, "temporal-timeline"),
    (
      ComprehensionDomain::ContingencyBranching,
      "contingency-branching",
    ),
    (ComprehensionDomain::CausalDebrief, "causal-debrief"),
  ];

  for (domain, name) in domains {
    assert_eq!(domain.as_str(), name);
    assert_eq!(ComprehensionDomain::from_str_name(name), Some(domain));
    assert_eq!(format!("{}", domain), name);
  }
  assert_eq!(ComprehensionDomain::from_str_name("unknown-domain"), None);

  let severities = [
    (DeficitSeverity::Negligible, "negligible"),
    (DeficitSeverity::ModerateFriction, "moderate-friction"),
    (DeficitSeverity::SignificantBarrier, "significant-barrier"),
  ];

  for (severity, name) in severities {
    assert_eq!(severity.as_str(), name);
    assert_eq!(DeficitSeverity::from_str_name(name), Some(severity));
    assert_eq!(format!("{}", severity), name);
  }
  assert_eq!(DeficitSeverity::from_str_name("critical-blocker"), None);

  let vision_statuses = [
    (GuiVisionStatus::FullVision, "full-vision"),
    (GuiVisionStatus::LastKnown, "last-known"),
    (GuiVisionStatus::ConcealedInFog, "concealed-in-fog"),
  ];

  for (status, name) in vision_statuses {
    assert_eq!(status.as_str(), name);
    assert_eq!(format!("{}", status), name);
  }
}

#[test]
fn presentation_need_evaluation_and_threshold_rules() {
  let deficits = vec![
    ComprehensionDeficit::new(
      ComprehensionDomain::SpatialTopology,
      DeficitSeverity::SignificantBarrier,
      6_000,
      "Spatial navigation difficulty",
      "Text coordinate list",
      "Vector map",
    ),
    ComprehensionDeficit::new(
      ComprehensionDomain::CausalDebrief,
      DeficitSeverity::ModerateFriction,
      4_000,
      "Causal attribution complexity",
      "Log paragraphs",
      "2D matrix",
    ),
  ];

  let assessment = evaluate_presentation_need(
    "assessment-001",
    "scenario-test-01",
    "MidLaner",
    deficits,
    "Valid rationale for GUI adoption",
  )
  .expect("evaluation should succeed");

  assert_eq!(assessment.assessment_id(), "assessment-001");
  assert_eq!(assessment.scenario_id(), "scenario-test-01");
  assert_eq!(assessment.observer_role(), "MidLaner");
  assert_eq!(assessment.mean_deficit_impact_bp(), 5_000);
  assert_eq!(assessment.max_deficit_impact_bp(), 6_000);
  assert!(assessment.gui_justified());

  // Test low-deficit case where GUI is NOT justified
  let low_deficits = vec![ComprehensionDeficit::new(
    ComprehensionDomain::TemporalTimeline,
    DeficitSeverity::Negligible,
    1_500,
    "Minor turn delay",
    "Text turn label",
    "Timeline badge",
  )];

  let low_assessment = evaluate_presentation_need(
    "assessment-002",
    "scenario-test-02",
    "Support",
    low_deficits,
    "Low deficit scenario",
  )
  .expect("evaluation should succeed");

  assert_eq!(low_assessment.mean_deficit_impact_bp(), 1_500);
  assert!(!low_assessment.gui_justified());
}

#[test]
fn fail_closed_validation_checks() {
  assert_eq!(
    evaluate_presentation_need("  ", "sc-01", "Mid", vec![], "rationale"),
    Err(PresentationNeedError::EmptyIdentifier("assessment_id"))
  );

  assert_eq!(
    evaluate_presentation_need("a-01", "  ", "Mid", vec![], "rationale"),
    Err(PresentationNeedError::EmptyIdentifier("scenario_id"))
  );

  assert_eq!(
    evaluate_presentation_need("a-01", "sc-01", "  ", vec![], "rationale"),
    Err(PresentationNeedError::EmptyIdentifier("observer_role"))
  );

  assert_eq!(
    evaluate_presentation_need("a-01", "sc-01", "Mid", vec![], "  "),
    Err(PresentationNeedError::EmptyDescription(
      "evidence_rationale"
    ))
  );

  assert_eq!(
    evaluate_presentation_need("a-01", "sc-01", "Mid", vec![], "rationale"),
    Err(PresentationNeedError::EmptyDeficitList)
  );

  let duplicate_deficits = vec![
    ComprehensionDeficit::new(
      ComprehensionDomain::SpatialTopology,
      DeficitSeverity::ModerateFriction,
      3_000,
      "desc1",
      "lim1",
      "mit1",
    ),
    ComprehensionDeficit::new(
      ComprehensionDomain::SpatialTopology,
      DeficitSeverity::SignificantBarrier,
      6_000,
      "desc2",
      "lim2",
      "mit2",
    ),
  ];

  assert_eq!(
    evaluate_presentation_need("a-01", "sc-01", "Mid", duplicate_deficits, "duplicate test"),
    Err(PresentationNeedError::DuplicateDomain(
      ComprehensionDomain::SpatialTopology
    ))
  );

  let out_of_range_deficits = vec![ComprehensionDeficit::new(
    ComprehensionDomain::SpatialTopology,
    DeficitSeverity::ModerateFriction,
    10_001,
    "desc",
    "lim",
    "mit",
  )];

  assert_eq!(
    evaluate_presentation_need(
      "a-01",
      "sc-01",
      "Mid",
      out_of_range_deficits,
      "out of range"
    ),
    Err(PresentationNeedError::DeficitScoreOutOfRange(10_001))
  );

  let empty_desc_deficits = vec![ComprehensionDeficit::new(
    ComprehensionDomain::SpatialTopology,
    DeficitSeverity::ModerateFriction,
    3_000,
    "   ",
    "lim",
    "mit",
  )];

  assert_eq!(
    evaluate_presentation_need("a-01", "sc-01", "Mid", empty_desc_deficits, "empty desc"),
    Err(PresentationNeedError::EmptyDescription("description"))
  );
}

#[test]
fn error_display_formatting_coverage() {
  let errors = [
    (
      PresentationNeedError::EmptyDeficitList,
      "deficit list must not be empty",
    ),
    (
      PresentationNeedError::EmptyIdentifier("assessment_id"),
      "identifier field 'assessment_id' must not be empty",
    ),
    (
      PresentationNeedError::DuplicateDomain(ComprehensionDomain::CausalDebrief),
      "duplicate evaluation for domain 'causal-debrief'",
    ),
    (
      PresentationNeedError::DeficitScoreOutOfRange(12_000),
      "deficit score 12000 exceeds 10,000 bp maximum",
    ),
    (
      PresentationNeedError::EmptyDescription("rationale"),
      "description field 'rationale' must not be empty",
    ),
  ];

  for (err, expected_str) in errors {
    assert_eq!(format!("{}", err), expected_str);
  }
}

#[test]
fn gui_dto_bundle_construction_and_validation() {
  let map_view = build_gui_map_view(
    1,
    "TopLaner",
    "Allied",
    vec![GuiMapLocationState {
      location_id: "Top".to_string(),
      terrain_kind: "lane".to_string(),
      vision_status: GuiVisionStatus::FullVision,
      last_seen_turn: Some(1),
    }],
    vec![GuiMapActorState {
      actor_role: "TopLaner".to_string(),
      team: "Allied".to_string(),
      location_id: "Top".to_string(),
      transit_destination: None,
      transit_beats_remaining: None,
      is_visible: true,
    }],
    vec![],
    vec![],
  );

  let timeline_view = build_gui_timeline_view(1, "Planning", 0, 0, vec![]);
  let plan_view = build_gui_plan_view(
    "TopLaner",
    "Stabilize",
    "Minions",
    "Standard",
    None,
    None,
    None,
    None,
  );
  let debrief_view = Some(build_gui_debrief_view(
    "CoordinatedTriumph",
    "High",
    "High",
    8_000,
    7_500,
    vec![],
    vec![],
  ));
  let accessibility = build_gui_accessibility(vec![], vec![], vec![]);

  let bundle = assemble_gui_presentation_bundle(
    "bundle-001",
    1,
    "TopLaner",
    map_view,
    timeline_view,
    plan_view,
    debrief_view,
    accessibility,
  )
  .expect("bundle should be valid");

  assert_eq!(bundle.schema_version, GUI_DTO_SCHEMA_VERSION);
  assert_eq!(bundle.bundle_id, "bundle-001");
  assert_eq!(bundle.turn, 1);
  assert_eq!(bundle.observer_role, "TopLaner");
  assert!(bundle.validate_invariants().is_ok());
}

#[test]
fn gui_invariant_rejection_of_latent_opponent_leakage() {
  let map_view = GuiMapViewDto {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    observer_role: "MidLaner".to_string(),
    observer_team: "Allied".to_string(),
    turn: 5,
    locations: vec![],
    actors: vec![GuiMapActorState {
      actor_role: "Jungler".to_string(),
      team: "Opposing".to_string(),
      location_id: "BotJungle".to_string(), // LEAK: Unseen opponent revealing hidden coordinate!
      transit_destination: None,
      transit_beats_remaining: None,
      is_visible: false,
    }],
    objectives: vec![],
    structures: vec![],
  };

  let bundle = GuiPresentationBundle {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    bundle_id: "leak-bundle".to_string(),
    turn: 5,
    observer_role: "MidLaner".to_string(),
    map_view,
    timeline_view: build_gui_timeline_view(5, "Planning", 0, 0, vec![]),
    plan_view: build_gui_plan_view(
      "MidLaner", "Contest", "Minions", "Standard", None, None, None, None,
    ),
    debrief_view: None,
    accessibility: build_gui_accessibility(vec![], vec![], vec![]),
  };

  assert_eq!(
    bundle.validate_invariants(),
    Err("unseen opposing actors must not reveal true location")
  );
}

#[test]
fn gui_debrief_chain_of_thought_omission_enforcement() {
  let mut debrief = build_gui_debrief_view(
    "CoordinatedTriumph",
    "High",
    "High",
    8_000,
    7_500,
    vec![],
    vec![],
  );
  debrief.chain_of_thought_omitted = false; // Violation!

  let map_view = build_gui_map_view(1, "TopLaner", "Allied", vec![], vec![], vec![], vec![]);
  let timeline_view = build_gui_timeline_view(1, "Debrief", 0, 0, vec![]);
  let plan_view = build_gui_plan_view(
    "TopLaner",
    "Stabilize",
    "Minions",
    "Standard",
    None,
    None,
    None,
    None,
  );
  let accessibility = build_gui_accessibility(vec![], vec![], vec![]);

  let bundle = GuiPresentationBundle {
    schema_version: GUI_DTO_SCHEMA_VERSION.to_string(),
    bundle_id: "cot-leak-bundle".to_string(),
    turn: 1,
    observer_role: "TopLaner".to_string(),
    map_view,
    timeline_view,
    plan_view,
    debrief_view: Some(debrief),
    accessibility,
  };

  assert_eq!(
    bundle.validate_invariants(),
    Err("debrief must omit private chain of thought")
  );
}

#[test]
fn catalog_scenarios_execute_and_verify_all_expectations() {
  let catalog = GuiScenarioCatalog::new();
  let scenarios = catalog.all_scenarios();
  assert_eq!(scenarios.len(), 3);

  for def in scenarios {
    let result = catalog
      .execute_scenario(def.scenario_id)
      .unwrap_or_else(|_| panic!("scenario '{}' should execute successfully", def.scenario_id));

    assert_eq!(result.scenario_id, def.scenario_id);
    assert!(result.expectations_verified);
    assert!(result.assessment.gui_justified());
    assert!(result.bundle.validate_invariants().is_ok());
  }

  assert!(catalog.execute_scenario("non-existent-scenario").is_err());
}

#[test]
fn markdown_report_rendering_hygiene() {
  let catalog = GuiScenarioCatalog::new();
  let result = catalog
    .execute_scenario("scenario-gui-map-flank-v1")
    .expect("scenario execution should succeed");

  let md = render_presentation_need_markdown(&result.assessment);
  assert!(md.starts_with("# Presentation Need and GUI Justification Report\n\n"));
  assert!(md.contains("- **Assessment ID:** assessment-scenario-gui-map-flank-v1"));
  assert!(md.contains("- **Scenario ID:** scenario-gui-map-flank-v1"));
  assert!(md.contains("- **Mean Deficit Impact:** 57.50%"));
  assert!(md.contains("- **GUI Justified:** [YES] Justified by Comprehension Evidence"));
  assert!(md.contains("## Comprehension Deficits"));
  assert!(md.contains("| spatial-topology | significant-barrier | 7000 bp |"));
  assert!(md.contains("| temporal-timeline | moderate-friction | 4500 bp |"));

  // Check no ANSI escape sequences
  assert!(!md.contains("\x1b["));
}

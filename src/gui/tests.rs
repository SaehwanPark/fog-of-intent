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

#[test]
fn active_tab_and_view_mode_round_trips() {
  use crate::gui::dto::{GuiActiveTab, GuiViewMode};

  let tabs = [
    (GuiActiveTab::MapView, "map-view"),
    (GuiActiveTab::TimelineView, "timeline-view"),
    (GuiActiveTab::PlanView, "plan-view"),
    (GuiActiveTab::DebriefView, "debrief-view"),
    (GuiActiveTab::AccessibilityView, "accessibility-view"),
  ];

  for (tab, name) in tabs {
    assert_eq!(tab.as_str(), name);
    assert_eq!(GuiActiveTab::from_str_name(name), Some(tab));
    assert_eq!(format!("{}", tab), name);
  }
  assert_eq!(GuiActiveTab::from_str_name("unknown-tab"), None);

  let modes = [
    (GuiViewMode::Standard, "standard"),
    (GuiViewMode::Compact, "compact"),
    (GuiViewMode::Inspector, "inspector"),
  ];

  for (mode, name) in modes {
    assert_eq!(mode.as_str(), name);
    assert_eq!(GuiViewMode::from_str_name(name), Some(mode));
    assert_eq!(format!("{}", mode), name);
  }
  assert_eq!(GuiViewMode::from_str_name("ultra-wide"), None);
}

#[test]
fn client_state_initialization_transitions_and_reversibility() {
  use crate::gui::dto::{GuiActiveTab, GuiViewMode};
  use crate::gui::state::{GuiClientEvent, GuiClientState, GuiPresentationAction};

  let catalog = GuiScenarioCatalog::new();
  let map_scen = catalog
    .get("scenario-gui-map-flank-v1")
    .expect("scenario must exist");

  let mut state = GuiClientState::new("MidLaner");
  assert!(state.is_neutral());
  assert_eq!(state.active_tab, GuiActiveTab::MapView);
  assert!(state.selection.is_empty());

  // Transition: SelectTab
  let event = state
    .transition(
      GuiPresentationAction::SelectTab(GuiActiveTab::TimelineView),
      &map_scen.sample_bundle,
    )
    .expect("transition should succeed");
  assert_eq!(
    event,
    GuiClientEvent::TabChanged(GuiActiveTab::TimelineView)
  );
  assert_eq!(state.active_tab, GuiActiveTab::TimelineView);
  assert!(!state.is_neutral());

  // Transition: SetViewMode
  let event = state
    .transition(
      GuiPresentationAction::SetViewMode(GuiViewMode::Compact),
      &map_scen.sample_bundle,
    )
    .expect("transition should succeed");
  assert_eq!(event, GuiClientEvent::ViewModeChanged(GuiViewMode::Compact));
  assert_eq!(state.display_options.view_mode, GuiViewMode::Compact);

  // Transition: SelectLocation
  let event = state
    .transition(
      GuiPresentationAction::SelectLocation("BotRiver".to_string()),
      &map_scen.sample_bundle,
    )
    .expect("transition should succeed");
  assert_eq!(
    event,
    GuiClientEvent::LocationSelected("BotRiver".to_string())
  );
  assert_eq!(
    state.selection.selected_location_id.as_deref(),
    Some("BotRiver")
  );

  // Transition: SelectActor
  let event = state
    .transition(
      GuiPresentationAction::SelectActor("MidLaner".to_string()),
      &map_scen.sample_bundle,
    )
    .expect("transition should succeed");
  assert_eq!(event, GuiClientEvent::ActorSelected("MidLaner".to_string()));
  assert_eq!(
    state.selection.selected_actor_role.as_deref(),
    Some("MidLaner")
  );

  // Transition: SelectObjective
  let event = state
    .transition(
      GuiPresentationAction::SelectObjective("BotRiverObjective".to_string()),
      &map_scen.sample_bundle,
    )
    .expect("transition should succeed");
  assert_eq!(
    event,
    GuiClientEvent::ObjectiveSelected("BotRiverObjective".to_string())
  );

  // Transition: SelectStructure
  let event = state
    .transition(
      GuiPresentationAction::SelectStructure("OuterTurret".to_string()),
      &map_scen.sample_bundle,
    )
    .expect("transition should succeed");
  assert_eq!(
    event,
    GuiClientEvent::StructureSelected("OuterTurret".to_string())
  );

  // Transition: ResetInspection (reversible: clears selection, preserves tab and options)
  let event = state
    .transition(
      GuiPresentationAction::ResetInspection,
      &map_scen.sample_bundle,
    )
    .expect("reset inspection should succeed");
  assert_eq!(event, GuiClientEvent::InspectionReset);
  assert!(state.selection.is_empty());
  assert_eq!(state.active_tab, GuiActiveTab::TimelineView);
  assert_eq!(state.display_options.view_mode, GuiViewMode::Compact);

  // Transition: ResetAll (complete reversion to default initial neutral state)
  let event = state
    .transition(GuiPresentationAction::ResetAll, &map_scen.sample_bundle)
    .expect("reset all should succeed");
  assert_eq!(event, GuiClientEvent::StateRevertedToDefault);
  assert!(state.is_neutral());
}

#[test]
fn client_state_zoom_bounds_and_display_toggles() {
  use crate::gui::state::{
    DEFAULT_ZOOM_LEVEL_BP, GuiClientError, GuiClientEvent, GuiClientState, GuiPresentationAction,
    MAX_ZOOM_LEVEL_BP, MIN_ZOOM_LEVEL_BP,
  };

  let catalog = GuiScenarioCatalog::new();
  let map_scen = catalog
    .get("scenario-gui-map-flank-v1")
    .expect("scenario must exist");

  let mut state = GuiClientState::new("MidLaner");
  assert_eq!(state.display_options.zoom_level_bp, DEFAULT_ZOOM_LEVEL_BP);

  // Zoom bounds
  assert_eq!(
    state.transition(
      GuiPresentationAction::SetZoom(4_999),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::InvalidZoomLevel(4_999))
  );
  assert_eq!(
    state.transition(
      GuiPresentationAction::SetZoom(20_001),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::InvalidZoomLevel(20_001))
  );

  let event = state
    .transition(
      GuiPresentationAction::SetZoom(MIN_ZOOM_LEVEL_BP),
      &map_scen.sample_bundle,
    )
    .expect("zoom at min bound should succeed");
  assert_eq!(event, GuiClientEvent::ZoomChanged(MIN_ZOOM_LEVEL_BP));
  assert_eq!(state.display_options.zoom_level_bp, MIN_ZOOM_LEVEL_BP);

  let event = state
    .transition(
      GuiPresentationAction::SetZoom(MAX_ZOOM_LEVEL_BP),
      &map_scen.sample_bundle,
    )
    .expect("zoom at max bound should succeed");
  assert_eq!(event, GuiClientEvent::ZoomChanged(MAX_ZOOM_LEVEL_BP));
  assert_eq!(state.display_options.zoom_level_bp, MAX_ZOOM_LEVEL_BP);

  // Toggles
  let event = state
    .transition(
      GuiPresentationAction::ToggleFogOverlay,
      &map_scen.sample_bundle,
    )
    .expect("toggle fog should succeed");
  assert_eq!(event, GuiClientEvent::FogOverlayToggled(false));
  assert!(!state.display_options.fog_overlay_enabled);

  let event = state
    .transition(
      GuiPresentationAction::ToggleHighContrast,
      &map_scen.sample_bundle,
    )
    .expect("toggle contrast should succeed");
  assert_eq!(event, GuiClientEvent::HighContrastToggled(true));
  assert!(state.display_options.high_contrast_enabled);

  let event = state
    .transition(
      GuiPresentationAction::ToggleReducedMotion,
      &map_scen.sample_bundle,
    )
    .expect("toggle motion should succeed");
  assert_eq!(event, GuiClientEvent::ReducedMotionToggled(true));
  assert!(state.display_options.reduced_motion_enabled);

  let event = state
    .transition(
      GuiPresentationAction::ToggleSymbolTags,
      &map_scen.sample_bundle,
    )
    .expect("toggle tags should succeed");
  assert_eq!(event, GuiClientEvent::SymbolTagsToggled(false));
  assert!(!state.display_options.symbol_tags_visible);
}

#[test]
fn client_state_fail_closed_validation() {
  use crate::gui::state::{GuiClientError, GuiClientState, GuiPresentationAction};

  let catalog = GuiScenarioCatalog::new();
  let map_scen = catalog
    .get("scenario-gui-map-flank-v1")
    .expect("scenario must exist");

  let mut state = GuiClientState::new("MidLaner");

  // Empty identifier checks
  assert_eq!(
    state.transition(
      GuiPresentationAction::SelectLocation("  ".to_string()),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::EmptyIdentifier("location_id"))
  );
  assert_eq!(
    state.transition(
      GuiPresentationAction::SelectActor("  ".to_string()),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::EmptyIdentifier("actor_role"))
  );
  assert_eq!(
    state.transition(
      GuiPresentationAction::SelectObjective("  ".to_string()),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::EmptyIdentifier("objective_kind"))
  );
  assert_eq!(
    state.transition(
      GuiPresentationAction::SelectStructure("  ".to_string()),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::EmptyIdentifier("structure_tier"))
  );
  assert_eq!(
    state.transition(
      GuiPresentationAction::SelectDebriefQuadrant("  ".to_string()),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::EmptyIdentifier("debrief_quadrant"))
  );

  // Unknown entity / invisible actor checks
  assert_eq!(
    state.transition(
      GuiPresentationAction::SelectLocation("NonExistentLocation".to_string()),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::UnknownLocationId(
      "NonExistentLocation".to_string()
    ))
  );

  // In map_flank_scenario, Opposing BotCarry is unseen (is_visible: false) -> must fail closed
  assert_eq!(
    state.transition(
      GuiPresentationAction::SelectActor("BotCarry".to_string()),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::UnknownActorRole("BotCarry".to_string()))
  );

  // Turn bounds
  assert_eq!(
    state.transition(
      GuiPresentationAction::SetTimelineTurn(0),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::TurnOutOfRange(0))
  );
  assert_eq!(
    state.transition(
      GuiPresentationAction::SetTimelineTurn(999),
      &map_scen.sample_bundle
    ),
    Err(GuiClientError::TurnOutOfRange(999))
  );
}

#[test]
fn client_state_error_display_formatting_coverage() {
  use crate::gui::state::GuiClientError;

  let errors = [
    (
      GuiClientError::EmptyIdentifier("actor_role"),
      "identifier field 'actor_role' must not be empty",
    ),
    (
      GuiClientError::InvalidZoomLevel(25_000),
      "zoom level 25000 bp is outside allowed range [5000..=20000] bp",
    ),
    (
      GuiClientError::UnknownLocationId("BaronPit".to_string()),
      "location 'BaronPit' is not visible on the map",
    ),
    (
      GuiClientError::UnknownActorRole("Jungler".to_string()),
      "actor role 'Jungler' is not visible or unknown",
    ),
    (
      GuiClientError::UnknownObjectiveKind("Dragon".to_string()),
      "objective 'Dragon' is not visible on the map",
    ),
    (
      GuiClientError::UnknownStructureTier("Inhibitor".to_string()),
      "structure tier 'Inhibitor' is not visible on the map",
    ),
    (
      GuiClientError::UnknownQuadrant("SuperVictory".to_string()),
      "debrief quadrant 'SuperVictory' is unrecognized",
    ),
    (
      GuiClientError::TurnOutOfRange(42),
      "turn 42 is out of range",
    ),
  ];

  for (err, expected_str) in errors {
    assert_eq!(format!("{}", err), expected_str);
  }
}

#[test]
fn presentation_parity_verification_success() {
  use crate::gui::parity::verify_presentation_parity;
  use crate::lane::{LaneSnapshot, ObservationId, observe_player};
  use crate::protocol::ActorObservationDto;

  let state = LaneSnapshot::initial();
  let cli_obs = observe_player(&state, ObservationId::new(1)).observation();
  let mcp_obs = ActorObservationDto::from_observation(cli_obs);

  let map_view = build_gui_map_view(
    0,
    "MidLaner",
    "Allied",
    vec![GuiMapLocationState {
      location_id: "Center".to_string(),
      terrain_kind: "lane".to_string(),
      vision_status: GuiVisionStatus::FullVision,
      last_seen_turn: Some(0),
    }],
    vec![GuiMapActorState {
      actor_role: "MidLaner".to_string(),
      team: "Allied".to_string(),
      location_id: "Center".to_string(),
      transit_destination: None,
      transit_beats_remaining: None,
      is_visible: true,
    }],
    vec![],
    vec![],
  );

  let bundle = assemble_gui_presentation_bundle(
    "bundle-parity-01",
    0,
    "MidLaner",
    map_view,
    build_gui_timeline_view(0, "Planning", 0, 0, vec![]),
    build_gui_plan_view(
      "MidLaner",
      "Stabilize",
      "Minions",
      "Standard",
      None,
      None,
      None,
      None,
    ),
    None,
    build_gui_accessibility(vec![], vec![], vec![]),
  )
  .expect("bundle assembly should succeed");

  let report = verify_presentation_parity("parity-rep-01", &cli_obs, &mcp_obs, &bundle)
    .expect("parity verification should succeed");

  assert_eq!(report.report_id, "parity-rep-01");
  assert_eq!(report.turn, 0);
  assert!(report.all_surfaces_in_parity);
  assert!(report.cli_parity_verified);
  assert!(report.mcp_parity_verified);
  assert!(report.gui_parity_verified);
  assert!(report.zero_hash_leakage_verified);
  assert!(report.zero_latent_leakage_verified);
  assert!(report.zero_cot_leakage_verified);
}

#[test]
fn presentation_parity_discrepancy_and_invariant_rejection() {
  use crate::gui::parity::{GuiParityError, verify_presentation_parity};
  use crate::lane::{LaneSnapshot, ObservationId, observe_player};
  use crate::protocol::ActorObservationDto;

  let state = LaneSnapshot::initial();
  let cli_obs = observe_player(&state, ObservationId::new(1)).observation();
  let mcp_obs = ActorObservationDto::from_observation(cli_obs);

  // Turn mismatch in GUI bundle
  let map_view = build_gui_map_view(2, "MidLaner", "Allied", vec![], vec![], vec![], vec![]);
  let bundle_wrong_turn = assemble_gui_presentation_bundle(
    "b-01",
    2, // Turn 2 vs Turn 0!
    "MidLaner",
    map_view,
    build_gui_timeline_view(2, "Planning", 0, 0, vec![]),
    build_gui_plan_view(
      "MidLaner",
      "Stabilize",
      "Minions",
      "Standard",
      None,
      None,
      None,
      None,
    ),
    None,
    build_gui_accessibility(vec![], vec![], vec![]),
  )
  .expect("bundle assembly should succeed");

  assert!(matches!(
    verify_presentation_parity("p-01", &cli_obs, &mcp_obs, &bundle_wrong_turn),
    Err(GuiParityError::TurnMismatch { .. })
  ));

  // Role mismatch in GUI bundle (same turn 0)
  let map_view_wrong_role =
    build_gui_map_view(0, "UnknownRole", "Allied", vec![], vec![], vec![], vec![]);
  let bundle_wrong_role = assemble_gui_presentation_bundle(
    "b-02",
    0,
    "UnknownRole",
    map_view_wrong_role,
    build_gui_timeline_view(0, "Planning", 0, 0, vec![]),
    build_gui_plan_view(
      "UnknownRole",
      "Stabilize",
      "Minions",
      "Standard",
      None,
      None,
      None,
      None,
    ),
    None,
    build_gui_accessibility(vec![], vec![], vec![]),
  )
  .expect("bundle assembly should succeed");

  assert!(matches!(
    verify_presentation_parity("p-02", &cli_obs, &mcp_obs, &bundle_wrong_role),
    Err(GuiParityError::RoleMismatch { .. })
  ));
}

#[test]
fn presentation_parity_error_display_coverage() {
  use crate::gui::parity::GuiParityError;

  let errors = [
    (
      GuiParityError::EmptyIdentifier("report_id"),
      "identifier field 'report_id' must not be empty",
    ),
    (
      GuiParityError::TurnMismatch {
        cli_turn: 1,
        mcp_turn: 2,
        gui_turn: 1,
      },
      "turn mismatch across surfaces: CLI=1, MCP=2, GUI=1",
    ),
    (
      GuiParityError::RoleMismatch {
        cli_role: "Laner".to_string(),
        mcp_role: "Observer-1".to_string(),
        gui_role: "UnknownRole".to_string(),
      },
      "role mismatch across surfaces: CLI='Laner', MCP='Observer-1', GUI='UnknownRole'",
    ),
    (
      GuiParityError::IntentSetMismatch("missing recall".to_string()),
      "legal intent set mismatch: missing recall",
    ),
    (
      GuiParityError::InvariantViolation("hash leak"),
      "presentation invariant violation: hash leak",
    ),
  ];

  for (err, expected_str) in errors {
    assert_eq!(format!("{}", err), expected_str);
  }
}

#[test]
fn state_catalog_scenarios_execute_and_verify_all_expectations() {
  use crate::gui::state_catalog::GuiStateScenarioCatalog;

  let catalog = GuiStateScenarioCatalog::new();
  let scenarios = catalog.all_scenarios();
  assert_eq!(scenarios.len(), 3);

  for def in scenarios {
    let result = catalog
      .execute_scenario(def.scenario_id)
      .unwrap_or_else(|_| {
        panic!(
          "state scenario '{}' should execute successfully",
          def.scenario_id
        )
      });

    assert_eq!(result.scenario_id, def.scenario_id);
    assert!(result.expectations_verified);
    assert_eq!(result.final_state.active_tab, def.expected_final_tab);
    assert_eq!(
      result.final_state.selection.is_empty(),
      def.expected_final_selection_empty
    );
    assert_eq!(
      result.final_state.display_options.zoom_level_bp,
      def.expected_final_zoom_bp
    );
    assert_eq!(result.final_state.is_neutral(), def.expected_final_neutral);
  }

  assert!(
    catalog
      .execute_scenario("non-existent-state-scenario")
      .is_err()
  );
}

#[test]
fn state_and_parity_markdown_rendering_hygiene() {
  use crate::gui::parity::render_parity_report_markdown;
  use crate::gui::state_catalog::GuiStateScenarioCatalog;

  let catalog = GuiStateScenarioCatalog::new();
  let result = catalog
    .execute_scenario("scenario-gui-state-map-inspection-v1")
    .expect("scenario execution should succeed");

  let state_md = result.final_state.render_client_state_markdown();
  assert!(state_md.starts_with("# GUI Presentation Client State\n\n"));
  assert!(state_md.contains("- **Observer Role:** MidLaner"));
  assert!(state_md.contains("- **Active Tab:** map-view"));
  assert!(state_md.contains("- **Display Zoom:** 125.00%"));
  assert!(state_md.contains("- **High Contrast:** Enabled"));
  assert!(!state_md.contains("\x1b["));

  let parity_report = crate::gui::parity::GuiParityCheckReport {
    schema_version: crate::gui::parity::GUI_PARITY_SCHEMA_VERSION.to_string(),
    report_id: "parity-md-test".to_string(),
    observer_role: "MidLaner".to_string(),
    turn: 8,
    all_surfaces_in_parity: true,
    cli_parity_verified: true,
    mcp_parity_verified: true,
    gui_parity_verified: true,
    zero_hash_leakage_verified: true,
    zero_latent_leakage_verified: true,
    zero_cot_leakage_verified: true,
    verified_intents: vec!["stabilize".to_string(), "contest".to_string()],
    discrepancies: vec![],
  };

  let parity_md = render_parity_report_markdown(&parity_report);
  assert!(parity_md.starts_with("# Presentation Projection Parity Report\n\n"));
  assert!(parity_md.contains("- **Report ID:** parity-md-test"));
  assert!(parity_md.contains("- **Observer Role:** MidLaner"));
  assert!(
    parity_md.contains("- **All Surfaces in Parity:** [YES] Exact Parity Across CLI, MCP, and GUI")
  );
  assert!(parity_md.contains("- **Zero State Hash Leakage:** [PASS] Strictly Redacted"));
  assert!(parity_md.contains("- `stabilize`"));
  assert!(!parity_md.contains("\x1b["));
}

//! Canonical benchmark scenarios for presentation need assessment and GUI DTO validation.

use crate::gui::dto::{
  GuiAccessibilityDto, GuiDebriefViewDto, GuiKpiCard, GuiMapActorState, GuiMapLocationState,
  GuiMapObjectiveState, GuiMapStructureState, GuiMapViewDto, GuiPlanViewDto, GuiPresentationBundle,
  GuiSymbolTag, GuiTimelineViewDto, GuiVisionStatus,
};
use crate::gui::need::{
  ComprehensionDeficit, ComprehensionDomain, DeficitSeverity, PresentationNeedAssessment,
  evaluate_presentation_need,
};

/// Schema version for GUI scenario catalog.
pub const GUI_CATALOG_SCHEMA_VERSION: &str = "m11-gui-scenario-catalog-v1";

/// Definition of a benchmark presentation and GUI evaluation scenario.
#[derive(Debug, Clone)]
pub struct GuiScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub observer_role: &'static str,
  pub deficits: Vec<ComprehensionDeficit>,
  pub evidence_rationale: &'static str,
  pub expected_mean_deficit_bp: u32,
  pub expected_gui_justified: bool,
  pub sample_bundle: GuiPresentationBundle,
}

/// Execution result for a benchmark GUI scenario evaluation.
#[derive(Debug, Clone)]
pub struct GuiScenarioExecutionResult {
  pub scenario_id: String,
  pub assessment: PresentationNeedAssessment,
  pub bundle: GuiPresentationBundle,
  pub expectations_verified: bool,
}

/// Canonical catalog of benchmark GUI scenarios.
#[derive(Debug, Default)]
pub struct GuiScenarioCatalog;

impl GuiScenarioCatalog {
  /// Create a new instance of the catalog.
  pub fn new() -> Self {
    Self
  }

  /// Look up a scenario definition by ID.
  pub fn get(&self, id: &str) -> Option<GuiScenarioDefinition> {
    match id {
      "scenario-gui-map-flank-v1" => Some(Self::map_flank_scenario()),
      "scenario-gui-debrief-quadrant-v1" => Some(Self::debrief_quadrant_scenario()),
      "scenario-gui-timeline-siege-v1" => Some(Self::timeline_siege_scenario()),
      _ => None,
    }
  }

  /// Return all registered benchmark scenarios.
  pub fn all_scenarios(&self) -> Vec<GuiScenarioDefinition> {
    vec![
      Self::map_flank_scenario(),
      Self::debrief_quadrant_scenario(),
      Self::timeline_siege_scenario(),
    ]
  }

  /// Execute and verify a canonical benchmark scenario.
  pub fn execute_scenario(&self, id: &str) -> Result<GuiScenarioExecutionResult, &'static str> {
    let def = self.get(id).ok_or("scenario not found in catalog")?;
    let assessment = evaluate_presentation_need(
      &format!("assessment-{}", def.scenario_id),
      def.scenario_id,
      def.observer_role,
      def.deficits.clone(),
      def.evidence_rationale,
    )
    .map_err(|_| "presentation need evaluation failed")?;

    def
      .sample_bundle
      .validate_invariants()
      .map_err(|_| "presentation bundle invariant validation failed")?;

    let expectations_verified = assessment.mean_deficit_impact_bp() == def.expected_mean_deficit_bp
      && assessment.gui_justified() == def.expected_gui_justified;

    Ok(GuiScenarioExecutionResult {
      scenario_id: def.scenario_id.to_string(),
      assessment,
      bundle: def.sample_bundle,
      expectations_verified,
    })
  }

  fn map_flank_scenario() -> GuiScenarioDefinition {
    let deficits = vec![
      ComprehensionDeficit::new(
        ComprehensionDomain::SpatialTopology,
        DeficitSeverity::SignificantBarrier,
        7_000,
        "Linear text listing coordinates obscures 3-way rotation angles and flank approach vector",
        "Spatial positioning must be reconstructed manually by cross-referencing node names",
        "2D interactive vector map displays vision radii, trajectory arrows, and fog boundaries",
      ),
      ComprehensionDeficit::new(
        ComprehensionDomain::TemporalTimeline,
        DeficitSeverity::ModerateFriction,
        4_500,
        "Multi-beat transit duration is disconnected from turn progression",
        "Transit progress requires inspecting discrete transit state fields across lines",
        "Timeline transit bar showing progress ticks and remaining arrival beats",
      ),
    ];

    let map_view = GuiMapViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      observer_role: "MidLaner".to_string(),
      observer_team: "Allied".to_string(),
      turn: 8,
      locations: vec![
        GuiMapLocationState {
          location_id: "Mid".to_string(),
          terrain_kind: "lane".to_string(),
          vision_status: GuiVisionStatus::FullVision,
          last_seen_turn: Some(8),
        },
        GuiMapLocationState {
          location_id: "BotRiver".to_string(),
          terrain_kind: "river".to_string(),
          vision_status: GuiVisionStatus::FullVision,
          last_seen_turn: Some(8),
        },
        GuiMapLocationState {
          location_id: "Bot".to_string(),
          terrain_kind: "lane".to_string(),
          vision_status: GuiVisionStatus::LastKnown,
          last_seen_turn: Some(7),
        },
      ],
      actors: vec![
        GuiMapActorState {
          actor_role: "MidLaner".to_string(),
          team: "Allied".to_string(),
          location_id: "BotRiver".to_string(),
          transit_destination: Some("Bot".to_string()),
          transit_beats_remaining: Some(1),
          is_visible: true,
        },
        GuiMapActorState {
          actor_role: "Jungler".to_string(),
          team: "Allied".to_string(),
          location_id: "BotRiver".to_string(),
          transit_destination: None,
          transit_beats_remaining: None,
          is_visible: true,
        },
        GuiMapActorState {
          actor_role: "BotCarry".to_string(),
          team: "Opposing".to_string(),
          location_id: "Unknown".to_string(),
          transit_destination: None,
          transit_beats_remaining: None,
          is_visible: false,
        },
      ],
      objectives: vec![GuiMapObjectiveState {
        objective_kind: "BotRiverObjective".to_string(),
        status: "Active".to_string(),
        health_percent_bp: 10_000,
        respawn_turns_remaining: None,
      }],
      structures: vec![GuiMapStructureState {
        structure_tier: "OuterTurret".to_string(),
        team: "Opposing".to_string(),
        lane: "Bot".to_string(),
        status: "Intact".to_string(),
        health_percent_bp: 8_500,
        is_vulnerable: true,
      }],
    };

    let timeline_view = GuiTimelineViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      current_turn: 8,
      current_phase: "Execution".to_string(),
      active_rotations_count: 1,
      pending_delayed_effects_count: 0,
      scheduled_objective_spawns: vec!["Drake Active".to_string()],
    };

    let plan_view = GuiPlanViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      observer_role: "MidLaner".to_string(),
      selected_intent: "Contest".to_string(),
      target_focus: "BotRiverObjective".to_string(),
      commitment: "High".to_string(),
      ping_signal: Some("OnMyWay".to_string()),
      abort_condition: Some("ThreatPresent".to_string()),
      fallback_behavior: Some("Withdraw".to_string()),
      staged_message_preview: Some("Flanking Bot via River".to_string()),
    };

    let accessibility = GuiAccessibilityDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      non_color_symbol_tags: vec![
        GuiSymbolTag {
          entity_id: "MidLaner".to_string(),
          symbol_code: "[ALLY-MID]".to_string(),
          label: "Allied Mid Laner in River".to_string(),
        },
        GuiSymbolTag {
          entity_id: "Opponent".to_string(),
          symbol_code: "[ENEMY-FOG]".to_string(),
          label: "Opposing Bot Carry in Fog".to_string(),
        },
      ],
      aria_announcements: vec![
        "Turn 8 Execution Phase: Mid Laner rotating to Bot through Bot River (1 beat remaining)"
          .to_string(),
      ],
      keyboard_focus_order: vec![
        "map-canvas".to_string(),
        "actor-MidLaner".to_string(),
        "objective-Drake".to_string(),
        "timeline-bar".to_string(),
      ],
      high_contrast_available: true,
      reduced_motion_compatible: true,
    };

    let sample_bundle = GuiPresentationBundle {
      schema_version: "m11-gui-dto-v1".to_string(),
      bundle_id: "bundle-map-flank-v1".to_string(),
      turn: 8,
      observer_role: "MidLaner".to_string(),
      map_view,
      timeline_view,
      plan_view,
      debrief_view: None,
      accessibility,
    };

    GuiScenarioDefinition {
      scenario_id: "scenario-gui-map-flank-v1",
      title: "Three-Lane Flank and Vision Trap",
      description: "Mid laner setting up a Bot flank through River while Jungler secures Dragon vision.",
      observer_role: "MidLaner",
      deficits,
      evidence_rationale: "Spatial rotation geometry across three lanes causes significant cognitive barrier in text mode.",
      expected_mean_deficit_bp: 5_750,
      expected_gui_justified: true,
      sample_bundle,
    }
  }

  fn debrief_quadrant_scenario() -> GuiScenarioDefinition {
    let deficits = vec![
      ComprehensionDeficit::new(
        ComprehensionDomain::CausalDebrief,
        DeficitSeverity::SignificantBarrier,
        7_500,
        "Text summary conflates team coordination success with mechanical execution loss",
        "Requires synthesizing multiple paragraphs to compute orthogonal coordination vs execution scores",
        "2D quadrant chart clearly mapping CoordinatedFailure with badge cards and KPI breakdown",
      ),
      ComprehensionDeficit::new(
        ComprehensionDomain::ContingencyBranching,
        DeficitSeverity::ModerateFriction,
        4_000,
        "Contingency abort evaluations are obscured in textual log records",
        "Reviewing why an abort did not trigger requires scanning log lines",
        "Visual contingency branch diagram displaying condition evaluation status",
      ),
    ];

    let map_view = GuiMapViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      observer_role: "TopLaner".to_string(),
      observer_team: "Allied".to_string(),
      turn: 15,
      locations: vec![GuiMapLocationState {
        location_id: "Top".to_string(),
        terrain_kind: "lane".to_string(),
        vision_status: GuiVisionStatus::FullVision,
        last_seen_turn: Some(15),
      }],
      actors: vec![GuiMapActorState {
        actor_role: "TopLaner".to_string(),
        team: "Allied".to_string(),
        location_id: "Top".to_string(),
        transit_destination: None,
        transit_beats_remaining: None,
        is_visible: true,
      }],
      objectives: vec![],
      structures: vec![],
    };

    let timeline_view = GuiTimelineViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      current_turn: 15,
      current_phase: "Debrief".to_string(),
      active_rotations_count: 0,
      pending_delayed_effects_count: 0,
      scheduled_objective_spawns: vec![],
    };

    let plan_view = GuiPlanViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      observer_role: "TopLaner".to_string(),
      selected_intent: "Stabilize".to_string(),
      target_focus: "Tower".to_string(),
      commitment: "Standard".to_string(),
      ping_signal: None,
      abort_condition: None,
      fallback_behavior: None,
      staged_message_preview: None,
    };

    let debrief_view = Some(GuiDebriefViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      quadrant: "CoordinatedFailure".to_string(),
      coordination_rating: "High".to_string(),
      execution_rating: "Unfavorable".to_string(),
      coordination_score_bp: 7_800,
      execution_score_bp: 3_200,
      kpi_cards: vec![
        GuiKpiCard {
          label: "Team Plan Alignment".to_string(),
          score_bp: 8_500,
          tier: "Excellent".to_string(),
        },
        GuiKpiCard {
          label: "Damage Output Conversion".to_string(),
          score_bp: 3_100,
          tier: "Low".to_string(),
        },
      ],
      causal_factor_tags: vec![
        "HighDirectiveCompliance".to_string(),
        "MechanicalOutplayByOpponent".to_string(),
      ],
      chain_of_thought_omitted: true,
    });

    let accessibility = GuiAccessibilityDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      non_color_symbol_tags: vec![GuiSymbolTag {
        entity_id: "Quadrant".to_string(),
        symbol_code: "[QUADRANT-COORD-FAIL]".to_string(),
        label: "Quadrant: Coordinated Failure (High Strategy, Low Execution)".to_string(),
      }],
      aria_announcements: vec![
        "Causal Debrief: Encounter classified into Coordinated Failure quadrant (Coordination 78%, Execution 32%)"
          .to_string(),
      ],
      keyboard_focus_order: vec![
        "debrief-quadrant-chart".to_string(),
        "kpi-card-1".to_string(),
        "kpi-card-2".to_string(),
      ],
      high_contrast_available: true,
      reduced_motion_compatible: true,
    };

    let sample_bundle = GuiPresentationBundle {
      schema_version: "m11-gui-dto-v1".to_string(),
      bundle_id: "bundle-debrief-quadrant-v1".to_string(),
      turn: 15,
      observer_role: "TopLaner".to_string(),
      map_view,
      timeline_view,
      plan_view,
      debrief_view,
      accessibility,
    };

    GuiScenarioDefinition {
      scenario_id: "scenario-gui-debrief-quadrant-v1",
      title: "Causal Debrief Decomposition with Decoupled Attribution",
      description: "Post-match causal analysis of a sound strategic dive defeated by mechanical clutch.",
      observer_role: "TopLaner",
      deficits,
      evidence_rationale: "Multi-dimensional causal attribution decomposition requires 2D matrix visualization to prevent outcome bias.",
      expected_mean_deficit_bp: 5_750,
      expected_gui_justified: true,
      sample_bundle,
    }
  }

  fn timeline_siege_scenario() -> GuiScenarioDefinition {
    let deficits = vec![
      ComprehensionDeficit::new(
        ComprehensionDomain::TemporalTimeline,
        DeficitSeverity::SignificantBarrier,
        6_500,
        "Simultaneous inhibitor respawn countdowns, turret damage, and super minion waves create temporal overload in text",
        "Tracking turn countdowns across multiple lines requires cognitive arithmetic",
        "Dynamic timeline with progress bars and synchronized objective spawn timers",
      ),
      ComprehensionDeficit::new(
        ComprehensionDomain::SpatialTopology,
        DeficitSeverity::ModerateFriction,
        4_200,
        "26 structure defense statuses listed as text lines require exhaustive vertical scanning",
        "Determining which inner turrets are vulnerable requires checking parent outer turret statuses",
        "Map structure overlay highlighting vulnerable defense points with distinct icons",
      ),
    ];

    let map_view = GuiMapViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      observer_role: "BotCarry".to_string(),
      observer_team: "Allied".to_string(),
      turn: 20,
      locations: vec![GuiMapLocationState {
        location_id: "Mid".to_string(),
        terrain_kind: "lane".to_string(),
        vision_status: GuiVisionStatus::FullVision,
        last_seen_turn: Some(20),
      }],
      actors: vec![GuiMapActorState {
        actor_role: "BotCarry".to_string(),
        team: "Allied".to_string(),
        location_id: "Mid".to_string(),
        transit_destination: None,
        transit_beats_remaining: None,
        is_visible: true,
      }],
      objectives: vec![],
      structures: vec![
        GuiMapStructureState {
          structure_tier: "OuterTurret".to_string(),
          team: "Opposing".to_string(),
          lane: "Mid".to_string(),
          status: "Destroyed".to_string(),
          health_percent_bp: 0,
          is_vulnerable: false,
        },
        GuiMapStructureState {
          structure_tier: "InnerTurret".to_string(),
          team: "Opposing".to_string(),
          lane: "Mid".to_string(),
          status: "Damaged".to_string(),
          health_percent_bp: 4_200,
          is_vulnerable: true,
        },
      ],
    };

    let timeline_view = GuiTimelineViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      current_turn: 20,
      current_phase: "Execution".to_string(),
      active_rotations_count: 0,
      pending_delayed_effects_count: 2,
      scheduled_objective_spawns: vec!["Baron Spawn (3 turns)".to_string()],
    };

    let plan_view = GuiPlanViewDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      observer_role: "BotCarry".to_string(),
      selected_intent: "Contest".to_string(),
      target_focus: "InnerTurret".to_string(),
      commitment: "High".to_string(),
      ping_signal: Some("Attack".to_string()),
      abort_condition: Some("AlliedRetreat".to_string()),
      fallback_behavior: Some("MaintainPlan".to_string()),
      staged_message_preview: Some("Focusing Mid Inner Turret".to_string()),
    };

    let accessibility = GuiAccessibilityDto {
      schema_version: "m11-gui-dto-v1".to_string(),
      non_color_symbol_tags: vec![GuiSymbolTag {
        entity_id: "InnerTurret".to_string(),
        symbol_code: "[TURRET-VULN-42%]".to_string(),
        label: "Mid Inner Turret Vulnerable (42% HP)".to_string(),
      }],
      aria_announcements: vec![
        "Turn 20: Mid Inner Turret vulnerable at 42% HP. Baron spawning in 3 turns.".to_string(),
      ],
      keyboard_focus_order: vec![
        "structure-Mid-InnerTurret".to_string(),
        "timeline-baron-countdown".to_string(),
      ],
      high_contrast_available: true,
      reduced_motion_compatible: true,
    };

    let sample_bundle = GuiPresentationBundle {
      schema_version: "m11-gui-dto-v1".to_string(),
      bundle_id: "bundle-timeline-siege-v1".to_string(),
      turn: 20,
      observer_role: "BotCarry".to_string(),
      map_view,
      timeline_view,
      plan_view,
      debrief_view: None,
      accessibility,
    };

    GuiScenarioDefinition {
      scenario_id: "scenario-gui-timeline-siege-v1",
      title: "Multi-Turn Inhibitor Siege and Super Minion Progression",
      description: "Late game siege tracking inhibitor countdowns, turret vulnerabilities, and Baron spawn timing.",
      observer_role: "BotCarry",
      deficits,
      evidence_rationale: "Complex multi-turn siege and objective countdowns introduce significant cognitive load in text stream.",
      expected_mean_deficit_bp: 5_350,
      expected_gui_justified: true,
      sample_bundle,
    }
  }
}

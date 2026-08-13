//! Attribution of coordination success and failure separately from execution.
//!
//! In Fog of Intent, strategic play involves human or agent coordinators formulating
//! plans and contingencies, while simulated actors execute them under bounded rationality
//! and uncertainty. Evaluating a strategic turn simply by net outcome (win/loss or damage)
//! conflates strategic coordination quality with tactical mechanical execution or stochastic
//! variance.
//!
//! This module formalizes the decoupling of coordination assessment from execution assessment,
//! classifying strategic encounters into the four canonical attribution quadrants:
//! 1. `CoordinatedTriumph`: Sound coordination and successful execution.
//! 2. `CoordinatedFailure`: Sound coordination but failed execution (e.g. mechanical outplay).
//! 3. `UncoordinatedBailout`: Failed/dissenting coordination carried by individual clutch execution.
//! 4. `CompoundedFailure`: Fractured coordination compounded by mechanical execution failure.

use core::fmt;

use crate::agent::simultaneous::{
  MAX_COHESION_BP, TeamCoordinationOutcome, TeamSimultaneousResolution,
};
use crate::lane::LaneOutcome;

/// Versioned schema for coordination and execution attribution structures.
pub const ATTRIBUTION_SCHEMA: &str = "m8-coordination-execution-attribution-v1";

/// Versioned schema for coordination and execution attribution reports.
pub const ATTRIBUTION_REPORT_SCHEMA: &str = "m8-coordination-execution-attribution-report-v1";

/// Versioned schema for the coordination attribution scenario catalog.
pub const ATTRIBUTION_CATALOG_SCHEMA: &str = "m8-coordination-attribution-catalog-v1";

/// Maximum basis points value ($10,000$ bp = 100%).
pub const MAX_ATTRIBUTION_BP: u32 = 10_000;

/// Threshold for effective coordination ($5,000$ bp = 50%).
pub const COORDINATION_THRESHOLD_BP: u32 = 5_000;

/// Threshold for effective execution ($5,000$ bp = 50%).
pub const EXECUTION_THRESHOLD_BP: u32 = 5_000;

/// High rating threshold ($7,500$ bp = 75%).
pub const HIGH_RATING_THRESHOLD_BP: u32 = 7_500;

/// Moderate rating threshold ($5,000$ bp = 50%).
pub const MODERATE_RATING_THRESHOLD_BP: u32 = 5_000;

/// Low rating threshold ($2,500$ bp = 25%).
pub const LOW_RATING_THRESHOLD_BP: u32 = 2_500;

/// Typed errors emitted during attribution evaluation and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamAttributionError {
  /// Safety violation: private chain-of-thought was detected or requested.
  ChainOfThoughtForbidden,
  /// Basis point value exceeded the maximum allowed bound ($10,000$ bp).
  BasisPointOutOfRange {
    /// Provided basis point value.
    bp: u32,
    /// Maximum allowed basis point value.
    max: u32,
  },
  /// Basis point contributions do not sum to exact conservation ($10,000$ bp).
  SumConservationViolation {
    /// Coordination contribution in bp.
    coordination_bp: u32,
    /// Execution contribution in bp.
    execution_bp: u32,
    /// Exogenous variance contribution in bp.
    exogenous_bp: u32,
    /// Total calculated bp.
    total_bp: u32,
  },
  /// Scenario not found in the attribution catalog.
  CatalogScenarioNotFound(&'static str),
  /// Empty or invalid scenario identifier.
  InvalidScenarioName,
}

impl fmt::Display for TeamAttributionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChainOfThoughtForbidden => {
        write!(
          f,
          "private chain-of-thought is strictly forbidden in attribution contracts"
        )
      }
      Self::BasisPointOutOfRange { bp, max } => {
        write!(
          f,
          "basis point value {bp} exceeds maximum allowed bound {max}"
        )
      }
      Self::SumConservationViolation {
        coordination_bp,
        execution_bp,
        exogenous_bp,
        total_bp,
      } => {
        write!(
          f,
          "attribution sum conservation violation: coordination {coordination_bp} + execution {execution_bp} + exogenous {exogenous_bp} = {total_bp} (expected {MAX_ATTRIBUTION_BP})"
        )
      }
      Self::CatalogScenarioNotFound(name) => {
        write!(f, "attribution scenario `{name}` was not found in catalog")
      }
      Self::InvalidScenarioName => {
        write!(f, "scenario name cannot be empty")
      }
    }
  }
}

/// The four canonical quadrants of strategic attribution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AttributionQuadrant {
  /// Sound team coordination followed by successful mechanical execution.
  CoordinatedTriumph,
  /// Sound team coordination, but execution failed due to mechanical outplay or bad luck.
  CoordinatedFailure,
  /// Flawed/dissenting team coordination bailed out by individual mechanical skill.
  UncoordinatedBailout,
  /// Fractured team coordination compounded by tactical execution collapse.
  CompoundedFailure,
}

impl AttributionQuadrant {
  /// Returns the canonical label for this attribution quadrant.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CoordinatedTriumph => "coordinated-triumph",
      Self::CoordinatedFailure => "coordinated-failure",
      Self::UncoordinatedBailout => "uncoordinated-bailout",
      Self::CompoundedFailure => "compounded-failure",
    }
  }

  /// Returns a concise descriptive summary of the quadrant.
  pub const fn description(self) -> &'static str {
    match self {
      Self::CoordinatedTriumph => {
        "High coordination alignment coupled with effective execution resolution"
      }
      Self::CoordinatedFailure => {
        "High coordination alignment undermined by mechanical execution failure or adversary outplay"
      }
      Self::UncoordinatedBailout => {
        "Low coordination cohesion or tactical dissent saved by individual execution success"
      }
      Self::CompoundedFailure => {
        "Low coordination cohesion and directive failure compounded by execution collapse"
      }
    }
  }

  /// Evaluates whether the coordination dimension was effective.
  pub const fn is_coordination_effective(self) -> bool {
    matches!(self, Self::CoordinatedTriumph | Self::CoordinatedFailure)
  }

  /// Evaluates whether the execution dimension was effective.
  pub const fn is_execution_effective(self) -> bool {
    matches!(self, Self::CoordinatedTriumph | Self::UncoordinatedBailout)
  }

  /// Classifies a quadrant from discrete coordination and execution basis point scores.
  pub fn classify(
    coordination_cohesion_bp: u32,
    execution_efficiency_bp: u32,
  ) -> Result<Self, TeamAttributionError> {
    if coordination_cohesion_bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: coordination_cohesion_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }
    if execution_efficiency_bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: execution_efficiency_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }

    let coord_ok = coordination_cohesion_bp >= COORDINATION_THRESHOLD_BP;
    let exec_ok = execution_efficiency_bp >= EXECUTION_THRESHOLD_BP;

    match (coord_ok, exec_ok) {
      (true, true) => Ok(Self::CoordinatedTriumph),
      (true, false) => Ok(Self::CoordinatedFailure),
      (false, true) => Ok(Self::UncoordinatedBailout),
      (false, false) => Ok(Self::CompoundedFailure),
    }
  }
}

/// Discrete performance rating for the coordination dimension.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinationRating {
  /// Strong team cohesion and compliance ($\ge 7,500$ bp).
  HighCoordination,
  /// Moderate alignment with minor dissent ($5,000..=7,499$ bp).
  ModerateCoordination,
  /// Divergent intents or weak compliance ($2,500..=4,999$ bp).
  LowCoordination,
  /// Communication failure or directive deadlock ($0..=2,499$ bp).
  FailedCoordination,
}

impl CoordinationRating {
  /// Returns the canonical label for the coordination rating.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::HighCoordination => "high-coordination",
      Self::ModerateCoordination => "moderate-coordination",
      Self::LowCoordination => "low-coordination",
      Self::FailedCoordination => "failed-coordination",
    }
  }

  /// Derives the rating from basis points.
  pub fn from_basis_points(bp: u32) -> Result<Self, TeamAttributionError> {
    if bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }
    if bp >= HIGH_RATING_THRESHOLD_BP {
      Ok(Self::HighCoordination)
    } else if bp >= MODERATE_RATING_THRESHOLD_BP {
      Ok(Self::ModerateCoordination)
    } else if bp >= LOW_RATING_THRESHOLD_BP {
      Ok(Self::LowCoordination)
    } else {
      Ok(Self::FailedCoordination)
    }
  }
}

/// Discrete performance rating for the mechanical execution dimension.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExecutionRating {
  /// Clean execution with decisive objective/trade gain ($\ge 7,500$ bp).
  FlawlessExecution,
  /// Competent execution meeting primary objectives ($5,000..=7,499$ bp).
  CompetentExecution,
  /// Degraded execution with substantial losses ($2,500..=4,999$ bp).
  CompromisedExecution,
  /// Complete mechanical collapse or forced withdrawal ($0..=2,499$ bp).
  FailedExecution,
}

impl ExecutionRating {
  /// Returns the canonical label for the execution rating.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FlawlessExecution => "flawless-execution",
      Self::CompetentExecution => "competent-execution",
      Self::CompromisedExecution => "compromised-execution",
      Self::FailedExecution => "failed-execution",
    }
  }

  /// Derives the rating from basis points.
  pub fn from_basis_points(bp: u32) -> Result<Self, TeamAttributionError> {
    if bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }
    if bp >= HIGH_RATING_THRESHOLD_BP {
      Ok(Self::FlawlessExecution)
    } else if bp >= MODERATE_RATING_THRESHOLD_BP {
      Ok(Self::CompetentExecution)
    } else if bp >= LOW_RATING_THRESHOLD_BP {
      Ok(Self::CompromisedExecution)
    } else {
      Ok(Self::FailedExecution)
    }
  }
}

/// Discrete causal factors explaining coordination quality.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinationCausalFactor {
  /// All participants committed to harmonious team plan roles.
  UnanimousAlignment,
  /// Teammates complied with designated shot-caller directives.
  DirectiveCompliance,
  /// Decentralized peer proposals successfully resolved via consensus rules.
  PeerConsensusArbitrated,
  /// Dissent occurred due to caller reputation/trust deficit.
  TrustDeficitDissent,
  /// Conflicting leadership calls or peer proposals deadlocked the team.
  ConflictingDirectives,
  /// Communication packets were delayed or dropped in transit.
  ChannelTransmissionLoss,
  /// Legitimate dissent triggered by tactical prerequisite failure (e.g. low health).
  ConditionUnmetDissent,
  /// Independent actors pursued disjoint individual objectives.
  DivergentStrategicPriorities,
}

impl CoordinationCausalFactor {
  /// Returns the canonical label for this causal factor.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::UnanimousAlignment => "unanimous-alignment",
      Self::DirectiveCompliance => "directive-compliance",
      Self::PeerConsensusArbitrated => "peer-consensus-arbitrated",
      Self::TrustDeficitDissent => "trust-deficit-dissent",
      Self::ConflictingDirectives => "conflicting-directives",
      Self::ChannelTransmissionLoss => "channel-transmission-loss",
      Self::ConditionUnmetDissent => "condition-unmet-dissent",
      Self::DivergentStrategicPriorities => "divergent-strategic-priorities",
    }
  }

  /// Indicates if this factor contributed positively to team coordination.
  pub const fn is_favorable(self) -> bool {
    matches!(
      self,
      Self::UnanimousAlignment | Self::DirectiveCompliance | Self::PeerConsensusArbitrated
    )
  }
}

/// Discrete causal factors explaining tactical execution quality.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExecutionCausalFactor {
  /// Dominant combat exchange efficiency.
  DecisiveDamageAdvantage,
  /// Territorial space or objective captured cleanly.
  ObjectiveSecured,
  /// Optimal tactical positioning achieved during the beat.
  FavorablePositioning,
  /// Adversary executed superior mechanical counterplay.
  OpponentMechanicalCounter,
  /// Attrition forced defensive posture or retreat.
  SevereHealthAttrition,
  /// Resource depletion (mana/cooldowns) limited effectiveness.
  ResourceDepletion,
  /// Minion wave pressure forced space concession.
  WavePressureDisadvantage,
  /// Stochastic variance/rolls favored adversary.
  UnfavorableStochasticRoll,
}

impl ExecutionCausalFactor {
  /// Returns the canonical label for this execution factor.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::DecisiveDamageAdvantage => "decisive-damage-advantage",
      Self::ObjectiveSecured => "objective-secured",
      Self::FavorablePositioning => "favorable-positioning",
      Self::OpponentMechanicalCounter => "opponent-mechanical-counter",
      Self::SevereHealthAttrition => "severe-health-attrition",
      Self::ResourceDepletion => "resource-depletion",
      Self::WavePressureDisadvantage => "wave-pressure-disadvantage",
      Self::UnfavorableStochasticRoll => "unfavorable-stochastic-roll",
    }
  }

  /// Indicates if this factor contributed positively to execution quality.
  pub const fn is_favorable(self) -> bool {
    matches!(
      self,
      Self::DecisiveDamageAdvantage | Self::ObjectiveSecured | Self::FavorablePositioning
    )
  }
}

/// Assessment of the team coordination dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinationAssessment {
  coordination_outcome: TeamCoordinationOutcome,
  cohesion_bp: u32,
  rating: CoordinationRating,
  primary_factor: CoordinationCausalFactor,
  secondary_factors: [Option<CoordinationCausalFactor>; 2],
}

impl CoordinationAssessment {
  /// Creates a validated coordination assessment.
  pub fn new(
    coordination_outcome: TeamCoordinationOutcome,
    cohesion_bp: u32,
    primary_factor: CoordinationCausalFactor,
    secondary_factors: [Option<CoordinationCausalFactor>; 2],
  ) -> Result<Self, TeamAttributionError> {
    if cohesion_bp > MAX_COHESION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: cohesion_bp,
        max: MAX_COHESION_BP,
      });
    }
    let rating = CoordinationRating::from_basis_points(cohesion_bp)?;
    Ok(Self {
      coordination_outcome,
      cohesion_bp,
      rating,
      primary_factor,
      secondary_factors,
    })
  }

  /// Returns the discrete coordination outcome.
  pub fn coordination_outcome(&self) -> TeamCoordinationOutcome {
    self.coordination_outcome
  }

  /// Returns the cohesion score in basis points.
  pub fn cohesion_bp(&self) -> u32 {
    self.cohesion_bp
  }

  /// Returns the discrete coordination rating.
  pub fn rating(&self) -> CoordinationRating {
    self.rating
  }

  /// Returns the primary causal factor.
  pub fn primary_factor(&self) -> CoordinationCausalFactor {
    self.primary_factor
  }

  /// Returns the secondary causal factors.
  pub fn secondary_factors(&self) -> [Option<CoordinationCausalFactor>; 2] {
    self.secondary_factors
  }
}

/// Assessment of the tactical mechanical execution dimension.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionAssessment {
  lane_outcome: LaneOutcome,
  execution_score_bp: u32,
  rating: ExecutionRating,
  primary_factor: ExecutionCausalFactor,
  secondary_factors: [Option<ExecutionCausalFactor>; 2],
}

impl ExecutionAssessment {
  /// Creates a validated execution assessment.
  pub fn new(
    lane_outcome: LaneOutcome,
    execution_score_bp: u32,
    primary_factor: ExecutionCausalFactor,
    secondary_factors: [Option<ExecutionCausalFactor>; 2],
  ) -> Result<Self, TeamAttributionError> {
    if execution_score_bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: execution_score_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }
    let rating = ExecutionRating::from_basis_points(execution_score_bp)?;
    Ok(Self {
      lane_outcome,
      execution_score_bp,
      rating,
      primary_factor,
      secondary_factors,
    })
  }

  /// Returns the discrete lane execution outcome.
  pub fn lane_outcome(&self) -> LaneOutcome {
    self.lane_outcome
  }

  /// Returns the execution score in basis points.
  pub fn execution_score_bp(&self) -> u32 {
    self.execution_score_bp
  }

  /// Returns the discrete execution rating.
  pub fn rating(&self) -> ExecutionRating {
    self.rating
  }

  /// Returns the primary causal factor.
  pub fn primary_factor(&self) -> ExecutionCausalFactor {
    self.primary_factor
  }

  /// Returns the secondary causal factors.
  pub fn secondary_factors(&self) -> [Option<ExecutionCausalFactor>; 2] {
    self.secondary_factors
  }
}

/// Relative causal contributions in exact integer basis points ($10,000$ bp = 100%).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttributionWeights {
  coordination_contribution_bp: u32,
  execution_contribution_bp: u32,
  exogenous_variance_bp: u32,
}

impl AttributionWeights {
  /// Creates a new attribution weight bundle enforcing sum conservation ($10,000$ bp).
  pub fn new(
    coordination_contribution_bp: u32,
    execution_contribution_bp: u32,
    exogenous_variance_bp: u32,
  ) -> Result<Self, TeamAttributionError> {
    if coordination_contribution_bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: coordination_contribution_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }
    if execution_contribution_bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: execution_contribution_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }
    if exogenous_variance_bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: exogenous_variance_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }

    let total = coordination_contribution_bp
      .saturating_add(execution_contribution_bp)
      .saturating_add(exogenous_variance_bp);

    if total != MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::SumConservationViolation {
        coordination_bp: coordination_contribution_bp,
        execution_bp: execution_contribution_bp,
        exogenous_bp: exogenous_variance_bp,
        total_bp: total,
      });
    }

    Ok(Self {
      coordination_contribution_bp,
      execution_contribution_bp,
      exogenous_variance_bp,
    })
  }

  /// Returns the coordination weight in basis points.
  pub fn coordination_contribution_bp(self) -> u32 {
    self.coordination_contribution_bp
  }

  /// Returns the execution weight in basis points.
  pub fn execution_contribution_bp(self) -> u32 {
    self.execution_contribution_bp
  }

  /// Returns the exogenous/luck weight in basis points.
  pub fn exogenous_variance_bp(self) -> u32 {
    self.exogenous_variance_bp
  }
}

/// Unified attribution model decoupling coordination from mechanical execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinationExecutionAttribution {
  quadrant: AttributionQuadrant,
  coordination_assessment: CoordinationAssessment,
  execution_assessment: ExecutionAssessment,
  weights: AttributionWeights,
  summary_notes: &'static str,
}

impl CoordinationExecutionAttribution {
  /// Creates a validated coordination and execution attribution bundle.
  pub fn new(
    coordination_assessment: CoordinationAssessment,
    execution_assessment: ExecutionAssessment,
    weights: AttributionWeights,
    summary_notes: &'static str,
  ) -> Result<Self, TeamAttributionError> {
    let quadrant = AttributionQuadrant::classify(
      coordination_assessment.cohesion_bp(),
      execution_assessment.execution_score_bp(),
    )?;

    Ok(Self {
      quadrant,
      coordination_assessment,
      execution_assessment,
      weights,
      summary_notes,
    })
  }

  /// Returns the strategic attribution quadrant.
  pub fn quadrant(&self) -> AttributionQuadrant {
    self.quadrant
  }

  /// Returns the coordination assessment.
  pub fn coordination_assessment(&self) -> &CoordinationAssessment {
    &self.coordination_assessment
  }

  /// Returns the execution assessment.
  pub fn execution_assessment(&self) -> &ExecutionAssessment {
    &self.execution_assessment
  }

  /// Returns the basis point contribution weights.
  pub fn weights(&self) -> AttributionWeights {
    self.weights
  }

  /// Returns human-readable summary notes.
  pub fn summary_notes(&self) -> &'static str {
    self.summary_notes
  }
}

/// Complete report container for causal debrief attribution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoordinationExecutionAttributionReport {
  schema: &'static str,
  turn: u32,
  observation_id: u64,
  attribution: CoordinationExecutionAttribution,
  chain_of_thought_free: bool,
}

impl CoordinationExecutionAttributionReport {
  /// Constructs a validated attribution report, strictly rejecting private chain-of-thought.
  pub fn new(
    turn: u32,
    observation_id: u64,
    attribution: CoordinationExecutionAttribution,
    chain_of_thought_present: bool,
  ) -> Result<Self, TeamAttributionError> {
    if chain_of_thought_present {
      return Err(TeamAttributionError::ChainOfThoughtForbidden);
    }

    Ok(Self {
      schema: ATTRIBUTION_REPORT_SCHEMA,
      turn,
      observation_id,
      attribution,
      chain_of_thought_free: true,
    })
  }

  /// Returns the schema identifier.
  pub fn schema(&self) -> &'static str {
    self.schema
  }

  /// Returns the turn index.
  pub fn turn(&self) -> u32 {
    self.turn
  }

  /// Returns the observation ID.
  pub fn observation_id(&self) -> u64 {
    self.observation_id
  }

  /// Returns the underlying attribution struct.
  pub fn attribution(&self) -> &CoordinationExecutionAttribution {
    &self.attribution
  }

  /// Formats the attribution report into Markdown debrief text.
  pub fn to_markdown(&self) -> String {
    let attr = &self.attribution;
    let coord = attr.coordination_assessment();
    let exec = attr.execution_assessment();
    let weights = attr.weights();

    let mut out = String::with_capacity(1024);
    out.push_str("# Strategic Attribution Debrief Report\n\n");
    out.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    out.push_str(&format!("- **Turn:** {}\n", self.turn));
    out.push_str(&format!("- **Observation ID:** {}\n", self.observation_id));
    out.push_str(&format!(
      "- **Attribution Quadrant:** `{}` ({})\n\n",
      attr.quadrant().as_str(),
      attr.quadrant().description()
    ));

    out.push_str("## 1. Coordination Assessment\n\n");
    out.push_str(&format!(
      "- **Outcome:** `{:?}`\n",
      coord.coordination_outcome()
    ));
    out.push_str(&format!(
      "- **Cohesion Score:** {} bp (Rating: `{}`)\n",
      coord.cohesion_bp(),
      coord.rating().as_str()
    ));
    out.push_str(&format!(
      "- **Primary Causal Factor:** `{}` (Favorable: {})\n",
      coord.primary_factor().as_str(),
      coord.primary_factor().is_favorable()
    ));
    for (i, opt) in coord.secondary_factors().iter().enumerate() {
      if let Some(factor) = opt {
        out.push_str(&format!(
          "- **Secondary Factor {}:** `{}`\n",
          i + 1,
          factor.as_str()
        ));
      }
    }

    out.push_str("\n## 2. Mechanical Execution Assessment\n\n");
    out.push_str(&format!(
      "- **Lane Outcome:** `{:?}`\n",
      exec.lane_outcome()
    ));
    out.push_str(&format!(
      "- **Execution Efficiency:** {} bp (Rating: `{}`)\n",
      exec.execution_score_bp(),
      exec.rating().as_str()
    ));
    out.push_str(&format!(
      "- **Primary Causal Factor:** `{}` (Favorable: {})\n",
      exec.primary_factor().as_str(),
      exec.primary_factor().is_favorable()
    ));
    for (i, opt) in exec.secondary_factors().iter().enumerate() {
      if let Some(factor) = opt {
        out.push_str(&format!(
          "- **Secondary Factor {}:** `{}`\n",
          i + 1,
          factor.as_str()
        ));
      }
    }

    out.push_str("\n## 3. Causal Impact Distribution\n\n");
    out.push_str(&format!(
      "- **Coordination Contribution:** {} bp\n",
      weights.coordination_contribution_bp()
    ));
    out.push_str(&format!(
      "- **Execution Contribution:** {} bp\n",
      weights.execution_contribution_bp()
    ));
    out.push_str(&format!(
      "- **Exogenous / Luck Variance:** {} bp\n",
      weights.exogenous_variance_bp()
    ));
    out.push_str(&format!("- **Notes:** {}\n\n", attr.summary_notes()));
    out.push_str("> [!NOTE]\n> Coordination and execution are decoupled to prevent outcome bias in team evaluation.\n");

    out
  }
}

/// Input parameters bundle for evaluating strategic attribution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributionEvaluationInput {
  /// Mechanical lane execution outcome.
  pub lane_outcome: LaneOutcome,
  /// Mechanical execution efficiency score in basis points.
  pub execution_score_bp: u32,
  /// Primary causal factor explaining coordination.
  pub primary_coord_factor: CoordinationCausalFactor,
  /// Secondary causal factors explaining coordination.
  pub secondary_coord_factors: [Option<CoordinationCausalFactor>; 2],
  /// Primary causal factor explaining execution.
  pub primary_exec_factor: ExecutionCausalFactor,
  /// Secondary causal factors explaining execution.
  pub secondary_exec_factors: [Option<ExecutionCausalFactor>; 2],
  /// Relative causal contribution weights.
  pub weights: AttributionWeights,
  /// Human-readable debrief summary notes.
  pub summary_notes: &'static str,
  /// Safety flag: must be false.
  pub chain_of_thought_present: bool,
}

/// Evaluator producing structured attribution from simultaneous resolution and execution.
pub struct TeamAttributionEvaluator;

impl TeamAttributionEvaluator {
  /// Evaluates a turn's coordination and execution results into a debrief attribution report.
  pub fn evaluate(
    simultaneous_resolution: &TeamSimultaneousResolution,
    input: AttributionEvaluationInput,
  ) -> Result<CoordinationExecutionAttributionReport, TeamAttributionError> {
    if input.chain_of_thought_present {
      return Err(TeamAttributionError::ChainOfThoughtForbidden);
    }

    let coord_assessment = CoordinationAssessment::new(
      simultaneous_resolution.coordination_outcome(),
      simultaneous_resolution.team_cohesion_bp(),
      input.primary_coord_factor,
      input.secondary_coord_factors,
    )?;

    let exec_assessment = ExecutionAssessment::new(
      input.lane_outcome,
      input.execution_score_bp,
      input.primary_exec_factor,
      input.secondary_exec_factors,
    )?;

    let attribution = CoordinationExecutionAttribution::new(
      coord_assessment,
      exec_assessment,
      input.weights,
      input.summary_notes,
    )?;

    CoordinationExecutionAttributionReport::new(
      simultaneous_resolution.turn(),
      simultaneous_resolution.observation_id(),
      attribution,
      false,
    )
  }
}

/// Canonical reference scenario definition for attribution testing and benchmark validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributionScenario {
  /// Scenario unique name identifier.
  pub name: &'static str,
  /// Human-readable description.
  pub description: &'static str,
  /// Input coordination outcome.
  pub coordination_outcome: TeamCoordinationOutcome,
  /// Input team cohesion in basis points.
  pub cohesion_bp: u32,
  /// Primary coordination causal factor.
  pub primary_coord_factor: CoordinationCausalFactor,
  /// Input lane execution outcome.
  pub lane_outcome: LaneOutcome,
  /// Input execution efficiency score in basis points.
  pub execution_score_bp: u32,
  /// Primary execution causal factor.
  pub primary_exec_factor: ExecutionCausalFactor,
  /// Expected quadrant classification.
  pub expected_quadrant: AttributionQuadrant,
  /// Expected coordination rating tier.
  pub expected_coordination_rating: CoordinationRating,
  /// Expected execution rating tier.
  pub expected_execution_rating: ExecutionRating,
  /// Coordination contribution bp.
  pub coordination_contribution_bp: u32,
  /// Execution contribution bp.
  pub execution_contribution_bp: u32,
  /// Exogenous variance bp.
  pub exogenous_variance_bp: u32,
  /// Debrief summary notes.
  pub summary_notes: &'static str,
}

impl AttributionScenario {
  /// Validates the scenario's internal consistency.
  pub fn validate(&self) -> Result<(), TeamAttributionError> {
    if self.name.is_empty() {
      return Err(TeamAttributionError::InvalidScenarioName);
    }
    if self.cohesion_bp > MAX_COHESION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: self.cohesion_bp,
        max: MAX_COHESION_BP,
      });
    }
    if self.execution_score_bp > MAX_ATTRIBUTION_BP {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: self.execution_score_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }

    let quad = AttributionQuadrant::classify(self.cohesion_bp, self.execution_score_bp)?;
    if quad != self.expected_quadrant {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: self.cohesion_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }

    let coord_rat = CoordinationRating::from_basis_points(self.cohesion_bp)?;
    if coord_rat != self.expected_coordination_rating {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: self.cohesion_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }

    let exec_rat = ExecutionRating::from_basis_points(self.execution_score_bp)?;
    if exec_rat != self.expected_execution_rating {
      return Err(TeamAttributionError::BasisPointOutOfRange {
        bp: self.execution_score_bp,
        max: MAX_ATTRIBUTION_BP,
      });
    }

    let _weights = AttributionWeights::new(
      self.coordination_contribution_bp,
      self.execution_contribution_bp,
      self.exogenous_variance_bp,
    )?;

    Ok(())
  }
}

/// Canonical catalog registering benchmark scenarios for coordination vs execution attribution.
pub struct CoordinationAttributionCatalog;

impl CoordinationAttributionCatalog {
  /// Canonical registered scenarios covering all 4 strategic quadrants and critical dilemma cases.
  pub const SCENARIOS: &[AttributionScenario] = &[
    AttributionScenario {
      name: "attr-coordinated-triumph-gank-v1",
      description: "High-trust gank directive executed cleanly with decisive damage advantage",
      coordination_outcome: TeamCoordinationOutcome::FullyCoordinated,
      cohesion_bp: 8_750,
      primary_coord_factor: CoordinationCausalFactor::DirectiveCompliance,
      lane_outcome: LaneOutcome::HeldSpace,
      execution_score_bp: 8_500,
      primary_exec_factor: ExecutionCausalFactor::DecisiveDamageAdvantage,
      expected_quadrant: AttributionQuadrant::CoordinatedTriumph,
      expected_coordination_rating: CoordinationRating::HighCoordination,
      expected_execution_rating: ExecutionRating::FlawlessExecution,
      coordination_contribution_bp: 5_500,
      execution_contribution_bp: 3_500,
      exogenous_variance_bp: 1_000,
      summary_notes: "Gank directive followed with full alignment and decisive execution",
    },
    AttributionScenario {
      name: "attr-coordinated-failure-overreach-v1",
      description: "Sound team consensus to contest, but enemy mechanical outplay reverses the trade",
      coordination_outcome: TeamCoordinationOutcome::FullyCoordinated,
      cohesion_bp: 8_000,
      primary_coord_factor: CoordinationCausalFactor::UnanimousAlignment,
      lane_outcome: LaneOutcome::ForcedOut,
      execution_score_bp: 2_000,
      primary_exec_factor: ExecutionCausalFactor::OpponentMechanicalCounter,
      expected_quadrant: AttributionQuadrant::CoordinatedFailure,
      expected_coordination_rating: CoordinationRating::HighCoordination,
      expected_execution_rating: ExecutionRating::FailedExecution,
      coordination_contribution_bp: 4_000,
      execution_contribution_bp: 5_000,
      exogenous_variance_bp: 1_000,
      summary_notes: "Unanimous contest agreement defeated by opponent tactical counterplay",
    },
    AttributionScenario {
      name: "attr-uncoordinated-bailout-clutch-v1",
      description: "Communication breakdown and dissent saved by an extraordinary solo mechanical duel",
      coordination_outcome: TeamCoordinationOutcome::CommunicationFailure,
      cohesion_bp: 1_500,
      primary_coord_factor: CoordinationCausalFactor::ChannelTransmissionLoss,
      lane_outcome: LaneOutcome::HeldSpace,
      execution_score_bp: 8_200,
      primary_exec_factor: ExecutionCausalFactor::DecisiveDamageAdvantage,
      expected_quadrant: AttributionQuadrant::UncoordinatedBailout,
      expected_coordination_rating: CoordinationRating::FailedCoordination,
      expected_execution_rating: ExecutionRating::FlawlessExecution,
      coordination_contribution_bp: 2_000,
      execution_contribution_bp: 7_500,
      exogenous_variance_bp: 500,
      summary_notes: "Channel loss caused plan failure but solo laner clutch secured the lane",
    },
    AttributionScenario {
      name: "attr-compounded-failure-deadlock-v1",
      description: "Conflicting directives deadlocked team action, resulting in severe attrition and wipe",
      coordination_outcome: TeamCoordinationOutcome::ConflictingDirectives,
      cohesion_bp: 1_200,
      primary_coord_factor: CoordinationCausalFactor::ConflictingDirectives,
      lane_outcome: LaneOutcome::ForcedOut,
      execution_score_bp: 1_000,
      primary_exec_factor: ExecutionCausalFactor::SevereHealthAttrition,
      expected_quadrant: AttributionQuadrant::CompoundedFailure,
      expected_coordination_rating: CoordinationRating::FailedCoordination,
      expected_execution_rating: ExecutionRating::FailedExecution,
      coordination_contribution_bp: 6_000,
      execution_contribution_bp: 3_500,
      exogenous_variance_bp: 500,
      summary_notes: "Directive deadlock split the team, precipitating total mechanical collapse",
    },
    AttributionScenario {
      name: "attr-legitimate-dissent-avoided-wipe-v1",
      description: "Low-health laner legitimately dissents from reckless dive, preserving resources and tower",
      coordination_outcome: TeamCoordinationOutcome::PartiallyCoordinated,
      cohesion_bp: 4_500,
      primary_coord_factor: CoordinationCausalFactor::ConditionUnmetDissent,
      lane_outcome: LaneOutcome::YieldedSpace,
      execution_score_bp: 6_000,
      primary_exec_factor: ExecutionCausalFactor::FavorablePositioning,
      expected_quadrant: AttributionQuadrant::UncoordinatedBailout,
      expected_coordination_rating: CoordinationRating::LowCoordination,
      expected_execution_rating: ExecutionRating::CompetentExecution,
      coordination_contribution_bp: 3_000,
      execution_contribution_bp: 6_000,
      exogenous_variance_bp: 1_000,
      summary_notes: "Condition dissent prevented overextension, allowing stable defensive yield",
    },
    AttributionScenario {
      name: "attr-trust-breakdown-execution-miss-v1",
      description: "Low-reputation caller ignored, resulting in disjointed positioning and wave concession",
      coordination_outcome: TeamCoordinationOutcome::DivergentIntents,
      cohesion_bp: 2_000,
      primary_coord_factor: CoordinationCausalFactor::TrustDeficitDissent,
      lane_outcome: LaneOutcome::YieldedSpace,
      execution_score_bp: 3_500,
      primary_exec_factor: ExecutionCausalFactor::WavePressureDisadvantage,
      expected_quadrant: AttributionQuadrant::CompoundedFailure,
      expected_coordination_rating: CoordinationRating::FailedCoordination,
      expected_execution_rating: ExecutionRating::CompromisedExecution,
      coordination_contribution_bp: 5_000,
      execution_contribution_bp: 4_000,
      exogenous_variance_bp: 1_000,
      summary_notes: "Trust deficit caused fragmented response and concession of wave pressure",
    },
  ];

  /// Returns all registered scenarios.
  pub fn all_scenarios() -> &'static [AttributionScenario] {
    Self::SCENARIOS
  }

  /// Looks up a scenario by unique name with fail-closed error handling.
  pub fn lookup(name: &str) -> Result<&'static AttributionScenario, TeamAttributionError> {
    if name.is_empty() {
      return Err(TeamAttributionError::InvalidScenarioName);
    }
    for scenario in Self::SCENARIOS {
      if scenario.name == name {
        return Ok(scenario);
      }
    }
    Err(TeamAttributionError::CatalogScenarioNotFound(
      "unknown-scenario",
    ))
  }

  /// Verifies all registered scenarios for internal mathematical consistency.
  pub fn all_scenarios_are_valid() -> Result<(), TeamAttributionError> {
    for scenario in Self::SCENARIOS {
      scenario.validate()?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn quadrant_classification_and_ratings_are_exact() {
    assert_eq!(
      AttributionQuadrant::classify(8_000, 8_000).unwrap(),
      AttributionQuadrant::CoordinatedTriumph
    );
    assert_eq!(
      AttributionQuadrant::classify(8_000, 2_000).unwrap(),
      AttributionQuadrant::CoordinatedFailure
    );
    assert_eq!(
      AttributionQuadrant::classify(2_000, 8_000).unwrap(),
      AttributionQuadrant::UncoordinatedBailout
    );
    assert_eq!(
      AttributionQuadrant::classify(2_000, 2_000).unwrap(),
      AttributionQuadrant::CompoundedFailure
    );

    // Boundary cases at 5,000 bp
    assert_eq!(
      AttributionQuadrant::classify(5_000, 5_000).unwrap(),
      AttributionQuadrant::CoordinatedTriumph
    );
    assert_eq!(
      AttributionQuadrant::classify(4_999, 5_000).unwrap(),
      AttributionQuadrant::UncoordinatedBailout
    );
    assert_eq!(
      AttributionQuadrant::classify(5_000, 4_999).unwrap(),
      AttributionQuadrant::CoordinatedFailure
    );
    assert_eq!(
      AttributionQuadrant::classify(4_999, 4_999).unwrap(),
      AttributionQuadrant::CompoundedFailure
    );
  }

  #[test]
  fn rating_tiers_and_predicates() {
    assert_eq!(
      CoordinationRating::from_basis_points(8_000).unwrap(),
      CoordinationRating::HighCoordination
    );
    assert_eq!(
      CoordinationRating::from_basis_points(6_000).unwrap(),
      CoordinationRating::ModerateCoordination
    );
    assert_eq!(
      CoordinationRating::from_basis_points(3_000).unwrap(),
      CoordinationRating::LowCoordination
    );
    assert_eq!(
      CoordinationRating::from_basis_points(1_000).unwrap(),
      CoordinationRating::FailedCoordination
    );

    assert_eq!(
      ExecutionRating::from_basis_points(9_000).unwrap(),
      ExecutionRating::FlawlessExecution
    );
    assert_eq!(
      ExecutionRating::from_basis_points(6_500).unwrap(),
      ExecutionRating::CompetentExecution
    );
    assert_eq!(
      ExecutionRating::from_basis_points(4_000).unwrap(),
      ExecutionRating::CompromisedExecution
    );
    assert_eq!(
      ExecutionRating::from_basis_points(1_500).unwrap(),
      ExecutionRating::FailedExecution
    );
  }

  #[test]
  fn sum_conservation_enforced() {
    let valid_weights = AttributionWeights::new(5_000, 4_000, 1_000);
    assert!(valid_weights.is_ok());

    let invalid_sum = AttributionWeights::new(5_000, 4_000, 2_000);
    assert!(matches!(
      invalid_sum,
      Err(TeamAttributionError::SumConservationViolation { .. })
    ));

    let out_of_range = AttributionWeights::new(10_001, 0, 0);
    assert!(matches!(
      out_of_range,
      Err(TeamAttributionError::BasisPointOutOfRange { .. })
    ));
  }

  #[test]
  fn chain_of_thought_is_strictly_forbidden() {
    let coord = CoordinationAssessment::new(
      TeamCoordinationOutcome::FullyCoordinated,
      8_000,
      CoordinationCausalFactor::UnanimousAlignment,
      [None, None],
    )
    .unwrap();

    let exec = ExecutionAssessment::new(
      LaneOutcome::HeldSpace,
      8_000,
      ExecutionCausalFactor::DecisiveDamageAdvantage,
      [None, None],
    )
    .unwrap();

    let weights = AttributionWeights::new(5_000, 4_000, 1_000).unwrap();

    let attr = CoordinationExecutionAttribution::new(coord, exec, weights, "Valid note").unwrap();

    let err = CoordinationExecutionAttributionReport::new(1, 100, attr, true);
    assert_eq!(err, Err(TeamAttributionError::ChainOfThoughtForbidden));
  }

  #[test]
  fn catalog_scenarios_all_pass_validation() {
    assert!(CoordinationAttributionCatalog::all_scenarios_are_valid().is_ok());

    let gank = CoordinationAttributionCatalog::lookup("attr-coordinated-triumph-gank-v1").unwrap();
    assert_eq!(
      gank.expected_quadrant,
      AttributionQuadrant::CoordinatedTriumph
    );

    let overreach =
      CoordinationAttributionCatalog::lookup("attr-coordinated-failure-overreach-v1").unwrap();
    assert_eq!(
      overreach.expected_quadrant,
      AttributionQuadrant::CoordinatedFailure
    );

    let bailout =
      CoordinationAttributionCatalog::lookup("attr-uncoordinated-bailout-clutch-v1").unwrap();
    assert_eq!(
      bailout.expected_quadrant,
      AttributionQuadrant::UncoordinatedBailout
    );

    let deadlock =
      CoordinationAttributionCatalog::lookup("attr-compounded-failure-deadlock-v1").unwrap();
    assert_eq!(
      deadlock.expected_quadrant,
      AttributionQuadrant::CompoundedFailure
    );

    let empty_lookup = CoordinationAttributionCatalog::lookup("");
    assert_eq!(empty_lookup, Err(TeamAttributionError::InvalidScenarioName));

    let unknown_lookup = CoordinationAttributionCatalog::lookup("nonexistent-scenario");
    assert!(matches!(
      unknown_lookup,
      Err(TeamAttributionError::CatalogScenarioNotFound(_))
    ));
  }

  #[test]
  fn markdown_report_generation() {
    let scenario =
      CoordinationAttributionCatalog::lookup("attr-coordinated-triumph-gank-v1").unwrap();

    let coord = CoordinationAssessment::new(
      scenario.coordination_outcome,
      scenario.cohesion_bp,
      scenario.primary_coord_factor,
      [Some(CoordinationCausalFactor::UnanimousAlignment), None],
    )
    .unwrap();

    let exec = ExecutionAssessment::new(
      scenario.lane_outcome,
      scenario.execution_score_bp,
      scenario.primary_exec_factor,
      [Some(ExecutionCausalFactor::ObjectiveSecured), None],
    )
    .unwrap();

    let weights = AttributionWeights::new(
      scenario.coordination_contribution_bp,
      scenario.execution_contribution_bp,
      scenario.exogenous_variance_bp,
    )
    .unwrap();

    let attr =
      CoordinationExecutionAttribution::new(coord, exec, weights, scenario.summary_notes).unwrap();

    let report = CoordinationExecutionAttributionReport::new(1, 42, attr, false).unwrap();
    let md = report.to_markdown();

    assert!(md.contains("# Strategic Attribution Debrief Report"));
    assert!(md.contains("coordinated-triumph"));
    assert!(md.contains("directive-compliance"));
    assert!(md.contains("decisive-damage-advantage"));
    assert!(md.contains("5500 bp"));
    assert!(md.contains("3500 bp"));
    assert!(md.contains("1000 bp"));
  }
}

//! Dimension-level usability and accessibility assessment framework for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Evaluates participant assessments across all 10 canonical evaluation dimensions
//! (onboarding, terminology clarity, command discoverability, pacing load, perceived agency,
//! delegated fairness, debrief causal utility, keyboard flow, non-color semantics,
//! and screen-reader suitability). Tracks discrete cognitive friction indicators, computes
//! dimension-level basis-point metrics ([0..=10,000] bp), and identifies strongest/weakest dimensions.

use core::fmt;

use super::protocol::{EvaluationDimension, ParticipantCohort, StudyProtocolDefinition};

pub const M10_DIMENSION_ASSESSMENT_SCHEMA_V1: &str = "m10-dimension-assessment-v1";

/// Discrete cognitive friction indicator observed during participant interaction.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CognitiveFrictionIndicator {
  /// Smooth interaction with no noticeable cognitive friction.
  None,
  /// Dense text, multiple numeric statistics, or visual clutter causing sensory overload.
  HighCognitiveLoad,
  /// Confusion over domain terms (e.g. intent vs execution, belief vs truth).
  AmbiguousTerminology,
  /// Action possibilities or command options difficult to discover or recall.
  HiddenActionAffordance,
  /// Inability to connect debrief causal factors back to player intent choices.
  UnclearCausalTrace,
  /// Turn cadence or decision duration feeling rushed or overwhelming.
  PacingOverwhelm,
  /// Difficulty navigating linear prompt flow or recalling session context.
  NavigationDisorientation,
}

impl CognitiveFrictionIndicator {
  pub const ALL: [Self; 7] = [
    Self::None,
    Self::HighCognitiveLoad,
    Self::AmbiguousTerminology,
    Self::HiddenActionAffordance,
    Self::UnclearCausalTrace,
    Self::PacingOverwhelm,
    Self::NavigationDisorientation,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::None => "none",
      Self::HighCognitiveLoad => "high-cognitive-load",
      Self::AmbiguousTerminology => "ambiguous-terminology",
      Self::HiddenActionAffordance => "hidden-action-affordance",
      Self::UnclearCausalTrace => "unclear-causal-trace",
      Self::PacingOverwhelm => "pacing-overwhelm",
      Self::NavigationDisorientation => "navigation-disorientation",
    }
  }

  pub const fn is_friction(self) -> bool {
    !matches!(self, Self::None)
  }
}

impl fmt::Display for CognitiveFrictionIndicator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Score and qualitative feedback for a single evaluation dimension.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DimensionScore {
  pub dimension: EvaluationDimension,
  /// Participant score in basis points ([0..=10,000] bp).
  pub score_bp: u16,
  pub friction: CognitiveFrictionIndicator,
  pub notes: &'static str,
}

/// Complete 10-dimension assessment submitted by one participant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParticipantDimensionAssessment {
  pub participant_id: &'static str,
  pub cohort: ParticipantCohort,
  pub scores: [DimensionScore; 10],
}

/// Aggregated metrics for a single evaluation dimension across a study cohort.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DimensionSummary {
  pub dimension: EvaluationDimension,
  pub mean_score_bp: u16,
  pub min_score_bp: u16,
  pub max_score_bp: u16,
  pub predominant_friction: CognitiveFrictionIndicator,
  pub meets_floor: bool,
}

/// Aggregated report from evaluating dimension assessments for an M10 study.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DimensionEvaluationReport {
  pub protocol_id: &'static str,
  pub assessment_count: usize,
  pub overall_mean_score_bp: u16,
  pub dimension_summaries: [DimensionSummary; 10],
  pub weakest_dimension: EvaluationDimension,
  pub strongest_dimension: EvaluationDimension,
  pub accessibility_dimensions_qualified: bool,
  pub evidence_boundary_statement: &'static str,
}

impl DimensionEvaluationReport {
  /// Render this dimension evaluation report as clean, structured Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Usability & Accessibility Dimension Evaluation Report\n\n");
    out.push_str(&format!("**Protocol:** `{}`\n", self.protocol_id));
    out.push_str(&format!(
      "**Sample Size:** {} assessments | **Overall Mean Score:** {} bp\n",
      self.assessment_count, self.overall_mean_score_bp
    ));
    out.push_str(&format!(
      "**Strongest Dimension:** `{}` | **Weakest Dimension:** `{}`\n\n",
      self.strongest_dimension, self.weakest_dimension
    ));

    out.push_str("## Dimension Breakdown\n\n");
    out.push_str(
      "| Dimension | Mean (bp) | Min (bp) | Max (bp) | Predominant Friction | Floor Status |\n",
    );
    out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
    for s in &self.dimension_summaries {
      out.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} |\n",
        s.dimension,
        s.mean_score_bp,
        s.min_score_bp,
        s.max_score_bp,
        s.predominant_friction,
        if s.meets_floor { "PASS" } else { "FAIL" }
      ));
    }
    out.push('\n');

    out.push_str("## Accessibility Qualification\n\n");
    out.push_str(&format!(
      "- Accessibility Dimensions Qualified: {}\n\n",
      if self.accessibility_dimensions_qualified {
        "QUALIFIED"
      } else {
        "DISQUALIFIED"
      }
    ));

    out.push_str("## Evidence Boundary\n\n");
    out.push_str(self.evidence_boundary_statement);
    out.push('\n');

    out
  }
}

/// Error conditions during dimension assessment evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DimensionEvaluationError {
  /// The provided assessment list was empty.
  EmptyAssessmentList,
  /// A participant ID was duplicated in the assessment list.
  DuplicateParticipantId(&'static str),
  /// A dimension score exceeded the 10,000 bp limit.
  ScoreOutOfRange {
    participant_id: &'static str,
    dimension: EvaluationDimension,
    score_bp: u16,
  },
  /// An assessment omitted one of the 10 canonical dimensions.
  MissingDimension {
    participant_id: &'static str,
    dimension: EvaluationDimension,
  },
  /// An assessment contained duplicate entries for a dimension.
  DuplicateDimensionInAssessment {
    participant_id: &'static str,
    dimension: EvaluationDimension,
  },
  /// The privacy consent declaration was invalid or incomplete.
  InvalidPrivacyDeclaration,
}

impl fmt::Display for DimensionEvaluationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyAssessmentList => f.write_str("dimension assessment list cannot be empty"),
      Self::DuplicateParticipantId(id) => write!(f, "duplicate participant id: {id}"),
      Self::ScoreOutOfRange {
        participant_id,
        dimension,
        score_bp,
      } => write!(
        f,
        "score {score_bp} bp exceeds 10000 bp limit for participant {participant_id} on {dimension}"
      ),
      Self::MissingDimension {
        participant_id,
        dimension,
      } => write!(
        f,
        "assessment for participant {participant_id} is missing dimension {dimension}"
      ),
      Self::DuplicateDimensionInAssessment {
        participant_id,
        dimension,
      } => write!(
        f,
        "assessment for participant {participant_id} contains duplicate dimension {dimension}"
      ),
      Self::InvalidPrivacyDeclaration => {
        f.write_str("privacy consent declaration is invalid or incomplete")
      }
    }
  }
}

/// Evaluates a cohort of participant dimension assessments against the study protocol.
pub fn evaluate_dimension_assessments(
  protocol: &StudyProtocolDefinition,
  assessments: &[ParticipantDimensionAssessment],
) -> Result<DimensionEvaluationReport, DimensionEvaluationError> {
  if !protocol.privacy_declaration.is_valid() {
    return Err(DimensionEvaluationError::InvalidPrivacyDeclaration);
  }
  if assessments.is_empty() {
    return Err(DimensionEvaluationError::EmptyAssessmentList);
  }

  // Validate assessments
  for i in 0..assessments.len() {
    let a_i = &assessments[i];

    // Check duplicate participant IDs
    for a_j in &assessments[i + 1..] {
      if a_i.participant_id == a_j.participant_id {
        return Err(DimensionEvaluationError::DuplicateParticipantId(
          a_i.participant_id,
        ));
      }
    }

    // Verify exactly all 10 canonical dimensions are present without duplicates or out-of-range scores
    let mut seen_dimensions = [false; 10];
    for score_entry in &a_i.scores {
      if score_entry.score_bp > 10_000 {
        return Err(DimensionEvaluationError::ScoreOutOfRange {
          participant_id: a_i.participant_id,
          dimension: score_entry.dimension,
          score_bp: score_entry.score_bp,
        });
      }
      let dim_idx = EvaluationDimension::ALL
        .iter()
        .position(|&d| d == score_entry.dimension)
        .expect("valid dimension");
      if seen_dimensions[dim_idx] {
        return Err(DimensionEvaluationError::DuplicateDimensionInAssessment {
          participant_id: a_i.participant_id,
          dimension: score_entry.dimension,
        });
      }
      seen_dimensions[dim_idx] = true;
    }

    for (dim_idx, &seen) in seen_dimensions.iter().enumerate() {
      if !seen {
        return Err(DimensionEvaluationError::MissingDimension {
          participant_id: a_i.participant_id,
          dimension: EvaluationDimension::ALL[dim_idx],
        });
      }
    }
  }

  let assessment_count = assessments.len();
  let count_u32 = u32::try_from(assessment_count).expect("assessment count fits in u32");

  let mut total_score_sum: u64 = 0;
  let mut dimension_summaries = [DimensionSummary {
    dimension: EvaluationDimension::Onboarding,
    mean_score_bp: 0,
    min_score_bp: 10_000,
    max_score_bp: 0,
    predominant_friction: CognitiveFrictionIndicator::None,
    meets_floor: false,
  }; 10];

  let mut min_dim_score = u16::MAX;
  let mut max_dim_score = u16::MIN;
  let mut weakest_dimension = EvaluationDimension::Onboarding;
  let mut strongest_dimension = EvaluationDimension::Onboarding;

  let mut accessibility_dimensions_qualified = true;

  for (dim_idx, &dim) in EvaluationDimension::ALL.iter().enumerate() {
    let mut dim_sum: u64 = 0;
    let mut min_score: u16 = 10_000;
    let mut max_score: u16 = 0;
    let mut friction_counts = [0usize; 7];

    for assessment in assessments {
      let score_entry = assessment
        .scores
        .iter()
        .find(|s| s.dimension == dim)
        .expect("dimension verified present");

      dim_sum += u64::from(score_entry.score_bp);
      if score_entry.score_bp < min_score {
        min_score = score_entry.score_bp;
      }
      if score_entry.score_bp > max_score {
        max_score = score_entry.score_bp;
      }

      let f_idx = CognitiveFrictionIndicator::ALL
        .iter()
        .position(|&f| f == score_entry.friction)
        .expect("valid friction indicator");
      friction_counts[f_idx] += 1;
    }

    let mean_bp = u16::try_from(dim_sum / u64::from(count_u32)).expect("mean fits in u16");
    total_score_sum += dim_sum;

    // Determine predominant friction
    let mut max_f_count = 0;
    let mut predominant_friction = CognitiveFrictionIndicator::None;
    for (f_idx, &cnt) in friction_counts.iter().enumerate() {
      if cnt > max_f_count {
        max_f_count = cnt;
        predominant_friction = CognitiveFrictionIndicator::ALL[f_idx];
      }
    }

    let meets_floor = mean_bp >= protocol.target_comprehension_floor_bp;

    if dim.is_accessibility() && !meets_floor {
      accessibility_dimensions_qualified = false;
    }

    if mean_bp < min_dim_score {
      min_dim_score = mean_bp;
      weakest_dimension = dim;
    }
    if mean_bp > max_dim_score {
      max_dim_score = mean_bp;
      strongest_dimension = dim;
    }

    dimension_summaries[dim_idx] = DimensionSummary {
      dimension: dim,
      mean_score_bp: mean_bp,
      min_score_bp: min_score,
      max_score_bp: max_score,
      predominant_friction,
      meets_floor,
    };
  }

  let total_measurements = count_u32 * 10;
  let overall_mean_score_bp = u16::try_from(total_score_sum / u64::from(total_measurements))
    .expect("overall mean fits in u16");

  Ok(DimensionEvaluationReport {
    protocol_id: protocol.protocol_id,
    assessment_count,
    overall_mean_score_bp,
    dimension_summaries,
    weakest_dimension,
    strongest_dimension,
    accessibility_dimensions_qualified,
    evidence_boundary_statement: super::evaluation::STANDARD_EVIDENCE_BOUNDARY,
  })
}

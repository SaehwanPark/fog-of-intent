//! Presentation need taxonomy, comprehension deficit assessment, and GUI justification evaluation.

use core::fmt;

/// Schema version for presentation need evaluation contracts.
pub const GUI_NEED_SCHEMA_VERSION: &str = "m11-gui-presentation-need-v1";

/// Minimum mean deficit impact in basis points (4,000 bp = 40.0%) to justify GUI adoption.
pub const GUI_JUSTIFICATION_THRESHOLD_BP: u32 = 4_000;

/// Minimum individual barrier deficit impact in basis points (5,000 bp = 50.0%) to trigger GUI justification.
pub const GUI_BARRIER_THRESHOLD_BP: u32 = 5_000;

/// Maximum basis point score (10,000 bp = 100.0%).
pub const MAX_BASIS_POINTS: u32 = 10_000;

/// Cognitive comprehension domains where pure text streams can exhibit friction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComprehensionDomain {
  /// Spatial topology, multi-lane navigation, fog-of-war vision coverage, and rotation routes.
  SpatialTopology,
  /// Multi-beat turn pacing, delayed effect queue resolution, and simultaneous phase synchronization.
  TemporalTimeline,
  /// Multi-step plan staging, conditional commitments, abort triggers, and fallback behaviors.
  ContingencyBranching,
  /// 2D orthogonal coordination vs execution quadrants, KPI cards, and counterfactual deltas.
  CausalDebrief,
}

impl ComprehensionDomain {
  /// Canonical string identifier for the domain.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::SpatialTopology => "spatial-topology",
      Self::TemporalTimeline => "temporal-timeline",
      Self::ContingencyBranching => "contingency-branching",
      Self::CausalDebrief => "causal-debrief",
    }
  }

  /// Parse domain from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "spatial-topology" => Some(Self::SpatialTopology),
      "temporal-timeline" => Some(Self::TemporalTimeline),
      "contingency-branching" => Some(Self::ContingencyBranching),
      "causal-debrief" => Some(Self::CausalDebrief),
      _ => None,
    }
  }
}

impl fmt::Display for ComprehensionDomain {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Severity classification of a presentation comprehension deficit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeficitSeverity {
  /// Minor friction; text remains adequate with slightly elevated cognitive load.
  Negligible,
  /// Noticeable friction; player or researcher requires repeated inspection or extra effort.
  ModerateFriction,
  /// Severe barrier; spatial/temporal/causal relationship is difficult to reconstruct from linear text.
  SignificantBarrier,
}

impl DeficitSeverity {
  /// Canonical string identifier for severity.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::Negligible => "negligible",
      Self::ModerateFriction => "moderate-friction",
      Self::SignificantBarrier => "significant-barrier",
    }
  }

  /// Parse severity from canonical string.
  pub fn from_str_name(name: &str) -> Option<Self> {
    match name {
      "negligible" => Some(Self::Negligible),
      "moderate-friction" => Some(Self::ModerateFriction),
      "significant-barrier" => Some(Self::SignificantBarrier),
      _ => None,
    }
  }
}

impl fmt::Display for DeficitSeverity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// A concrete cognitive comprehension deficit observed or modeled in pure text presentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComprehensionDeficit {
  domain: ComprehensionDomain,
  severity: DeficitSeverity,
  deficit_impact_bp: u32,
  description: String,
  text_modality_limitation: String,
  gui_mitigation: String,
}

impl ComprehensionDeficit {
  /// Construct a new comprehension deficit record.
  pub fn new(
    domain: ComprehensionDomain,
    severity: DeficitSeverity,
    deficit_impact_bp: u32,
    description: impl Into<String>,
    text_modality_limitation: impl Into<String>,
    gui_mitigation: impl Into<String>,
  ) -> Self {
    Self {
      domain,
      severity,
      deficit_impact_bp,
      description: description.into(),
      text_modality_limitation: text_modality_limitation.into(),
      gui_mitigation: gui_mitigation.into(),
    }
  }

  /// Target domain.
  pub const fn domain(&self) -> ComprehensionDomain {
    self.domain
  }

  /// Deficit severity.
  pub const fn severity(&self) -> DeficitSeverity {
    self.severity
  }

  /// Deficit impact score in basis points (0..=10,000).
  pub const fn deficit_impact_bp(&self) -> u32 {
    self.deficit_impact_bp
  }

  /// Human-readable deficit description.
  pub fn description(&self) -> &str {
    &self.description
  }

  /// Specific limitation of pure text presentation.
  pub fn text_modality_limitation(&self) -> &str {
    &self.text_modality_limitation
  }

  /// Expected mitigation provided by graphical presentation.
  pub fn gui_mitigation(&self) -> &str {
    &self.gui_mitigation
  }
}

/// Evaluation assessment measuring whether graphical presentation is justified for a scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresentationNeedAssessment {
  assessment_id: String,
  scenario_id: String,
  observer_role: String,
  deficits: Vec<ComprehensionDeficit>,
  mean_deficit_impact_bp: u32,
  max_deficit_impact_bp: u32,
  gui_justified: bool,
  evidence_rationale: String,
}

impl PresentationNeedAssessment {
  /// Assessment identifier.
  pub fn assessment_id(&self) -> &str {
    &self.assessment_id
  }

  /// Target scenario identifier.
  pub fn scenario_id(&self) -> &str {
    &self.scenario_id
  }

  /// Observer role.
  pub fn observer_role(&self) -> &str {
    &self.observer_role
  }

  /// List of evaluated deficits.
  pub fn deficits(&self) -> &[ComprehensionDeficit] {
    &self.deficits
  }

  /// Mean deficit impact in basis points.
  pub const fn mean_deficit_impact_bp(&self) -> u32 {
    self.mean_deficit_impact_bp
  }

  /// Maximum individual deficit impact in basis points.
  pub const fn max_deficit_impact_bp(&self) -> u32 {
    self.max_deficit_impact_bp
  }

  /// Whether graphical presentation is justified by the evidence.
  pub const fn gui_justified(&self) -> bool {
    self.gui_justified
  }

  /// Evidence rationale.
  pub fn evidence_rationale(&self) -> &str {
    &self.evidence_rationale
  }
}

/// Fail-closed error types for presentation need assessment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresentationNeedError {
  /// Deficit list is empty.
  EmptyDeficitList,
  /// Identifier string is empty or blank.
  EmptyIdentifier(&'static str),
  /// Duplicate comprehension domain encountered in single assessment.
  DuplicateDomain(ComprehensionDomain),
  /// Deficit impact score exceeds basis point ceiling (10,000 bp).
  DeficitScoreOutOfRange(u32),
  /// Text description or limitation string is empty.
  EmptyDescription(&'static str),
}

impl fmt::Display for PresentationNeedError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyDeficitList => write!(f, "deficit list must not be empty"),
      Self::EmptyIdentifier(field) => write!(f, "identifier field '{}' must not be empty", field),
      Self::DuplicateDomain(domain) => write!(f, "duplicate evaluation for domain '{}'", domain),
      Self::DeficitScoreOutOfRange(score) => {
        write!(f, "deficit score {} exceeds 10,000 bp maximum", score)
      }
      Self::EmptyDescription(field) => write!(f, "description field '{}' must not be empty", field),
    }
  }
}

/// Deterministically evaluate presentation need and GUI justification.
pub fn evaluate_presentation_need(
  assessment_id: &str,
  scenario_id: &str,
  observer_role: &str,
  deficits: Vec<ComprehensionDeficit>,
  evidence_rationale: &str,
) -> Result<PresentationNeedAssessment, PresentationNeedError> {
  if assessment_id.trim().is_empty() {
    return Err(PresentationNeedError::EmptyIdentifier("assessment_id"));
  }
  if scenario_id.trim().is_empty() {
    return Err(PresentationNeedError::EmptyIdentifier("scenario_id"));
  }
  if observer_role.trim().is_empty() {
    return Err(PresentationNeedError::EmptyIdentifier("observer_role"));
  }
  if evidence_rationale.trim().is_empty() {
    return Err(PresentationNeedError::EmptyDescription(
      "evidence_rationale",
    ));
  }
  if deficits.is_empty() {
    return Err(PresentationNeedError::EmptyDeficitList);
  }

  let mut seen_domains = [false; 4];
  let mut total_score_bp: u64 = 0;
  let mut max_score_bp: u32 = 0;
  let mut has_barrier = false;

  for deficit in &deficits {
    let domain_idx = match deficit.domain {
      ComprehensionDomain::SpatialTopology => 0,
      ComprehensionDomain::TemporalTimeline => 1,
      ComprehensionDomain::ContingencyBranching => 2,
      ComprehensionDomain::CausalDebrief => 3,
    };
    if seen_domains[domain_idx] {
      return Err(PresentationNeedError::DuplicateDomain(deficit.domain));
    }
    seen_domains[domain_idx] = true;

    if deficit.deficit_impact_bp > MAX_BASIS_POINTS {
      return Err(PresentationNeedError::DeficitScoreOutOfRange(
        deficit.deficit_impact_bp,
      ));
    }
    if deficit.description.trim().is_empty() {
      return Err(PresentationNeedError::EmptyDescription("description"));
    }
    if deficit.text_modality_limitation.trim().is_empty() {
      return Err(PresentationNeedError::EmptyDescription(
        "text_modality_limitation",
      ));
    }
    if deficit.gui_mitigation.trim().is_empty() {
      return Err(PresentationNeedError::EmptyDescription("gui_mitigation"));
    }

    total_score_bp = total_score_bp.saturating_add(u64::from(deficit.deficit_impact_bp));
    if deficit.deficit_impact_bp > max_score_bp {
      max_score_bp = deficit.deficit_impact_bp;
    }
    if deficit.severity == DeficitSeverity::SignificantBarrier
      && deficit.deficit_impact_bp >= GUI_BARRIER_THRESHOLD_BP
    {
      has_barrier = true;
    }
  }

  let count = u64::try_from(deficits.len()).unwrap_or(1);
  let mean_deficit_impact_bp = u32::try_from(total_score_bp / count).unwrap_or(MAX_BASIS_POINTS);
  let gui_justified = mean_deficit_impact_bp >= GUI_JUSTIFICATION_THRESHOLD_BP || has_barrier;

  Ok(PresentationNeedAssessment {
    assessment_id: assessment_id.to_string(),
    scenario_id: scenario_id.to_string(),
    observer_role: observer_role.to_string(),
    deficits,
    mean_deficit_impact_bp,
    max_deficit_impact_bp: max_score_bp,
    gui_justified,
    evidence_rationale: evidence_rationale.to_string(),
  })
}

/// Render a clean Markdown report for a presentation need assessment.
pub fn render_presentation_need_markdown(assessment: &PresentationNeedAssessment) -> String {
  let mut md = String::new();
  md.push_str("# Presentation Need and GUI Justification Report\n\n");
  md.push_str(&format!(
    "- **Assessment ID:** {}\n",
    assessment.assessment_id
  ));
  md.push_str(&format!("- **Scenario ID:** {}\n", assessment.scenario_id));
  md.push_str(&format!(
    "- **Observer Role:** {}\n",
    assessment.observer_role
  ));
  md.push_str(&format!(
    "- **Mean Deficit Impact:** {}.{:02}%\n",
    assessment.mean_deficit_impact_bp / 100,
    assessment.mean_deficit_impact_bp % 100
  ));
  md.push_str(&format!(
    "- **Max Deficit Impact:** {}.{:02}%\n",
    assessment.max_deficit_impact_bp / 100,
    assessment.max_deficit_impact_bp % 100
  ));
  md.push_str(&format!(
    "- **GUI Justified:** {}\n",
    if assessment.gui_justified {
      "[YES] Justified by Comprehension Evidence"
    } else {
      "[NO] Text Presentation Remains Sufficient"
    }
  ));
  md.push_str(&format!(
    "- **Evidence Rationale:** {}\n\n",
    assessment.evidence_rationale
  ));

  md.push_str("## Comprehension Deficits\n\n");
  md.push_str("| Domain | Severity | Impact (bp) | Text Limitation | GUI Mitigation |\n");
  md.push_str("| --- | --- | --- | --- | --- |\n");
  for d in &assessment.deficits {
    md.push_str(&format!(
      "| {} | {} | {} bp | {} | {} |\n",
      d.domain.as_str(),
      d.severity.as_str(),
      d.deficit_impact_bp,
      d.text_modality_limitation,
      d.gui_mitigation
    ));
  }
  md.push('\n');

  md
}

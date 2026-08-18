//! M10 Human Usability and Accessibility Alpha synthesis and evidence reporting.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Synthesizes participant cohort metrics, dimension assessments, interaction audits,
//! remediation plans, and sampling limits into an authoritative, deterministic M10 Alpha
//! Synthesis Report. Evaluates overall alpha readiness gates and distinguishes observed
//! empirical facts from inferred design hypotheses.

use core::fmt;

use super::dimension::DimensionEvaluationReport;
use super::evaluation::StudyEvaluationReport;
use super::interaction::InteractionAuditReport;
use super::remediation::RemediationEvaluationReport;
use super::sampling::ParticipantSamplingReport;

pub const M10_ALPHA_SYNTHESIS_SCHEMA_V1: &str = "m10-alpha-synthesis-v1";

/// Overall readiness gate evaluation across all 5 M10 quality and accessibility dimensions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AlphaReadinessGateStatus {
  /// Overall study completion rate meets target floor (e.g. >= 7,500 bp = 75%).
  pub study_completion_floor_met: bool,
  /// Overall debrief comprehension meets target floor (e.g. >= 7,000 bp = 70%).
  pub comprehension_floor_met: bool,
  /// Accessibility dimension score meets floor (>= 6,000 bp) and interaction audit passes.
  pub accessibility_floor_met: bool,
  /// Remediation plan verified share meets readiness gate (>= 5,000 bp) with 0 unresolved blockers.
  pub remediation_readiness_met: bool,
  /// Participant sampling quotas and access needs representation pass.
  pub sampling_diversity_met: bool,
}

impl AlphaReadinessGateStatus {
  /// Returns true if every readiness gate passes.
  pub const fn all_gates_passed(&self) -> bool {
    self.study_completion_floor_met
      && self.comprehension_floor_met
      && self.accessibility_floor_met
      && self.remediation_readiness_met
      && self.sampling_diversity_met
  }
}

/// Final alpha milestone readiness disposition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AlphaDisposition {
  /// All 5 readiness gates pass with zero unresolved blockers; ready for alpha play.
  AlphaReady,
  /// Completion and comprehension pass, but documented sampling limits or minor cautions exist.
  ConditionallyReadyWithLimitations,
  /// One or more critical readiness gates fail or unresolved blockers remain.
  BlockedByReadinessGates,
}

impl AlphaDisposition {
  pub const ALL: [Self; 3] = [
    Self::AlphaReady,
    Self::ConditionallyReadyWithLimitations,
    Self::BlockedByReadinessGates,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::AlphaReady => "alpha-ready",
      Self::ConditionallyReadyWithLimitations => "conditionally-ready-with-limitations",
      Self::BlockedByReadinessGates => "blocked-by-readiness-gates",
    }
  }
}

impl fmt::Display for AlphaDisposition {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Clear separation between observed empirical facts and inferred design hypotheses.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmpiricalFactVsInferredHypothesis {
  pub observed_completion_rate_bp: u16,
  pub observed_comprehension_score_bp: u16,
  pub observed_mean_dimension_score_bp: u16,
  pub observed_blocker_count: usize,
  pub verified_remediation_count: usize,
  pub inferred_design_hypotheses: &'static [&'static str],
}

/// Authoritative synthesized evidence report for M10 Alpha.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlphaEvidenceSynthesis {
  pub synthesis_id: &'static str,
  pub protocol_id: &'static str,
  pub study_report: StudyEvaluationReport,
  pub dimension_report: DimensionEvaluationReport,
  pub interaction_report: InteractionAuditReport,
  pub remediation_report: RemediationEvaluationReport,
  pub sampling_report: ParticipantSamplingReport,
  pub gates: AlphaReadinessGateStatus,
  pub disposition: AlphaDisposition,
  pub empirical_vs_inferred: EmpiricalFactVsInferredHypothesis,
}

impl AlphaEvidenceSynthesis {
  /// Formats a clean, structured Markdown report without private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::with_capacity(2048);
    out.push_str("# M10 Human Usability & Accessibility Alpha Evidence Synthesis\n\n");
    out.push_str(&format!("- **Synthesis ID:** `{}`\n", self.synthesis_id));
    out.push_str(&format!("- **Protocol ID:** `{}`\n", self.protocol_id));
    out.push_str(&format!(
      "- **Milestone Disposition:** `{}`\n",
      self.disposition.as_str()
    ));
    out.push_str(&format!(
      "- **All Readiness Gates Passed:** {}\n\n",
      if self.gates.all_gates_passed() {
        "YES [PASS]"
      } else {
        "NO [FAIL]"
      }
    ));

    out.push_str("## Readiness Gates Evaluation\n\n");
    out.push_str("| Gate | Status | Detail |\n");
    out.push_str("| :--- | :--- | :--- |\n");
    out.push_str(&format!(
      "| Study Completion Floor | {} | Overall Completion: {} bp |\n",
      if self.gates.study_completion_floor_met {
        "[PASS]"
      } else {
        "[FAIL]"
      },
      self.study_report.overall_completion_rate_bp
    ));
    out.push_str(&format!(
      "| Debrief Comprehension Floor | {} | Comprehension Score: {} bp |\n",
      if self.gates.comprehension_floor_met {
        "[PASS]"
      } else {
        "[FAIL]"
      },
      self.study_report.overall_avg_comprehension_bp
    ));
    out.push_str(&format!(
      "| Accessibility Qualification | {} | Dimension Qualified: {}, Audit: {} |\n",
      if self.gates.accessibility_floor_met {
        "[PASS]"
      } else {
        "[FAIL]"
      },
      self.dimension_report.accessibility_dimensions_qualified,
      if self.interaction_report.all_passed {
        "PASS"
      } else {
        "FAIL"
      }
    ));
    out.push_str(&format!(
      "| Remediation Action Readiness | {} | Verified Action Share: {} bp, Unresolved Blockers: {} |\n",
      if self.gates.remediation_readiness_met { "[PASS]" } else { "[FAIL]" },
      self.remediation_report.verified_actions_share_bp,
      self.study_report.unresolved_accessibility_blockers
    ));
    out.push_str(&format!(
      "| Sampling Diversity Quotas | {} | Sample Size: {}, Access Needs: {} bp |\n\n",
      if self.gates.sampling_diversity_met {
        "[PASS]"
      } else {
        "[FAIL]"
      },
      self.sampling_report.sample_size,
      self
        .sampling_report
        .access_needs_breakdown
        .access_needs_share_bp
    ));

    out.push_str("## Empirical Facts vs Inferred Design Hypotheses\n\n");
    out.push_str("### Observed Empirical Facts\n\n");
    out.push_str(&format!(
      "- Total Evaluated Participants: {}\n",
      self.study_report.total_participants
    ));
    out.push_str(&format!(
      "- Completed Sessions: {} / {}\n",
      self.study_report.completed_count, self.study_report.total_participants
    ));
    out.push_str(&format!(
      "- Overall Average Explanation Quality: {} bp\n",
      self.study_report.overall_avg_explanation_bp
    ));
    out.push_str(&format!(
      "- Overall Average Debrief Comprehension: {} bp\n",
      self.study_report.overall_avg_comprehension_bp
    ));
    out.push_str(&format!(
      "- Weakest Dimension: `{}`\n",
      self.dimension_report.weakest_dimension.as_str()
    ));
    out.push_str(&format!(
      "- Strongest Dimension: `{}`\n",
      self.dimension_report.strongest_dimension.as_str()
    ));
    out.push_str(&format!(
      "- Total Verified Remediation Actions: {}\n\n",
      self.empirical_vs_inferred.verified_remediation_count
    ));

    out.push_str("### Inferred Design Hypotheses\n\n");
    for (i, hyp) in self
      .empirical_vs_inferred
      .inferred_design_hypotheses
      .iter()
      .enumerate()
    {
      let idx = i.saturating_add(1);
      out.push_str(&format!("{idx}. {hyp}\n"));
    }
    out.push('\n');

    out.push_str("## Untested Populations Disclosure\n\n");
    for disc in self.sampling_report.untested_disclosures {
      out.push_str(&format!(
        "- **`{}`**: {}\n",
        disc.category.as_str(),
        disc.rationale
      ));
    }
    out.push('\n');

    out
  }
}

/// Errors encountered during synthesis evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynthesisEvaluationError {
  /// Synthesis ID was empty.
  EmptySynthesisId,
  /// Sample sizes did not match across reports.
  SampleSizeMismatch {
    study_sample_size: usize,
    sampling_sample_size: usize,
  },
  /// Inferred design hypotheses list was empty.
  EmptyInferredHypotheses,
}

impl fmt::Display for SynthesisEvaluationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptySynthesisId => f.write_str("synthesis id cannot be empty"),
      Self::SampleSizeMismatch {
        study_sample_size,
        sampling_sample_size,
      } => write!(
        f,
        "sample size mismatch: study report has {study_sample_size}, sampling report has {sampling_sample_size}"
      ),
      Self::EmptyInferredHypotheses => f.write_str("inferred design hypotheses cannot be empty"),
    }
  }
}

/// Synthesizes reports across all M10 evaluation dimensions and determines the milestone disposition.
pub fn synthesize_alpha_evidence(
  synthesis_id: &'static str,
  study_report: StudyEvaluationReport,
  dimension_report: DimensionEvaluationReport,
  interaction_report: InteractionAuditReport,
  remediation_report: RemediationEvaluationReport,
  sampling_report: ParticipantSamplingReport,
  inferred_hypotheses: &'static [&'static str],
) -> Result<AlphaEvidenceSynthesis, SynthesisEvaluationError> {
  if synthesis_id.trim().is_empty() {
    return Err(SynthesisEvaluationError::EmptySynthesisId);
  }
  if study_report.total_participants != sampling_report.sample_size {
    return Err(SynthesisEvaluationError::SampleSizeMismatch {
      study_sample_size: study_report.total_participants,
      sampling_sample_size: sampling_report.sample_size,
    });
  }
  if inferred_hypotheses.is_empty() {
    return Err(SynthesisEvaluationError::EmptyInferredHypotheses);
  }

  // Evaluate gates
  let study_completion_floor_met = study_report.completion_target_met;
  let comprehension_floor_met = study_report.comprehension_target_met;
  let accessibility_floor_met =
    dimension_report.accessibility_dimensions_qualified && interaction_report.all_passed;
  let remediation_readiness_met = remediation_report.remediation_readiness_gate_passed
    && study_report.unresolved_accessibility_blockers == 0;
  let sampling_diversity_met = sampling_report.sampling_gate_passed;

  let gates = AlphaReadinessGateStatus {
    study_completion_floor_met,
    comprehension_floor_met,
    accessibility_floor_met,
    remediation_readiness_met,
    sampling_diversity_met,
  };

  let disposition = if gates.all_gates_passed() {
    AlphaDisposition::AlphaReady
  } else if study_completion_floor_met
    && comprehension_floor_met
    && remediation_readiness_met
    && study_report.unresolved_accessibility_blockers == 0
  {
    AlphaDisposition::ConditionallyReadyWithLimitations
  } else {
    AlphaDisposition::BlockedByReadinessGates
  };

  let empirical_vs_inferred = EmpiricalFactVsInferredHypothesis {
    observed_completion_rate_bp: study_report.overall_completion_rate_bp,
    observed_comprehension_score_bp: study_report.overall_avg_comprehension_bp,
    observed_mean_dimension_score_bp: dimension_report.overall_mean_score_bp,
    observed_blocker_count: study_report.blocker_count,
    verified_remediation_count: remediation_report.actions_verified_count,
    inferred_design_hypotheses: inferred_hypotheses,
  };

  Ok(AlphaEvidenceSynthesis {
    synthesis_id,
    protocol_id: study_report.protocol_id,
    study_report,
    dimension_report,
    interaction_report,
    remediation_report,
    sampling_report,
    gates,
    disposition,
    empirical_vs_inferred,
  })
}

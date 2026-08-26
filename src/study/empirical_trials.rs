//! Empirical Multi-Cohort Study Trials Evaluator and Report Engine for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! This module formalizes structured empirical playtest trial evaluation across all 4
//! canonical participant cohorts (`StrategyGamer`, `MobaPlayer`, `AccessNeeds`, `NoviceStrategy`).
//! It computes deterministic basis-point metrics for completion rates, decision explanation
//! quality, debrief causal comprehension, cognitive friction density, and accessibility
//! qualification with fail-closed validation and structured Markdown report generation.

use core::fmt;
use std::fmt::Write as _;

use super::dimension::CognitiveFrictionIndicator;
use super::finding::{FindingCategory, FindingRecord, FindingSeverity};
use super::protocol::{
  EvaluationDimension, ParticipantCohort, PrivacyConsentDeclaration, StudyProtocolDefinition,
};
use super::session::ParticipantSessionRecord;

/// Schema identifier for empirical cohort trials.
pub const M10_EMPI_COHORT_TRIALS_SCHEMA_V1: &str = "m10-empirical-cohort-trials-v1";

/// Standard baseline empirical study protocol definition for M10 alpha evaluation.
pub const EMPIRICAL_ALPHA_PROTOCOL: StudyProtocolDefinition = StudyProtocolDefinition {
  protocol_id: "protocol-m10-alpha-empirical-v1",
  title: "Milestone M10 Human Usability and Accessibility Alpha Study Protocol",
  research_question: "Does the intent-driven interface and causal debrief afford strategic clarity across representative gamer and accessibility cohorts?",
  target_completion_floor_bp: 7_500,
  target_comprehension_floor_bp: 7_500,
  privacy_declaration: PrivacyConsentDeclaration::standard(),
};

/// Error conditions in empirical cohort study trial evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EmpiricalCohortError {
  /// Session collection is empty.
  EmptySessionList,
  /// Target protocol target completion floor is invalid (> 10,000 bp).
  InvalidCompletionFloor { floor_bp: u16 },
  /// Target protocol comprehension floor is invalid (> 10,000 bp).
  InvalidComprehensionFloor { floor_bp: u16 },
  /// Participant record contains invalid explanation quality score (> 10,000 bp).
  InvalidExplanationScore {
    participant_id: &'static str,
    score_bp: u16,
  },
  /// Participant record contains invalid debrief comprehension score (> 10,000 bp).
  InvalidComprehensionScore {
    participant_id: &'static str,
    score_bp: u16,
  },
  /// Missing representation for a required cohort.
  MissingRequiredCohort { cohort: ParticipantCohort },
  /// Duplicate participant identifier.
  DuplicateParticipantId { participant_id: &'static str },
}

impl fmt::Display for EmpiricalCohortError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptySessionList => write!(f, "empirical-trials: session list cannot be empty"),
      Self::InvalidCompletionFloor { floor_bp } => {
        write!(
          f,
          "empirical-trials: invalid completion floor {floor_bp} bp (max 10000 bp)"
        )
      }
      Self::InvalidComprehensionFloor { floor_bp } => {
        write!(
          f,
          "empirical-trials: invalid comprehension floor {floor_bp} bp (max 10000 bp)"
        )
      }
      Self::InvalidExplanationScore {
        participant_id,
        score_bp,
      } => {
        write!(
          f,
          "empirical-trials: participant '{participant_id}' has invalid explanation score {score_bp} bp"
        )
      }
      Self::InvalidComprehensionScore {
        participant_id,
        score_bp,
      } => {
        write!(
          f,
          "empirical-trials: participant '{participant_id}' has invalid debrief score {score_bp} bp"
        )
      }
      Self::MissingRequiredCohort { cohort } => {
        write!(
          f,
          "empirical-trials: missing required participant cohort '{cohort}'"
        )
      }
      Self::DuplicateParticipantId { participant_id } => {
        write!(
          f,
          "empirical-trials: duplicate participant id '{participant_id}'"
        )
      }
    }
  }
}

/// Detailed playtest trial session descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmpiricalTrialSession {
  pub session: ParticipantSessionRecord,
  pub primary_dimension_focus: EvaluationDimension,
  pub reported_frictions: Vec<CognitiveFrictionIndicator>,
  pub qualitative_notes: &'static str,
}

/// Metrics summary for an individual cohort in the trial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CohortTrialSummary {
  pub cohort: ParticipantCohort,
  pub total_participants: usize,
  pub completed_sessions: usize,
  pub abandoned_sessions: usize,
  pub completion_rate_bp: u16,
  pub mean_explanation_bp: u16,
  pub mean_debrief_bp: u16,
  pub total_friction_incidents: usize,
  pub target_met: bool,
}

/// Aggregated empirical cohort trial evaluation report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmpiricalCohortTrialReport {
  pub schema: &'static str,
  pub protocol: StudyProtocolDefinition,
  pub total_participants: usize,
  pub total_completed: usize,
  pub total_abandoned: usize,
  pub overall_completion_rate_bp: u16,
  pub overall_mean_explanation_bp: u16,
  pub overall_mean_debrief_bp: u16,
  pub accessibility_qualified: bool,
  pub completion_target_met: bool,
  pub comprehension_target_met: bool,
  pub cohort_summaries: [CohortTrialSummary; 4],
  pub findings: Vec<FindingRecord>,
}

impl EmpiricalCohortTrialReport {
  /// Whether all alpha readiness gates are satisfied by this empirical trial suite.
  #[must_use]
  pub fn is_alpha_ready(&self) -> bool {
    self.completion_target_met
      && self.comprehension_target_met
      && self.accessibility_qualified
      && !self.has_unresolved_blockers()
  }

  /// Check if there are any unresolved blockers among findings.
  #[must_use]
  pub fn has_unresolved_blockers(&self) -> bool {
    self
      .findings
      .iter()
      .any(|f| f.disposition.is_unresolved_blocker(f.severity))
  }

  /// Render structured, human-readable Markdown report.
  #[must_use]
  pub fn render_markdown(&self) -> String {
    let mut out = String::with_capacity(4096);
    let _ = writeln!(
      out,
      "### Empirical Cohort Study Trial Report (`{}`)\n",
      self.schema
    );
    let _ = writeln!(
      out,
      "- **Protocol:** {} (`{}`)",
      self.protocol.title, self.protocol.protocol_id
    );
    let _ = writeln!(out, "- **Total Participants:** {}", self.total_participants);
    let _ = writeln!(
      out,
      "- **Overall Completion Rate:** {} bp ({} completed, {} abandoned)",
      self.overall_completion_rate_bp, self.total_completed, self.total_abandoned
    );
    let _ = writeln!(
      out,
      "- **Mean Explanation Quality:** {} bp",
      self.overall_mean_explanation_bp
    );
    let _ = writeln!(
      out,
      "- **Mean Debrief Comprehension:** {} bp",
      self.overall_mean_debrief_bp
    );
    let _ = writeln!(
      out,
      "- **Accessibility Claims Qualified:** {}",
      if self.accessibility_qualified {
        "YES (PASSED)"
      } else {
        "NO (DISQUALIFIED)"
      }
    );
    let _ = writeln!(
      out,
      "- **Alpha Readiness Gate:** {}\n",
      if self.is_alpha_ready() {
        "PASS (READY FOR ALPHA)"
      } else {
        "BLOCKED (REMEDIATION REQUIRED)"
      }
    );

    out.push_str("#### Cohort Breakdown\n\n");
    out.push_str("| Cohort | Participants | Completed | Abandoned | Completion (bp) | Explanation (bp) | Debrief (bp) | Frictions | Status |\n");
    out.push_str("|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|\n");

    for summary in &self.cohort_summaries {
      let _ = writeln!(
        out,
        "| `{}` | {} | {} | {} | {} | {} | {} | {} | {} |",
        summary.cohort.as_str(),
        summary.total_participants,
        summary.completed_sessions,
        summary.abandoned_sessions,
        summary.completion_rate_bp,
        summary.mean_explanation_bp,
        summary.mean_debrief_bp,
        summary.total_friction_incidents,
        if summary.target_met { "PASSED" } else { "FAIL" }
      );
    }
    out.push('\n');

    if !self.findings.is_empty() {
      out.push_str("#### Finding Dispositions\n\n");
      for finding in &self.findings {
        let _ = writeln!(
          out,
          "- **[{}] [{}]** `{}` — {} (Disposition: `{}`)",
          finding.severity.as_str(),
          finding.category.as_str(),
          finding.finding_id,
          finding.description,
          finding.disposition.disposition_name()
        );
      }
      out.push('\n');
    }

    out.push_str("> [!NOTE]\n");
    out.push_str("> Empirical trial scores represent structured playtest sessions with recorded participants.\n");
    out.push_str("> Findings qualify software behavior and technical accessibility claims, not universal human enjoyment.\n");

    out
  }
}

/// Pure evaluation function — evaluate empirical trial sessions against study protocol and findings.
pub fn evaluate_empirical_trials(
  protocol: &StudyProtocolDefinition,
  sessions: &[EmpiricalTrialSession],
  findings: &[FindingRecord],
) -> Result<EmpiricalCohortTrialReport, EmpiricalCohortError> {
  if sessions.is_empty() {
    return Err(EmpiricalCohortError::EmptySessionList);
  }
  if protocol.target_completion_floor_bp > 10_000 {
    return Err(EmpiricalCohortError::InvalidCompletionFloor {
      floor_bp: protocol.target_completion_floor_bp,
    });
  }
  if protocol.target_comprehension_floor_bp > 10_000 {
    return Err(EmpiricalCohortError::InvalidComprehensionFloor {
      floor_bp: protocol.target_comprehension_floor_bp,
    });
  }

  // Validate session score bounds and participant ID uniqueness
  let mut seen_ids = Vec::with_capacity(sessions.len());
  for s in sessions {
    let pid = s.session.participant_id;
    if seen_ids.contains(&pid) {
      return Err(EmpiricalCohortError::DuplicateParticipantId {
        participant_id: pid,
      });
    }
    seen_ids.push(pid);

    if s.session.explanation_quality_bp > 10_000 {
      return Err(EmpiricalCohortError::InvalidExplanationScore {
        participant_id: pid,
        score_bp: s.session.explanation_quality_bp,
      });
    }
    if s.session.debrief_comprehension_bp > 10_000 {
      return Err(EmpiricalCohortError::InvalidComprehensionScore {
        participant_id: pid,
        score_bp: s.session.debrief_comprehension_bp,
      });
    }
  }

  let total_participants = sessions.len();
  let total_completed = sessions
    .iter()
    .filter(|s| s.session.completion_status.is_completed())
    .count();
  let total_abandoned = sessions
    .iter()
    .filter(|s| s.session.completion_status.is_abandoned())
    .count();

  let overall_completion_rate_bp = calculate_bp(total_completed, total_participants);

  let sum_expl: u64 = sessions
    .iter()
    .map(|s| u64::from(s.session.explanation_quality_bp))
    .sum();
  let overall_mean_explanation_bp = safe_div_u16(sum_expl, total_participants);

  let sum_debrief: u64 = sessions
    .iter()
    .map(|s| u64::from(s.session.debrief_comprehension_bp))
    .sum();
  let overall_mean_debrief_bp = safe_div_u16(sum_debrief, total_participants);

  // Compute per-cohort summaries
  let mut summaries = [
    CohortTrialSummary {
      cohort: ParticipantCohort::StrategyGamer,
      total_participants: 0,
      completed_sessions: 0,
      abandoned_sessions: 0,
      completion_rate_bp: 0,
      mean_explanation_bp: 0,
      mean_debrief_bp: 0,
      total_friction_incidents: 0,
      target_met: false,
    },
    CohortTrialSummary {
      cohort: ParticipantCohort::MobaPlayer,
      total_participants: 0,
      completed_sessions: 0,
      abandoned_sessions: 0,
      completion_rate_bp: 0,
      mean_explanation_bp: 0,
      mean_debrief_bp: 0,
      total_friction_incidents: 0,
      target_met: false,
    },
    CohortTrialSummary {
      cohort: ParticipantCohort::AccessNeeds,
      total_participants: 0,
      completed_sessions: 0,
      abandoned_sessions: 0,
      completion_rate_bp: 0,
      mean_explanation_bp: 0,
      mean_debrief_bp: 0,
      total_friction_incidents: 0,
      target_met: false,
    },
    CohortTrialSummary {
      cohort: ParticipantCohort::NoviceStrategy,
      total_participants: 0,
      completed_sessions: 0,
      abandoned_sessions: 0,
      completion_rate_bp: 0,
      mean_explanation_bp: 0,
      mean_debrief_bp: 0,
      total_friction_incidents: 0,
      target_met: false,
    },
  ];

  for (i, cohort) in ParticipantCohort::ALL.iter().enumerate() {
    let cohort_sessions: Vec<&EmpiricalTrialSession> = sessions
      .iter()
      .filter(|s| s.session.cohort == *cohort)
      .collect();

    let count = cohort_sessions.len();
    if count == 0 {
      return Err(EmpiricalCohortError::MissingRequiredCohort { cohort: *cohort });
    }

    let completed = cohort_sessions
      .iter()
      .filter(|s| s.session.completion_status.is_completed())
      .count();
    let abandoned = cohort_sessions
      .iter()
      .filter(|s| s.session.completion_status.is_abandoned())
      .count();
    let completion_bp = calculate_bp(completed, count);

    let c_sum_expl: u64 = cohort_sessions
      .iter()
      .map(|s| u64::from(s.session.explanation_quality_bp))
      .sum();
    let mean_expl = safe_div_u16(c_sum_expl, count);

    let c_sum_debrief: u64 = cohort_sessions
      .iter()
      .map(|s| u64::from(s.session.debrief_comprehension_bp))
      .sum();
    let mean_deb = safe_div_u16(c_sum_debrief, count);

    let total_frictions: usize = cohort_sessions
      .iter()
      .map(|s| s.reported_frictions.len())
      .sum();

    let target_met = completion_bp >= protocol.target_completion_floor_bp
      && mean_deb >= protocol.target_comprehension_floor_bp;

    summaries[i] = CohortTrialSummary {
      cohort: *cohort,
      total_participants: count,
      completed_sessions: completed,
      abandoned_sessions: abandoned,
      completion_rate_bp: completion_bp,
      mean_explanation_bp: mean_expl,
      mean_debrief_bp: mean_deb,
      total_friction_incidents: total_frictions,
      target_met,
    };
  }

  let completion_target_met = overall_completion_rate_bp >= protocol.target_completion_floor_bp;
  let comprehension_target_met = overall_mean_debrief_bp >= protocol.target_comprehension_floor_bp;

  // Accessibility qualification checks
  let access_summary = summaries[2]; // ParticipantCohort::AccessNeeds
  let has_open_accessibility_blocker = findings.iter().any(|f| {
    f.category == FindingCategory::Accessibility
      && f.severity == FindingSeverity::Blocker
      && !f.disposition.is_resolved_or_mitigated()
  });

  let accessibility_qualified = access_summary.completion_rate_bp
    >= protocol.target_completion_floor_bp
    && access_summary.mean_debrief_bp >= protocol.target_comprehension_floor_bp
    && !has_open_accessibility_blocker;

  Ok(EmpiricalCohortTrialReport {
    schema: M10_EMPI_COHORT_TRIALS_SCHEMA_V1,
    protocol: *protocol,
    total_participants,
    total_completed,
    total_abandoned,
    overall_completion_rate_bp,
    overall_mean_explanation_bp,
    overall_mean_debrief_bp,
    accessibility_qualified,
    completion_target_met,
    comprehension_target_met,
    cohort_summaries: summaries,
    findings: findings.to_vec(),
  })
}

fn calculate_bp(numerator: usize, denominator: usize) -> u16 {
  if denominator == 0 {
    return 0;
  }
  let num = u64::try_from(numerator).unwrap_or(0);
  let den = u64::try_from(denominator).unwrap_or(1);
  let bp = (num.saturating_mul(10_000)).checked_div(den).unwrap_or(0);
  u16::try_from(bp.min(10_000)).unwrap_or(10_000)
}

fn safe_div_u16(sum: u64, count: usize) -> u16 {
  if count == 0 {
    return 0;
  }
  let c = u64::try_from(count).unwrap_or(1);
  let avg = sum.checked_div(c).unwrap_or(0);
  u16::try_from(avg.min(10_000)).unwrap_or(10_000)
}

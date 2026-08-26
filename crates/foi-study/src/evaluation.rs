//! Pure deterministic study cohort evaluation and report generation for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Evaluates participant session outcomes, debrief comprehension, finding
//! distributions, and accessibility claims gates over declared study cohorts.
//! Computes exact integer basis points ([0..=10,000] bp) without floating-point math,
//! wall-clock timing, or hidden simulation state.

use core::fmt;

use super::finding::{FindingCategory, FindingRecord, FindingSeverity};
use super::protocol::{ParticipantCohort, StudyProtocolDefinition};
use super::session::ParticipantSessionRecord;

pub const M10_STUDY_EVALUATION_SCHEMA_V1: &str = "m10-study-evaluation-v1";

pub const STANDARD_EVIDENCE_BOUNDARY: &str = "Findings reflect observed human participant evaluations under the declared M10 \
   study protocol; no universal accessibility, commercial readiness, or human-equilibrium \
   validity is claimed.";

/// Error conditions encountered during study cohort evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StudyEvaluationError {
  /// The provided session list was empty.
  EmptyPopulation,
  /// A participant ID was duplicated in the session list.
  DuplicateParticipantId(&'static str),
  /// A finding ID was duplicated in the finding list.
  DuplicateFindingId(&'static str),
  /// A score exceeded the 10,000 bp maximum.
  ScoreOutOfRange {
    participant_id: &'static str,
    score_bp: u16,
  },
  /// A finding referenced a participant ID that was not in the session list.
  UnlinkedFindingParticipant {
    finding_id: &'static str,
    participant_id: &'static str,
  },
  /// The privacy consent declaration was invalid or incomplete.
  InvalidPrivacyDeclaration,
}

impl fmt::Display for StudyEvaluationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyPopulation => f.write_str("study population cannot be empty"),
      Self::DuplicateParticipantId(id) => write!(f, "duplicate participant id: {id}"),
      Self::DuplicateFindingId(id) => write!(f, "duplicate finding id: {id}"),
      Self::ScoreOutOfRange {
        participant_id,
        score_bp,
      } => write!(
        f,
        "score {score_bp} bp exceeds 10000 bp limit for participant {participant_id}"
      ),
      Self::UnlinkedFindingParticipant {
        finding_id,
        participant_id,
      } => write!(
        f,
        "finding {finding_id} references unlinked participant {participant_id}"
      ),
      Self::InvalidPrivacyDeclaration => {
        f.write_str("privacy consent declaration is invalid or incomplete")
      }
    }
  }
}

/// Metrics aggregated for a single participant cohort.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CohortMetrics {
  pub cohort: ParticipantCohort,
  pub participant_count: usize,
  pub completed_count: usize,
  /// Completion rate within this cohort (bp [0..=10,000]).
  pub completion_rate_bp: u16,
  /// Average explanation quality rating within this cohort (bp [0..=10,000]).
  pub avg_explanation_quality_bp: u16,
  /// Average debrief comprehension score within this cohort (bp [0..=10,000]).
  pub avg_debrief_comprehension_bp: u16,
}

/// Aggregated report from evaluating an M10 human usability study cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StudyEvaluationReport {
  pub protocol_id: &'static str,
  pub total_participants: usize,
  pub completed_count: usize,
  pub overall_completion_rate_bp: u16,
  pub overall_avg_explanation_bp: u16,
  pub overall_avg_comprehension_bp: u16,
  pub cohort_metrics: [CohortMetrics; 4],
  pub blocker_count: usize,
  pub major_barrier_count: usize,
  pub minor_friction_count: usize,
  pub positive_insight_count: usize,
  pub unresolved_accessibility_blockers: usize,
  pub unresolved_usability_blockers: usize,
  pub accessibility_claims_qualified: bool,
  pub completion_target_met: bool,
  pub comprehension_target_met: bool,
  pub evidence_boundary_statement: &'static str,
}

impl StudyEvaluationReport {
  /// Render this evaluation report as clean, structured Markdown.
  pub fn to_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# Usability & Accessibility Study Evaluation Report\n\n");
    out.push_str(&format!("**Protocol:** `{}`\n", self.protocol_id));
    out.push_str(&format!(
      "**Participants:** {} total ({} completed, {} bp completion rate)\n",
      self.total_participants, self.completed_count, self.overall_completion_rate_bp
    ));
    out.push_str(&format!(
      "**Overall Scores:** Explanation Quality: {} bp | Debrief Comprehension: {} bp\n\n",
      self.overall_avg_explanation_bp, self.overall_avg_comprehension_bp
    ));

    out.push_str("## Cohort Performance\n\n");
    out.push_str("| Cohort | Count | Completed | Rate (bp) | Expl. (bp) | Debrief (bp) |\n");
    out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
    for m in &self.cohort_metrics {
      out.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} |\n",
        m.cohort,
        m.participant_count,
        m.completed_count,
        m.completion_rate_bp,
        m.avg_explanation_quality_bp,
        m.avg_debrief_comprehension_bp
      ));
    }
    out.push('\n');

    out.push_str("## Finding Breakdown & Disposition\n\n");
    out.push_str(&format!("- **Blockers:** {}\n", self.blocker_count));
    out.push_str(&format!(
      "- **Major Barriers:** {}\n",
      self.major_barrier_count
    ));
    out.push_str(&format!(
      "- **Minor Friction:** {}\n",
      self.minor_friction_count
    ));
    out.push_str(&format!(
      "- **Positive Insights:** {}\n",
      self.positive_insight_count
    ));
    out.push_str(&format!(
      "- **Unresolved Usability Blockers:** {}\n",
      self.unresolved_usability_blockers
    ));
    out.push_str(&format!(
      "- **Unresolved Accessibility Blockers:** {}\n\n",
      self.unresolved_accessibility_blockers
    ));

    out.push_str("## Target Gates\n\n");
    out.push_str(&format!(
      "- Completion Target Met: {}\n",
      if self.completion_target_met {
        "PASS"
      } else {
        "FAIL"
      }
    ));
    out.push_str(&format!(
      "- Comprehension Target Met: {}\n",
      if self.comprehension_target_met {
        "PASS"
      } else {
        "FAIL"
      }
    ));
    out.push_str(&format!(
      "- Accessibility Claims Qualified: {}\n\n",
      if self.accessibility_claims_qualified {
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

/// Truncated basis-point share of `part` over `whole`.
fn share_bp(part: u32, whole: u32) -> u16 {
  u16::try_from(u64::from(part) * 10_000 / u64::from(whole)).expect("share is at most 10,000 bp")
}

/// Average in basis points from a sum and non-zero count.
fn avg_bp(sum: u64, count: u32) -> u16 {
  u16::try_from(sum / u64::from(count)).expect("average is at most 10,000 bp")
}

/// Evaluates an M10 human usability and accessibility study cohort.
pub fn evaluate_study_cohort(
  protocol: &StudyProtocolDefinition,
  sessions: &[ParticipantSessionRecord],
  findings: &[FindingRecord],
) -> Result<StudyEvaluationReport, StudyEvaluationError> {
  if !protocol.privacy_declaration.is_valid() {
    return Err(StudyEvaluationError::InvalidPrivacyDeclaration);
  }
  if sessions.is_empty() {
    return Err(StudyEvaluationError::EmptyPopulation);
  }

  // Validate sessions: duplicate IDs and score bounds.
  for i in 0..sessions.len() {
    let s_i = &sessions[i];
    if s_i.explanation_quality_bp > 10_000 {
      return Err(StudyEvaluationError::ScoreOutOfRange {
        participant_id: s_i.participant_id,
        score_bp: s_i.explanation_quality_bp,
      });
    }
    if s_i.debrief_comprehension_bp > 10_000 {
      return Err(StudyEvaluationError::ScoreOutOfRange {
        participant_id: s_i.participant_id,
        score_bp: s_i.debrief_comprehension_bp,
      });
    }
    for s_j in &sessions[i + 1..] {
      if s_i.participant_id == s_j.participant_id {
        return Err(StudyEvaluationError::DuplicateParticipantId(
          s_i.participant_id,
        ));
      }
    }
  }

  // Validate findings: duplicate IDs and participant linkage.
  for i in 0..findings.len() {
    let f_i = &findings[i];
    let linked = sessions
      .iter()
      .any(|s| s.participant_id == f_i.participant_id);
    if !linked {
      return Err(StudyEvaluationError::UnlinkedFindingParticipant {
        finding_id: f_i.finding_id,
        participant_id: f_i.participant_id,
      });
    }
    for f_j in &findings[i + 1..] {
      if f_i.finding_id == f_j.finding_id {
        return Err(StudyEvaluationError::DuplicateFindingId(f_i.finding_id));
      }
    }
  }

  let total_participants = sessions.len();
  let total_participants_u32 =
    u32::try_from(total_participants).expect("session count fits in u32");
  let completed_count = sessions
    .iter()
    .filter(|s| s.completion_status.is_completed())
    .count();
  let completed_count_u32 = u32::try_from(completed_count).expect("completed count fits in u32");

  let overall_completion_rate_bp = share_bp(completed_count_u32, total_participants_u32);

  let total_expl: u64 = sessions
    .iter()
    .map(|s| u64::from(s.explanation_quality_bp))
    .sum();
  let overall_avg_explanation_bp = avg_bp(total_expl, total_participants_u32);

  let total_comp: u64 = sessions
    .iter()
    .map(|s| u64::from(s.debrief_comprehension_bp))
    .sum();
  let overall_avg_comprehension_bp = avg_bp(total_comp, total_participants_u32);

  // Cohort breakdown
  let mut cohort_metrics = [CohortMetrics {
    cohort: ParticipantCohort::StrategyGamer,
    participant_count: 0,
    completed_count: 0,
    completion_rate_bp: 0,
    avg_explanation_quality_bp: 0,
    avg_debrief_comprehension_bp: 0,
  }; 4];

  let mut access_needs_evaluated = false;
  let mut access_needs_comprehension_passes = true;

  for (idx, &cohort) in ParticipantCohort::ALL.iter().enumerate() {
    let cohort_sessions: Vec<&ParticipantSessionRecord> =
      sessions.iter().filter(|s| s.cohort == cohort).collect();
    let count = cohort_sessions.len();
    if count == 0 {
      cohort_metrics[idx] = CohortMetrics {
        cohort,
        participant_count: 0,
        completed_count: 0,
        completion_rate_bp: 0,
        avg_explanation_quality_bp: 0,
        avg_debrief_comprehension_bp: 0,
      };
    } else {
      let count_u32 = u32::try_from(count).expect("cohort count fits in u32");
      let comp_count = cohort_sessions
        .iter()
        .filter(|s| s.completion_status.is_completed())
        .count();
      let comp_count_u32 = u32::try_from(comp_count).expect("cohort completed count fits in u32");
      let rate_bp = share_bp(comp_count_u32, count_u32);
      let c_expl: u64 = cohort_sessions
        .iter()
        .map(|s| u64::from(s.explanation_quality_bp))
        .sum();
      let c_comp: u64 = cohort_sessions
        .iter()
        .map(|s| u64::from(s.debrief_comprehension_bp))
        .sum();
      let avg_expl = avg_bp(c_expl, count_u32);
      let avg_comp = avg_bp(c_comp, count_u32);

      if cohort == ParticipantCohort::AccessNeeds {
        access_needs_evaluated = true;
        if avg_comp < protocol.target_comprehension_floor_bp {
          access_needs_comprehension_passes = false;
        }
      }

      cohort_metrics[idx] = CohortMetrics {
        cohort,
        participant_count: count,
        completed_count: comp_count,
        completion_rate_bp: rate_bp,
        avg_explanation_quality_bp: avg_expl,
        avg_debrief_comprehension_bp: avg_comp,
      };
    }
  }

  // Findings breakdown
  let mut blocker_count = 0;
  let mut major_barrier_count = 0;
  let mut minor_friction_count = 0;
  let mut positive_insight_count = 0;
  let mut unresolved_accessibility_blockers = 0;
  let mut unresolved_usability_blockers = 0;

  for f in findings {
    match f.severity {
      FindingSeverity::Blocker => {
        blocker_count += 1;
        if f.disposition.is_unresolved_blocker(f.severity) {
          match f.category {
            FindingCategory::Accessibility => unresolved_accessibility_blockers += 1,
            FindingCategory::Usability => unresolved_usability_blockers += 1,
            FindingCategory::GameplayBalance | FindingCategory::BehavioralModel => {}
          }
        }
      }
      FindingSeverity::MajorBarrier => major_barrier_count += 1,
      FindingSeverity::MinorFriction => minor_friction_count += 1,
      FindingSeverity::PositiveInsight => positive_insight_count += 1,
    }
  }

  let accessibility_claims_qualified = access_needs_evaluated
    && unresolved_accessibility_blockers == 0
    && access_needs_comprehension_passes;

  let completion_target_met = overall_completion_rate_bp >= protocol.target_completion_floor_bp;
  let comprehension_target_met =
    overall_avg_comprehension_bp >= protocol.target_comprehension_floor_bp;

  Ok(StudyEvaluationReport {
    protocol_id: protocol.protocol_id,
    total_participants,
    completed_count,
    overall_completion_rate_bp,
    overall_avg_explanation_bp,
    overall_avg_comprehension_bp,
    cohort_metrics,
    blocker_count,
    major_barrier_count,
    minor_friction_count,
    positive_insight_count,
    unresolved_accessibility_blockers,
    unresolved_usability_blockers,
    accessibility_claims_qualified,
    completion_target_met,
    comprehension_target_met,
    evidence_boundary_statement: STANDARD_EVIDENCE_BOUNDARY,
  })
}

//! Participant sampling limits, cohort diversity auditing, and untested population disclosures for M10.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Evaluates participant cohort representation shares in basis points ([0..=10,000] bp),
//! audits declared access-needs distribution, and formalizes explicit disclosures of
//! untested populations to prevent overclaiming usability or accessibility coverage.

use core::fmt;

use super::protocol::ParticipantCohort;
use super::remediation::BP_SCALE;
use super::session::ParticipantSessionRecord;

pub const M10_SAMPLING_LIMITS_SCHEMA_V1: &str = "m10-sampling-limits-v1";

/// Default minimum cohort representation floor (15% = 1,500 bp).
pub const DEFAULT_MIN_COHORT_FLOOR_BP: u16 = 1_500;

/// Categories of specialized user populations explicitly untested in the alpha study.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UntestedPopulationCategory {
  /// Assistive switch or single-button physical access users.
  MotorImpairmentSwitchAccess,
  /// Hardware tactile refreshable Braille display users.
  RefreshableBrailleDisplay,
  /// Non-English speakers or right-to-left (RTL) localization.
  NonEnglishLocale,
  /// Severe cognitive or developmental processing differences.
  SevereCognitiveImpairment,
  /// Mobile touchscreens and responsive handheld viewports.
  MobileTouchInterface,
}

impl UntestedPopulationCategory {
  pub const ALL: [Self; 5] = [
    Self::MotorImpairmentSwitchAccess,
    Self::RefreshableBrailleDisplay,
    Self::NonEnglishLocale,
    Self::SevereCognitiveImpairment,
    Self::MobileTouchInterface,
  ];

  pub const fn as_str(self) -> &'static str {
    match self {
      Self::MotorImpairmentSwitchAccess => "motor-impairment-switch-access",
      Self::RefreshableBrailleDisplay => "refreshable-braille-display",
      Self::NonEnglishLocale => "non-english-locale",
      Self::SevereCognitiveImpairment => "severe-cognitive-impairment",
      Self::MobileTouchInterface => "mobile-touch-interface",
    }
  }

  pub const fn index(self) -> usize {
    match self {
      Self::MotorImpairmentSwitchAccess => 0,
      Self::RefreshableBrailleDisplay => 1,
      Self::NonEnglishLocale => 2,
      Self::SevereCognitiveImpairment => 3,
      Self::MobileTouchInterface => 4,
    }
  }
}

impl fmt::Display for UntestedPopulationCategory {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

/// Explicit disclosure record describing why a population remains untested.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UntestedPopulationDisclosure {
  pub category: UntestedPopulationCategory,
  pub rationale: &'static str,
  pub future_mitigation_plan: &'static str,
}

/// Declaration of sampling parameters and methodology limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SamplingLimitsDeclaration {
  pub declaration_id: &'static str,
  pub methodology: &'static str,
  pub target_sample_size: usize,
  pub min_cohort_floor_bp: u16,
  pub untested_disclosures: &'static [UntestedPopulationDisclosure],
}

impl SamplingLimitsDeclaration {
  pub const fn standard_alpha() -> Self {
    Self {
      declaration_id: "m10-alpha-sampling-limits-v1",
      methodology: "Purposive quota sampling across 4 distinct strategy and accessibility cohorts",
      target_sample_size: 16,
      min_cohort_floor_bp: DEFAULT_MIN_COHORT_FLOOR_BP,
      untested_disclosures: &STANDARD_UNTESTED_DISCLOSURES,
    }
  }
}

pub const STANDARD_UNTESTED_DISCLOSURES: [UntestedPopulationDisclosure; 5] = [
  UntestedPopulationDisclosure {
    category: UntestedPopulationCategory::MotorImpairmentSwitchAccess,
    rationale: "Command-line REPL requires multi-key keyboard input; single-switch scanning is not yet implemented.",
    future_mitigation_plan: "Evaluate single-switch sequential command menus in M11 graphical/terminal adapter.",
  },
  UntestedPopulationDisclosure {
    category: UntestedPopulationCategory::RefreshableBrailleDisplay,
    rationale: "Study sessions tested screen reader speech synthesis (VoiceOver/NVDA), not refreshable tactile pins.",
    future_mitigation_plan: "Conduct dedicated tactile line-length and tabular layout tests in M12 research release.",
  },
  UntestedPopulationDisclosure {
    category: UntestedPopulationCategory::NonEnglishLocale,
    rationale: "All vocabulary, command names, and debrief text are authored in English only.",
    future_mitigation_plan: "Introduce gettext / localized string dictionaries prior to public multi-lingual alpha.",
  },
  UntestedPopulationDisclosure {
    category: UntestedPopulationCategory::SevereCognitiveImpairment,
    rationale: "Turn-based multi-actor uncertainty modeling requires abstract strategic counterfactual reasoning.",
    future_mitigation_plan: "Design simplified tutorial sandbox with constrained 1-actor scenarios for broader accessibility.",
  },
  UntestedPopulationDisclosure {
    category: UntestedPopulationCategory::MobileTouchInterface,
    rationale: "Terminal CLI executable runs in desktop shell environments (macOS/Linux/Windows).",
    future_mitigation_plan: "Evaluate responsive web / mobile canvas UI during M11 GUI milestone.",
  },
];

/// Distribution of declared access needs within the sampled population.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AccessNeedsBreakdown {
  pub screen_reader_users: usize,
  pub color_vision_deficiency_users: usize,
  pub keyboard_only_users: usize,
  pub no_declared_access_needs: usize,
  pub total_with_access_needs: usize,
  /// Share of sample declaring at least one access need (in bp, [0..=10,000]).
  pub access_needs_share_bp: u16,
}

/// Representation metrics for a single participant cohort.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CohortRepresentation {
  pub cohort: ParticipantCohort,
  pub count: usize,
  /// Share of total sample in basis points ([0..=10,000]).
  pub share_bp: u16,
  pub meets_floor: bool,
}

/// Comprehensive report evaluating participant sampling diversity and limits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticipantSamplingReport {
  pub declaration_id: &'static str,
  pub sample_size: usize,
  pub cohort_representations: [CohortRepresentation; 4],
  pub access_needs_breakdown: AccessNeedsBreakdown,
  pub untested_disclosures: &'static [UntestedPopulationDisclosure],
  pub all_cohorts_meet_floor: bool,
  pub has_access_needs_representation: bool,
  pub sampling_gate_passed: bool,
}

impl ParticipantSamplingReport {
  /// Formats a clean, structured Markdown report without private chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::with_capacity(1024);
    out.push_str("# M10 Participant Sampling & Limitations Report\n\n");
    out.push_str(&format!(
      "- **Declaration ID:** `{}`\n",
      self.declaration_id
    ));
    out.push_str(&format!(
      "- **Total Sample Size:** {} participants\n",
      self.sample_size
    ));
    out.push_str(&format!(
      "- **Sampling Gate Passed:** {}\n",
      if self.sampling_gate_passed {
        "YES [PASS]"
      } else {
        "NO [FAIL]"
      }
    ));
    out.push_str(&format!(
      "- **Access Needs Representation:** {} participants ({} bp)\n\n",
      self.access_needs_breakdown.total_with_access_needs,
      self.access_needs_breakdown.access_needs_share_bp
    ));

    out.push_str("## Cohort Representation Breakdown\n\n");
    out.push_str("| Cohort | Count | Share (bp) | Status |\n");
    out.push_str("| :--- | :--- | :--- | :--- |\n");
    for rep in &self.cohort_representations {
      out.push_str(&format!(
        "| `{}` | {} | {} bp | {} |\n",
        rep.cohort.as_str(),
        rep.count,
        rep.share_bp,
        if rep.meets_floor {
          "[OK] Floor Met"
        } else {
          "[WARN] Below Floor"
        }
      ));
    }
    out.push('\n');

    out.push_str("## Access Needs Distribution\n\n");
    out.push_str(&format!(
      "- Screen Reader Users: {}\n",
      self.access_needs_breakdown.screen_reader_users
    ));
    out.push_str(&format!(
      "- Color Vision Deficiency: {}\n",
      self.access_needs_breakdown.color_vision_deficiency_users
    ));
    out.push_str(&format!(
      "- Keyboard-Only Navigators: {}\n",
      self.access_needs_breakdown.keyboard_only_users
    ));
    out.push_str(&format!(
      "- No Declared Access Needs: {}\n\n",
      self.access_needs_breakdown.no_declared_access_needs
    ));

    out.push_str("## Untested Populations & Alpha Claim Boundaries\n\n");
    for disc in self.untested_disclosures {
      out.push_str(&format!("### {}\n", disc.category.as_str()));
      out.push_str(&format!("- **Rationale:** {}\n", disc.rationale));
      out.push_str(&format!(
        "- **Future Mitigation Plan:** {}\n\n",
        disc.future_mitigation_plan
      ));
    }

    out
  }
}

/// Errors encountered when evaluating participant sampling limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SamplingEvaluationError {
  /// The provided session list was empty.
  EmptySessionList,
  /// Methodology description was empty.
  EmptyMethodology,
  /// Untested population disclosures list was empty.
  EmptyUntestedDisclosures,
  /// An untested population category was disclosed multiple times.
  DuplicateUntestedCategory(UntestedPopulationCategory),
  /// Disclosure rationale or mitigation plan was empty.
  EmptyDisclosureText(UntestedPopulationCategory),
}

impl fmt::Display for SamplingEvaluationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptySessionList => f.write_str("participant session list cannot be empty"),
      Self::EmptyMethodology => f.write_str("sampling methodology cannot be empty"),
      Self::EmptyUntestedDisclosures => {
        f.write_str("untested population disclosures list cannot be empty")
      }
      Self::DuplicateUntestedCategory(cat) => write!(f, "duplicate disclosure for category: {cat}"),
      Self::EmptyDisclosureText(cat) => {
        write!(f, "empty rationale or mitigation plan for category: {cat}")
      }
    }
  }
}

/// Deterministically audits participant sampling distribution and generates the sampling report.
pub fn evaluate_participant_sampling(
  declaration: &SamplingLimitsDeclaration,
  sessions: &[ParticipantSessionRecord],
) -> Result<ParticipantSamplingReport, SamplingEvaluationError> {
  if sessions.is_empty() {
    return Err(SamplingEvaluationError::EmptySessionList);
  }
  if declaration.methodology.trim().is_empty() {
    return Err(SamplingEvaluationError::EmptyMethodology);
  }
  if declaration.untested_disclosures.is_empty() {
    return Err(SamplingEvaluationError::EmptyUntestedDisclosures);
  }

  // Check unique untested categories and non-empty text
  let mut seen_mask: u8 = 0;
  for disc in declaration.untested_disclosures {
    if disc.rationale.trim().is_empty() || disc.future_mitigation_plan.trim().is_empty() {
      return Err(SamplingEvaluationError::EmptyDisclosureText(disc.category));
    }
    let bit = 1u8 << disc.category.index();
    if seen_mask & bit != 0 {
      return Err(SamplingEvaluationError::DuplicateUntestedCategory(
        disc.category,
      ));
    }
    seen_mask |= bit;
  }

  let total_count = sessions.len();

  let mut sr_users = 0usize;
  let mut cvd_users = 0usize;
  let mut kb_users = 0usize;
  let mut none_users = 0usize;
  let mut total_access_needs = 0usize;

  for session in sessions {
    let needs = session.access_needs;
    if !needs.has_any_need() {
      none_users = none_users.saturating_add(1);
    } else {
      total_access_needs = total_access_needs.saturating_add(1);
      if needs.screen_reader_user {
        sr_users = sr_users.saturating_add(1);
      }
      if needs.color_vision_deficiency {
        cvd_users = cvd_users.saturating_add(1);
      }
      if needs.keyboard_only_user {
        kb_users = kb_users.saturating_add(1);
      }
    }
  }

  let bp_scale_usize = usize::from(BP_SCALE);

  let mut all_cohorts_meet_floor = true;
  let mut representations = [CohortRepresentation {
    cohort: ParticipantCohort::StrategyGamer,
    count: 0,
    share_bp: 0,
    meets_floor: false,
  }; 4];

  for (i, &cohort) in ParticipantCohort::ALL.iter().enumerate() {
    let count = sessions.iter().filter(|s| s.cohort == cohort).count();
    let product = count.saturating_mul(bp_scale_usize);
    let share_bp = product
      .checked_div(total_count)
      .and_then(|v| u16::try_from(v).ok())
      .unwrap_or(0);
    let meets_floor = share_bp >= declaration.min_cohort_floor_bp;
    if !meets_floor {
      all_cohorts_meet_floor = false;
    }
    representations[i] = CohortRepresentation {
      cohort,
      count,
      share_bp,
      meets_floor,
    };
  }

  let access_product = total_access_needs.saturating_mul(bp_scale_usize);
  let access_needs_share_bp = access_product
    .checked_div(total_count)
    .and_then(|v| u16::try_from(v).ok())
    .unwrap_or(0);

  let has_access_needs_representation =
    total_access_needs > 0 && access_needs_share_bp >= declaration.min_cohort_floor_bp;
  let sampling_gate_passed = all_cohorts_meet_floor && has_access_needs_representation;

  Ok(ParticipantSamplingReport {
    declaration_id: declaration.declaration_id,
    sample_size: total_count,
    cohort_representations: representations,
    access_needs_breakdown: AccessNeedsBreakdown {
      screen_reader_users: sr_users,
      color_vision_deficiency_users: cvd_users,
      keyboard_only_users: kb_users,
      no_declared_access_needs: none_users,
      total_with_access_needs: total_access_needs,
      access_needs_share_bp,
    },
    untested_disclosures: declaration.untested_disclosures,
    all_cohorts_meet_floor,
    has_access_needs_representation,
    sampling_gate_passed,
  })
}

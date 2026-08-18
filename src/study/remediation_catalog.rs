//! Canonical benchmark scenarios for M10 informal check and remediation evaluation.
//!
//! Milestone: M10 — Human Usability and Accessibility Alpha
//!
//! Registers 3 canonical remediation scenarios:
//! 1. `scenario-remediation-alpha-baseline-v1`: Initial onboarding and status tag friction resolution (100% verified).
//! 2. `scenario-remediation-accessibility-priority-v1`: Screen-reader and non-color semantics priority remediations (100% verified).
//! 3. `scenario-remediation-mixed-progress-v1`: Work-in-progress remediation with pending study items (fails readiness gate).

use super::informal_check::{
  InformalCheckMode, InformalCheckPhase, InformalCheckSession, IssueLinkedNote, NoteDisposition,
};
use super::protocol::EvaluationDimension;
use super::remediation::{
  RemediationAction, RemediationEvaluationError, RemediationEvaluationReport, RemediationTarget,
  RemediationVerificationStatus, evaluate_remediation_plan,
};

pub const M10_REMEDIATION_CATALOG_SCHEMA_V1: &str = "m10-remediation-catalog-v1";

/// Definition of a benchmark remediation scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RemediationScenarioDefinition {
  pub scenario_id: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub sessions: &'static [InformalCheckSession],
  pub actions: &'static [RemediationAction],
  pub expected_total_notes: usize,
  pub expected_total_actions: usize,
  pub expected_addressed_notes_share_bp: u16,
  pub expected_verified_share_bp: u16,
  pub expected_readiness_gate: bool,
}

impl RemediationScenarioDefinition {
  /// Execute this benchmark scenario and verify all expectations.
  pub fn execute(&self) -> Result<RemediationScenarioExecutionResult, RemediationEvaluationError> {
    let report = evaluate_remediation_plan(self.sessions, self.actions)?;
    let expectations_met = report.total_notes == self.expected_total_notes
      && report.total_actions == self.expected_total_actions
      && report.addressed_notes_share_bp == self.expected_addressed_notes_share_bp
      && report.verified_actions_share_bp == self.expected_verified_share_bp
      && report.remediation_readiness_gate_passed == self.expected_readiness_gate;

    Ok(RemediationScenarioExecutionResult {
      scenario_id: self.scenario_id,
      report,
      expectations_met,
    })
  }
}

/// Result of executing a remediation benchmark scenario.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemediationScenarioExecutionResult {
  pub scenario_id: &'static str,
  pub report: RemediationEvaluationReport,
  pub expectations_met: bool,
}

// ---------------------------------------------------------------------------
// Scenario 1: Alpha Baseline Remediation
// ---------------------------------------------------------------------------

static ALPHA_BASELINE_SESSIONS: [InformalCheckSession; 3] = [
  InformalCheckSession {
    session_id: "check-sess-001",
    tester_id: "tester-alpha-01",
    check_mode: InformalCheckMode::InteractiveTty,
    notes: &[
      IssueLinkedNote {
        note_id: "note-001",
        issue_ref: "ISSUE-101",
        phase: InformalCheckPhase::InitialOnboarding,
        dimension: EvaluationDimension::CommandDiscoverability,
        observation: "Initial help prompt unclear on available intent verbs without typing help",
        disposition: NoteDisposition::AddressedInCode,
      },
      IssueLinkedNote {
        note_id: "note-002",
        issue_ref: "ISSUE-102",
        phase: InformalCheckPhase::TurnDecisionMaking,
        dimension: EvaluationDimension::TerminologyClarity,
        observation: "Stabilize vs Contest intent distinction clear after first turn",
        disposition: NoteDisposition::ClarifiedInDoc,
      },
    ],
  },
  InformalCheckSession {
    session_id: "check-sess-002",
    tester_id: "tester-alpha-02",
    check_mode: InformalCheckMode::PipedStream,
    notes: &[
      IssueLinkedNote {
        note_id: "note-003",
        issue_ref: "ISSUE-103",
        phase: InformalCheckPhase::ContingencyPlanning,
        dimension: EvaluationDimension::PerceivedAgency,
        observation: "Abort condition options easy to select and verify via prompt",
        disposition: NoteDisposition::AddressedInCode,
      },
      IssueLinkedNote {
        note_id: "note-004",
        issue_ref: "ISSUE-104",
        phase: InformalCheckPhase::DebriefAnalysis,
        dimension: EvaluationDimension::DebriefCausalUtility,
        observation: "Decoupled coordination attribution highlights teammate impact clearly",
        disposition: NoteDisposition::AddressedInCode,
      },
    ],
  },
  InformalCheckSession {
    session_id: "check-sess-003",
    tester_id: "tester-alpha-03",
    check_mode: InformalCheckMode::InteractiveTty,
    notes: &[
      IssueLinkedNote {
        note_id: "note-005",
        issue_ref: "ISSUE-105",
        phase: InformalCheckPhase::TurnDecisionMaking,
        dimension: EvaluationDimension::PacingLoad,
        observation: "Turn progression comfortable for strategy gamer pacing",
        disposition: NoteDisposition::LoggedForStudy,
      },
      IssueLinkedNote {
        note_id: "note-006",
        issue_ref: "ISSUE-106",
        phase: InformalCheckPhase::DebriefAnalysis,
        dimension: EvaluationDimension::DelegatedFairness,
        observation: "Delegated execution outcome feels predictable given posture",
        disposition: NoteDisposition::LoggedForStudy,
      },
    ],
  },
];

static ALPHA_BASELINE_ACTIONS: [RemediationAction; 4] = [
  RemediationAction {
    action_id: "act-001",
    note_ref: "note-001",
    target: RemediationTarget::CommandVocabulary,
    dimension: EvaluationDimension::CommandDiscoverability,
    description: "Add tab completion suggestions for all legal intent verbs in REPL prompt",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 3_000,
  },
  RemediationAction {
    action_id: "act-002",
    note_ref: "note-002",
    target: RemediationTarget::DocumentationOnboarding,
    dimension: EvaluationDimension::TerminologyClarity,
    description: "Expand Quickstart guide with concise intent contrast table",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 2_500,
  },
  RemediationAction {
    action_id: "act-003",
    note_ref: "note-003",
    target: RemediationTarget::ContingencyAffordance,
    dimension: EvaluationDimension::PerceivedAgency,
    description: "Enforce explicit abort condition confirmation in draft staging",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 2_000,
  },
  RemediationAction {
    action_id: "act-004",
    note_ref: "note-004",
    target: RemediationTarget::DebriefExplanation,
    dimension: EvaluationDimension::DebriefCausalUtility,
    description: "Format causal attribution quadrant with explicit integer basis points",
    verification: RemediationVerificationStatus::ValidatedInStudyCohort,
    expected_impact_bp: 3_500,
  },
];

// ---------------------------------------------------------------------------
// Scenario 2: Accessibility Priority Remediation
// ---------------------------------------------------------------------------

static ACCESSIBILITY_PRIORITY_SESSIONS: [InformalCheckSession; 2] = [
  InformalCheckSession {
    session_id: "check-sess-access-01",
    tester_id: "tester-access-01",
    check_mode: InformalCheckMode::AssistedScreenReader,
    notes: &[
      IssueLinkedNote {
        note_id: "note-acc-001",
        issue_ref: "ISSUE-201",
        phase: InformalCheckPhase::InitialOnboarding,
        dimension: EvaluationDimension::ScreenReaderSuitability,
        observation: "ASCII map rendering disorients screen reader linear traversal",
        disposition: NoteDisposition::AddressedInCode,
      },
      IssueLinkedNote {
        note_id: "note-acc-002",
        issue_ref: "ISSUE-202",
        phase: InformalCheckPhase::TurnDecisionMaking,
        dimension: EvaluationDimension::NonColorSemantics,
        observation: "Color-only health status requires explicit bracketed tags like [OK] and [WARN]",
        disposition: NoteDisposition::AddressedInCode,
      },
    ],
  },
  InformalCheckSession {
    session_id: "check-sess-access-02",
    tester_id: "tester-access-02",
    check_mode: InformalCheckMode::InteractiveTty,
    notes: &[
      IssueLinkedNote {
        note_id: "note-acc-003",
        issue_ref: "ISSUE-203",
        phase: InformalCheckPhase::ContingencyPlanning,
        dimension: EvaluationDimension::KeyboardFlow,
        observation: "All choices accessible via keyboard without requiring mouse clicks",
        disposition: NoteDisposition::AddressedInCode,
      },
      IssueLinkedNote {
        note_id: "note-acc-004",
        issue_ref: "ISSUE-204",
        phase: InformalCheckPhase::DebriefAnalysis,
        dimension: EvaluationDimension::ScreenReaderSuitability,
        observation: "Screen reader debrief text flows linearly with standard markdown headers",
        disposition: NoteDisposition::LoggedForStudy,
      },
    ],
  },
];

static ACCESSIBILITY_PRIORITY_ACTIONS: [RemediationAction; 3] = [
  RemediationAction {
    action_id: "act-acc-001",
    note_ref: "note-acc-001",
    target: RemediationTarget::PresentationOutput,
    dimension: EvaluationDimension::ScreenReaderSuitability,
    description: "Replace bare ASCII art with structured plain-text location lists under NoColor",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 4_500,
  },
  RemediationAction {
    action_id: "act-acc-002",
    note_ref: "note-acc-002",
    target: RemediationTarget::PresentationOutput,
    dimension: EvaluationDimension::NonColorSemantics,
    description: "Add symbolic status indicators [OK], [WARN], [CRIT] alongside colored text",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 4_000,
  },
  RemediationAction {
    action_id: "act-acc-003",
    note_ref: "note-acc-003",
    target: RemediationTarget::CommandVocabulary,
    dimension: EvaluationDimension::KeyboardFlow,
    description: "Standardize keyboard navigation affordances and command aliases",
    verification: RemediationVerificationStatus::ValidatedInStudyCohort,
    expected_impact_bp: 3_000,
  },
];

// ---------------------------------------------------------------------------
// Scenario 3: Mixed Progress Remediation
// ---------------------------------------------------------------------------

static MIXED_PROGRESS_SESSIONS: [InformalCheckSession; 2] = [
  InformalCheckSession {
    session_id: "check-sess-mixed-01",
    tester_id: "tester-mixed-01",
    check_mode: InformalCheckMode::InteractiveTty,
    notes: &[
      IssueLinkedNote {
        note_id: "note-mix-001",
        issue_ref: "ISSUE-301",
        phase: InformalCheckPhase::InitialOnboarding,
        dimension: EvaluationDimension::Onboarding,
        observation: "Tutorial introduction feels overly long for experienced MOBA players",
        disposition: NoteDisposition::AddressedInCode,
      },
      IssueLinkedNote {
        note_id: "note-mix-002",
        issue_ref: "ISSUE-302",
        phase: InformalCheckPhase::TurnDecisionMaking,
        dimension: EvaluationDimension::PacingLoad,
        observation: "Turn timers feel restrictive when evaluating complex 3-lane states",
        disposition: NoteDisposition::WontFixWithRationale,
      },
      IssueLinkedNote {
        note_id: "note-mix-003",
        issue_ref: "ISSUE-303",
        phase: InformalCheckPhase::ContingencyPlanning,
        dimension: EvaluationDimension::PerceivedAgency,
        observation: "Contingency trigger rules need additional explanatory feedback",
        disposition: NoteDisposition::ClarifiedInDoc,
      },
    ],
  },
  InformalCheckSession {
    session_id: "check-sess-mixed-02",
    tester_id: "tester-mixed-02",
    check_mode: InformalCheckMode::PipedStream,
    notes: &[
      IssueLinkedNote {
        note_id: "note-mix-004",
        issue_ref: "ISSUE-304",
        phase: InformalCheckPhase::DebriefAnalysis,
        dimension: EvaluationDimension::DebriefCausalUtility,
        observation: "Debrief requires more granular breakdown of objective trades",
        disposition: NoteDisposition::LoggedForStudy,
      },
      IssueLinkedNote {
        note_id: "note-mix-005",
        issue_ref: "ISSUE-305",
        phase: InformalCheckPhase::TurnDecisionMaking,
        dimension: EvaluationDimension::CommandDiscoverability,
        observation: "Multi-parameter commands benefit from auto-suggest parameter hints",
        disposition: NoteDisposition::LoggedForStudy,
      },
    ],
  },
];

static MIXED_PROGRESS_ACTIONS: [RemediationAction; 4] = [
  RemediationAction {
    action_id: "act-mix-001",
    note_ref: "note-mix-001",
    target: RemediationTarget::DocumentationOnboarding,
    dimension: EvaluationDimension::Onboarding,
    description: "Provide quickstart skip option for veteran MOBA strategy players",
    verification: RemediationVerificationStatus::VerifiedInRegression,
    expected_impact_bp: 2_000,
  },
  RemediationAction {
    action_id: "act-mix-002",
    note_ref: "note-mix-003",
    target: RemediationTarget::DocumentationOnboarding,
    dimension: EvaluationDimension::PerceivedAgency,
    description: "Document contingency trigger condition mechanics in reference guide",
    verification: RemediationVerificationStatus::PendingImplementation,
    expected_impact_bp: 1_500,
  },
  RemediationAction {
    action_id: "act-mix-003",
    note_ref: "note-mix-004",
    target: RemediationTarget::DebriefExplanation,
    dimension: EvaluationDimension::DebriefCausalUtility,
    description: "Enhance debrief with objective trade counterfactual deltas",
    verification: RemediationVerificationStatus::PendingImplementation,
    expected_impact_bp: 2_500,
  },
  RemediationAction {
    action_id: "act-mix-004",
    note_ref: "note-mix-005",
    target: RemediationTarget::CommandVocabulary,
    dimension: EvaluationDimension::CommandDiscoverability,
    description: "Add parameter hint completions to REPL command loop",
    verification: RemediationVerificationStatus::PendingImplementation,
    expected_impact_bp: 1_800,
  },
];

// ---------------------------------------------------------------------------
// Remediation Catalog Definition
// ---------------------------------------------------------------------------

pub struct RemediationCatalog;

impl RemediationCatalog {
  pub const SCENARIO_ALPHA_BASELINE_V1: &'static str = "scenario-remediation-alpha-baseline-v1";
  pub const SCENARIO_ACCESSIBILITY_PRIORITY_V1: &'static str =
    "scenario-remediation-accessibility-priority-v1";
  pub const SCENARIO_MIXED_PROGRESS_V1: &'static str = "scenario-remediation-mixed-progress-v1";

  pub const ALL: [RemediationScenarioDefinition; 3] = [
    RemediationScenarioDefinition {
      scenario_id: Self::SCENARIO_ALPHA_BASELINE_V1,
      title: "Alpha Baseline Remediation Benchmark",
      description: "Complete remediation of initial onboarding and command discoverability frictions with 100% verified actions.",
      sessions: &ALPHA_BASELINE_SESSIONS,
      actions: &ALPHA_BASELINE_ACTIONS,
      expected_total_notes: 6,
      expected_total_actions: 4,
      // 4 addressed out of 6 notes = 6,666 bp
      expected_addressed_notes_share_bp: 6_666,
      // 4 verified out of 4 actions = 10,000 bp
      expected_verified_share_bp: 10_000,
      expected_readiness_gate: true,
    },
    RemediationScenarioDefinition {
      scenario_id: Self::SCENARIO_ACCESSIBILITY_PRIORITY_V1,
      title: "Accessibility Priority Remediation Benchmark",
      description: "Screen-reader and non-color semantics priority remediations with high impact and 100% verified actions.",
      sessions: &ACCESSIBILITY_PRIORITY_SESSIONS,
      actions: &ACCESSIBILITY_PRIORITY_ACTIONS,
      expected_total_notes: 4,
      expected_total_actions: 3,
      // 3 addressed out of 4 notes = 7,500 bp
      expected_addressed_notes_share_bp: 7_500,
      // 3 verified out of 3 actions = 10,000 bp
      expected_verified_share_bp: 10_000,
      expected_readiness_gate: true,
    },
    RemediationScenarioDefinition {
      scenario_id: Self::SCENARIO_MIXED_PROGRESS_V1,
      title: "Mixed Progress Remediation Benchmark",
      description: "Work-in-progress remediation with 3 pending actions, failing the 50% verified readiness gate.",
      sessions: &MIXED_PROGRESS_SESSIONS,
      actions: &MIXED_PROGRESS_ACTIONS,
      expected_total_notes: 5,
      expected_total_actions: 4,
      // 2 addressed out of 5 notes = 4,000 bp
      expected_addressed_notes_share_bp: 4_000,
      // 1 verified out of 4 actions = 2,500 bp
      expected_verified_share_bp: 2_500,
      expected_readiness_gate: false,
    },
  ];

  pub fn all_scenarios() -> &'static [RemediationScenarioDefinition] {
    &Self::ALL
  }

  pub fn find_scenario(id: &str) -> Option<&'static RemediationScenarioDefinition> {
    Self::ALL.iter().find(|s| s.scenario_id == id)
  }

  pub fn execute_scenario(
    id: &str,
  ) -> Result<RemediationScenarioExecutionResult, RemediationEvaluationError> {
    let scenario = Self::find_scenario(id).ok_or(RemediationEvaluationError::EmptySessionList)?;
    scenario.execute()
  }
}

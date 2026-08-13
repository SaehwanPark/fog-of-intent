//! Private submissions and simultaneous resolution in multi-agent team decision windows.
//!
//! This module defines how autonomous teammates privately formulate their decisions
//! (intents, communication, individual plans) without leaking uncommitted state to peers,
//! and how the host deterministically evaluates simultaneous multi-agent resolution across
//! leadership structures, team-plan alignments, trust matrices, and communication channels.

use core::fmt;

use crate::agent::communication::{TeamCommunicationError, TeamMessageEnvelope};
use crate::agent::leadership::{
  LeadershipEvaluationReport, LeadershipStructure, PeerPlanProposal, ShotCallerDirective,
  TeamLeadershipError, TeamLeadershipEvaluator,
};
use crate::agent::team_plan::{
  AlignmentEvaluation, IndividualPlanDefinition, TeamPlanAlignmentType, TeamPlanDefinition,
  TeamPlanError, TeamPlanEvaluator,
};
use crate::agent::trust::{
  CommunicationClarity, TeamTrustError, TeamTrustEvaluator, TeamTrustMatrix,
  TrustComplianceDecision,
};
use crate::lane::{
  LaneActorRole, LaneCommitment, LaneIntent, LanePingSignal, LaneTargetFocus, LanerObservation,
};

/// Versioned schema for team simultaneous submission envelopes.
pub const TEAM_SIMULTANEOUS_SUBMISSION_SCHEMA: &str = "m8-team-simultaneous-submission-v1";

/// Versioned schema for team simultaneous resolution reports.
pub const TEAM_SIMULTANEOUS_RESOLUTION_SCHEMA: &str = "m8-team-simultaneous-resolution-v1";

/// Versioned schema for the team simultaneous catalog.
pub const TEAM_SIMULTANEOUS_CATALOG_SCHEMA: &str = "m8-team-simultaneous-catalog-v1";

/// Maximum number of participating roles supported in a single simultaneous window.
pub const MAX_ROLES_IN_SIMULTANEOUS_WINDOW: usize = 4;

/// Basis points upper bound ($10,000$ bp = 100%).
pub const MAX_COHESION_BP: u32 = 10_000;

/// High cohesion threshold for fully-coordinated resolutions (75% = 7,500 bp).
pub const FULL_COHESION_THRESHOLD_BP: u32 = 7_500;

/// Moderate cohesion threshold for partially-coordinated resolutions (50% = 5,000 bp).
pub const PARTIAL_COHESION_THRESHOLD_BP: u32 = 5_000;

/// Low cohesion threshold for divergent-intents resolutions (25% = 2,500 bp).
pub const DIVERGENT_COHESION_THRESHOLD_BP: u32 = 2_500;

/// Minimal cohesion threshold for conflicting-directives resolutions (10% = 1,000 bp).
pub const CONFLICTING_COHESION_THRESHOLD_BP: u32 = 1_000;

/// Typed errors emitted during team simultaneous submission collection and resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TeamSimultaneousError {
  /// Safety violation: private chain-of-thought was detected or requested.
  ChainOfThoughtForbidden,
  /// Submission attempted on a closed window.
  Closed,
  /// Submission observation ID did not match the window's expected observation ID.
  StaleObservation {
    /// Observation ID attached to the submission.
    submitted: u64,
    /// Observation ID expected by the window.
    expected: u64,
  },
  /// Submission role was not registered for this simultaneous window.
  RoleNotRegistered(LaneActorRole),
  /// Duplicate submission from the same actor role.
  DuplicateSubmission(LaneActorRole),
  /// Window is not yet ready for resolution (awaiting further submissions).
  NotReady,
  /// Missing submission from a required registered actor role.
  MissingSubmission(LaneActorRole),
  /// Registered roles list was empty or invalid.
  InvalidRoleCount,
  /// Duplicate role in window registration.
  DuplicateRegisteredRole(LaneActorRole),
  /// Basis point value exceeded the maximum allowed bound ($10,000$ bp).
  BasisPointOutOfRange {
    /// Supplied basis point value.
    bp: u32,
    /// Maximum allowed basis point value.
    max: u32,
  },
  /// Simultaneous scenario was not found in the catalog.
  CatalogScenarioNotFound(&'static str),
  /// Underlying team communication protocol error.
  CommunicationError(TeamCommunicationError),
  /// Underlying team trust evaluation error.
  TrustError(TeamTrustError),
  /// Underlying team leadership evaluation error.
  LeadershipError(TeamLeadershipError),
  /// Underlying team plan evaluation error.
  PlanError(TeamPlanError),
}

impl fmt::Display for TeamSimultaneousError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::ChainOfThoughtForbidden => {
        write!(
          f,
          "private chain-of-thought is strictly forbidden in simultaneous submission contracts"
        )
      }
      Self::Closed => write!(f, "simultaneous window is closed to submissions"),
      Self::StaleObservation {
        submitted,
        expected,
      } => {
        write!(
          f,
          "stale observation in submission: submitted {submitted}, expected {expected}"
        )
      }
      Self::RoleNotRegistered(role) => {
        write!(
          f,
          "actor role `{}` is not registered in this simultaneous window",
          role.as_str()
        )
      }
      Self::DuplicateSubmission(role) => {
        write!(
          f,
          "duplicate submission received for actor role `{}`",
          role.as_str()
        )
      }
      Self::NotReady => {
        write!(
          f,
          "simultaneous window is not ready for resolution (awaiting submissions)"
        )
      }
      Self::MissingSubmission(role) => {
        write!(
          f,
          "missing submission from registered role `{}`",
          role.as_str()
        )
      }
      Self::InvalidRoleCount => {
        write!(
          f,
          "registered role count must be between 1 and {MAX_ROLES_IN_SIMULTANEOUS_WINDOW}"
        )
      }
      Self::DuplicateRegisteredRole(role) => {
        write!(
          f,
          "duplicate role `{}` registered in simultaneous window",
          role.as_str()
        )
      }
      Self::BasisPointOutOfRange { bp, max } => {
        write!(f, "basis points {bp} exceeded maximum allowed bound {max}")
      }
      Self::CatalogScenarioNotFound(id) => {
        write!(f, "simultaneous scenario `{id}` not found in catalog")
      }
      Self::CommunicationError(err) => write!(f, "communication error: {err:?}"),
      Self::TrustError(err) => write!(f, "trust error: {err}"),
      Self::LeadershipError(err) => write!(f, "leadership error: {err}"),
      Self::PlanError(err) => write!(f, "team plan error: {err}"),
    }
  }
}

impl core::error::Error for TeamSimultaneousError {}

impl From<TeamCommunicationError> for TeamSimultaneousError {
  fn from(err: TeamCommunicationError) -> Self {
    Self::CommunicationError(err)
  }
}

impl From<TeamTrustError> for TeamSimultaneousError {
  fn from(err: TeamTrustError) -> Self {
    Self::TrustError(err)
  }
}

impl From<TeamLeadershipError> for TeamSimultaneousError {
  fn from(err: TeamLeadershipError) -> Self {
    Self::LeadershipError(err)
  }
}

impl From<TeamPlanError> for TeamSimultaneousError {
  fn from(err: TeamPlanError) -> Self {
    Self::PlanError(err)
  }
}

/// Lifecycle phase of a simultaneous decision window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TeamSimultaneousPhase {
  /// Window is open and accepting private submissions from registered roles.
  CollectingSubmissions,
  /// All registered roles have submitted; window is ready for simultaneous resolution.
  Ready,
  /// Submissions have been simultaneously evaluated and resolved.
  Resolved,
  /// Window is closed; no further submissions or mutations are accepted.
  Closed,
}

impl TeamSimultaneousPhase {
  /// Return canonical string label.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::CollectingSubmissions => "collecting-submissions",
      Self::Ready => "ready",
      Self::Resolved => "resolved",
      Self::Closed => "closed",
    }
  }

  /// Parse lifecycle phase from canonical string label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "collecting-submissions" => Some(Self::CollectingSubmissions),
      "ready" => Some(Self::Ready),
      "resolved" => Some(Self::Resolved),
      "closed" => Some(Self::Closed),
      _ => None,
    }
  }
}

/// Discrete coordination outcome resulting from simultaneous multi-actor evaluation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TeamCoordinationOutcome {
  /// All roles align on the active team plan and directives with high cohesion ($>= 7,500$ bp).
  FullyCoordinated,
  /// Roles align on the primary objective but differ in tactical posture ($5,000$ to $7,499$ bp).
  PartiallyCoordinated,
  /// Roles choose conflicting individual intents without consensus ($2,500$ to $4,999$ bp).
  DivergentIntents,
  /// Contradictory directives from multiple callers cause coordination deadlock ($1,000$ to $2,499$ bp).
  ConflictingDirectives,
  /// Critical messages lost in transmission or timeout resulting in uncoordinated fallback ($< 1,000$ bp).
  CommunicationFailure,
}

impl TeamCoordinationOutcome {
  /// Return canonical string label.
  pub const fn as_str(self) -> &'static str {
    match self {
      Self::FullyCoordinated => "fully-coordinated",
      Self::PartiallyCoordinated => "partially-coordinated",
      Self::DivergentIntents => "divergent-intents",
      Self::ConflictingDirectives => "conflicting-directives",
      Self::CommunicationFailure => "communication-failure",
    }
  }

  /// Parse coordination outcome from canonical string label.
  pub fn parse(label: &str) -> Option<Self> {
    match label {
      "fully-coordinated" => Some(Self::FullyCoordinated),
      "partially-coordinated" => Some(Self::PartiallyCoordinated),
      "divergent-intents" => Some(Self::DivergentIntents),
      "conflicting-directives" => Some(Self::ConflictingDirectives),
      "communication-failure" => Some(Self::CommunicationFailure),
      _ => None,
    }
  }

  /// Return all coordination outcomes in descending cohesion order.
  pub const fn all() -> [Self; 5] {
    [
      Self::FullyCoordinated,
      Self::PartiallyCoordinated,
      Self::DivergentIntents,
      Self::ConflictingDirectives,
      Self::CommunicationFailure,
    ]
  }
}

/// Private submission envelope containing an actor's chosen intent, tactical parameters, and communicative acts.
#[derive(Clone, Eq, PartialEq)]
pub struct TeamSubmissionEnvelope {
  schema: &'static str,
  role: LaneActorRole,
  observation_id: u64,
  turn: u32,
  intent: LaneIntent,
  target_focus: LaneTargetFocus,
  commitment: LaneCommitment,
  ping_signal: LanePingSignal,
  staged_message: Option<TeamMessageEnvelope>,
  individual_plan: Option<IndividualPlanDefinition>,
  chain_of_thought_present: bool,
}

impl fmt::Debug for TeamSubmissionEnvelope {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TeamSubmissionEnvelope")
      .field("schema", &self.schema)
      .field("role", &self.role)
      .field("observation_id", &self.observation_id)
      .field("turn", &self.turn)
      .field("intent", &self.intent)
      .field("target_focus", &self.target_focus)
      .field("commitment", &self.commitment)
      .field("ping_signal", &self.ping_signal)
      .field("has_message", &self.staged_message.is_some())
      .field("has_plan", &self.individual_plan.is_some())
      .field("chain_of_thought_present", &self.chain_of_thought_present)
      .finish()
  }
}

impl TeamSubmissionEnvelope {
  /// Create a new private submission envelope with fail-closed chain-of-thought checking.
  #[expect(
    clippy::too_many_arguments,
    reason = "private submission envelope captures full multi-field actor intent"
  )]
  pub fn new(
    role: LaneActorRole,
    observation_id: u64,
    turn: u32,
    intent: LaneIntent,
    target_focus: LaneTargetFocus,
    commitment: LaneCommitment,
    ping_signal: LanePingSignal,
    staged_message: Option<TeamMessageEnvelope>,
    individual_plan: Option<IndividualPlanDefinition>,
    chain_of_thought_present: bool,
  ) -> Result<Self, TeamSimultaneousError> {
    if chain_of_thought_present {
      return Err(TeamSimultaneousError::ChainOfThoughtForbidden);
    }
    if let Some(msg) = &staged_message
      && msg.chain_of_thought_present()
    {
      return Err(TeamSimultaneousError::ChainOfThoughtForbidden);
    }
    if let Some(plan) = &individual_plan
      && plan.chain_of_thought_present
    {
      return Err(TeamSimultaneousError::ChainOfThoughtForbidden);
    }
    Ok(Self {
      schema: TEAM_SIMULTANEOUS_SUBMISSION_SCHEMA,
      role,
      observation_id,
      turn,
      intent,
      target_focus,
      commitment,
      ping_signal,
      staged_message,
      individual_plan,
      chain_of_thought_present: false,
    })
  }

  /// Return schema string.
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Return actor role.
  pub const fn role(&self) -> LaneActorRole {
    self.role
  }

  /// Return bound observation ID.
  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  /// Return bound turn index.
  pub const fn turn(&self) -> u32 {
    self.turn
  }

  /// Return chosen intent.
  pub const fn intent(&self) -> LaneIntent {
    self.intent
  }

  /// Return chosen target focus.
  pub const fn target_focus(&self) -> LaneTargetFocus {
    self.target_focus
  }

  /// Return chosen commitment level.
  pub const fn commitment(&self) -> LaneCommitment {
    self.commitment
  }

  /// Return chosen ping signal.
  pub const fn ping_signal(&self) -> LanePingSignal {
    self.ping_signal
  }

  /// Return staged message envelope if present.
  pub const fn staged_message(&self) -> Option<&TeamMessageEnvelope> {
    self.staged_message.as_ref()
  }

  /// Return staged individual plan if present.
  pub const fn individual_plan(&self) -> Option<&IndividualPlanDefinition> {
    self.individual_plan.as_ref()
  }

  /// Return whether chain of thought is present.
  pub const fn chain_of_thought_present(&self) -> bool {
    self.chain_of_thought_present
  }
}

/// Payload-free receipt confirming acceptance of an actor's private submission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TeamSubmissionReceipt {
  role: LaneActorRole,
  observation_id: u64,
  turn: u32,
  accepted: bool,
  message_staged: bool,
  plan_staged: bool,
  chain_of_thought_free: bool,
}

impl TeamSubmissionReceipt {
  /// Return actor role.
  pub const fn role(self) -> LaneActorRole {
    self.role
  }

  /// Return observation ID.
  pub const fn observation_id(self) -> u64 {
    self.observation_id
  }

  /// Return turn index.
  pub const fn turn(self) -> u32 {
    self.turn
  }

  /// Return whether the submission was accepted.
  pub const fn accepted(self) -> bool {
    self.accepted
  }

  /// Return whether a message was staged.
  pub const fn message_staged(self) -> bool {
    self.message_staged
  }

  /// Return whether an individual plan was staged.
  pub const fn plan_staged(self) -> bool {
    self.plan_staged
  }

  /// Return whether chain-of-thought was verified absent.
  pub const fn chain_of_thought_free(self) -> bool {
    self.chain_of_thought_free
  }
}

/// Bounded multi-agent simultaneous submission collection window.
#[derive(Clone, Eq, PartialEq)]
pub struct TeamSimultaneousWindow {
  schema: &'static str,
  observation_id: u64,
  turn: u32,
  registered_roles: [Option<LaneActorRole>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW],
  registered_count: usize,
  submissions: [Option<TeamSubmissionEnvelope>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW],
  submitted_count: usize,
  phase: TeamSimultaneousPhase,
}

impl fmt::Debug for TeamSimultaneousWindow {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Redact uncommitted intent contents during collection to preserve actor privacy
    f.debug_struct("TeamSimultaneousWindow")
      .field("schema", &self.schema)
      .field("observation_id", &self.observation_id)
      .field("turn", &self.turn)
      .field("registered_count", &self.registered_count)
      .field("submitted_count", &self.submitted_count)
      .field("phase", &self.phase)
      .finish()
  }
}

impl TeamSimultaneousWindow {
  /// Create a new two-role simultaneous decision window.
  pub fn new_two_role(
    first: LaneActorRole,
    second: LaneActorRole,
    observation_id: u64,
    turn: u32,
  ) -> Result<Self, TeamSimultaneousError> {
    if first == second {
      return Err(TeamSimultaneousError::DuplicateRegisteredRole(first));
    }
    let mut registered_roles = [None; MAX_ROLES_IN_SIMULTANEOUS_WINDOW];
    registered_roles[0] = Some(first);
    registered_roles[1] = Some(second);

    Ok(Self {
      schema: TEAM_SIMULTANEOUS_SUBMISSION_SCHEMA,
      observation_id,
      turn,
      registered_roles,
      registered_count: 2,
      submissions: [None, None, None, None],
      submitted_count: 0,
      phase: TeamSimultaneousPhase::CollectingSubmissions,
    })
  }

  /// Create a new team simultaneous window with an explicit slice of registered roles.
  pub fn new_team(
    roles: &[LaneActorRole],
    observation_id: u64,
    turn: u32,
  ) -> Result<Self, TeamSimultaneousError> {
    if roles.is_empty() || roles.len() > MAX_ROLES_IN_SIMULTANEOUS_WINDOW {
      return Err(TeamSimultaneousError::InvalidRoleCount);
    }
    let mut registered_roles = [None; MAX_ROLES_IN_SIMULTANEOUS_WINDOW];
    for (idx, &role) in roles.iter().enumerate() {
      for prior in registered_roles.iter().take(idx).flatten() {
        if *prior == role {
          return Err(TeamSimultaneousError::DuplicateRegisteredRole(role));
        }
      }
      registered_roles[idx] = Some(role);
    }

    Ok(Self {
      schema: TEAM_SIMULTANEOUS_SUBMISSION_SCHEMA,
      observation_id,
      turn,
      registered_roles,
      registered_count: roles.len(),
      submissions: [None, None, None, None],
      submitted_count: 0,
      phase: TeamSimultaneousPhase::CollectingSubmissions,
    })
  }

  /// Return schema string.
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Return bound observation ID.
  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  /// Return bound turn index.
  pub const fn turn(&self) -> u32 {
    self.turn
  }

  /// Return current lifecycle phase.
  pub const fn phase(&self) -> TeamSimultaneousPhase {
    self.phase
  }

  /// Return whether all registered roles have submitted and the window is ready.
  pub const fn is_ready(&self) -> bool {
    matches!(self.phase, TeamSimultaneousPhase::Ready)
  }

  /// Return count of registered roles.
  pub const fn registered_count(&self) -> usize {
    self.registered_count
  }

  /// Return count of received submissions.
  pub const fn submitted_count(&self) -> usize {
    self.submitted_count
  }

  /// Return registered roles.
  pub const fn registered_roles(
    &self,
  ) -> &[Option<LaneActorRole>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW] {
    &self.registered_roles
  }

  /// Collect one actor's private submission without exposing choices to peers.
  pub fn submit(
    &mut self,
    submission: TeamSubmissionEnvelope,
  ) -> Result<TeamSubmissionReceipt, TeamSimultaneousError> {
    if self.phase == TeamSimultaneousPhase::Closed || self.phase == TeamSimultaneousPhase::Resolved
    {
      return Err(TeamSimultaneousError::Closed);
    }
    if submission.observation_id() != self.observation_id {
      return Err(TeamSimultaneousError::StaleObservation {
        submitted: submission.observation_id(),
        expected: self.observation_id,
      });
    }

    // Find registered index for role
    let mut role_idx = None;
    for (idx, registered) in self.registered_roles.iter().enumerate() {
      if let Some(r) = registered
        && *r == submission.role()
      {
        role_idx = Some(idx);
        break;
      }
    }

    let idx = role_idx.ok_or(TeamSimultaneousError::RoleNotRegistered(submission.role()))?;

    if self.submissions[idx].is_some() {
      return Err(TeamSimultaneousError::DuplicateSubmission(
        submission.role(),
      ));
    }

    let message_staged = submission.staged_message().is_some();
    let plan_staged = submission.individual_plan().is_some();
    let role = submission.role();

    self.submissions[idx] = Some(submission);
    self.submitted_count = self.submitted_count.saturating_add(1);

    if self.submitted_count == self.registered_count {
      self.phase = TeamSimultaneousPhase::Ready;
    }

    Ok(TeamSubmissionReceipt {
      role,
      observation_id: self.observation_id,
      turn: self.turn,
      accepted: true,
      message_staged,
      plan_staged,
      chain_of_thought_free: true,
    })
  }

  /// Retrieve a role's submission; protected so it cannot be inspected while collecting submissions.
  pub fn get_submission(
    &self,
    role: LaneActorRole,
  ) -> Result<Option<&TeamSubmissionEnvelope>, TeamSimultaneousError> {
    if self.phase == TeamSimultaneousPhase::CollectingSubmissions {
      return Err(TeamSimultaneousError::NotReady);
    }
    for (idx, registered) in self.registered_roles.iter().enumerate() {
      if let Some(r) = registered
        && *r == role
      {
        return Ok(self.submissions[idx].as_ref());
      }
    }
    Err(TeamSimultaneousError::RoleNotRegistered(role))
  }

  /// Retrieve all submissions once the window is ready or resolved.
  pub fn submissions(
    &self,
  ) -> Result<
    &[Option<TeamSubmissionEnvelope>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW],
    TeamSimultaneousError,
  > {
    if self.phase == TeamSimultaneousPhase::CollectingSubmissions {
      return Err(TeamSimultaneousError::NotReady);
    }
    Ok(&self.submissions)
  }

  /// Mark the window as resolved.
  pub fn mark_resolved(&mut self) -> Result<(), TeamSimultaneousError> {
    if self.phase != TeamSimultaneousPhase::Ready {
      return Err(TeamSimultaneousError::NotReady);
    }
    self.phase = TeamSimultaneousPhase::Resolved;
    Ok(())
  }

  /// Close the window against further submissions.
  pub fn close(&mut self) {
    self.phase = TeamSimultaneousPhase::Closed;
  }
}

/// Resolved action details for a single participating role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoleResolvedIntent {
  role: LaneActorRole,
  intent: LaneIntent,
  target_focus: LaneTargetFocus,
  commitment: LaneCommitment,
  ping_signal: LanePingSignal,
}

impl RoleResolvedIntent {
  /// Create a new resolved intent entry.
  pub const fn new(
    role: LaneActorRole,
    intent: LaneIntent,
    target_focus: LaneTargetFocus,
    commitment: LaneCommitment,
    ping_signal: LanePingSignal,
  ) -> Self {
    Self {
      role,
      intent,
      target_focus,
      commitment,
      ping_signal,
    }
  }

  /// Return actor role.
  pub const fn role(self) -> LaneActorRole {
    self.role
  }

  /// Return chosen intent.
  pub const fn intent(self) -> LaneIntent {
    self.intent
  }

  /// Return chosen target focus.
  pub const fn target_focus(self) -> LaneTargetFocus {
    self.target_focus
  }

  /// Return chosen commitment level.
  pub const fn commitment(self) -> LaneCommitment {
    self.commitment
  }

  /// Return chosen ping signal.
  pub const fn ping_signal(self) -> LanePingSignal {
    self.ping_signal
  }
}

/// Report detailing the simultaneous evaluation and resolution of a multi-agent decision window.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamSimultaneousResolution {
  schema: &'static str,
  observation_id: u64,
  turn: u32,
  active_team_plan_id: Option<&'static str>,
  coordination_outcome: TeamCoordinationOutcome,
  team_cohesion_bp: u32,
  resolved_roles: [Option<RoleResolvedIntent>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW],
  resolved_count: usize,
  leadership_report: Option<LeadershipEvaluationReport>,
  alignment_evaluations: [Option<AlignmentEvaluation>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW],
  trust_decisions: [Option<TrustComplianceDecision>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW],
  chain_of_thought_free: bool,
}

impl TeamSimultaneousResolution {
  /// Return schema string.
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  /// Return bound observation ID.
  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  /// Return bound turn index.
  pub const fn turn(&self) -> u32 {
    self.turn
  }

  /// Return active team plan ID if evaluated.
  pub const fn active_team_plan_id(&self) -> Option<&'static str> {
    self.active_team_plan_id
  }

  /// Return resulting coordination outcome.
  pub const fn coordination_outcome(&self) -> TeamCoordinationOutcome {
    self.coordination_outcome
  }

  /// Return overall team cohesion in basis points ($[0..=10,000]$ bp).
  pub const fn team_cohesion_bp(&self) -> u32 {
    self.team_cohesion_bp
  }

  /// Return count of resolved roles.
  pub const fn resolved_count(&self) -> usize {
    self.resolved_count
  }

  /// Return resolved role intents.
  pub const fn resolved_roles(
    &self,
  ) -> &[Option<RoleResolvedIntent>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW] {
    &self.resolved_roles
  }

  /// Return leadership evaluation report if present.
  pub const fn leadership_report(&self) -> Option<&LeadershipEvaluationReport> {
    self.leadership_report.as_ref()
  }

  /// Return alignment evaluations.
  pub const fn alignment_evaluations(
    &self,
  ) -> &[Option<AlignmentEvaluation>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW] {
    &self.alignment_evaluations
  }

  /// Return trust compliance decisions.
  pub const fn trust_decisions(
    &self,
  ) -> &[Option<TrustComplianceDecision>; MAX_ROLES_IN_SIMULTANEOUS_WINDOW] {
    &self.trust_decisions
  }

  /// Return whether chain of thought is absent.
  pub const fn chain_of_thought_free(&self) -> bool {
    self.chain_of_thought_free
  }

  /// Format markdown summary of the simultaneous resolution.
  pub fn render_markdown(&self) -> String {
    let mut out = String::with_capacity(1024);
    out.push_str("# Team Simultaneous Resolution Report\n\n");
    out.push_str(&format!("- **Schema:** `{}`\n", self.schema));
    out.push_str(&format!(
      "- **Observation ID:** `{}`\n",
      self.observation_id
    ));
    out.push_str(&format!("- **Turn:** `{}`\n", self.turn));
    if let Some(plan_id) = self.active_team_plan_id {
      out.push_str(&format!("- **Active Team Plan:** `{plan_id}`\n"));
    }
    out.push_str(&format!(
      "- **Coordination Outcome:** `{}`\n",
      self.coordination_outcome.as_str()
    ));
    out.push_str(&format!(
      "- **Team Cohesion:** {} bp ({}%)\n\n",
      self.team_cohesion_bp,
      self.team_cohesion_bp / 100
    ));

    out.push_str("## Resolved Role Submissions\n\n");
    for resolved in self.resolved_roles.iter().flatten() {
      out.push_str(&format!(
        "- **{}:** Intent `{}`, Focus `{:?}`, Commitment `{:?}`, Ping `{:?}`\n",
        resolved.role().as_str(),
        resolved.intent().as_str(),
        resolved.target_focus(),
        resolved.commitment(),
        resolved.ping_signal()
      ));
    }
    out.push('\n');

    if let Some(report) = &self.leadership_report {
      out.push_str("## Leadership Resolution\n\n");
      out.push_str(&format!("- **Structure:** `{}`\n", report.leadership_type));
      out.push_str(&format!(
        "- **Resolution Outcome:** `{}`\n",
        report.resolution.as_str()
      ));
      out.push_str(&format!(
        "- **Leadership Compliance:** {} bp\n\n",
        report.overall_compliance_bp
      ));
    }

    out
  }
}

/// Pure deterministic evaluator for multi-agent simultaneous decision resolution.
pub struct TeamSimultaneousResolver;

impl TeamSimultaneousResolver {
  /// Simultaneously evaluate all private submissions collected in a ready window.
  pub fn resolve(
    window: &mut TeamSimultaneousWindow,
    team_plan: Option<&TeamPlanDefinition>,
    leadership_structure: Option<&LeadershipStructure>,
    directives: &[ShotCallerDirective],
    peer_proposals: &[PeerPlanProposal],
    trust_matrix: Option<&TeamTrustMatrix>,
    observation: &LanerObservation,
  ) -> Result<TeamSimultaneousResolution, TeamSimultaneousError> {
    if !window.is_ready() {
      return Err(TeamSimultaneousError::NotReady);
    }

    let mut resolved_roles = [None; MAX_ROLES_IN_SIMULTANEOUS_WINDOW];
    let mut alignment_evaluations = [None; MAX_ROLES_IN_SIMULTANEOUS_WINDOW];
    let mut trust_decisions = [None; MAX_ROLES_IN_SIMULTANEOUS_WINDOW];

    let mut total_cohesion_sum: u32 = 0;
    let mut evaluated_factors: u32 = 0;

    let mut intent_match_count: u32 = 0;
    let mut total_roles_count: u32 = 0;
    let mut first_intent: Option<LaneIntent> = None;
    let mut all_intents_identical = true;

    for (idx, sub_opt) in window.submissions()?.iter().enumerate() {
      if let Some(sub) = sub_opt {
        total_roles_count = total_roles_count.saturating_add(1);
        let role_resolved = RoleResolvedIntent::new(
          sub.role(),
          sub.intent(),
          sub.target_focus(),
          sub.commitment(),
          sub.ping_signal(),
        );
        resolved_roles[idx] = Some(role_resolved);

        if let Some(first) = first_intent {
          if first == sub.intent() {
            intent_match_count = intent_match_count.saturating_add(1);
          } else {
            all_intents_identical = false;
          }
        } else {
          first_intent = Some(sub.intent());
          intent_match_count = 1;
        }

        // Evaluate plan alignment if team plan and individual plan are provided
        if let (Some(plan), Some(indiv_plan)) = (team_plan, sub.individual_plan()) {
          let align_eval =
            TeamPlanEvaluator::evaluate_alignment(plan, indiv_plan, Some(observation))?;
          let align_bp = match align_eval.alignment_type {
            TeamPlanAlignmentType::Aligned => 10_000,
            TeamPlanAlignmentType::ConditionalCompliance => 7_500,
            TeamPlanAlignmentType::Independent => 5_000,
            TeamPlanAlignmentType::Divergent => 2_500,
            TeamPlanAlignmentType::Conflicted => 0,
          };
          total_cohesion_sum = total_cohesion_sum.saturating_add(align_bp);
          evaluated_factors = evaluated_factors.saturating_add(1);
          alignment_evaluations[idx] = Some(align_eval);
        }

        // Evaluate trust compliance if staged message is present
        if let (Some(msg), Some(matrix)) = (sub.staged_message(), trust_matrix) {
          let caller_rep = matrix.get(sub.role());
          let report = TeamTrustEvaluator::evaluate_proposal(
            msg,
            caller_rep,
            CommunicationClarity::Crisp,
            observation,
            LaneActorRole::AlliedAutonomous,
          )?;
          let compliance_bp = match report.decision {
            TrustComplianceDecision::Comply => 10_000,
            TrustComplianceDecision::Clarify => 5_000,
            TrustComplianceDecision::Dissent(_) => 2_500,
          };
          total_cohesion_sum = total_cohesion_sum.saturating_add(compliance_bp);
          evaluated_factors = evaluated_factors.saturating_add(1);
          trust_decisions[idx] = Some(report.decision);
        }
      }
    }

    // Evaluate leadership structure if provided
    let mut leadership_report = None;
    if let (Some(structure), Some(matrix)) = (leadership_structure, trust_matrix) {
      let lead_report = TeamLeadershipEvaluator::evaluate_leadership(
        structure,
        directives,
        peer_proposals,
        matrix,
        observation,
      )?;
      total_cohesion_sum = total_cohesion_sum.saturating_add(lead_report.overall_compliance_bp);
      evaluated_factors = evaluated_factors.saturating_add(1);
      leadership_report = Some(lead_report);
    }

    // Compute base intent agreement cohesion if no other factors were evaluated
    if evaluated_factors == 0 {
      let intent_agreement_bp = if total_roles_count > 0 && all_intents_identical {
        10_000
      } else if total_roles_count > 1 {
        (intent_match_count.saturating_mul(10_000)) / total_roles_count
      } else {
        5_000
      };
      total_cohesion_sum = intent_agreement_bp;
      evaluated_factors = 1;
    }

    let team_cohesion_bp = total_cohesion_sum / evaluated_factors;

    let coordination_outcome = if team_cohesion_bp >= FULL_COHESION_THRESHOLD_BP {
      TeamCoordinationOutcome::FullyCoordinated
    } else if team_cohesion_bp >= PARTIAL_COHESION_THRESHOLD_BP {
      TeamCoordinationOutcome::PartiallyCoordinated
    } else if team_cohesion_bp >= DIVERGENT_COHESION_THRESHOLD_BP {
      TeamCoordinationOutcome::DivergentIntents
    } else if team_cohesion_bp >= CONFLICTING_COHESION_THRESHOLD_BP {
      TeamCoordinationOutcome::ConflictingDirectives
    } else {
      TeamCoordinationOutcome::CommunicationFailure
    };

    window.mark_resolved()?;

    Ok(TeamSimultaneousResolution {
      schema: TEAM_SIMULTANEOUS_RESOLUTION_SCHEMA,
      observation_id: window.observation_id(),
      turn: window.turn(),
      active_team_plan_id: team_plan.map(|p| p.plan_id),
      coordination_outcome,
      team_cohesion_bp,
      resolved_roles,
      resolved_count: usize::try_from(total_roles_count).unwrap_or(0),
      leadership_report,
      alignment_evaluations,
      trust_decisions,
      chain_of_thought_free: true,
    })
  }
}

/// Canonical scenario entry in the simultaneous resolution catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamSimultaneousScenario {
  scenario_id: &'static str,
  description: &'static str,
  observation_id: u64,
  turn: u32,
  team_plan_id: Option<&'static str>,
  expected_outcome: TeamCoordinationOutcome,
  min_cohesion_bp: u32,
}

impl TeamSimultaneousScenario {
  /// Return scenario ID.
  pub const fn scenario_id(&self) -> &'static str {
    self.scenario_id
  }

  /// Return scenario description.
  pub const fn description(&self) -> &'static str {
    self.description
  }

  /// Return expected observation ID.
  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  /// Return expected turn index.
  pub const fn turn(&self) -> u32 {
    self.turn
  }

  /// Return associated team plan ID.
  pub const fn team_plan_id(&self) -> Option<&'static str> {
    self.team_plan_id
  }

  /// Return expected coordination outcome.
  pub const fn expected_outcome(&self) -> TeamCoordinationOutcome {
    self.expected_outcome
  }

  /// Return minimum expected cohesion in basis points.
  pub const fn min_cohesion_bp(&self) -> u32 {
    self.min_cohesion_bp
  }
}

/// Catalog of canonical reference scenarios for simultaneous multi-agent resolution.
pub struct TeamSimultaneousCatalog;

impl TeamSimultaneousCatalog {
  /// Registered canonical simultaneous resolution scenarios.
  pub const SCENARIOS: &'static [TeamSimultaneousScenario] = &[
    TeamSimultaneousScenario {
      scenario_id: "simultaneous-gank-coordinated-v1",
      description: "Human laner and allied support commit to coordinated gank execution under high trust",
      observation_id: 101,
      turn: 1,
      team_plan_id: Some("plan-gank-setup-v1"),
      expected_outcome: TeamCoordinationOutcome::FullyCoordinated,
      min_cohesion_bp: 7_500,
    },
    TeamSimultaneousScenario {
      scenario_id: "simultaneous-defensive-fallback-v1",
      description: "Human laner and allied support both yield space under visible threat report",
      observation_id: 102,
      turn: 2,
      team_plan_id: Some("plan-defensive-hold-v1"),
      expected_outcome: TeamCoordinationOutcome::FullyCoordinated,
      min_cohesion_bp: 7_500,
    },
    TeamSimultaneousScenario {
      scenario_id: "simultaneous-dissent-tradeoff-v1",
      description: "Allied support dissents from aggressive contest due to low health, adapting to stabilization",
      observation_id: 103,
      turn: 3,
      team_plan_id: Some("plan-gank-setup-v1"),
      expected_outcome: TeamCoordinationOutcome::PartiallyCoordinated,
      min_cohesion_bp: 5_000,
    },
    TeamSimultaneousScenario {
      scenario_id: "simultaneous-conflicting-directives-v1",
      description: "Split calls from teammates without consensus resolution causing divergent intents",
      observation_id: 104,
      turn: 4,
      team_plan_id: Some("plan-resource-farming-v1"),
      expected_outcome: TeamCoordinationOutcome::DivergentIntents,
      min_cohesion_bp: 2_500,
    },
    TeamSimultaneousScenario {
      scenario_id: "simultaneous-communication-failure-v1",
      description: "Dropped packets across communication channel leading to uncoordinated fallback execution",
      observation_id: 105,
      turn: 5,
      team_plan_id: Some("plan-tactical-reset-v1"),
      expected_outcome: TeamCoordinationOutcome::CommunicationFailure,
      min_cohesion_bp: 0,
    },
  ];

  /// Look up a canonical scenario by its unique identifier.
  pub fn get_scenario(
    scenario_id: &str,
  ) -> Result<&'static TeamSimultaneousScenario, TeamSimultaneousError> {
    for scenario in Self::SCENARIOS {
      if scenario.scenario_id == scenario_id {
        return Ok(scenario);
      }
    }
    Err(TeamSimultaneousError::CatalogScenarioNotFound(
      "scenario not found",
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::agent::communication::{
    TeamConfidenceLevel, TeamMessageCondition, TeamMessageUrgency, TeamMessageVisibility,
    TeamRecipient, TeamSpeechAct,
  };
  use crate::agent::leadership::FallbackLeadershipMode;
  use crate::agent::team_plan::{TeamPlanCatalog, TeamStrategicObjective};
  use crate::lane::{
    LaneAbortCondition, LaneFallbackBehavior, LaneSnapshot, ObservationId, observe_player,
  };

  fn make_test_observation(threat: bool) -> LanerObservation {
    let initial = LaneSnapshot::initial();
    let state = if threat {
      LaneSnapshot::new(
        initial.ruleset(),
        initial.turn(),
        crate::lane::LaneStatus::Open,
        initial.player(),
        initial.opponent(),
        initial.wave(),
        crate::lane::JungleThreatTruth::RiverSide,
      )
    } else {
      initial
    };
    observe_player(&state, ObservationId::new(100)).observation()
  }

  #[test]
  fn test_team_simultaneous_envelope_creation_and_privacy() {
    let envelope = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      42,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      false,
    )
    .expect("valid envelope");

    assert_eq!(envelope.schema(), TEAM_SIMULTANEOUS_SUBMISSION_SCHEMA);
    assert_eq!(envelope.role(), LaneActorRole::HumanLaner);
    assert_eq!(envelope.observation_id(), 42);
    assert_eq!(envelope.turn(), 1);
    assert_eq!(envelope.intent(), LaneIntent::Contest);
    assert!(!envelope.chain_of_thought_present());

    // Fail closed on chain-of-thought presence
    let cot_err = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      42,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      true,
    );
    assert_eq!(cot_err, Err(TeamSimultaneousError::ChainOfThoughtForbidden));
  }

  #[test]
  fn test_team_simultaneous_window_lifecycle_and_privacy() {
    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      100,
      1,
    )
    .expect("valid two-role window");

    assert_eq!(window.phase(), TeamSimultaneousPhase::CollectingSubmissions);
    assert!(!window.is_ready());
    assert_eq!(window.registered_count(), 2);
    assert_eq!(window.submitted_count(), 0);

    // Queries during collection must fail to protect actor privacy
    assert_eq!(
      window.get_submission(LaneActorRole::HumanLaner),
      Err(TeamSimultaneousError::NotReady)
    );
    assert_eq!(window.submissions(), Err(TeamSimultaneousError::NotReady));

    // First submission from human laner
    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      100,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      None,
      None,
      false,
    )
    .unwrap();

    let receipt1 = window.submit(sub1).expect("receipt 1");
    assert!(receipt1.accepted());
    assert_eq!(receipt1.role(), LaneActorRole::HumanLaner);
    assert_eq!(window.submitted_count(), 1);
    assert!(!window.is_ready());

    // Reject duplicate submission
    let sub1_dup = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      100,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      false,
    )
    .unwrap();
    assert_eq!(
      window.submit(sub1_dup),
      Err(TeamSimultaneousError::DuplicateSubmission(
        LaneActorRole::HumanLaner
      ))
    );

    // Reject unregistered role
    let sub_wrong = TeamSubmissionEnvelope::new(
      LaneActorRole::OpposingLaner,
      100,
      1,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      false,
    )
    .unwrap();
    assert_eq!(
      window.submit(sub_wrong),
      Err(TeamSimultaneousError::RoleNotRegistered(
        LaneActorRole::OpposingLaner
      ))
    );

    // Reject stale observation ID
    let sub_stale = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      99,
      1,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      false,
    )
    .unwrap();
    assert_eq!(
      window.submit(sub_stale),
      Err(TeamSimultaneousError::StaleObservation {
        submitted: 99,
        expected: 100,
      })
    );

    // Second submission completes the window
    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      100,
      1,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::Assist,
      None,
      None,
      false,
    )
    .unwrap();

    let receipt2 = window.submit(sub2).expect("receipt 2");
    assert!(receipt2.accepted());
    assert_eq!(receipt2.role(), LaneActorRole::AlliedAutonomous);
    assert_eq!(window.submitted_count(), 2);
    assert!(window.is_ready());
    assert_eq!(window.phase(), TeamSimultaneousPhase::Ready);

    // Once ready, submissions can be read by the resolver
    let human_sub = window
      .get_submission(LaneActorRole::HumanLaner)
      .unwrap()
      .unwrap();
    assert_eq!(human_sub.intent(), LaneIntent::Contest);

    let ally_sub = window
      .get_submission(LaneActorRole::AlliedAutonomous)
      .unwrap()
      .unwrap();
    assert_eq!(ally_sub.intent(), LaneIntent::Stabilize);
  }

  #[test]
  fn test_team_simultaneous_resolver_coordinated_gank() {
    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      100,
      1,
    )
    .unwrap();

    let team_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1").unwrap();

    let msg = TeamMessageEnvelope::new(
      "msg-gank-proposal-v1",
      LaneActorRole::HumanLaner,
      TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
      TeamSpeechAct::Proposal,
      Some(LaneIntent::Contest),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      1,
      "Initiate gank setup",
    );

    let indiv_plan_human = IndividualPlanDefinition {
      plan_id: "plan-human-gank-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::OnMyWay,
      chain_of_thought_present: false,
    };

    let indiv_plan_ally = IndividualPlanDefinition {
      plan_id: "plan-ally-gank-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::Assist,
      chain_of_thought_present: false,
    };

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      100,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::OnMyWay,
      Some(msg),
      Some(indiv_plan_human),
      false,
    )
    .unwrap();

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      100,
      1,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::Assist,
      None,
      Some(indiv_plan_ally),
      false,
    )
    .unwrap();

    window.submit(sub1).unwrap();
    window.submit(sub2).unwrap();
    assert!(window.is_ready());

    let trust_matrix = TeamTrustMatrix::new();
    let obs = make_test_observation(false);

    let leadership = LeadershipStructure::DesignatedShotCaller {
      caller: LaneActorRole::HumanLaner,
      fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
    };

    let directive = ShotCallerDirective::new(
      LaneActorRole::HumanLaner,
      "plan-gank-setup-v1",
      TeamStrategicObjective::GankSetup,
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      "Execute gank",
    )
    .unwrap();

    let resolution = TeamSimultaneousResolver::resolve(
      &mut window,
      Some(team_plan),
      Some(&leadership),
      &[directive],
      &[],
      Some(&trust_matrix),
      &obs,
    )
    .expect("successful simultaneous resolution");

    assert_eq!(resolution.schema(), TEAM_SIMULTANEOUS_RESOLUTION_SCHEMA);
    assert_eq!(
      resolution.coordination_outcome(),
      TeamCoordinationOutcome::FullyCoordinated
    );
    assert!(resolution.team_cohesion_bp() >= FULL_COHESION_THRESHOLD_BP);
    assert_eq!(resolution.resolved_count(), 2);
    assert!(resolution.chain_of_thought_free());

    let md = resolution.render_markdown();
    assert!(md.contains("# Team Simultaneous Resolution Report"));
    assert!(md.contains("fully-coordinated"));
    assert!(md.contains("human-laner"));
    assert!(md.contains("allied-autonomous"));
  }

  #[test]
  fn test_team_simultaneous_catalog_validation() {
    assert_eq!(TeamSimultaneousCatalog::SCENARIOS.len(), 5);

    let gank = TeamSimultaneousCatalog::get_scenario("simultaneous-gank-coordinated-v1").unwrap();
    assert_eq!(
      gank.expected_outcome(),
      TeamCoordinationOutcome::FullyCoordinated
    );
    assert_eq!(gank.min_cohesion_bp(), 7_500);

    let defense =
      TeamSimultaneousCatalog::get_scenario("simultaneous-defensive-fallback-v1").unwrap();
    assert_eq!(
      defense.expected_outcome(),
      TeamCoordinationOutcome::FullyCoordinated
    );

    let dissent =
      TeamSimultaneousCatalog::get_scenario("simultaneous-dissent-tradeoff-v1").unwrap();
    assert_eq!(
      dissent.expected_outcome(),
      TeamCoordinationOutcome::PartiallyCoordinated
    );

    let conflicting =
      TeamSimultaneousCatalog::get_scenario("simultaneous-conflicting-directives-v1").unwrap();
    assert_eq!(
      conflicting.expected_outcome(),
      TeamCoordinationOutcome::DivergentIntents
    );

    let comm_fail =
      TeamSimultaneousCatalog::get_scenario("simultaneous-communication-failure-v1").unwrap();
    assert_eq!(
      comm_fail.expected_outcome(),
      TeamCoordinationOutcome::CommunicationFailure
    );

    assert_eq!(
      TeamSimultaneousCatalog::get_scenario("unknown-scenario"),
      Err(TeamSimultaneousError::CatalogScenarioNotFound(
        "scenario not found"
      ))
    );
  }

  #[test]
  fn test_team_simultaneous_error_formatting() {
    let err_cot = TeamSimultaneousError::ChainOfThoughtForbidden;
    assert_eq!(
      format!("{}", err_cot),
      "private chain-of-thought is strictly forbidden in simultaneous submission contracts"
    );

    let err_closed = TeamSimultaneousError::Closed;
    assert_eq!(
      format!("{}", err_closed),
      "simultaneous window is closed to submissions"
    );

    let err_stale = TeamSimultaneousError::StaleObservation {
      submitted: 10,
      expected: 20,
    };
    assert_eq!(
      format!("{}", err_stale),
      "stale observation in submission: submitted 10, expected 20"
    );

    let err_role = TeamSimultaneousError::RoleNotRegistered(LaneActorRole::OpposingLaner);
    assert_eq!(
      format!("{}", err_role),
      "actor role `opposing-laner` is not registered in this simultaneous window"
    );

    let err_dup = TeamSimultaneousError::DuplicateSubmission(LaneActorRole::HumanLaner);
    assert_eq!(
      format!("{}", err_dup),
      "duplicate submission received for actor role `human-laner`"
    );

    let err_ready = TeamSimultaneousError::NotReady;
    assert_eq!(
      format!("{}", err_ready),
      "simultaneous window is not ready for resolution (awaiting submissions)"
    );

    let err_missing = TeamSimultaneousError::MissingSubmission(LaneActorRole::AlliedAutonomous);
    assert_eq!(
      format!("{}", err_missing),
      "missing submission from registered role `allied-autonomous`"
    );

    let err_roles = TeamSimultaneousError::InvalidRoleCount;
    assert_eq!(
      format!("{}", err_roles),
      "registered role count must be between 1 and 4"
    );

    let err_dup_role = TeamSimultaneousError::DuplicateRegisteredRole(LaneActorRole::HumanLaner);
    assert_eq!(
      format!("{}", err_dup_role),
      "duplicate role `human-laner` registered in simultaneous window"
    );

    let err_bp = TeamSimultaneousError::BasisPointOutOfRange {
      bp: 12_000,
      max: 10_000,
    };
    assert_eq!(
      format!("{}", err_bp),
      "basis points 12000 exceeded maximum allowed bound 10000"
    );

    let err_cat = TeamSimultaneousError::CatalogScenarioNotFound("test");
    assert_eq!(
      format!("{}", err_cat),
      "simultaneous scenario `test` not found in catalog"
    );
  }

  #[test]
  fn test_team_simultaneous_phase_and_outcome_round_trips() {
    for phase in [
      TeamSimultaneousPhase::CollectingSubmissions,
      TeamSimultaneousPhase::Ready,
      TeamSimultaneousPhase::Resolved,
      TeamSimultaneousPhase::Closed,
    ] {
      let label = phase.as_str();
      assert_eq!(TeamSimultaneousPhase::parse(label), Some(phase));
    }
    assert_eq!(TeamSimultaneousPhase::parse("unknown"), None);

    for outcome in TeamCoordinationOutcome::all() {
      let label = outcome.as_str();
      assert_eq!(TeamCoordinationOutcome::parse(label), Some(outcome));
    }
    assert_eq!(TeamCoordinationOutcome::parse("invalid-outcome"), None);
  }

  #[test]
  fn test_team_simultaneous_window_registration_edge_cases() {
    // Duplicate roles in 2-role constructor
    assert_eq!(
      TeamSimultaneousWindow::new_two_role(
        LaneActorRole::HumanLaner,
        LaneActorRole::HumanLaner,
        100,
        1
      ),
      Err(TeamSimultaneousError::DuplicateRegisteredRole(
        LaneActorRole::HumanLaner
      ))
    );

    // Empty roles in team constructor
    assert_eq!(
      TeamSimultaneousWindow::new_team(&[], 100, 1),
      Err(TeamSimultaneousError::InvalidRoleCount)
    );

    // Too many roles
    let too_many = [
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      LaneActorRole::OpposingLaner,
      LaneActorRole::OpposingJungleThreat,
      LaneActorRole::HumanLaner,
    ];
    assert_eq!(
      TeamSimultaneousWindow::new_team(&too_many, 100, 1),
      Err(TeamSimultaneousError::InvalidRoleCount)
    );

    // Duplicate in team constructor
    let dup_team = [
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      LaneActorRole::HumanLaner,
    ];
    assert_eq!(
      TeamSimultaneousWindow::new_team(&dup_team, 100, 1),
      Err(TeamSimultaneousError::DuplicateRegisteredRole(
        LaneActorRole::HumanLaner
      ))
    );
  }

  #[test]
  fn test_team_simultaneous_divergent_resolution() {
    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      104,
      4,
    )
    .unwrap();

    let team_plan = TeamPlanCatalog::lookup("plan-resource-farming-v1").unwrap();

    // Human Laner commits to Contest (diverging from plan-resource-farming which assigns Stabilize)
    let indiv_plan_human = IndividualPlanDefinition {
      plan_id: "plan-human-contest-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::MaintainPlan,
      ping_signal: LanePingSignal::None,
      chain_of_thought_present: false,
    };

    // Allied Autonomous commits to Yield (matching plan assignment)
    let indiv_plan_ally = IndividualPlanDefinition {
      plan_id: "plan-ally-yield-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Yield,
      target_focus: LaneTargetFocus::Minions,
      commitment: LaneCommitment::Cautious,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::RetreatToTower,
      ping_signal: LanePingSignal::None,
      chain_of_thought_present: false,
    };

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      104,
      4,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      Some(indiv_plan_human),
      false,
    )
    .unwrap();

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      104,
      4,
      LaneIntent::Yield,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LanePingSignal::None,
      None,
      Some(indiv_plan_ally),
      false,
    )
    .unwrap();

    window.submit(sub1).unwrap();
    window.submit(sub2).unwrap();
    assert!(window.is_ready());

    let obs = make_test_observation(false);

    let resolution =
      TeamSimultaneousResolver::resolve(&mut window, Some(team_plan), None, &[], &[], None, &obs)
        .expect("resolution");

    assert_eq!(
      resolution.coordination_outcome(),
      TeamCoordinationOutcome::PartiallyCoordinated
    );
    assert_eq!(resolution.team_cohesion_bp(), 6_250);
    assert_eq!(window.phase(), TeamSimultaneousPhase::Resolved);

    // After resolution, new submissions fail closed
    let late_sub = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      104,
      4,
      LaneIntent::Contest,
      LaneTargetFocus::Minions,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      None,
      false,
    )
    .unwrap();
    assert_eq!(window.submit(late_sub), Err(TeamSimultaneousError::Closed));
  }

  #[test]
  fn test_team_simultaneous_both_divergent_resolution() {
    let mut window = TeamSimultaneousWindow::new_two_role(
      LaneActorRole::HumanLaner,
      LaneActorRole::AlliedAutonomous,
      104,
      4,
    )
    .unwrap();

    let team_plan = TeamPlanCatalog::lookup("plan-resource-farming-v1").unwrap();

    // Both actors choose Contest (diverging from plan-resource-farming-v1 assignments)
    let indiv_plan_human = IndividualPlanDefinition {
      plan_id: "plan-human-contest-v1",
      actor: LaneActorRole::HumanLaner,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::MaintainPlan,
      ping_signal: LanePingSignal::None,
      chain_of_thought_present: false,
    };

    let indiv_plan_ally = IndividualPlanDefinition {
      plan_id: "plan-ally-contest-v1",
      actor: LaneActorRole::AlliedAutonomous,
      selected_intent: LaneIntent::Contest,
      target_focus: LaneTargetFocus::OpposingLaner,
      commitment: LaneCommitment::Standard,
      abort_condition: LaneAbortCondition::None,
      fallback_behavior: LaneFallbackBehavior::MaintainPlan,
      ping_signal: LanePingSignal::None,
      chain_of_thought_present: false,
    };

    let sub1 = TeamSubmissionEnvelope::new(
      LaneActorRole::HumanLaner,
      104,
      4,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      Some(indiv_plan_human),
      false,
    )
    .unwrap();

    let sub2 = TeamSubmissionEnvelope::new(
      LaneActorRole::AlliedAutonomous,
      104,
      4,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LanePingSignal::None,
      None,
      Some(indiv_plan_ally),
      false,
    )
    .unwrap();

    window.submit(sub1).unwrap();
    window.submit(sub2).unwrap();
    assert!(window.is_ready());

    let obs = make_test_observation(false);

    let resolution =
      TeamSimultaneousResolver::resolve(&mut window, Some(team_plan), None, &[], &[], None, &obs)
        .expect("resolution");

    assert_eq!(
      resolution.coordination_outcome(),
      TeamCoordinationOutcome::DivergentIntents
    );
    assert_eq!(resolution.team_cohesion_bp(), 2_500);
    assert_eq!(window.phase(), TeamSimultaneousPhase::Resolved);
  }
}

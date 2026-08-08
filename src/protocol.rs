//! Versioned actor-protocol DTOs at the M5 adapter boundary.
//!
//! The DTOs contain only bounded actor-visible observation, action, metadata,
//! lifecycle, result, and committed-facts review data. They do not validate
//! legality, resolve execution, mutate history, or depend on a transport,
//! async runtime, or provider SDK.

use crate::kernel::ActorId;
use crate::lane::{
  LaneIntent, LaneIntentRequest, LaneOutcome, LanerObservation, ObjectiveDisposition,
  ObservationId, ScenarioDebriefReport, ScenarioWindow,
};
use std::fmt::Write as _;

/// Versioned actor-protocol vocabulary for this bounded slice.
pub const ACTOR_PROTOCOL_SCHEMA: &str = "m5-actor-protocol-v1";

/// Versioned observation DTO identity.
pub const ACTOR_OBSERVATION_SCHEMA: &str = "m5-actor-observation-v1";

/// Versioned intent-action DTO identity.
pub const ACTOR_ACTION_SCHEMA: &str = "m5-actor-action-v1";

/// Versioned actor-safe action-result identity.
pub const ACTOR_ACTION_RESULT_SCHEMA: &str = "m5-actor-action-result-v1";

/// Versioned actor-visible completed-run debrief summary identity.
pub const ACTOR_DEBRIEF_SCHEMA: &str = "m5-actor-debrief-v1";

/// Versioned actor commit command identity.
pub const ACTOR_COMMIT_SCHEMA: &str = "m5-actor-commit-v1";

/// Versioned actor commit acknowledgement identity.
pub const ACTOR_COMMIT_RESULT_SCHEMA: &str = "m5-actor-commit-result-v1";

/// Versioned actor message/plan/contingency metadata identity.
pub const ACTOR_DRAFT_SCHEMA: &str = "m5-actor-draft-v1";

/// Versioned actor-visible bounded history-status identity.
pub const ACTOR_HISTORY_SCHEMA: &str = "m5-actor-history-v1";

/// Versioned line-oriented codec identity for the bounded DTOs.
pub const ACTOR_PROTOCOL_CODEC_SCHEMA: &str = "m5-actor-codec-v1";

/// Historical actor-facing validation-error identity from the initial closed vocabulary.
pub const ACTOR_PROTOCOL_ERROR_SCHEMA_V1: &str = "m5-actor-error-v1";

/// Current actor-facing validation-error identity after the debrief error pair was added.
pub const ACTOR_PROTOCOL_ERROR_SCHEMA: &str = "m5-actor-error-v2";

/// Maximum encoded DTO size accepted by the bounded parser.
pub const MAX_ACTOR_PROTOCOL_BYTES: usize = 4096;

/// Maximum UTF-8 payload size for one actor draft metadata value.
pub const MAX_ACTOR_DRAFT_VALUE_BYTES: usize = 256;

/// Closed intent vocabulary exposed by the actor protocol.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolIntent {
  Stabilize,
  Contest,
  Yield,
  Recall,
  Withdraw,
}

impl ActorProtocolIntent {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Stabilize => "stabilize",
      Self::Contest => "contest",
      Self::Yield => "yield",
      Self::Recall => "recall",
      Self::Withdraw => "withdraw",
    }
  }

  const fn from_lane_intent(intent: LaneIntent) -> Self {
    match intent {
      LaneIntent::Stabilize => Self::Stabilize,
      LaneIntent::Contest => Self::Contest,
      LaneIntent::Yield => Self::Yield,
      LaneIntent::Recall => Self::Recall,
      LaneIntent::Withdraw => Self::Withdraw,
    }
  }

  const fn to_lane_intent(self) -> LaneIntent {
    match self {
      Self::Stabilize => LaneIntent::Stabilize,
      Self::Contest => LaneIntent::Contest,
      Self::Yield => LaneIntent::Yield,
      Self::Recall => LaneIntent::Recall,
      Self::Withdraw => LaneIntent::Withdraw,
    }
  }
}

/// Actor-visible observation DTO for the bounded intent-only protocol slice.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActorObservationDto {
  schema: &'static str,
  observer: u8,
  turn: u32,
  observation_id: u64,
  available_actions: Vec<ActorProtocolIntent>,
  visible_threat_response: Option<ActorProtocolIntent>,
}

impl ActorObservationDto {
  /// Project an actor-visible lane observation without exposing domain state.
  pub fn from_observation(observation: LanerObservation) -> Self {
    let visible_threat_response = observation
      .available_threat_response()
      .map(ActorProtocolIntent::from_lane_intent);
    let mut available_actions = Vec::with_capacity(5);
    for intent in observation.available_intents() {
      available_actions.push(ActorProtocolIntent::from_lane_intent(intent));
    }
    if let Some(threat_response) = visible_threat_response
      && !available_actions.contains(&threat_response)
    {
      available_actions.push(threat_response);
    }
    Self {
      schema: ACTOR_OBSERVATION_SCHEMA,
      observer: observation.observer().value(),
      turn: observation.turn().value(),
      observation_id: observation.observation_id().value(),
      available_actions,
      visible_threat_response,
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> u8 {
    self.observer
  }

  pub const fn turn(&self) -> u32 {
    self.turn
  }

  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  pub fn available_actions(&self) -> &[ActorProtocolIntent] {
    &self.available_actions
  }

  pub const fn visible_threat_response(&self) -> Option<ActorProtocolIntent> {
    self.visible_threat_response
  }

  pub fn advertises(&self, intent: ActorProtocolIntent) -> bool {
    self.available_actions.contains(&intent)
  }

  /// Encode the bounded observation DTO as stable line-oriented text.
  pub fn encode(&self) -> String {
    let mut output = String::new();
    output.push_str("schema=");
    output.push_str(self.schema);
    output.push('\n');
    output.push_str("observer=");
    write!(output, "{}", self.observer).expect("writing to String cannot fail");
    output.push('\n');
    output.push_str("turn=");
    write!(output, "{}", self.turn).expect("writing to String cannot fail");
    output.push('\n');
    output.push_str("observation_id=");
    write!(output, "{}", self.observation_id).expect("writing to String cannot fail");
    output.push('\n');
    output.push_str("actions=");
    for (index, intent) in self.available_actions.iter().enumerate() {
      if index > 0 {
        output.push(',');
      }
      output.push_str(intent.id());
    }
    output.push('\n');
    output.push_str("threat=");
    output.push_str(
      self
        .visible_threat_response
        .map_or("unknown", ActorProtocolIntent::id),
    );
    output.push('\n');
    output
  }

  /// Decode a bounded line-oriented observation DTO.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 6)?;
    let mut schema = None;
    let mut observer = None;
    let mut turn = None;
    let mut observation_id = None;
    let mut actions = None;
    let mut threat = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "turn" => &mut turn,
        "observation_id" => &mut observation_id,
        "actions" => &mut actions,
        "threat" => &mut threat,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_OBSERVATION_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let observer = observer
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let turn = turn
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u32>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let observation_id = observation_id
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u64>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let actions = actions.ok_or(ActorProtocolCodecError::MissingField)?;
    let mut available_actions = Vec::with_capacity(5);
    for raw_intent in actions.split(',') {
      let intent = ActorProtocolIntent::parse_id(raw_intent)?;
      if available_actions.contains(&intent) || available_actions.len() == 5 {
        return Err(ActorProtocolCodecError::InvalidValue);
      }
      available_actions.push(intent);
    }
    if !(4..=5).contains(&available_actions.len()) {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    let base_actions = [
      ActorProtocolIntent::Stabilize,
      ActorProtocolIntent::Contest,
      ActorProtocolIntent::Yield,
      ActorProtocolIntent::Recall,
    ];
    if available_actions.get(..4) != Some(base_actions.as_slice()) {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    let visible_threat_response = match threat.ok_or(ActorProtocolCodecError::MissingField)? {
      "unknown" => None,
      value => Some(ActorProtocolIntent::parse_id(value)?),
    };
    if visible_threat_response.is_some_and(|intent| {
      intent != ActorProtocolIntent::Withdraw || !available_actions.contains(&intent)
    }) || (visible_threat_response.is_none() && available_actions.len() == 5)
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_OBSERVATION_SCHEMA,
      observer,
      turn,
      observation_id,
      available_actions,
      visible_threat_response,
    })
  }
}

/// Bounded actor action DTO carrying only an observer-bound intent request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorActionDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  intent: ActorProtocolIntent,
}

impl ActorActionDto {
  pub const fn new(observer: u8, observation_id: u64, intent: ActorProtocolIntent) -> Self {
    Self {
      schema: ACTOR_ACTION_SCHEMA,
      observer,
      observation_id,
      intent,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn observer(self) -> u8 {
    self.observer
  }

  pub const fn observation_id(self) -> u64 {
    self.observation_id
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  /// Convert to the host-bound request; legality remains a host concern.
  pub fn to_lane_request(self) -> LaneIntentRequest {
    LaneIntentRequest::new(
      ActorId::new(self.observer),
      ObservationId::new(self.observation_id),
      self.intent.to_lane_intent(),
    )
  }

  /// Encode the bounded action DTO as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nintent={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.intent.id()
    )
  }

  /// Decode a bounded line-oriented action DTO.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 4)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut intent = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "intent" => &mut intent,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_ACTION_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self {
      schema: ACTOR_ACTION_SCHEMA,
      observer: observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id: observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      intent: ActorProtocolIntent::parse_id(intent.ok_or(ActorProtocolCodecError::MissingField)?)?,
    })
  }
}

/// Observation-bound actor command that commits one explicit intent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorCommitDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  intent: ActorProtocolIntent,
}

impl ActorCommitDto {
  pub const fn new(observer: u8, observation_id: u64, intent: ActorProtocolIntent) -> Self {
    Self {
      schema: ACTOR_COMMIT_SCHEMA,
      observer,
      observation_id,
      intent,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn observer(self) -> u8 {
    self.observer
  }

  pub const fn observation_id(self) -> u64 {
    self.observation_id
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  pub(crate) fn to_lane_intent(self) -> LaneIntent {
    self.intent.to_lane_intent()
  }

  /// Encode the observation-bound commit command as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nintent={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.intent.id()
    )
  }

  /// Decode a bounded commit command without staging or advancing the host.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 4)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut intent = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "intent" => &mut intent,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_COMMIT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      observer
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u8>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      observation_id
        .ok_or(ActorProtocolCodecError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ActorProtocolCodecError::InvalidValue)?,
      ActorProtocolIntent::parse_id(intent.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}

/// Bounded actor-safe acknowledgement after a host-owned commit.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorCommitResultDto {
  schema: &'static str,
  intent: ActorProtocolIntent,
}

impl ActorCommitResultDto {
  pub const fn new(intent: ActorProtocolIntent) -> Self {
    Self {
      schema: ACTOR_COMMIT_RESULT_SCHEMA,
      intent,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  /// Encode the bounded commit acknowledgement as stable line-oriented text.
  pub fn encode(self) -> String {
    format!("schema={}\nintent={}\n", self.schema, self.intent.id())
  }

  /// Decode a bounded commit acknowledgement without transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 2)?;
    let mut schema = None;
    let mut intent = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "intent" => &mut intent,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_COMMIT_RESULT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(ActorProtocolIntent::parse_id(
      intent.ok_or(ActorProtocolCodecError::MissingField)?,
    )?))
  }
}

/// Closed fixture window labels in an actor action result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorActionResultWindow {
  First,
  Second,
}

impl ActorActionResultWindow {
  pub const fn id(self) -> &'static str {
    match self {
      Self::First => "first",
      Self::Second => "second",
    }
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "first" => Ok(Self::First),
      "second" => Ok(Self::Second),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Closed categorical outcomes in an actor action result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorActionResultOutcome {
  HeldSpace,
  YieldedSpace,
  ForcedOut,
}

impl ActorActionResultOutcome {
  pub const fn id(self) -> &'static str {
    match self {
      Self::HeldSpace => "held_space",
      Self::YieldedSpace => "yielded_space",
      Self::ForcedOut => "forced_out",
    }
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "held_space" => Ok(Self::HeldSpace),
      "yielded_space" => Ok(Self::YieldedSpace),
      "forced_out" => Ok(Self::ForcedOut),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }

  const fn from_lane_outcome(outcome: LaneOutcome) -> Self {
    match outcome {
      LaneOutcome::HeldSpace => Self::HeldSpace,
      LaneOutcome::YieldedSpace => Self::YieldedSpace,
      LaneOutcome::ForcedOut => Self::ForcedOut,
    }
  }
}

/// Bounded actor-safe result returned after a successful actor action.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorActionResultDto {
  schema: &'static str,
  window: ActorActionResultWindow,
  outcome: ActorActionResultOutcome,
}

impl ActorActionResultDto {
  pub const fn new(window: ActorActionResultWindow, outcome: ActorActionResultOutcome) -> Self {
    Self {
      schema: ACTOR_ACTION_RESULT_SCHEMA,
      window,
      outcome,
    }
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn window(self) -> ActorActionResultWindow {
    self.window
  }

  pub const fn outcome(self) -> ActorActionResultOutcome {
    self.outcome
  }

  /// Encode the bounded action result as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nwindow={}\noutcome={}\n",
      self.schema,
      self.window.id(),
      self.outcome.id()
    )
  }

  /// Decode a bounded action result without transition or history authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut window = None;
    let mut outcome = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "window" => &mut window,
        "outcome" => &mut outcome,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_ACTION_RESULT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self::new(
      ActorActionResultWindow::parse_id(window.ok_or(ActorProtocolCodecError::MissingField)?)?,
      ActorActionResultOutcome::parse_id(outcome.ok_or(ActorProtocolCodecError::MissingField)?)?,
    ))
  }
}

/// Closed objective dispositions in an actor-visible debrief summary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorDebriefObjective {
  GoalAchieved,
  GoalPartiallyAchieved,
  GoalMissed,
}

impl ActorDebriefObjective {
  pub const fn id(self) -> &'static str {
    match self {
      Self::GoalAchieved => "goal_achieved",
      Self::GoalPartiallyAchieved => "goal_partially_achieved",
      Self::GoalMissed => "goal_missed",
    }
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "goal_achieved" => Ok(Self::GoalAchieved),
      "goal_partially_achieved" => Ok(Self::GoalPartiallyAchieved),
      "goal_missed" => Ok(Self::GoalMissed),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }

  const fn from_lane_disposition(disposition: ObjectiveDisposition) -> Self {
    match disposition {
      ObjectiveDisposition::GoalAchieved => Self::GoalAchieved,
      ObjectiveDisposition::GoalPartiallyAchieved => Self::GoalPartiallyAchieved,
      ObjectiveDisposition::GoalMissed => Self::GoalMissed,
    }
  }
}

/// Static attribution boundary carried by an actor-visible debrief summary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorDebriefAttributionLimit {
  CommittedFactsOnly,
}

impl ActorDebriefAttributionLimit {
  pub const fn id(self) -> &'static str {
    "committed_facts_only"
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "committed_facts_only" => Ok(Self::CommittedFactsOnly),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// One fixed-window actor-visible debrief summary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorDebriefWindow {
  window: ActorActionResultWindow,
  intent: ActorProtocolIntent,
  outcome: ActorActionResultOutcome,
  objective: ActorDebriefObjective,
}

impl ActorDebriefWindow {
  pub const fn new(
    window: ActorActionResultWindow,
    intent: ActorProtocolIntent,
    outcome: ActorActionResultOutcome,
    objective: ActorDebriefObjective,
  ) -> Self {
    Self {
      window,
      intent,
      outcome,
      objective,
    }
  }

  pub const fn window(self) -> ActorActionResultWindow {
    self.window
  }

  pub const fn intent(self) -> ActorProtocolIntent {
    self.intent
  }

  pub const fn outcome(self) -> ActorActionResultOutcome {
    self.outcome
  }

  pub const fn objective(self) -> ActorDebriefObjective {
    self.objective
  }

  fn from_report_window(window: crate::lane::VisibleWindowDebriefSummary) -> Self {
    let window_id = match window.window() {
      ScenarioWindow::First => ActorActionResultWindow::First,
      ScenarioWindow::Second => ActorActionResultWindow::Second,
    };
    Self::new(
      window_id,
      ActorProtocolIntent::from_lane_intent(window.intent()),
      ActorActionResultOutcome::from_lane_outcome(window.outcome()),
      ActorDebriefObjective::from_lane_disposition(window.objective()),
    )
  }

  fn encode_value(self) -> String {
    format!(
      "{},{},{}",
      self.intent.id(),
      self.outcome.id(),
      self.objective.id()
    )
  }

  fn decode_value(
    window: ActorActionResultWindow,
    value: &str,
  ) -> Result<Self, ActorProtocolCodecError> {
    let parts = value.split(',').collect::<Vec<_>>();
    if parts.len() != 3 {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self::new(
      window,
      ActorProtocolIntent::parse_id(parts[0])?,
      ActorActionResultOutcome::parse_id(parts[1])?,
      ActorDebriefObjective::parse_id(parts[2])?,
    ))
  }
}

/// Bounded actor-visible committed-facts summary for the completed fixture.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorDebriefDto {
  schema: &'static str,
  first: ActorDebriefWindow,
  second: ActorDebriefWindow,
  final_objective: ActorDebriefObjective,
  attribution_limit: ActorDebriefAttributionLimit,
}

impl ActorDebriefDto {
  pub fn new(
    first: ActorDebriefWindow,
    second: ActorDebriefWindow,
    final_objective: ActorDebriefObjective,
  ) -> Result<Self, ActorProtocolCodecError> {
    if first.window() != ActorActionResultWindow::First
      || second.window() != ActorActionResultWindow::Second
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_DEBRIEF_SCHEMA,
      first,
      second,
      final_objective,
      attribution_limit: ActorDebriefAttributionLimit::CommittedFactsOnly,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn first(self) -> ActorDebriefWindow {
    self.first
  }

  pub const fn second(self) -> ActorDebriefWindow {
    self.second
  }

  pub const fn final_objective(self) -> ActorDebriefObjective {
    self.final_objective
  }

  pub const fn attribution_limit(self) -> ActorDebriefAttributionLimit {
    self.attribution_limit
  }

  pub(crate) fn from_report(report: ScenarioDebriefReport) -> Self {
    let windows = report.windows();
    Self::new(
      ActorDebriefWindow::from_report_window(windows[0]),
      ActorDebriefWindow::from_report_window(windows[1]),
      ActorDebriefObjective::from_lane_disposition(report.final_objective()),
    )
    .expect("scenario debrief report contains first and second windows")
  }

  /// Encode the completed-run summary as exact bounded line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nfirst={}\nsecond={}\nfinal_objective={}\nattribution={}\n",
      self.schema,
      self.first.encode_value(),
      self.second.encode_value(),
      self.final_objective.id(),
      self.attribution_limit.id(),
    )
  }

  /// Decode the completed-run summary without state, replay, or transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 5)?;
    let mut schema = None;
    let mut first = None;
    let mut second = None;
    let mut final_objective = None;
    let mut attribution = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "first" => &mut first,
        "second" => &mut second,
        "final_objective" => &mut final_objective,
        "attribution" => &mut attribution,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_DEBRIEF_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let first = ActorDebriefWindow::decode_value(
      ActorActionResultWindow::First,
      first.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    let second = ActorDebriefWindow::decode_value(
      ActorActionResultWindow::Second,
      second.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    let final_objective = ActorDebriefObjective::parse_id(
      final_objective.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    ActorDebriefAttributionLimit::parse_id(
      attribution.ok_or(ActorProtocolCodecError::MissingField)?,
    )?;
    Self::new(first, second, final_objective)
  }
}

/// Closed actor-draft metadata fields.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorDraftField {
  Message,
  Plan,
  Contingency,
}

impl ActorDraftField {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Message => "message",
      Self::Plan => "plan",
      Self::Contingency => "contingency",
    }
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "message" => Ok(Self::Message),
      "plan" => Ok(Self::Plan),
      "contingency" => Ok(Self::Contingency),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Versioned bounded actor message/plan/contingency metadata.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ActorDraftDto {
  schema: &'static str,
  observer: u8,
  observation_id: u64,
  field: ActorDraftField,
  value: String,
}

impl ActorDraftDto {
  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn observer(&self) -> u8 {
    self.observer
  }

  pub const fn observation_id(&self) -> u64 {
    self.observation_id
  }

  pub const fn field(&self) -> ActorDraftField {
    self.field
  }

  pub fn value(&self) -> &str {
    &self.value
  }

  /// Build bounded metadata without staging or submitting it to the host.
  pub fn new(
    observer: u8,
    observation_id: u64,
    field: ActorDraftField,
    value: &str,
  ) -> Result<Self, ActorProtocolCodecError> {
    if value.is_empty()
      || value.len() > MAX_ACTOR_DRAFT_VALUE_BYTES
      || value.chars().any(char::is_control)
      || (field == ActorDraftField::Plan && ActorProtocolIntent::parse_id(value).is_err())
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_DRAFT_SCHEMA,
      observer,
      observation_id,
      field,
      value: value.to_owned(),
    })
  }

  /// Encode bounded metadata as stable line-oriented text.
  pub fn encode(&self) -> String {
    format!(
      "schema={}\nobserver={}\nobservation_id={}\nfield={}\nvalue={}\n",
      self.schema,
      self.observer,
      self.observation_id,
      self.field.id(),
      self.value,
    )
  }

  /// Decode bounded metadata without assigning host or transition authority.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 5)?;
    let mut schema = None;
    let mut observer = None;
    let mut observation_id = None;
    let mut field = None;
    let mut value = None;
    for (key, field_value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "observer" => &mut observer,
        "observation_id" => &mut observation_id,
        "field" => &mut field,
        "value" => &mut value,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(field_value);
    }
    if schema != Some(ACTOR_DRAFT_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let observer = observer
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let observation_id = observation_id
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u64>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let field = ActorDraftField::parse_id(field.ok_or(ActorProtocolCodecError::MissingField)?)?;
    Self::new(
      observer,
      observation_id,
      field,
      value.ok_or(ActorProtocolCodecError::MissingField)?,
    )
  }
}

/// Closed actor-visible lifecycle status for the bounded fixture history.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorHistoryStatus {
  Open,
  Complete,
  Closed,
}

impl ActorHistoryStatus {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Open => "open",
      Self::Complete => "complete",
      Self::Closed => "closed",
    }
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "open" => Ok(Self::Open),
      "complete" => Ok(Self::Complete),
      "closed" => Ok(Self::Closed),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Bounded actor-visible history count and lifecycle status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorHistoryDto {
  schema: &'static str,
  records: u8,
  status: ActorHistoryStatus,
}

impl ActorHistoryDto {
  pub fn new(records: u8, status: ActorHistoryStatus) -> Result<Self, ActorProtocolCodecError> {
    if records > 2
      || (status == ActorHistoryStatus::Open && records == 2)
      || (status == ActorHistoryStatus::Complete && records != 2)
    {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    Ok(Self {
      schema: ACTOR_HISTORY_SCHEMA,
      records,
      status,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn records(self) -> u8 {
    self.records
  }

  pub const fn status(self) -> ActorHistoryStatus {
    self.status
  }

  /// Encode bounded history status as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\nrecords={}\nstatus={}\n",
      self.schema,
      self.records,
      self.status.id()
    )
  }

  /// Decode bounded history status without exposing state hashes or snapshots.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut records = None;
    let mut status = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "records" => &mut records,
        "status" => &mut status,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_HISTORY_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    let records = records
      .ok_or(ActorProtocolCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ActorProtocolCodecError::InvalidValue)?;
    let status =
      ActorHistoryStatus::parse_id(status.ok_or(ActorProtocolCodecError::MissingField)?)?;
    Self::new(records, status)
  }
}

/// Bounded protocol codec failures.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
}

/// Closed actor-facing validation-error categories.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolErrorCode {
  OversizedInput,
  UnexpectedLineCount,
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
  ActorMismatch,
  ObservationAlreadyOpen,
  NoObservation,
  StaleObservation,
  DuplicateSubmission,
  ClosedSession,
  WindowClosed,
  HostValidationRejected,
  HostTransitionRejected,
  DraftBoundary,
  DebriefUnavailable,
}

impl ActorProtocolErrorCode {
  pub const fn id(self) -> &'static str {
    match self {
      Self::OversizedInput => "oversized_input",
      Self::UnexpectedLineCount => "unexpected_line_count",
      Self::UnknownField => "unknown_field",
      Self::DuplicateField => "duplicate_field",
      Self::MissingField => "missing_field",
      Self::UnsupportedSchema => "unsupported_schema",
      Self::InvalidValue => "invalid_value",
      Self::ActorMismatch => "actor_mismatch",
      Self::ObservationAlreadyOpen => "observation_already_open",
      Self::NoObservation => "no_observation",
      Self::StaleObservation => "stale_observation",
      Self::DuplicateSubmission => "duplicate_submission",
      Self::ClosedSession => "closed_session",
      Self::WindowClosed => "window_closed",
      Self::HostValidationRejected => "host_validation_rejected",
      Self::HostTransitionRejected => "host_transition_rejected",
      Self::DraftBoundary => "draft_boundary",
      Self::DebriefUnavailable => "debrief_unavailable",
    }
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "oversized_input" => Ok(Self::OversizedInput),
      "unexpected_line_count" => Ok(Self::UnexpectedLineCount),
      "unknown_field" => Ok(Self::UnknownField),
      "duplicate_field" => Ok(Self::DuplicateField),
      "missing_field" => Ok(Self::MissingField),
      "unsupported_schema" => Ok(Self::UnsupportedSchema),
      "invalid_value" => Ok(Self::InvalidValue),
      "actor_mismatch" => Ok(Self::ActorMismatch),
      "observation_already_open" => Ok(Self::ObservationAlreadyOpen),
      "no_observation" => Ok(Self::NoObservation),
      "stale_observation" => Ok(Self::StaleObservation),
      "duplicate_submission" => Ok(Self::DuplicateSubmission),
      "closed_session" => Ok(Self::ClosedSession),
      "window_closed" => Ok(Self::WindowClosed),
      "host_validation_rejected" => Ok(Self::HostValidationRejected),
      "host_transition_rejected" => Ok(Self::HostTransitionRejected),
      "draft_boundary" => Ok(Self::DraftBoundary),
      "debrief_unavailable" => Ok(Self::DebriefUnavailable),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Deterministic caller guidance for one actor-facing validation failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ActorProtocolRepairHint {
  RetryWithinSizeBound,
  ResendExactPayload,
  ResendCompletePayload,
  UseSupportedSchema,
  ResendValidPayload,
  UseBoundActor,
  SubmitCurrentAction,
  RequestObservation,
  RequestFreshObservation,
  AwaitNextObservation,
  StartNewSession,
  ResendAdvertisedAction,
  AwaitCompletion,
}

impl ActorProtocolRepairHint {
  pub const fn id(self) -> &'static str {
    match self {
      Self::RetryWithinSizeBound => "retry_within_size_bound",
      Self::ResendExactPayload => "resend_exact_payload",
      Self::ResendCompletePayload => "resend_complete_payload",
      Self::UseSupportedSchema => "use_supported_schema",
      Self::ResendValidPayload => "resend_valid_payload",
      Self::UseBoundActor => "use_bound_actor",
      Self::SubmitCurrentAction => "submit_current_action",
      Self::RequestObservation => "request_observation",
      Self::RequestFreshObservation => "request_fresh_observation",
      Self::AwaitNextObservation => "await_next_observation",
      Self::StartNewSession => "start_new_session",
      Self::ResendAdvertisedAction => "resend_advertised_action",
      Self::AwaitCompletion => "await_completion",
    }
  }

  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "retry_within_size_bound" => Ok(Self::RetryWithinSizeBound),
      "resend_exact_payload" => Ok(Self::ResendExactPayload),
      "resend_complete_payload" => Ok(Self::ResendCompletePayload),
      "use_supported_schema" => Ok(Self::UseSupportedSchema),
      "resend_valid_payload" => Ok(Self::ResendValidPayload),
      "use_bound_actor" => Ok(Self::UseBoundActor),
      "submit_current_action" => Ok(Self::SubmitCurrentAction),
      "request_observation" => Ok(Self::RequestObservation),
      "request_fresh_observation" => Ok(Self::RequestFreshObservation),
      "await_next_observation" => Ok(Self::AwaitNextObservation),
      "start_new_session" => Ok(Self::StartNewSession),
      "resend_advertised_action" => Ok(Self::ResendAdvertisedAction),
      "await_completion" => Ok(Self::AwaitCompletion),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

/// Bounded actor-facing validation error with a deterministic repair hint.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ActorProtocolError {
  schema: &'static str,
  code: ActorProtocolErrorCode,
  repair: ActorProtocolRepairHint,
}

impl ActorProtocolError {
  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn code(self) -> ActorProtocolErrorCode {
    self.code
  }

  pub const fn repair(self) -> ActorProtocolRepairHint {
    self.repair
  }

  pub(crate) const fn new(code: ActorProtocolErrorCode, repair: ActorProtocolRepairHint) -> Self {
    Self {
      schema: ACTOR_PROTOCOL_ERROR_SCHEMA,
      code,
      repair,
    }
  }

  /// Encode the bounded actor-safe error as stable line-oriented text.
  pub fn encode(self) -> String {
    format!(
      "schema={}\ncode={}\nrepair={}\n",
      self.schema,
      self.code.id(),
      self.repair.id()
    )
  }

  /// Decode a bounded actor-safe error without raw payload or domain detail.
  pub fn decode(input: &str) -> Result<Self, ActorProtocolCodecError> {
    let fields = parse_fields(input, 3)?;
    let mut schema = None;
    let mut code = None;
    let mut repair = None;
    for (key, value) in fields {
      let slot = match key {
        "schema" => &mut schema,
        "code" => &mut code,
        "repair" => &mut repair,
        _ => return Err(ActorProtocolCodecError::UnknownField),
      };
      if slot.is_some() {
        return Err(ActorProtocolCodecError::DuplicateField);
      }
      *slot = Some(value);
    }
    if schema != Some(ACTOR_PROTOCOL_ERROR_SCHEMA) {
      return Err(ActorProtocolCodecError::UnsupportedSchema);
    }
    Ok(Self {
      schema: ACTOR_PROTOCOL_ERROR_SCHEMA,
      code: ActorProtocolErrorCode::parse_id(code.ok_or(ActorProtocolCodecError::MissingField)?)?,
      repair: ActorProtocolRepairHint::parse_id(
        repair.ok_or(ActorProtocolCodecError::MissingField)?,
      )?,
    })
  }
}

impl ActorProtocolCodecError {
  /// Project a codec failure without retaining input or parser details.
  pub const fn to_actor_error(self) -> ActorProtocolError {
    match self {
      Self::Oversized => ActorProtocolError::new(
        ActorProtocolErrorCode::OversizedInput,
        ActorProtocolRepairHint::RetryWithinSizeBound,
      ),
      Self::UnexpectedLineCount { .. } => ActorProtocolError::new(
        ActorProtocolErrorCode::UnexpectedLineCount,
        ActorProtocolRepairHint::ResendExactPayload,
      ),
      Self::UnknownField => ActorProtocolError::new(
        ActorProtocolErrorCode::UnknownField,
        ActorProtocolRepairHint::ResendExactPayload,
      ),
      Self::DuplicateField => ActorProtocolError::new(
        ActorProtocolErrorCode::DuplicateField,
        ActorProtocolRepairHint::ResendExactPayload,
      ),
      Self::MissingField => ActorProtocolError::new(
        ActorProtocolErrorCode::MissingField,
        ActorProtocolRepairHint::ResendCompletePayload,
      ),
      Self::UnsupportedSchema => ActorProtocolError::new(
        ActorProtocolErrorCode::UnsupportedSchema,
        ActorProtocolRepairHint::UseSupportedSchema,
      ),
      Self::InvalidValue => ActorProtocolError::new(
        ActorProtocolErrorCode::InvalidValue,
        ActorProtocolRepairHint::ResendValidPayload,
      ),
    }
  }
}

impl ActorProtocolIntent {
  fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "stabilize" => Ok(Self::Stabilize),
      "contest" => Ok(Self::Contest),
      "yield" => Ok(Self::Yield),
      "recall" => Ok(Self::Recall),
      "withdraw" => Ok(Self::Withdraw),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }
}

fn parse_fields(
  input: &str,
  expected_lines: usize,
) -> Result<Vec<(&str, &str)>, ActorProtocolCodecError> {
  if input.len() > MAX_ACTOR_PROTOCOL_BYTES {
    return Err(ActorProtocolCodecError::Oversized);
  }
  let actual_lines = input.lines().count();
  if actual_lines > expected_lines {
    return Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: expected_lines,
      actual: actual_lines,
    });
  }
  let mut fields = Vec::with_capacity(expected_lines);
  for line in input.lines() {
    let (key, value) = line
      .split_once('=')
      .ok_or(ActorProtocolCodecError::InvalidValue)?;
    if key.is_empty() || value.is_empty() {
      return Err(ActorProtocolCodecError::InvalidValue);
    }
    fields.push((key, value));
  }
  if fields.len() < expected_lines {
    return Ok(fields);
  }
  Ok(fields)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::lane::{
    JungleThreatTruth, LaneIntent, LaneSnapshot, LaneStatus, ObservationId, observe_player,
    validate_lane_request,
  };

  #[test]
  fn observation_dto_is_versioned_bounded_and_actor_visible() {
    let state = LaneSnapshot::initial();
    let observation = observe_player(&state, ObservationId::new(23)).observation();
    let dto = ActorObservationDto::from_observation(observation);

    assert_eq!(ACTOR_PROTOCOL_SCHEMA, "m5-actor-protocol-v1");
    assert_eq!(dto.schema(), "m5-actor-observation-v1");
    assert_eq!(dto.observer(), observation.observer().value());
    assert_eq!(dto.turn(), observation.turn().value());
    assert_eq!(dto.observation_id(), 23);
    assert_eq!(dto.available_actions().len(), 4);
    assert_eq!(
      dto.available_actions(),
      &[
        ActorProtocolIntent::Stabilize,
        ActorProtocolIntent::Contest,
        ActorProtocolIntent::Yield,
        ActorProtocolIntent::Recall,
      ]
    );
    assert!(dto.advertises(ActorProtocolIntent::Contest));
    assert!(!dto.advertises(ActorProtocolIntent::Withdraw));
    assert_eq!(dto.visible_threat_response(), None);
    assert!(!format!("{dto:?}").contains("StateHash"));
    assert!(!format!("{dto:?}").contains("LaneSnapshot"));
  }

  #[test]
  fn visible_threat_is_projected_as_one_additional_action() {
    let initial = LaneSnapshot::initial();
    let threat_state = LaneSnapshot::new(
      initial.ruleset(),
      initial.turn(),
      LaneStatus::Open,
      initial.player(),
      initial.opponent(),
      initial.wave(),
      JungleThreatTruth::RiverSide,
    );
    let observation = observe_player(&threat_state, ObservationId::new(24)).observation();
    let dto = ActorObservationDto::from_observation(observation);

    assert_eq!(dto.available_actions().len(), 5);
    assert_eq!(
      dto.visible_threat_response(),
      Some(ActorProtocolIntent::Withdraw)
    );
    assert_eq!(
      dto.available_actions().last(),
      Some(&ActorProtocolIntent::Withdraw)
    );
    assert_eq!(
      ActorObservationDto::decode(&dto.encode()).expect("threat observation decodes"),
      dto
    );
  }

  #[test]
  fn action_dto_round_trips_to_host_validated_intent_request() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(25));
    let dto = ActorActionDto::new(1, 25, ActorProtocolIntent::Contest);
    let request = dto.to_lane_request();

    assert_eq!(dto.schema(), "m5-actor-action-v1");
    assert_eq!(dto.intent().id(), "contest");
    assert_eq!(request.actor(), receipt.observation().observer());
    assert_eq!(
      request.observation_id(),
      receipt.observation().observation_id()
    );
    assert_eq!(request.intent(), LaneIntent::Contest);
    validate_lane_request(&state, &receipt, &request).expect("protocol request is host-valid");
  }

  #[test]
  fn actor_commit_command_and_result_codecs_are_observation_bound_and_closed() {
    let commit = ActorCommitDto::new(1, 41, ActorProtocolIntent::Contest);
    assert_eq!(commit.schema(), "m5-actor-commit-v1");
    assert_eq!(commit.observer(), 1);
    assert_eq!(commit.observation_id(), 41);
    assert_eq!(commit.intent(), ActorProtocolIntent::Contest);
    assert_eq!(
      commit.encode(),
      "schema=m5-actor-commit-v1\nobserver=1\nobservation_id=41\nintent=contest\n"
    );
    assert_eq!(ActorCommitDto::decode(&commit.encode()), Ok(commit));

    assert_eq!(
      ActorCommitDto::decode(
        "schema=m5-actor-commit-v1\nobserver=1\nobservation_id=41\nunknown=contest\n"
      ),
      Err(ActorProtocolCodecError::UnknownField)
    );
    assert_eq!(
      ActorCommitDto::decode("schema=m5-actor-commit-v1\nobserver=1\nobserver=1\nintent=contest\n"),
      Err(ActorProtocolCodecError::DuplicateField)
    );
    assert_eq!(
      ActorCommitDto::decode("schema=m5-actor-commit-v1\nobserver=1\nobservation_id=41\n"),
      Err(ActorProtocolCodecError::MissingField)
    );
    assert_eq!(
      ActorCommitDto::decode(
        "schema=m5-actor-commit-v0\nobserver=1\nobservation_id=41\nintent=contest\n"
      ),
      Err(ActorProtocolCodecError::UnsupportedSchema)
    );
    assert_eq!(
      ActorCommitDto::decode(
        "schema=m5-actor-commit-v1\nobserver=nope\nobservation_id=41\nintent=contest\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorCommitDto::decode(
        "schema=m5-actor-commit-v1\nobserver=1\nobservation_id=41\nintent=contest\nextra=x\n"
      ),
      Err(ActorProtocolCodecError::UnexpectedLineCount {
        expected: 4,
        actual: 5,
      })
    );

    let result = ActorCommitResultDto::new(ActorProtocolIntent::Contest);
    assert_eq!(result.schema(), "m5-actor-commit-result-v1");
    assert_eq!(
      result.encode(),
      "schema=m5-actor-commit-result-v1\nintent=contest\n"
    );
    assert_eq!(ActorCommitResultDto::decode(&result.encode()), Ok(result));
    assert_eq!(
      ActorCommitResultDto::decode("schema=m5-actor-commit-result-v1\nunknown=contest\n"),
      Err(ActorProtocolCodecError::UnknownField)
    );
    assert_eq!(
      ActorCommitResultDto::decode(
        "schema=m5-actor-commit-result-v1\nschema=m5-actor-commit-result-v1\n"
      ),
      Err(ActorProtocolCodecError::DuplicateField)
    );
    assert_eq!(
      ActorCommitResultDto::decode("schema=m5-actor-commit-result-v1\n"),
      Err(ActorProtocolCodecError::MissingField)
    );
    assert_eq!(
      ActorCommitResultDto::decode("schema=m5-actor-commit-result-v0\nintent=contest\n"),
      Err(ActorProtocolCodecError::UnsupportedSchema)
    );
    assert_eq!(
      ActorCommitResultDto::decode("schema=m5-actor-commit-result-v1\nintent=unknown\n"),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorCommitResultDto::decode("schema=m5-actor-commit-result-v1\nintent=contest\nextra=x\n"),
      Err(ActorProtocolCodecError::UnexpectedLineCount {
        expected: 2,
        actual: 3,
      })
    );
    assert!(!format!("{commit:?}").contains("StateHash"));
    assert!(!format!("{result:?}").contains("execution"));
  }

  #[test]
  fn actor_action_result_codec_round_trips_closed_window_and_outcome_ids() {
    let windows = [
      ActorActionResultWindow::First,
      ActorActionResultWindow::Second,
    ];
    let outcomes = [
      ActorActionResultOutcome::HeldSpace,
      ActorActionResultOutcome::YieldedSpace,
      ActorActionResultOutcome::ForcedOut,
    ];
    for window in windows {
      for outcome in outcomes {
        let dto = ActorActionResultDto::new(window, outcome);
        assert_eq!(dto.schema(), "m5-actor-action-result-v1");
        assert_eq!(ActorActionResultDto::decode(&dto.encode()), Ok(dto));
      }
    }
    let canonical = ActorActionResultDto::new(
      ActorActionResultWindow::First,
      ActorActionResultOutcome::HeldSpace,
    );
    assert_eq!(
      canonical.encode(),
      "schema=m5-actor-action-result-v1\nwindow=first\noutcome=held_space\n"
    );
    assert_eq!(
      ActorActionResultDto::decode(
        "schema=m5-actor-action-result-v1\nwindow=third\noutcome=held_space\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorActionResultDto::decode(
        "schema=m5-actor-action-result-v1\nwindow=first\noutcome=unknown\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert!(!format!("{canonical:?}").contains("hash"));
  }

  #[test]
  fn actor_debrief_codec_round_trips_committed_facts_summary() {
    let dto = ActorDebriefDto::new(
      ActorDebriefWindow::new(
        ActorActionResultWindow::First,
        ActorProtocolIntent::Contest,
        ActorActionResultOutcome::HeldSpace,
        ActorDebriefObjective::GoalAchieved,
      ),
      ActorDebriefWindow::new(
        ActorActionResultWindow::Second,
        ActorProtocolIntent::Stabilize,
        ActorActionResultOutcome::YieldedSpace,
        ActorDebriefObjective::GoalPartiallyAchieved,
      ),
      ActorDebriefObjective::GoalPartiallyAchieved,
    )
    .expect("window order is bounded");
    assert_eq!(dto.schema(), "m5-actor-debrief-v1");
    assert_eq!(
      dto.encode(),
      "schema=m5-actor-debrief-v1\nfirst=contest,held_space,goal_achieved\nsecond=stabilize,yielded_space,goal_partially_achieved\nfinal_objective=goal_partially_achieved\nattribution=committed_facts_only\n"
    );
    assert_eq!(ActorDebriefDto::decode(&dto.encode()), Ok(dto));
    assert_eq!(
      ActorDebriefDto::decode(
        "schema=m5-actor-debrief-v1\nfirst=contest,held_space,unknown\nsecond=stabilize,yielded_space,goal_missed\nfinal_objective=goal_missed\nattribution=committed_facts_only\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorDebriefDto::decode(
        "schema=m5-actor-debrief-v1\nfirst=contest,held_space,goal_achieved\nsecond=stabilize,yielded_space,goal_missed\nfinal_objective=goal_missed\nattribution=other\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorDebriefDto::decode(
        "schema=m5-actor-debrief-v1\nfirst=contest,held_space,goal_achieved\nfirst=stabilize,yielded_space,goal_missed\nfinal_objective=goal_missed\nattribution=committed_facts_only\n"
      ),
      Err(ActorProtocolCodecError::DuplicateField)
    );
    assert_eq!(
      ActorDebriefDto::decode(
        "schema=m5-actor-debrief-v1\nfirst=contest,held_space,goal_achieved\nsecond=stabilize,yielded_space,goal_missed\nfinal_objective=goal_missed\n"
      ),
      Err(ActorProtocolCodecError::MissingField)
    );
    assert_eq!(
      ActorDebriefDto::decode(
        "schema=m5-actor-debrief-v2\nfirst=contest,held_space,goal_achieved\nsecond=stabilize,yielded_space,goal_missed\nfinal_objective=goal_missed\nattribution=committed_facts_only\n"
      ),
      Err(ActorProtocolCodecError::UnsupportedSchema)
    );
    assert_eq!(
      ActorDebriefDto::decode(
        "schema=m5-actor-debrief-v1\nfirst=contest,held_space,goal_achieved\nsecond=stabilize,yielded_space,goal_missed\nfinal_objective=goal_missed\nattribution=committed_facts_only\nextra=x\n"
      ),
      Err(ActorProtocolCodecError::UnexpectedLineCount {
        expected: 5,
        actual: 6,
      })
    );
    assert!(!format!("{dto:?}").contains("StateHash"));
    assert!(!format!("{dto:?}").contains("trace"));
  }

  #[test]
  fn protocol_intent_ids_are_closed_and_stable() {
    assert_eq!(ActorProtocolIntent::Stabilize.id(), "stabilize");
    assert_eq!(ActorProtocolIntent::Contest.id(), "contest");
    assert_eq!(ActorProtocolIntent::Yield.id(), "yield");
    assert_eq!(ActorProtocolIntent::Recall.id(), "recall");
    assert_eq!(ActorProtocolIntent::Withdraw.id(), "withdraw");
  }

  #[test]
  fn protocol_dtos_round_trip_through_bounded_codec() {
    let state = LaneSnapshot::initial();
    let observation = ActorObservationDto::from_observation(
      observe_player(&state, ObservationId::new(32)).observation(),
    );
    let action = ActorActionDto::new(1, 32, ActorProtocolIntent::Contest);

    assert_eq!(
      ActorObservationDto::decode(&observation.encode()).expect("observation decodes"),
      observation
    );
    assert_eq!(
      ActorActionDto::decode(&action.encode()).expect("action decodes"),
      action
    );
    assert_eq!(ACTOR_PROTOCOL_CODEC_SCHEMA, "m5-actor-codec-v1");
  }

  #[test]
  fn actor_draft_dtos_round_trip_all_bounded_fields() {
    let cases = [
      (ActorDraftField::Message, "ping ally"),
      (ActorDraftField::Plan, "contest"),
      (ActorDraftField::Contingency, "retreat if threat"),
    ];
    for (field, value) in cases {
      let dto = ActorDraftDto::new(1, 36, field, value).expect("draft metadata is bounded");
      assert_eq!(dto.schema(), "m5-actor-draft-v1");
      assert_eq!(dto.field().id(), field.id());
      assert_eq!(dto.value(), value);
      if field == ActorDraftField::Message {
        assert_eq!(
          dto.encode(),
          "schema=m5-actor-draft-v1\nobserver=1\nobservation_id=36\nfield=message\nvalue=ping ally\n"
        );
      }
      assert_eq!(ActorDraftDto::decode(&dto.encode()), Ok(dto.clone()));
      assert!(!format!("{dto:?}").contains("hash"));
    }
  }

  #[test]
  fn actor_draft_codec_rejects_unbounded_or_noncanonical_values() {
    let max_value = "x".repeat(MAX_ACTOR_DRAFT_VALUE_BYTES);
    assert!(ActorDraftDto::new(1, 36, ActorDraftField::Message, &max_value).is_ok());
    assert_eq!(
      ActorDraftDto::new(1, 36, ActorDraftField::Message, ""),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorDraftDto::new(1, 36, ActorDraftField::Message, "line\nfeed"),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorDraftDto::new(
        1,
        36,
        ActorDraftField::Contingency,
        &"x".repeat(MAX_ACTOR_DRAFT_VALUE_BYTES + 1),
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorDraftDto::new(1, 36, ActorDraftField::Plan, "unknown"),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorDraftDto::decode(
        "schema=m5-actor-draft-v1\nobserver=1\nobservation_id=36\nfield=plan\nvalue=unknown\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
  }

  #[test]
  fn actor_history_codec_round_trips_bounded_lifecycle_statuses() {
    let cases = [
      (0, ActorHistoryStatus::Open),
      (1, ActorHistoryStatus::Open),
      (2, ActorHistoryStatus::Complete),
      (0, ActorHistoryStatus::Closed),
      (1, ActorHistoryStatus::Closed),
      (2, ActorHistoryStatus::Closed),
    ];
    for (records, status) in cases {
      let dto = ActorHistoryDto::new(records, status).expect("history status is bounded");
      assert_eq!(dto.schema(), "m5-actor-history-v1");
      assert_eq!(dto.records(), records);
      assert_eq!(dto.status(), status);
      if records == 0 && status == ActorHistoryStatus::Open {
        assert_eq!(
          dto.encode(),
          "schema=m5-actor-history-v1\nrecords=0\nstatus=open\n"
        );
      }
      assert_eq!(ActorHistoryDto::decode(&dto.encode()), Ok(dto));
    }
    for (records, status) in [
      (2, ActorHistoryStatus::Open),
      (0, ActorHistoryStatus::Complete),
      (1, ActorHistoryStatus::Complete),
      (3, ActorHistoryStatus::Open),
      (3, ActorHistoryStatus::Complete),
      (3, ActorHistoryStatus::Closed),
    ] {
      assert_eq!(
        ActorHistoryDto::new(records, status),
        Err(ActorProtocolCodecError::InvalidValue)
      );
    }
    assert_eq!(
      ActorHistoryDto::decode("schema=m5-actor-history-v1\nrecords=3\nstatus=closed\n"),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorHistoryDto::decode("schema=m5-actor-history-v1\nrecords=0\nstatus=unknown\n"),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorHistoryDto::decode("schema=m5-actor-history-v1\nrecords=0\nstatus=open\nextra=x\n"),
      Err(ActorProtocolCodecError::UnexpectedLineCount {
        expected: 3,
        actual: 4,
      })
    );
  }

  #[test]
  fn actor_error_codec_round_trips_all_closed_ids_without_raw_detail() {
    assert_eq!(ACTOR_PROTOCOL_ERROR_SCHEMA_V1, "m5-actor-error-v1");
    assert_eq!(ACTOR_PROTOCOL_ERROR_SCHEMA, "m5-actor-error-v2");
    let codes = [
      ActorProtocolErrorCode::OversizedInput,
      ActorProtocolErrorCode::UnexpectedLineCount,
      ActorProtocolErrorCode::UnknownField,
      ActorProtocolErrorCode::DuplicateField,
      ActorProtocolErrorCode::MissingField,
      ActorProtocolErrorCode::UnsupportedSchema,
      ActorProtocolErrorCode::InvalidValue,
      ActorProtocolErrorCode::ActorMismatch,
      ActorProtocolErrorCode::ObservationAlreadyOpen,
      ActorProtocolErrorCode::NoObservation,
      ActorProtocolErrorCode::StaleObservation,
      ActorProtocolErrorCode::DuplicateSubmission,
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolErrorCode::HostValidationRejected,
      ActorProtocolErrorCode::HostTransitionRejected,
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolErrorCode::DebriefUnavailable,
    ];
    for code in codes {
      let error = ActorProtocolError::new(code, ActorProtocolRepairHint::ResendValidPayload);
      assert_eq!(ActorProtocolError::decode(&error.encode()), Ok(error));
      assert!(!format!("{error:?}").contains("hash"));
    }
    let repairs = [
      ActorProtocolRepairHint::RetryWithinSizeBound,
      ActorProtocolRepairHint::ResendExactPayload,
      ActorProtocolRepairHint::ResendCompletePayload,
      ActorProtocolRepairHint::UseSupportedSchema,
      ActorProtocolRepairHint::ResendValidPayload,
      ActorProtocolRepairHint::UseBoundActor,
      ActorProtocolRepairHint::SubmitCurrentAction,
      ActorProtocolRepairHint::RequestObservation,
      ActorProtocolRepairHint::RequestFreshObservation,
      ActorProtocolRepairHint::AwaitNextObservation,
      ActorProtocolRepairHint::StartNewSession,
      ActorProtocolRepairHint::ResendAdvertisedAction,
      ActorProtocolRepairHint::AwaitCompletion,
    ];
    for repair in repairs {
      let error = ActorProtocolError::new(ActorProtocolErrorCode::InvalidValue, repair);
      assert_eq!(ActorProtocolError::decode(&error.encode()), Ok(error));
    }
    let canonical = ActorProtocolError::new(
      ActorProtocolErrorCode::StaleObservation,
      ActorProtocolRepairHint::RequestFreshObservation,
    );
    assert_eq!(
      canonical.encode(),
      "schema=m5-actor-error-v2\ncode=stale_observation\nrepair=request_fresh_observation\n"
    );
    assert_eq!(
      ActorProtocolError::decode(
        "schema=m5-actor-error-v2\ncode=unknown\nrepair=request_observation\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    let debrief_unavailable = ActorProtocolError::new(
      ActorProtocolErrorCode::DebriefUnavailable,
      ActorProtocolRepairHint::AwaitCompletion,
    );
    assert_eq!(
      debrief_unavailable.encode(),
      "schema=m5-actor-error-v2\ncode=debrief_unavailable\nrepair=await_completion\n"
    );
    assert_eq!(
      ActorProtocolError::decode(&debrief_unavailable.encode()),
      Ok(debrief_unavailable)
    );
    assert_eq!(
      ActorProtocolError::decode(
        "schema=m5-actor-error-v1\ncode=stale_observation\nrepair=request_fresh_observation\n"
      ),
      Err(ActorProtocolCodecError::UnsupportedSchema)
    );
    assert_eq!(
      ActorProtocolError::decode("schema=m5-actor-error-v2\ncode=invalid_value\nrepair=unknown\n"),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorProtocolError::decode(
        "schema=m5-actor-error-v2\ncode=invalid_value\nrepair=resend_valid_payload\nextra=x\n"
      ),
      Err(ActorProtocolCodecError::UnexpectedLineCount {
        expected: 3,
        actual: 4,
      })
    );
  }

  #[test]
  fn protocol_codec_rejects_unknown_duplicate_missing_and_invalid_fields() {
    let observation = "schema=m5-actor-observation-v1\nobserver=1\nturn=0\nobservation_id=33\nactions=stabilize,contest,yield,recall\nthreat=unknown\n";
    assert_eq!(
      ActorObservationDto::decode(&observation.replace("turn=0", "extra=x")),
      Err(ActorProtocolCodecError::UnknownField)
    );
    assert_eq!(
      ActorActionDto::decode("schema=m5-actor-action-v1\nobserver=1\nobserver=1\nintent=contest\n"),
      Err(ActorProtocolCodecError::DuplicateField)
    );
    assert_eq!(
      ActorActionDto::decode("schema=m5-actor-action-v1\nobserver=1\nintent=contest\n"),
      Err(ActorProtocolCodecError::MissingField)
    );
    assert_eq!(
      ActorActionDto::decode(
        "schema=m5-actor-action-v1\nobserver=1\nobservation_id=33\nintent=unknown\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorObservationDto::decode(
        "schema=m5-actor-observation-v1\nobserver=1\nturn=0\nobservation_id=33\nactions=stabilize,contest,yield,recall,withdraw\nthreat=contest\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
    assert_eq!(
      ActorObservationDto::decode(
        "schema=m5-actor-observation-v1\nobserver=1\nturn=0\nobservation_id=33\nactions=stabilize,contest,yield,withdraw\nthreat=unknown\n"
      ),
      Err(ActorProtocolCodecError::InvalidValue)
    );
  }

  #[test]
  fn protocol_codec_rejects_oversized_and_extra_lines_before_projection() {
    let oversized = "x".repeat(MAX_ACTOR_PROTOCOL_BYTES + 1);
    assert_eq!(
      ActorActionDto::decode(&oversized),
      Err(ActorProtocolCodecError::Oversized)
    );
    let extra =
      "schema=m5-actor-action-v1\nobserver=1\nobservation_id=34\nintent=contest\nextra=x\nmore=y\n";
    assert_eq!(
      ActorActionDto::decode(extra),
      Err(ActorProtocolCodecError::UnexpectedLineCount {
        expected: 4,
        actual: 6
      })
    );
  }

  #[test]
  fn codec_errors_project_to_bounded_repair_hints() {
    let cases = [
      (
        ActorProtocolCodecError::Oversized,
        "oversized_input",
        "retry_within_size_bound",
      ),
      (
        ActorProtocolCodecError::UnexpectedLineCount {
          expected: 4,
          actual: 6,
        },
        "unexpected_line_count",
        "resend_exact_payload",
      ),
      (
        ActorProtocolCodecError::UnknownField,
        "unknown_field",
        "resend_exact_payload",
      ),
      (
        ActorProtocolCodecError::DuplicateField,
        "duplicate_field",
        "resend_exact_payload",
      ),
      (
        ActorProtocolCodecError::MissingField,
        "missing_field",
        "resend_complete_payload",
      ),
      (
        ActorProtocolCodecError::UnsupportedSchema,
        "unsupported_schema",
        "use_supported_schema",
      ),
      (
        ActorProtocolCodecError::InvalidValue,
        "invalid_value",
        "resend_valid_payload",
      ),
    ];
    for (error, code, repair) in cases {
      let projected = error.to_actor_error();
      assert_eq!(projected.schema(), "m5-actor-error-v2");
      assert_eq!(projected.code().id(), code);
      assert_eq!(projected.repair().id(), repair);
      let debug = format!("{projected:?}");
      assert!(!debug.contains("input=") && !debug.contains("hash"));
    }
  }

  #[test]
  fn decoded_action_still_requires_host_validation() {
    let state = LaneSnapshot::initial();
    let receipt = observe_player(&state, ObservationId::new(35));
    let encoded = ActorActionDto::new(1, 35, ActorProtocolIntent::Contest).encode();
    let action = ActorActionDto::decode(&encoded).expect("action decodes");

    validate_lane_request(&state, &receipt, &action.to_lane_request())
      .expect("decoded action is accepted by host validator");
  }
}

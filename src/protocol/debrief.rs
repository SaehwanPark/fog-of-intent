//! Actor debrief summary and window DTOs.

use super::action::{ActorActionResultOutcome, ActorActionResultWindow};
use super::codec::{ActorProtocolCodecError, parse_fields};
use super::intents::ActorProtocolIntent;
use crate::lane::{ObjectiveDisposition, ScenarioDebriefReport, ScenarioWindow};

/// Versioned actor-visible completed-run debrief summary identity.
pub const ACTOR_DEBRIEF_SCHEMA: &str = "m5-actor-debrief-v1";

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

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
    match value {
      "goal_achieved" => Ok(Self::GoalAchieved),
      "goal_partially_achieved" => Ok(Self::GoalPartiallyAchieved),
      "goal_missed" => Ok(Self::GoalMissed),
      _ => Err(ActorProtocolCodecError::InvalidValue),
    }
  }

  pub(crate) const fn from_lane_disposition(disposition: ObjectiveDisposition) -> Self {
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

  pub(crate) fn parse_id(value: &str) -> Result<Self, ActorProtocolCodecError> {
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

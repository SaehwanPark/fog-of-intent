//! Fixed-fixture scenario selections, stress cases, frequency reports, and comparisons.

use super::experiment::ScriptedAgentExperimentManifest;
use super::operational::ScriptedAgentBuildId;
use super::policy::ScriptedAgent;
use super::profile::ScriptedAgentProfile;
use super::tally::{
  ScriptedAgentMatchedScenarioSample, ScriptedAgentMatchedScenarioSampleError,
  ScriptedAgentMatchedScenarioTallyReport,
};
use crate::kernel::ActorId;
use crate::lane::{
  JungleThreatTruth, LaneIntent, LaneSnapshot, LaneStatus, LanerObservation, ObservationId,
  observe_player,
};

/// Versioned identity for the closed fixture-scenario catalog.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA: &str =
  "m6-scripted-agent-fixture-scenarios-v1";

/// Stable ID for the no-threat fixture variant.
pub const SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID: &str = "safe-fixture-v1";

/// Stable ID for the visible RiverSide-threat fixture variant.
pub const SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID: &str = "river-side-threat-v1";

/// Versioned identity for a deterministic fixed-fixture population.
pub const SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA: &str =
  "m6-scripted-agent-fixture-population-v1";

/// Maximum number of selected fixed-fixture scenarios in one request.
pub const MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS: usize = 4;

/// Versioned identity for the bounded caller-declared stress-case matrix.
pub const SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA: &str = "m6-scripted-agent-stress-population-v1";

/// Versioned identity for bounded caller-declared degenerate-policy evidence.
pub const SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA: &str =
  "m6-scripted-agent-degenerate-policy-population-v1";

/// Maximum observations in one fixed degenerate-policy population.
pub const MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION: usize = 4;

/// Versioned identity for bounded fixed-fixture exploit-seeking evidence.
pub const SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA: &str =
  "m6-scripted-agent-exploit-seeking-population-v1";

/// Maximum observations in one fixed exploit-seeking policy population.
pub const MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION: usize = 4;

/// Versioned identity for bounded fixed-fixture scenario-frequency evidence.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA: &str =
  "m6-scripted-agent-fixture-frequency-v1";

/// Maximum encoded scenario-frequency report size before parsing/allocation.
pub const MAX_SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_BYTES: usize = 4096;

/// Integer basis-point scale for the bounded scenario distribution projection.
pub const SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE: u16 = 10_000;

/// Versioned identity for bounded fixed-fixture frequency baseline comparisons.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_COMPARISON_SCHEMA: &str =
  "m6-scripted-agent-fixture-frequency-compare-v1";

/// Stable identity for the fixed-fixture no-change regression gate.
pub const SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_REGRESSION_RULE: &str =
  "m6-fixed-frequency-no-change-v1";

/// Bounded failures from fixed-fixture scenario selection.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixtureScenarioSelectionError {
  EmptySelection,
  SelectionTooLarge { max: usize, actual: usize },
  UnknownScenario,
  MismatchedObservationPairCount { expected: usize, actual: usize },
  DuplicateObservationId,
}

/// Bounded failures from deterministic fixed-fixture population generation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixturePopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  ObservationIdOverflow,
  InvalidSelection(ScriptedAgentFixtureScenarioSelectionError),
}

/// Closed fixture variants available to the bounded M6 scenario selector.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixtureScenario {
  Safe,
  RiverSideThreat,
}

impl ScriptedAgentFixtureScenario {
  pub const fn id(self) -> &'static str {
    match self {
      Self::Safe => SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      Self::RiverSideThreat => SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    }
  }

  pub(crate) fn parse_id(value: &str) -> Result<Self, ScriptedAgentFixtureScenarioSelectionError> {
    match value {
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID => Ok(Self::Safe),
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID => Ok(Self::RiverSideThreat),
      _ => Err(ScriptedAgentFixtureScenarioSelectionError::UnknownScenario),
    }
  }

  pub(crate) fn observations(self, observation_ids: [ObservationId; 2]) -> [LanerObservation; 2] {
    let initial = LaneSnapshot::initial();
    let second = match self {
      Self::Safe => initial,
      Self::RiverSideThreat => LaneSnapshot::new(
        initial.ruleset(),
        initial.turn(),
        LaneStatus::Open,
        initial.player(),
        initial.opponent(),
        initial.wave(),
        JungleThreatTruth::RiverSide,
      ),
    };
    [
      observe_player(&initial, observation_ids[0]).observation(),
      observe_player(&second, observation_ids[1]).observation(),
    ]
  }
}

/// Ordered selection of caller-ID-bound fixed fixture scenarios.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioSelection {
  schema: &'static str,
  scenarios: Vec<ScriptedAgentFixtureScenario>,
  observation_ids: Vec<[ObservationId; 2]>,
}

impl ScriptedAgentFixtureScenarioSelection {
  /// Select closed fixture IDs and bind each to two distinct caller IDs.
  pub fn from_ids(
    scenario_ids: &[&str],
    observation_ids: &[[ObservationId; 2]],
  ) -> Result<Self, ScriptedAgentFixtureScenarioSelectionError> {
    if scenario_ids.is_empty() {
      return Err(ScriptedAgentFixtureScenarioSelectionError::EmptySelection);
    }
    if scenario_ids.len() > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS {
      return Err(
        ScriptedAgentFixtureScenarioSelectionError::SelectionTooLarge {
          max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
          actual: scenario_ids.len(),
        },
      );
    }
    if scenario_ids.len() != observation_ids.len() {
      return Err(
        ScriptedAgentFixtureScenarioSelectionError::MismatchedObservationPairCount {
          expected: scenario_ids.len(),
          actual: observation_ids.len(),
        },
      );
    }
    let mut scenarios = Vec::with_capacity(scenario_ids.len());
    for scenario_id in scenario_ids {
      let scenario = ScriptedAgentFixtureScenario::parse_id(scenario_id)?;
      scenarios.push(scenario);
    }
    let mut seen_ids = Vec::with_capacity(observation_ids.len() * 2);
    for pair in observation_ids {
      for observation_id in pair {
        if seen_ids.contains(observation_id) {
          return Err(ScriptedAgentFixtureScenarioSelectionError::DuplicateObservationId);
        }
        seen_ids.push(*observation_id);
      }
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA,
      scenarios,
      observation_ids: observation_ids.to_vec(),
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub fn scenarios(&self) -> &[ScriptedAgentFixtureScenario] {
    &self.scenarios
  }

  pub fn observation_ids(&self) -> &[[ObservationId; 2]] {
    &self.observation_ids
  }

  /// Build the actor-visible pairs in the selected order.
  pub fn observations(&self) -> Vec<[LanerObservation; 2]> {
    self
      .scenarios
      .iter()
      .copied()
      .zip(self.observation_ids.iter().copied())
      .map(|(scenario, observation_ids)| scenario.observations(observation_ids))
      .collect()
  }

  /// Compose the generated pairs through the existing verified sample path.
  pub fn matched_sample(
    &self,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<ScriptedAgentMatchedScenarioSample, ScriptedAgentMatchedScenarioSampleError> {
    let observations = self.observations();
    ScriptedAgentMatchedScenarioSample::from_observations(&observations, manifests)
  }
}

/// A deterministic population over the closed fixture catalog, bound to a
/// caller-supplied starting observation ID.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioPopulation {
  schema: &'static str,
  selection: ScriptedAgentFixtureScenarioSelection,
}

impl ScriptedAgentFixtureScenarioPopulation {
  /// Generate an alternating safe/threat population from a starting ID.
  pub fn generate(
    count: usize,
    first_observation_id: u64,
  ) -> Result<Self, ScriptedAgentFixturePopulationError> {
    if count == 0 {
      return Err(ScriptedAgentFixturePopulationError::EmptyPopulation);
    }
    if count > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS {
      return Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
        actual: count,
      });
    }
    let mut scenario_ids = Vec::with_capacity(count);
    for index in 0..count {
      scenario_ids.push(if index % 2 == 0 {
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID
      } else {
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID
      });
    }
    Self::generate_from_scenario_ids(&scenario_ids, first_observation_id)
  }

  /// Generate a caller-declared ordered composition from the closed catalog.
  pub fn generate_from_scenario_ids(
    scenario_ids: &[&str],
    first_observation_id: u64,
  ) -> Result<Self, ScriptedAgentFixturePopulationError> {
    if scenario_ids.is_empty() {
      return Err(ScriptedAgentFixturePopulationError::EmptyPopulation);
    }
    if scenario_ids.len() > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS {
      return Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
        actual: scenario_ids.len(),
      });
    }
    for scenario_id in scenario_ids {
      ScriptedAgentFixtureScenario::parse_id(scenario_id)
        .map_err(ScriptedAgentFixturePopulationError::InvalidSelection)?;
    }
    let mut observation_ids = Vec::with_capacity(scenario_ids.len());
    for index in 0..scenario_ids.len() {
      let offset = index
        .checked_mul(2)
        .and_then(|value| u64::try_from(value).ok())
        .ok_or(ScriptedAgentFixturePopulationError::ObservationIdOverflow)?;
      let first = first_observation_id
        .checked_add(offset)
        .ok_or(ScriptedAgentFixturePopulationError::ObservationIdOverflow)?;
      let second = first
        .checked_add(1)
        .ok_or(ScriptedAgentFixturePopulationError::ObservationIdOverflow)?;
      observation_ids.push([ObservationId::new(first), ObservationId::new(second)]);
    }
    let selection = ScriptedAgentFixtureScenarioSelection::from_ids(scenario_ids, &observation_ids)
      .map_err(ScriptedAgentFixturePopulationError::InvalidSelection)?;
    Ok(Self {
      schema: SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA,
      selection,
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub fn selection(&self) -> &ScriptedAgentFixtureScenarioSelection {
    &self.selection
  }

  pub fn scenarios(&self) -> &[ScriptedAgentFixtureScenario] {
    self.selection.scenarios()
  }

  pub fn observation_ids(&self) -> &[[ObservationId; 2]] {
    self.selection.observation_ids()
  }

  pub fn observations(&self) -> Vec<[LanerObservation; 2]> {
    self.selection.observations()
  }

  pub fn matched_sample(
    &self,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<ScriptedAgentMatchedScenarioSample, ScriptedAgentMatchedScenarioSampleError> {
    self.selection.matched_sample(manifests)
  }

  /// Aggregate the population's verified sample without rerunning policy evaluation.
  pub fn matched_tally(
    &self,
    manifests: &[ScriptedAgentExperimentManifest],
  ) -> Result<ScriptedAgentMatchedScenarioTallyReport, ScriptedAgentMatchedScenarioSampleError> {
    let sample = self.matched_sample(manifests)?;
    Ok(ScriptedAgentMatchedScenarioTallyReport::from_sample(
      &sample,
    ))
  }
}

/// Closed stress cases used by the bounded M6 population matrix.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentStressCase {
  IllegalCommand,
  ExploitSeeking,
  CommunicationAbuse,
  DegeneratePolicy,
}

impl ScriptedAgentStressCase {
  pub const fn id(self) -> &'static str {
    match self {
      Self::IllegalCommand => "illegal-command-v1",
      Self::ExploitSeeking => "exploit-seeking-v1",
      Self::CommunicationAbuse => "communication-abuse-v1",
      Self::DegeneratePolicy => "degenerate-policy-v1",
    }
  }

  pub const fn ordered() -> [Self; 4] {
    [
      Self::IllegalCommand,
      Self::ExploitSeeking,
      Self::CommunicationAbuse,
      Self::DegeneratePolicy,
    ]
  }
}

/// Closed categorical result IDs for the stress-case matrix.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentStressResult {
  HostValidationRejected,
  StaleObservation,
  MessageInvalidValue,
  RepeatedStabilize,
}

impl ScriptedAgentStressResult {
  pub const fn id(self) -> &'static str {
    match self {
      Self::HostValidationRejected => "host_validation_rejected",
      Self::StaleObservation => "stale_observation",
      Self::MessageInvalidValue => "message_invalid_value",
      Self::RepeatedStabilize => "repeated_stabilize",
    }
  }
}

/// One bounded caller-declared stress-case result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentStressPopulationEntry {
  case: ScriptedAgentStressCase,
  result: ScriptedAgentStressResult,
}

impl ScriptedAgentStressPopulationEntry {
  pub const fn case(self) -> ScriptedAgentStressCase {
    self.case
  }

  pub const fn result(self) -> ScriptedAgentStressResult {
    self.result
  }
}

/// Bounded caller-declared stress-case evidence over existing boundaries.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentStressPopulationReport {
  schema: &'static str,
  degenerate_stabilize_count: u8,
  entries: [ScriptedAgentStressPopulationEntry; 4],
}

/// Failures from constructing the closed stress-case report.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentStressPopulationError {
  UnexpectedResult,
  InvalidDegenerateCount,
}

impl ScriptedAgentStressPopulationReport {
  /// Bind the four expected categorical results and one degenerate count.
  pub fn from_results(
    results: [ScriptedAgentStressResult; 4],
    degenerate_stabilize_count: u8,
  ) -> Result<Self, ScriptedAgentStressPopulationError> {
    let expected = [
      ScriptedAgentStressResult::HostValidationRejected,
      ScriptedAgentStressResult::StaleObservation,
      ScriptedAgentStressResult::MessageInvalidValue,
      ScriptedAgentStressResult::RepeatedStabilize,
    ];
    if results != expected {
      return Err(ScriptedAgentStressPopulationError::UnexpectedResult);
    }
    if !(1..=MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS)
      .contains(&usize::from(degenerate_stabilize_count))
    {
      return Err(ScriptedAgentStressPopulationError::InvalidDegenerateCount);
    }
    let cases = ScriptedAgentStressCase::ordered();
    Ok(Self {
      schema: SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA,
      degenerate_stabilize_count,
      entries: [
        ScriptedAgentStressPopulationEntry {
          case: cases[0],
          result: results[0],
        },
        ScriptedAgentStressPopulationEntry {
          case: cases[1],
          result: results[1],
        },
        ScriptedAgentStressPopulationEntry {
          case: cases[2],
          result: results[2],
        },
        ScriptedAgentStressPopulationEntry {
          case: cases[3],
          result: results[3],
        },
      ],
    })
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn degenerate_stabilize_count(&self) -> u8 {
    self.degenerate_stabilize_count
  }

  pub const fn entries(&self) -> &[ScriptedAgentStressPopulationEntry; 4] {
    &self.entries
  }

  /// Render the bounded matrix without performing I/O.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Scripted Agent Stress Population\n\n- schema: {}\n- degenerate_stabilize_count: {}\n\n| case_id | result_id |\n| --- | --- |\n| {} | {} |\n| {} | {} |\n| {} | {} |\n| {} | {} |\n",
      self.schema,
      self.degenerate_stabilize_count,
      self.entries[0].case.id(),
      self.entries[0].result.id(),
      self.entries[1].case.id(),
      self.entries[1].result.id(),
      self.entries[2].case.id(),
      self.entries[2].result.id(),
      self.entries[3].case.id(),
      self.entries[3].result.id(),
    )
  }
}

/// Bounded failures from fixed degenerate-policy population construction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentDegeneratePolicyPopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  MismatchedObserver,
  DuplicateObservationId,
  UnexpectedIntent,
}

/// Verified caller-declared population whose cautious policy repeats one intent.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentDegeneratePolicyPopulationReport {
  schema: &'static str,
  profile_id: &'static str,
  evaluation_rule: &'static str,
  observer: ActorId,
  observation_count: u8,
  selected_intent: LaneIntent,
}

impl ScriptedAgentDegeneratePolicyPopulationReport {
  /// Build fixed degenerate evidence from actor-visible observations only.
  pub fn from_observations(
    observations: &[LanerObservation],
  ) -> Result<Self, ScriptedAgentDegeneratePolicyPopulationError> {
    if observations.is_empty() {
      return Err(ScriptedAgentDegeneratePolicyPopulationError::EmptyPopulation);
    }
    if observations.len() > MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION {
      return Err(
        ScriptedAgentDegeneratePolicyPopulationError::PopulationTooLarge {
          max: MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION,
          actual: observations.len(),
        },
      );
    }
    let observer = observations[0].observer();
    let mut seen_ids = Vec::with_capacity(observations.len());
    for observation in observations {
      if observation.observer() != observer {
        return Err(ScriptedAgentDegeneratePolicyPopulationError::MismatchedObserver);
      }
      if seen_ids.contains(&observation.observation_id()) {
        return Err(ScriptedAgentDegeneratePolicyPopulationError::DuplicateObservationId);
      }
      seen_ids.push(observation.observation_id());
      if ScriptedAgent::cautious_v1()
        .choose(*observation)
        .selected_intent()
        != LaneIntent::Stabilize
      {
        return Err(ScriptedAgentDegeneratePolicyPopulationError::UnexpectedIntent);
      }
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA,
      profile_id: ScriptedAgentProfile::cautious_v1().profile_id(),
      evaluation_rule: ScriptedAgentProfile::cautious_v1().evaluation_rule(),
      observer,
      observation_count: u8::try_from(observations.len()).expect("population cap fits in u8"),
      selected_intent: LaneIntent::Stabilize,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn observer(self) -> ActorId {
    self.observer
  }

  pub const fn observation_count(self) -> u8 {
    self.observation_count
  }

  pub const fn selected_intent(self) -> LaneIntent {
    self.selected_intent
  }
}

/// Bounded failures from fixed exploit-seeking policy population construction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentExploitSeekingPopulationError {
  EmptyPopulation,
  PopulationTooLarge { max: usize, actual: usize },
  MismatchedObserver,
  DuplicateObservationId,
  UnexpectedIntent,
}

/// Verified caller-declared population whose risk-taking policy selects Contest.
///
/// This is fixed-fixture selected-intent evidence only; it does not search for
/// exploits or establish adversarial prevalence, outcomes, or strategy quality.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentExploitSeekingPopulationReport {
  schema: &'static str,
  profile_id: &'static str,
  evaluation_rule: &'static str,
  observer: ActorId,
  observation_count: u8,
  selected_intent: LaneIntent,
}

impl ScriptedAgentExploitSeekingPopulationReport {
  /// Build fixed risk-taking evidence from actor-visible observations only.
  pub fn from_observations(
    observations: &[LanerObservation],
  ) -> Result<Self, ScriptedAgentExploitSeekingPopulationError> {
    if observations.is_empty() {
      return Err(ScriptedAgentExploitSeekingPopulationError::EmptyPopulation);
    }
    if observations.len() > MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION {
      return Err(
        ScriptedAgentExploitSeekingPopulationError::PopulationTooLarge {
          max: MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION,
          actual: observations.len(),
        },
      );
    }
    let observer = observations[0].observer();
    let mut seen_ids = Vec::with_capacity(observations.len());
    for observation in observations {
      if observation.observer() != observer {
        return Err(ScriptedAgentExploitSeekingPopulationError::MismatchedObserver);
      }
      if seen_ids.contains(&observation.observation_id()) {
        return Err(ScriptedAgentExploitSeekingPopulationError::DuplicateObservationId);
      }
      seen_ids.push(observation.observation_id());
      if ScriptedAgent::risk_taking_v1()
        .choose(*observation)
        .selected_intent()
        != LaneIntent::Contest
      {
        return Err(ScriptedAgentExploitSeekingPopulationError::UnexpectedIntent);
      }
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA,
      profile_id: ScriptedAgentProfile::risk_taking_v1().profile_id(),
      evaluation_rule: ScriptedAgentProfile::risk_taking_v1().evaluation_rule(),
      observer,
      observation_count: u8::try_from(observations.len()).expect("population cap fits in u8"),
      selected_intent: LaneIntent::Contest,
    })
  }

  pub const fn schema(self) -> &'static str {
    self.schema
  }

  pub const fn profile_id(self) -> &'static str {
    self.profile_id
  }

  pub const fn evaluation_rule(self) -> &'static str {
    self.evaluation_rule
  }

  pub const fn observer(self) -> ActorId {
    self.observer
  }

  pub const fn observation_count(self) -> u8 {
    self.observation_count
  }

  pub const fn selected_intent(self) -> LaneIntent {
    self.selected_intent
  }
}

/// One closed fixture-scenario frequency row.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyEntry {
  pub(crate) scenario_id: &'static str,
  pub(crate) count: u8,
}

impl ScriptedAgentFixtureScenarioFrequencyEntry {
  pub const fn scenario_id(self) -> &'static str {
    self.scenario_id
  }

  pub const fn count(self) -> u8 {
    self.count
  }
}

/// Bounded failures from a labeled frequency comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentBuildComparisonError {
  MatchingBuildIds,
}

/// Bounded frequency evidence over one validated fixed-fixture selection.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyReport {
  schema: &'static str,
  selection_count: u8,
  entries: [ScriptedAgentFixtureScenarioFrequencyEntry; 2],
}

/// Bounded failures from the scenario-frequency codec.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ScriptedAgentFixtureScenarioFrequencyCodecError {
  Oversized,
  UnexpectedLineCount { expected: usize, actual: usize },
  UnknownField,
  DuplicateField,
  MissingField,
  UnsupportedSchema,
  InvalidValue,
  InputMismatch,
}

impl ScriptedAgentFixtureScenarioFrequencyReport {
  /// Count explicit scenario selections without rerunning policy evaluation.
  pub fn from_selection(selection: &ScriptedAgentFixtureScenarioSelection) -> Self {
    let mut safe_count = 0_u8;
    let mut river_side_count = 0_u8;
    for scenario in selection.scenarios() {
      match scenario {
        ScriptedAgentFixtureScenario::Safe => safe_count += 1,
        ScriptedAgentFixtureScenario::RiverSideThreat => river_side_count += 1,
      }
    }
    Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA,
      selection_count: safe_count + river_side_count,
      entries: [
        ScriptedAgentFixtureScenarioFrequencyEntry {
          scenario_id: SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
          count: safe_count,
        },
        ScriptedAgentFixtureScenarioFrequencyEntry {
          scenario_id: SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
          count: river_side_count,
        },
      ],
    }
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn selection_count(&self) -> u8 {
    self.selection_count
  }

  pub const fn entries(&self) -> &[ScriptedAgentFixtureScenarioFrequencyEntry; 2] {
    &self.entries
  }

  /// Encode the verified frequency report as bounded line-oriented text.
  pub fn encode(&self) -> String {
    format!(
      "schema={}\nselection_count={}\nentries=2\nrow={}|{}\nrow={}|{}\n",
      self.schema,
      self.selection_count,
      self.entries[0].scenario_id,
      self.entries[0].count,
      self.entries[1].scenario_id,
      self.entries[1].count,
    )
  }

  /// Render the verified report as a concise, non-persistent Markdown summary.
  pub fn to_markdown(&self) -> String {
    format!(
      "# Scenario Frequency\n\n- schema: {}\n- selection_count: {}\n\n| scenario_id | count |\n| --- | ---: |\n| {} | {} |\n| {} | {} |\n",
      self.schema,
      self.selection_count,
      self.entries[0].scenario_id,
      self.entries[0].count,
      self.entries[1].scenario_id,
      self.entries[1].count,
    )
  }

  /// Return ordered integer basis-point shares at a 10,000-point scale.
  ///
  /// The first row uses floor division and the second row receives the
  /// remainder, so the two shares always sum to exactly 10,000.
  pub fn distribution_basis_points(&self) -> [u16; 2] {
    let selection_count = u16::from(self.selection_count);
    let first = u16::from(self.entries[0].count) * SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE
      / selection_count;
    [first, SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE - first]
  }

  /// Render the bounded distribution projection without performing I/O.
  pub fn to_distribution_markdown(&self) -> String {
    let shares = self.distribution_basis_points();
    format!(
      "# Scenario Distribution\n\n- schema: {}\n- selection_count: {}\n- share_scale_basis_points: {}\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| {} | {} | {} |\n| {} | {} | {} |\n",
      self.schema,
      self.selection_count,
      SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE,
      self.entries[0].scenario_id,
      self.entries[0].count,
      shares[0],
      self.entries[1].scenario_id,
      self.entries[1].count,
      shares[1],
    )
  }

  /// Decode and validate a report against an already verified value.
  pub fn decode(
    input: &str,
    expected: &Self,
  ) -> Result<Self, ScriptedAgentFixtureScenarioFrequencyCodecError> {
    let decoded = Self::decode_unverified(input)?;
    if decoded != *expected {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InputMismatch);
    }
    Ok(decoded)
  }

  fn decode_unverified(
    input: &str,
  ) -> Result<Self, ScriptedAgentFixtureScenarioFrequencyCodecError> {
    if input.len() > MAX_SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_BYTES {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::Oversized);
    }
    let lines = input.lines().collect::<Vec<_>>();
    let mut schema = None;
    let mut selection_count = None;
    let mut entries_count = None;
    let mut rows = Vec::new();
    for line in lines.iter() {
      let (key, value) = line
        .split_once('=')
        .ok_or(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
      if key.is_empty() || value.is_empty() {
        return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
      }
      match key {
        "schema" => {
          if schema.is_some() {
            return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField);
          }
          schema = Some(value);
        }
        "selection_count" => {
          if selection_count.is_some() {
            return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField);
          }
          selection_count = Some(value);
        }
        "entries" => {
          if entries_count.is_some() {
            return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField);
          }
          entries_count = Some(value);
        }
        "row" => rows.push(value),
        _ => return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::UnknownField),
      }
    }
    if schema != Some(SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA) {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::UnsupportedSchema);
    }
    let selection_count = selection_count
      .ok_or(ScriptedAgentFixtureScenarioFrequencyCodecError::MissingField)?
      .parse::<u8>()
      .map_err(|_| ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
    if !(1..=MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS).contains(&usize::from(selection_count)) {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
    }
    let entries_count = entries_count
      .ok_or(ScriptedAgentFixtureScenarioFrequencyCodecError::MissingField)?
      .parse::<usize>()
      .map_err(|_| ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
    if entries_count != 2 {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
    }
    if lines.len() != 5 || rows.len() != 2 {
      return Err(
        ScriptedAgentFixtureScenarioFrequencyCodecError::UnexpectedLineCount {
          expected: 5,
          actual: lines.len(),
        },
      );
    }
    let mut entries = [
      ScriptedAgentFixtureScenarioFrequencyEntry {
        scenario_id: SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        count: 0,
      },
      ScriptedAgentFixtureScenarioFrequencyEntry {
        scenario_id: SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        count: 0,
      },
    ];
    for (index, row) in rows.into_iter().enumerate() {
      let fields = row.split('|').collect::<Vec<_>>();
      if fields.len() != 2 {
        return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
      }
      if fields[0] != entries[index].scenario_id {
        return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
      }
      entries[index].count = fields[1]
        .parse::<u8>()
        .map_err(|_| ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue)?;
    }
    if u16::from(entries[0].count) + u16::from(entries[1].count) != u16::from(selection_count) {
      return Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue);
    }
    Ok(Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA,
      selection_count,
      entries,
    })
  }
}

/// One actor-safe row in a declared baseline comparison.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
  scenario_id: &'static str,
  baseline_count: u8,
  candidate_count: u8,
}

impl ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
  pub const fn scenario_id(self) -> &'static str {
    self.scenario_id
  }

  pub const fn baseline_count(self) -> u8 {
    self.baseline_count
  }

  pub const fn candidate_count(self) -> u8 {
    self.candidate_count
  }

  pub fn delta(self) -> i16 {
    i16::from(self.candidate_count) - i16::from(self.baseline_count)
  }
}

/// Bounded row deltas between two caller-declared verified frequency reports.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScriptedAgentFixtureScenarioFrequencyComparisonReport {
  schema: &'static str,
  baseline_build_id: Option<ScriptedAgentBuildId>,
  candidate_build_id: Option<ScriptedAgentBuildId>,
  baseline_selection_count: u8,
  candidate_selection_count: u8,
  entries: [ScriptedAgentFixtureScenarioFrequencyComparisonEntry; 2],
}

impl ScriptedAgentFixtureScenarioFrequencyComparisonReport {
  /// Compare two verified reports without rerunning selection or policy code.
  pub fn from_reports(
    baseline: &ScriptedAgentFixtureScenarioFrequencyReport,
    candidate: &ScriptedAgentFixtureScenarioFrequencyReport,
  ) -> Self {
    Self {
      schema: SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_COMPARISON_SCHEMA,
      baseline_build_id: None,
      candidate_build_id: None,
      baseline_selection_count: baseline.selection_count,
      candidate_selection_count: candidate.selection_count,
      entries: [
        ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
          scenario_id: baseline.entries[0].scenario_id,
          baseline_count: baseline.entries[0].count,
          candidate_count: candidate.entries[0].count,
        },
        ScriptedAgentFixtureScenarioFrequencyComparisonEntry {
          scenario_id: baseline.entries[1].scenario_id,
          baseline_count: baseline.entries[1].count,
          candidate_count: candidate.entries[1].count,
        },
      ],
    }
  }

  /// Compare verified reports while retaining distinct caller-declared labels.
  pub fn from_reports_with_build_ids(
    baseline: &ScriptedAgentFixtureScenarioFrequencyReport,
    candidate: &ScriptedAgentFixtureScenarioFrequencyReport,
    baseline_build_id: ScriptedAgentBuildId,
    candidate_build_id: ScriptedAgentBuildId,
  ) -> Result<Self, ScriptedAgentBuildComparisonError> {
    if baseline_build_id == candidate_build_id {
      return Err(ScriptedAgentBuildComparisonError::MatchingBuildIds);
    }
    let mut comparison = Self::from_reports(baseline, candidate);
    comparison.baseline_build_id = Some(baseline_build_id);
    comparison.candidate_build_id = Some(candidate_build_id);
    Ok(comparison)
  }

  pub const fn schema(&self) -> &'static str {
    self.schema
  }

  pub const fn baseline_build_id(&self) -> Option<ScriptedAgentBuildId> {
    self.baseline_build_id
  }

  pub const fn candidate_build_id(&self) -> Option<ScriptedAgentBuildId> {
    self.candidate_build_id
  }

  pub const fn baseline_selection_count(&self) -> u8 {
    self.baseline_selection_count
  }

  pub const fn candidate_selection_count(&self) -> u8 {
    self.candidate_selection_count
  }

  pub const fn regression_rule(&self) -> &'static str {
    SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_REGRESSION_RULE
  }

  pub const fn passes_no_change_gate(&self) -> bool {
    self.baseline_selection_count == self.candidate_selection_count
      && self.entries[0].baseline_count == self.entries[0].candidate_count
      && self.entries[1].baseline_count == self.entries[1].candidate_count
  }

  pub const fn entries(&self) -> &[ScriptedAgentFixtureScenarioFrequencyComparisonEntry; 2] {
    &self.entries
  }
}

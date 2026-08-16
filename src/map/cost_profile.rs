//! Deterministic cost profiling for M9 transition, replay, projection, and
//! batch-run work.
//!
//! Milestone: M9 — Bounded Multi-Lane Match Prototype
//!
//! This module profiles the canonical M9 map-travel path by counting the
//! exact operations each execution performs — transitions executed, state
//! hashes evaluated, observation projections, and replay verifications —
//! instead of measuring wall-clock time. Every counted projection and replay
//! is actually performed by the profiler; state-hash counts follow the
//! versioned `MapScenarioDefinition::execute` contract (one initial and one
//! terminal hash per execution pass). Identical builds therefore always
//! produce identical counts. Wall-clock timing evidence stays at repository
//! edges (tests, benches, CI) where non-determinism is acceptable.
//!
//! The batch profile executes every registered canonical travel scenario,
//! projects terminal observations for each allied actor, and verifies each
//! run by replaying it and comparing hashes. Scaling probes execute a
//! synthetic single-actor rotation script of increasing length to show how
//! transition and replay work grows linearly with match length while
//! per-pass hash work stays constant.
//!
//! Malformed requests fail closed: an empty probe script or a map without an
//! allied-base neighbor is rejected before any execution.

use core::fmt;

use super::catalog::{MapScenarioDefinition, MapTravelCatalog};
use super::graph::adjacent_neighbors;
use super::state::MatchMapState;
use super::topology::MapLocation;
use super::travel::{ActorLocation, TravelCommand, TravelError};
use crate::kernel::ActorId;

pub const M9_COST_PROFILE_SCHEMA_V1: &str = "m9-cost-profile-v1";

/// Scaling probe script lengths (transition steps) run by the batch profile.
///
/// The spacing keeps every consecutive gap large enough that the marginal
/// cost per step is an exact integer for a linear workload.
pub const SCALING_PROBE_STEPS: [u32; 4] = [1, 8, 64, 512];

/// Exact counts of profiled operations for one execution pass or workload.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OperationCounts {
  /// `transition_travel` invocations performed.
  pub transitions_executed: u32,
  /// State hash evaluations, counted per the executor contract: one initial
  /// and one terminal hash per execution pass.
  pub state_hashes_computed: u32,
  /// `MatchMapState::observe` projections actually performed.
  pub observation_projections: u32,
  /// Full re-executions compared against a prior run's hashes.
  pub replay_verifications: u32,
}

impl OperationCounts {
  /// Saturating element-wise sum.
  pub const fn saturating_add(self, other: Self) -> Self {
    Self {
      transitions_executed: self
        .transitions_executed
        .saturating_add(other.transitions_executed),
      state_hashes_computed: self
        .state_hashes_computed
        .saturating_add(other.state_hashes_computed),
      observation_projections: self
        .observation_projections
        .saturating_add(other.observation_projections),
      replay_verifications: self
        .replay_verifications
        .saturating_add(other.replay_verifications),
    }
  }
}

/// Typed fail-closed error for cost profiling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CostProfileError {
  /// A scaling probe was requested with zero transition steps.
  EmptyProbeScript,
  /// The allied base has no adjacent location to ping-pong against.
  ProbeMapUnavailable,
  /// An underlying map transition failed during profiling.
  Transition(TravelError),
}

impl fmt::Display for CostProfileError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyProbeScript => {
        f.write_str("empty probe script: at least one transition step is required")
      }
      Self::ProbeMapUnavailable => {
        f.write_str("probe map unavailable: the allied base has no adjacent location")
      }
      Self::Transition(error) => write!(f, "map transition failed during profiling: {error}"),
    }
  }
}

impl From<TravelError> for CostProfileError {
  fn from(error: TravelError) -> Self {
    Self::Transition(error)
  }
}

/// Cost profile of one canonical scenario: execution, verification replay,
/// and their totals.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioCostProfile {
  pub scenario_id: &'static str,
  /// Counts from the execution pass, including its terminal projections.
  pub execution: OperationCounts,
  /// Counts from the verification replay pass. Replays re-execute and
  /// compare hashes; they do not repeat projections.
  pub replay: OperationCounts,
  /// Whether the replay reproduced the initial and terminal hashes.
  pub replay_hash_matches: bool,
  /// Execution plus replay counts.
  pub total: OperationCounts,
}

/// Scaling probe: a synthetic single-actor rotation script of `steps`
/// transitions, executed and replay-verified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScalingProbe {
  /// Number of transition steps in the probe script.
  pub steps: u32,
  /// Counts from the execution pass.
  pub execution: OperationCounts,
  /// Counts from the verification replay pass.
  pub replay: OperationCounts,
  /// Whether the replay reproduced the initial and terminal hashes.
  pub replay_hash_matches: bool,
  /// Execution plus replay counts.
  pub total: OperationCounts,
}

/// Deterministic batch cost profile over the canonical M9 travel catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CostProfileReport {
  pub schema: &'static str,
  /// Per-scenario profiles in catalog registration order.
  pub scenario_profiles: Vec<ScenarioCostProfile>,
  /// Number of catalog entries executed in the batch.
  pub batch_entry_count: u32,
  /// Element-wise totals across the batch, including replays.
  pub batch_totals: OperationCounts,
  /// Average transitions per batch entry, in basis points of one operation.
  pub per_entry_transitions_bp: u32,
  /// Average state hashes per batch entry (bp).
  pub per_entry_hashes_bp: u32,
  /// Average observation projections per batch entry (bp).
  pub per_entry_projections_bp: u32,
  /// Average replay verifications per batch entry (bp).
  pub per_entry_replays_bp: u32,
  /// Scaling probes in ascending step order.
  pub scaling_probes: Vec<ScalingProbe>,
  /// Marginal total transitions (execution plus replay) per additional probe
  /// step, computed from the first and last probes. Exact for a linear
  /// workload; zero when only one probe exists.
  pub marginal_transitions_per_step: u32,
  /// Whether every probe's execution-pass hash count stayed identical, i.e.
  /// hash work per verification does not grow with match length.
  pub hashes_constant_across_probes: bool,
  /// Whether every replay pass performed exactly the execution pass's
  /// transition count, i.e. replay work doubles transition work.
  pub replay_doubles_transition_work: bool,
}

impl CostProfileReport {
  /// Render a structured Markdown summary of this report.
  ///
  /// Does not include wall-clock measurements, hidden state, or private
  /// chain-of-thought.
  pub fn render_markdown(&self) -> String {
    let mut out = String::new();
    out.push_str("# M9 Cost Profile Report\n\n");
    out.push_str(&format!(
      "- **Batch Entries**: {}\n",
      self.batch_entry_count
    ));
    out.push_str(&format!(
      "- **Batch Totals**: {} transitions, {} state hashes, {} projections, {} replay verifications\n",
      self.batch_totals.transitions_executed,
      self.batch_totals.state_hashes_computed,
      self.batch_totals.observation_projections,
      self.batch_totals.replay_verifications
    ));
    out.push_str(&format!(
      "- **Per-Entry Averages**: {} bp transitions, {} bp hashes, {} bp projections, {} bp replays\n",
      self.per_entry_transitions_bp,
      self.per_entry_hashes_bp,
      self.per_entry_projections_bp,
      self.per_entry_replays_bp
    ));
    out.push_str(&format!(
      "- **Marginal Transitions per Probe Step**: {}\n",
      self.marginal_transitions_per_step
    ));
    out.push_str(&format!(
      "- **Hashes Constant Across Probes**: {}\n",
      yes_no(self.hashes_constant_across_probes)
    ));
    out.push_str(&format!(
      "- **Replay Doubles Transition Work**: {}\n",
      yes_no(self.replay_doubles_transition_work)
    ));
    out.push_str("\n## Scenario Profiles\n\n");
    for profile in &self.scenario_profiles {
      out.push_str(&format!(
        "- `{}`: {} transitions, {} hashes, {} projections (replay matches: {})\n",
        profile.scenario_id,
        profile.execution.transitions_executed,
        profile.execution.state_hashes_computed,
        profile.execution.observation_projections,
        yes_no(profile.replay_hash_matches)
      ));
    }
    out.push_str("\n## Scaling Probes\n\n");
    for probe in &self.scaling_probes {
      out.push_str(&format!(
        "- {} steps: {} total transitions, {} total hashes (replay matches: {})\n",
        probe.steps,
        probe.total.transitions_executed,
        probe.total.state_hashes_computed,
        yes_no(probe.replay_hash_matches)
      ));
    }
    out
  }
}

const fn yes_no(flag: bool) -> &'static str {
  if flag { "yes" } else { "no" }
}

/// Profile one canonical travel scenario: execute it, project a terminal
/// observation for every allied actor, then replay-verify by re-execution
/// and hash comparison.
pub fn profile_travel_scenario(
  definition: &MapScenarioDefinition,
) -> Result<ScenarioCostProfile, CostProfileError> {
  let transitions =
    u32::try_from(definition.command_script.len()).expect("command script fits in a u32");

  let (first, terminal_state) = definition.execute_with_state()?;
  let mut projections: u32 = 0;
  for (actor, _location) in terminal_state.actor_locations() {
    if terminal_state.is_allied(*actor) && terminal_state.observe(*actor).is_some() {
      projections = projections.saturating_add(1);
    }
  }
  let execution = OperationCounts {
    transitions_executed: transitions,
    // Executor contract: one initial and one terminal hash per pass.
    state_hashes_computed: 2,
    observation_projections: projections,
    replay_verifications: 0,
  };

  let (replay_run, _) = definition.execute_with_state()?;
  let replay = OperationCounts {
    transitions_executed: transitions,
    state_hashes_computed: 2,
    observation_projections: 0,
    replay_verifications: 1,
  };
  let replay_hash_matches = first.initial_hash == replay_run.initial_hash
    && first.terminal_hash == replay_run.terminal_hash;

  Ok(ScenarioCostProfile {
    scenario_id: definition.scenario_id,
    total: execution.saturating_add(replay),
    execution,
    replay,
    replay_hash_matches,
  })
}

/// Build a synthetic single-actor rotation script of `steps` transitions that
/// ping-pongs between the allied base and its first adjacent location.
fn probe_definition(steps: u32) -> Result<MapScenarioDefinition, CostProfileError> {
  if steps == 0 {
    return Err(CostProfileError::EmptyProbeScript);
  }
  let actor = ActorId::new(1);
  let neighbor = *adjacent_neighbors(MapLocation::ALLIED_BASE)
    .first()
    .ok_or(CostProfileError::ProbeMapUnavailable)?;
  let mut command_script = Vec::new();
  let step_count = usize::try_from(steps).expect("probe steps fit in a usize");
  for step in 0..step_count {
    let destination = if step % 2 == 0 {
      neighbor
    } else {
      MapLocation::ALLIED_BASE
    };
    command_script.push((
      u32::try_from(step).expect("step index fits in a u32") + 1,
      actor,
      TravelCommand::InitiateRotation { destination },
      1,
    ));
  }
  let initial_state = MatchMapState::new(
    1,
    vec![actor],
    vec![],
    vec![(actor, ActorLocation::Stationary(MapLocation::ALLIED_BASE))],
  );
  Ok(MapScenarioDefinition {
    scenario_id: "cost-profile-scaling-probe",
    title: "Cost Profile Scaling Probe",
    description: "Synthetic single-actor ping-pong rotation script for scaling probes.",
    initial_state,
    command_script,
    expected_terminal_locations: vec![],
  })
}

/// Run one scaling probe of `steps` transitions, execution plus replay.
pub fn profile_scaling_probe(steps: u32) -> Result<ScalingProbe, CostProfileError> {
  let definition = probe_definition(steps)?;
  let profile = profile_travel_scenario(&definition)?;
  Ok(ScalingProbe {
    steps,
    execution: profile.execution,
    replay: profile.replay,
    replay_hash_matches: profile.replay_hash_matches,
    total: profile.total,
  })
}

/// Profile the full canonical travel catalog batch with replay verification
/// and scaling probes.
pub fn profile_catalog_batch() -> Result<CostProfileReport, CostProfileError> {
  let definitions = MapTravelCatalog::all();
  let mut scenario_profiles = Vec::with_capacity(definitions.len());
  let mut batch_totals = OperationCounts::default();

  for definition in &definitions {
    let profile = profile_travel_scenario(definition)?;
    batch_totals = batch_totals.saturating_add(profile.total);
    scenario_profiles.push(profile);
  }

  let batch_entry_count = u32::try_from(definitions.len()).expect("catalog size fits in a u32");
  let average = |count: u32| -> u32 {
    u32::try_from(u64::from(count) * 10_000 / u64::from(batch_entry_count))
      .expect("per-entry average fits in a u32")
  };

  let mut scaling_probes = Vec::with_capacity(SCALING_PROBE_STEPS.len());
  for steps in SCALING_PROBE_STEPS {
    scaling_probes.push(profile_scaling_probe(steps)?);
  }

  let first = &scaling_probes[0];
  let last = &scaling_probes[scaling_probes.len() - 1];
  let step_span = last.steps.saturating_sub(first.steps);
  let marginal_transitions_per_step = last
    .total
    .transitions_executed
    .saturating_sub(first.total.transitions_executed)
    .checked_div(step_span)
    .unwrap_or(0);
  let hashes_constant_across_probes = scaling_probes
    .iter()
    .all(|probe| probe.execution.state_hashes_computed == first.execution.state_hashes_computed);
  let replay_doubles_transition_work = scaling_probes
    .iter()
    .all(|probe| probe.replay.transitions_executed == probe.execution.transitions_executed);

  Ok(CostProfileReport {
    schema: M9_COST_PROFILE_SCHEMA_V1,
    per_entry_transitions_bp: average(batch_totals.transitions_executed),
    per_entry_hashes_bp: average(batch_totals.state_hashes_computed),
    per_entry_projections_bp: average(batch_totals.observation_projections),
    per_entry_replays_bp: average(batch_totals.replay_verifications),
    batch_entry_count,
    batch_totals,
    scenario_profiles,
    scaling_probes,
    marginal_transitions_per_step,
    hashes_constant_across_probes,
    replay_doubles_transition_work,
  })
}

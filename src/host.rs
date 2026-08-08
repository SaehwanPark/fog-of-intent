//! Synchronous application-host orchestration for the bounded M3 transcript.
//!
//! The host owns lifecycle, draft, save/load, and history coordination while
//! delegating validation and transition evaluation to the lane contract. It
//! accepts resolved execution inputs explicitly and returns actor-valid
//! projections; it does not render terminal output or expose true state.

use crate::cli::{
  CliCommand, CliParseError, CliProcessError, CliProcessRequest, CliReadError, CliReadRequest,
  CliSessionError, CliSessionRequest, CliWriteError, CliWriteRequest, parse_command,
  process_request, read_request, session_request, write_request,
};
use crate::host_artifact::CliHostArtifact;
use crate::kernel::{DrawId, InputTrace, StreamId};
use crate::lane::{
  LaneDamage, LaneIntent, LaneIntentRequest, LaneOutcome, LaneResolvedInputs, LaneScenarioHistory,
  LaneWaveResult, ObservationId, PLAYER_LANER, ScenarioDebriefReport, ScenarioWindow,
  build_scenario_debrief, observe_player,
};
use crate::run_store::{CliRunStore, CliRunStoreError};

/// Versioned contract for the bounded synchronous host fixture.
pub const CLI_HOST_SCHEMA: &str = "m3-cli-host-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
struct HostDraft {
  message: Option<String>,
  plan: Option<String>,
  contingency: Option<String>,
}

impl HostDraft {
  fn is_empty(&self) -> bool {
    self.message.is_none() && self.plan.is_none() && self.contingency.is_none()
  }
}

#[derive(Clone)]
struct SavedRun {
  run_id: String,
  artifact: String,
}

/// Actor-valid results returned by [`CliScenarioHost::apply_line`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliHostOutput {
  Help,
  Observation(crate::lane::LanerObservation),
  History {
    records: u8,
    complete: bool,
  },
  DraftStaged {
    field: &'static str,
  },
  Committed {
    intent: LaneIntent,
  },
  Advanced {
    window: ScenarioWindow,
    outcome: LaneOutcome,
  },
  Review {
    records: u8,
    complete: bool,
  },
  Debrief(ScenarioDebriefReport),
  ReplayVerified {
    run_id: Option<String>,
    records: u8,
  },
  Saved {
    run_id: String,
    records: u8,
  },
  Loaded {
    run_id: String,
    records: u8,
  },
  Undone,
  Quit,
}

/// Errors raised before or while applying a CLI command at the host boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CliHostError<'a> {
  Closed,
  Parse(CliParseError<'a>),
  Read(CliReadError<'a>),
  Write(CliWriteError),
  Process(CliProcessError),
  Session(CliSessionError),
  UnsupportedCommand { verb: &'static str },
  InvalidPlan { text: String },
  CommittedBoundary { verb: &'static str },
  MissingPlan,
  MissingCommittedIntent,
  NothingToUndo,
  RunNotFound { run_id: String },
  AdvanceRejected,
  ReplayRejected,
  DebriefUnavailable,
  ScenarioComplete,
  StorageUnavailable,
}

/// A bounded host for the existing deterministic two-window lane scenario.
///
/// `execution_inputs` are already resolved at construction. The host never
/// creates random values and never returns a true-state snapshot to callers.
pub struct CliScenarioHost {
  history: LaneScenarioHistory,
  execution_inputs: [LaneResolvedInputs; 2],
  draft: HostDraft,
  committed_intent: Option<LaneIntent>,
  saved: Option<SavedRun>,
  store: Option<CliRunStore>,
  closed: bool,
}

impl CliScenarioHost {
  /// Build a host with explicit inputs for the first and second windows.
  pub fn new(execution_inputs: [LaneResolvedInputs; 2]) -> Self {
    Self {
      history: LaneScenarioHistory::new(crate::lane::LaneSnapshot::initial())
        .expect("initial lane fixture must be valid"),
      execution_inputs,
      draft: HostDraft {
        message: None,
        plan: None,
        contingency: None,
      },
      committed_intent: None,
      saved: None,
      store: None,
      closed: false,
    }
  }

  /// Build the deterministic two-window fixture used by host transcript tests.
  pub fn fixture() -> Self {
    Self::new([
      fixture_inputs(1, LaneWaveResult::Advanced, 1),
      fixture_inputs(0, LaneWaveResult::Held, 2),
    ])
  }

  /// Build a deterministic fixture host backed by an explicit artifact store.
  pub fn fixture_with_store(store: CliRunStore) -> Self {
    Self::with_store(
      [
        fixture_inputs(1, LaneWaveResult::Advanced, 1),
        fixture_inputs(0, LaneWaveResult::Held, 2),
      ],
      store,
    )
  }

  /// Build a host with explicit inputs and an injected artifact store.
  pub fn with_store(execution_inputs: [LaneResolvedInputs; 2], store: CliRunStore) -> Self {
    let mut host = Self::new(execution_inputs);
    host.store = Some(store);
    host
  }

  /// Return the stable host schema identifier.
  pub const fn schema() -> &'static str {
    CLI_HOST_SCHEMA
  }

  /// Return the current actor-visible observation.
  pub fn observation(&self) -> crate::lane::LanerObservation {
    observe_player(&self.history.current_state(), self.next_observation_id()).observation()
  }

  /// Return the number of committed scenario windows.
  pub fn record_count(&self) -> u8 {
    u8::try_from(self.history.records().len()).expect("two-window history fits in u8")
  }

  /// Whether both bounded scenario windows have been committed.
  pub fn is_complete(&self) -> bool {
    self.history.records().len() == 2
  }

  /// Apply one parsed-and-mapped CLI line at the host boundary.
  pub fn apply_line<'a>(&mut self, line: &'a str) -> Result<CliHostOutput, CliHostError<'a>> {
    if self.closed {
      return Err(CliHostError::Closed);
    }
    let command = parse_command(line).map_err(CliHostError::Parse)?;
    match command {
      CliCommand::Help | CliCommand::Observe | CliCommand::Inspect(_) => {
        let request = read_request(command).map_err(CliHostError::Read)?;
        self.apply_read(request)
      }
      CliCommand::Message(_)
      | CliCommand::Plan(_)
      | CliCommand::Contingency(_)
      | CliCommand::Commit
      | CliCommand::Advance => {
        let request = write_request(command).map_err(CliHostError::Write)?;
        self.apply_write(request)
      }
      CliCommand::Review | CliCommand::Debrief | CliCommand::Replay(_) | CliCommand::Branch(_) => {
        let request = process_request(command).map_err(CliHostError::Process)?;
        self.apply_process(request)
      }
      CliCommand::Save(_) | CliCommand::Load(_) | CliCommand::Undo | CliCommand::Quit => {
        let request = session_request(command).map_err(CliHostError::Session)?;
        self.apply_session(request)
      }
    }
  }

  fn apply_read(&self, request: CliReadRequest) -> Result<CliHostOutput, CliHostError<'static>> {
    match request {
      CliReadRequest::Help => Ok(CliHostOutput::Help),
      CliReadRequest::Observe
      | CliReadRequest::Inspect(crate::cli::CliInspectTarget::CurrentObservation) => {
        Ok(CliHostOutput::Observation(self.observation()))
      }
      CliReadRequest::Inspect(crate::cli::CliInspectTarget::VisibleHistoryReport) => {
        Ok(CliHostOutput::History {
          records: self.record_count(),
          complete: self.is_complete(),
        })
      }
    }
  }

  fn apply_write<'a>(
    &mut self,
    request: CliWriteRequest<'a>,
  ) -> Result<CliHostOutput, CliHostError<'a>> {
    match request {
      CliWriteRequest::Message { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "message" });
        }
        self.draft.message = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged { field: "message" })
      }
      CliWriteRequest::Plan { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "plan" });
        }
        self.draft.plan = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged { field: "plan" })
      }
      CliWriteRequest::Contingency { text } => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary {
            verb: "contingency",
          });
        }
        self.draft.contingency = Some(text.to_owned());
        Ok(CliHostOutput::DraftStaged {
          field: "contingency",
        })
      }
      CliWriteRequest::Commit => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "commit" });
        }
        let text = self
          .draft
          .plan
          .as_deref()
          .ok_or(CliHostError::MissingPlan)?;
        let intent = parse_plan_intent(text).ok_or_else(|| CliHostError::InvalidPlan {
          text: text.to_owned(),
        })?;
        self.committed_intent = Some(intent);
        self.draft = HostDraft {
          message: None,
          plan: None,
          contingency: None,
        };
        Ok(CliHostOutput::Committed { intent })
      }
      CliWriteRequest::Advance => self.advance(),
    }
  }

  fn apply_process<'a>(
    &mut self,
    request: CliProcessRequest<'a>,
  ) -> Result<CliHostOutput, CliHostError<'a>> {
    match request {
      CliProcessRequest::Review => Ok(CliHostOutput::Review {
        records: self.record_count(),
        complete: self.is_complete(),
      }),
      CliProcessRequest::Debrief => build_scenario_debrief(&self.history)
        .map(|record| CliHostOutput::Debrief(record.report()))
        .map_err(|_| CliHostError::DebriefUnavailable),
      CliProcessRequest::Replay { run_id } => {
        let (run_id, records) = if let Some(run_id) = run_id {
          let requested = run_id.as_str();
          let artifact = CliHostArtifact::decode(&self.load_artifact(requested)?)
            .map_err(|_| CliHostError::ReplayRejected)?;
          if artifact.run_id() != requested {
            return Err(CliHostError::ReplayRejected);
          }
          let history = self.restore_artifact(&artifact)?;
          (Some(requested.to_owned()), history.records().len())
        } else {
          self
            .history
            .verify_replay()
            .map_err(|_| CliHostError::ReplayRejected)?;
          (None, self.history.records().len())
        };
        Ok(CliHostOutput::ReplayVerified {
          run_id,
          records: u8::try_from(records).expect("two-window history fits in u8"),
        })
      }
      CliProcessRequest::Branch { .. } => Err(CliHostError::UnsupportedCommand { verb: "branch" }),
    }
  }

  fn apply_session<'a>(
    &mut self,
    request: CliSessionRequest<'a>,
  ) -> Result<CliHostOutput, CliHostError<'a>> {
    match request {
      CliSessionRequest::Save { run_id } => {
        let run_id = run_id.as_str().to_owned();
        let artifact = CliHostArtifact::encode(&run_id, &self.history)
          .map_err(|_| CliHostError::ReplayRejected)?;
        if let Some(store) = self.store.as_ref() {
          store
            .save(&run_id, &artifact)
            .map_err(|_| CliHostError::StorageUnavailable)?;
        }
        self.saved = Some(SavedRun {
          run_id: run_id.clone(),
          artifact,
        });
        Ok(CliHostOutput::Saved {
          run_id,
          records: self.record_count(),
        })
      }
      CliSessionRequest::Load { run_id } => {
        let requested = run_id.as_str();
        let artifact = CliHostArtifact::decode(&self.load_artifact(requested)?)
          .map_err(|_| CliHostError::ReplayRejected)?;
        if artifact.run_id() != requested {
          return Err(CliHostError::ReplayRejected);
        }
        self.history = self.restore_artifact(&artifact)?;
        self.draft = HostDraft {
          message: None,
          plan: None,
          contingency: None,
        };
        self.committed_intent = None;
        Ok(CliHostOutput::Loaded {
          run_id: requested.to_owned(),
          records: self.record_count(),
        })
      }
      CliSessionRequest::Undo => {
        if self.committed_intent.is_some() {
          return Err(CliHostError::CommittedBoundary { verb: "undo" });
        }
        if self.draft.is_empty() {
          return Err(CliHostError::NothingToUndo);
        }
        self.draft = HostDraft {
          message: None,
          plan: None,
          contingency: None,
        };
        Ok(CliHostOutput::Undone)
      }
      CliSessionRequest::Quit => {
        self.closed = true;
        Ok(CliHostOutput::Quit)
      }
    }
  }

  fn load_artifact(&self, run_id: &str) -> Result<String, CliHostError<'static>> {
    if let Some(store) = self.store.as_ref() {
      return store.load(run_id).map_err(|error| match error {
        CliRunStoreError::Read {
          kind: std::io::ErrorKind::NotFound,
        } => CliHostError::RunNotFound {
          run_id: run_id.to_owned(),
        },
        _ => CliHostError::StorageUnavailable,
      });
    }
    self
      .saved
      .as_ref()
      .filter(|saved| saved.run_id == run_id)
      .map(|saved| saved.artifact.clone())
      .ok_or_else(|| CliHostError::RunNotFound {
        run_id: run_id.to_owned(),
      })
  }

  fn restore_artifact(
    &self,
    artifact: &CliHostArtifact,
  ) -> Result<LaneScenarioHistory, CliHostError<'static>> {
    if artifact.replay_id() != crate::lane::M2_TWO_WINDOW_REPLAY_ID {
      return Err(CliHostError::ReplayRejected);
    }
    let mut history = LaneScenarioHistory::new(crate::lane::LaneSnapshot::initial())
      .map_err(|_| CliHostError::ReplayRejected)?;
    for record in artifact.records() {
      let index = record.index();
      let inputs = self
        .execution_inputs
        .get(index)
        .copied()
        .ok_or(CliHostError::ReplayRejected)?;
      let state = history.current_state();
      if state.hash() != record.prior_hash() {
        return Err(CliHostError::ReplayRejected);
      }
      let receipt = observe_player(&state, self.next_observation_id_for(&history));
      let request = LaneIntentRequest::new(
        PLAYER_LANER,
        receipt.observation().observation_id(),
        record.intent(),
      );
      let result = history
        .append(&receipt, &request, inputs)
        .map_err(|_| CliHostError::ReplayRejected)?;
      let restored_record = history
        .records()
        .last()
        .ok_or(CliHostError::ReplayRejected)?;
      if result.state_hash() != record.state_hash()
        || crate::lane::lane_record_identity(restored_record.transition()) != record.identity_hash()
      {
        return Err(CliHostError::ReplayRejected);
      }
    }
    history
      .verify_replay()
      .map_err(|_| CliHostError::ReplayRejected)?;
    Ok(history)
  }

  fn advance(&mut self) -> Result<CliHostOutput, CliHostError<'static>> {
    let intent = self
      .committed_intent
      .ok_or(CliHostError::MissingCommittedIntent)?;
    let index = self.history.records().len();
    let inputs = self
      .execution_inputs
      .get(index)
      .copied()
      .ok_or(CliHostError::ScenarioComplete)?;
    let state = self.history.current_state();
    let receipt = observe_player(&state, self.next_observation_id());
    let request =
      LaneIntentRequest::new(PLAYER_LANER, receipt.observation().observation_id(), intent);
    let result = self
      .history
      .append(&receipt, &request, inputs)
      .map_err(|_| CliHostError::AdvanceRejected)?;
    self.committed_intent = None;
    self.draft = HostDraft {
      message: None,
      plan: None,
      contingency: None,
    };
    let window = match index {
      0 => ScenarioWindow::First,
      1 => ScenarioWindow::Second,
      _ => return Err(CliHostError::ScenarioComplete),
    };
    Ok(CliHostOutput::Advanced {
      window,
      outcome: result.outcome(),
    })
  }

  fn next_observation_id(&self) -> ObservationId {
    self.next_observation_id_for(&self.history)
  }

  fn next_observation_id_for(&self, history: &LaneScenarioHistory) -> ObservationId {
    ObservationId::new(
      u64::try_from(history.records().len() + 1).expect("two-window observation count fits in u64"),
    )
  }

  #[cfg(test)]
  pub(crate) fn history_for_artifact_test(&self) -> &LaneScenarioHistory {
    &self.history
  }
}

fn parse_plan_intent(text: &str) -> Option<LaneIntent> {
  match text.trim() {
    "stabilize" => Some(LaneIntent::Stabilize),
    "contest" => Some(LaneIntent::Contest),
    "yield" => Some(LaneIntent::Yield),
    "recall" => Some(LaneIntent::Recall),
    "withdraw" => Some(LaneIntent::Withdraw),
    _ => None,
  }
}

fn fixture_inputs(
  opponent_damage: u8,
  wave_result: LaneWaveResult,
  stream: u8,
) -> LaneResolvedInputs {
  LaneResolvedInputs::new(
    InputTrace::new(StreamId::new(stream), DrawId::new(1)),
    InputTrace::new(StreamId::new(stream), DrawId::new(2)),
    InputTrace::new(StreamId::new(stream), DrawId::new(3)),
    InputTrace::new(StreamId::new(stream), DrawId::new(4)),
    crate::lane::LaneExecutionInputs::new(
      InputTrace::new(StreamId::new(stream), DrawId::new(5)),
      LaneDamage::zero(),
      LaneDamage::new(opponent_damage).expect("fixture damage must be bounded"),
      wave_result,
    ),
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::atomic::{AtomicU64, Ordering};

  static NEXT_STORE_ROOT: AtomicU64 = AtomicU64::new(0);

  fn temporary_store_root() -> std::path::PathBuf {
    let id = NEXT_STORE_ROOT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
      "fog-of-intent-host-store-{}-{id}",
      std::process::id()
    ))
  }

  #[test]
  fn fixture_transcript_completes_save_load_replay_and_debrief() {
    assert_eq!(CliScenarioHost::schema(), CLI_HOST_SCHEMA);
    let mut host = CliScenarioHost::fixture();
    let transcript = [
      "observe",
      "message ping ally",
      "plan contest",
      "contingency retreat if threat",
      "undo",
      "plan contest",
      "commit",
      "advance",
      "save first-window",
      "plan stabilize",
      "commit",
      "advance",
      "replay first-window",
      "load first-window",
      "plan stabilize",
      "commit",
      "advance",
      "save complete-run",
      "load complete-run",
      "replay complete-run",
      "debrief",
      "quit",
    ];

    let outputs = transcript
      .into_iter()
      .map(|line| host.apply_line(line).expect("fixture transcript command"))
      .collect::<Vec<_>>();

    assert_eq!(host.record_count(), 2);
    assert!(host.is_complete());
    assert!(matches!(outputs[0], CliHostOutput::Observation(_)));
    assert!(outputs.iter().any(|output| {
      matches!(
        output,
        CliHostOutput::ReplayVerified {
          run_id: Some(run_id),
          records: 2,
        } if run_id == "complete-run"
      )
    }));
    assert!(outputs.iter().any(|output| {
      matches!(
        output,
        CliHostOutput::Loaded {
          run_id,
          records: 1,
        } if run_id == "first-window"
      )
    }));
    assert!(outputs.iter().any(|output| {
      matches!(
        output,
        CliHostOutput::ReplayVerified {
          run_id: Some(run_id),
          records: 1,
        } if run_id == "first-window"
      )
    }));
    assert!(outputs.iter().any(|output| {
      matches!(output, CliHostOutput::Debrief(report) if report.windows().len() == 2)
    }));
    assert!(matches!(outputs.last(), Some(CliHostOutput::Quit)));
  }

  #[test]
  fn artifact_restore_rejects_divergent_resolved_inputs() {
    let mut source = CliScenarioHost::fixture();
    for command in ["plan contest", "commit", "advance"] {
      source.apply_line(command).expect("source fixture command");
    }
    let artifact = CliHostArtifact::encode("first-window", source.history_for_artifact_test())
      .expect("artifact encodes");

    let mut divergent = CliScenarioHost::new([
      fixture_inputs(2, LaneWaveResult::Advanced, 1),
      fixture_inputs(0, LaneWaveResult::Held, 2),
    ]);
    divergent.saved = Some(SavedRun {
      run_id: "first-window".to_owned(),
      artifact,
    });

    assert_eq!(
      divergent.apply_line("load first-window"),
      Err(CliHostError::ReplayRejected)
    );
  }

  #[test]
  fn artifact_restore_rejects_run_id_mismatch() {
    let mut host = CliScenarioHost::fixture();
    host
      .apply_line("save first-window")
      .expect("empty fixture saves");
    let saved = host.saved.as_mut().expect("saved artifact");
    saved.artifact = saved
      .artifact
      .replace("run_id=first-window", "run_id=other");

    assert_eq!(
      host.apply_line("load first-window"),
      Err(CliHostError::ReplayRejected)
    );
  }

  #[test]
  fn artifact_restore_rejects_valid_intent_tampering() {
    let mut source = CliScenarioHost::fixture();
    for command in ["plan stabilize", "commit", "advance"] {
      source.apply_line(command).expect("source fixture command");
    }
    let artifact = CliHostArtifact::encode("first-window", source.history_for_artifact_test())
      .expect("artifact encodes")
      .replace("intent=stabilize", "intent=yield");
    let mut tampered = CliScenarioHost::fixture();
    tampered.saved = Some(SavedRun {
      run_id: "first-window".to_owned(),
      artifact,
    });

    assert_eq!(
      tampered.apply_line("load first-window"),
      Err(CliHostError::ReplayRejected)
    );
  }

  #[test]
  fn artifact_restore_rejects_hash_tampering() {
    let mut source = CliScenarioHost::fixture();
    for command in ["plan contest", "commit", "advance"] {
      source.apply_line(command).expect("source fixture command");
    }
    let artifact = CliHostArtifact::encode("first-window", source.history_for_artifact_test())
      .expect("artifact encodes");

    for field in ["prior_hash", "state_hash", "identity_hash"] {
      let mut tampered = CliScenarioHost::fixture();
      tampered.saved = Some(SavedRun {
        run_id: "first-window".to_owned(),
        artifact: replace_artifact_field(&artifact, field, "0"),
      });
      assert_eq!(
        tampered.apply_line("load first-window"),
        Err(CliHostError::ReplayRejected),
        "tampered {field} must fail closed"
      );
    }
  }

  #[test]
  fn file_store_round_trip_survives_a_fresh_host() {
    let root = temporary_store_root();
    let store = CliRunStore::new(&root);
    let mut source = CliScenarioHost::fixture_with_store(store.clone());
    for command in ["plan contest", "commit", "advance", "save first-window"] {
      source.apply_line(command).expect("source store command");
    }
    source
      .apply_line("plan stabilize")
      .expect("second-window draft");
    source.apply_line("commit").expect("second-window commit");
    source.apply_line("advance").expect("second-window advance");

    let mut fresh = CliScenarioHost::fixture_with_store(store);
    assert_eq!(
      fresh.apply_line("load first-window"),
      Ok(CliHostOutput::Loaded {
        run_id: "first-window".to_owned(),
        records: 1
      })
    );
    assert_eq!(fresh.record_count(), 1);
    assert_eq!(
      fresh.apply_line("replay first-window"),
      Ok(CliHostOutput::ReplayVerified {
        run_id: Some("first-window".to_owned()),
        records: 1
      })
    );
    let _ = std::fs::remove_dir_all(root);
  }

  #[test]
  fn file_store_failure_is_bounded_at_the_host_boundary() {
    let root = temporary_store_root();
    std::fs::write(&root, "not a directory").expect("root fixture");
    let mut host = CliScenarioHost::fixture_with_store(CliRunStore::new(&root));
    assert_eq!(
      host.apply_line("save run"),
      Err(CliHostError::StorageUnavailable)
    );
    let _ = std::fs::remove_file(root);
  }

  #[test]
  fn file_store_tampering_is_rejected_before_history_replacement() {
    let root = temporary_store_root();
    let store = CliRunStore::new(&root);
    let mut source = CliScenarioHost::fixture_with_store(store.clone());
    source.apply_line("save run").expect("save fixture");
    std::fs::write(root.join("run.foi-artifact"), "malformed").expect("tampered artifact");

    let mut fresh = CliScenarioHost::fixture_with_store(store);
    fresh.apply_line("plan contest").expect("local plan");
    fresh.apply_line("commit").expect("local commit");
    fresh.apply_line("advance").expect("local advance");
    let before = fresh.observation();
    assert_eq!(fresh.record_count(), 1);
    assert_eq!(
      fresh.apply_line("load run"),
      Err(CliHostError::ReplayRejected)
    );
    assert_eq!(fresh.record_count(), 1);
    assert_eq!(fresh.observation(), before);
    let _ = std::fs::remove_dir_all(root);
  }

  #[test]
  fn host_rejects_invalid_plan_and_pre_host_errors() {
    let mut host = CliScenarioHost::fixture();
    assert_eq!(
      host.apply_line("plan ???"),
      Ok(CliHostOutput::DraftStaged { field: "plan" })
    );
    assert_eq!(
      host.apply_line("commit"),
      Err(CliHostError::InvalidPlan {
        text: "???".to_owned(),
      })
    );
    assert_eq!(
      host.apply_line("advance"),
      Err(CliHostError::MissingCommittedIntent)
    );
    host.apply_line("plan contest").expect("valid plan staging");
    host.apply_line("commit").expect("valid commit");
    for (line, verb) in [
      ("plan stabilize", "plan"),
      ("message late", "message"),
      ("contingency late", "contingency"),
      ("commit", "commit"),
      ("undo", "undo"),
    ] {
      assert_eq!(
        host.apply_line(line),
        Err(CliHostError::CommittedBoundary { verb })
      );
    }
    host.apply_line("advance").expect("first window advances");
    host
      .apply_line("plan stabilize")
      .expect("next-window plan staging");
    host.apply_line("commit").expect("next-window commit");
    host.apply_line("advance").expect("second window advances");
    assert_eq!(
      host.apply_line("load missing"),
      Err(CliHostError::RunNotFound {
        run_id: "missing".to_owned(),
      })
    );
    assert_eq!(
      host.apply_line("branch point-0"),
      Err(CliHostError::UnsupportedCommand { verb: "branch" })
    );
  }

  fn replace_artifact_field(artifact: &str, field: &str, value: &str) -> String {
    artifact
      .lines()
      .map(|line| {
        line
          .split_whitespace()
          .map(|word| {
            if word.starts_with(&format!("{field}=")) {
              format!("{field}={value}")
            } else {
              word.to_owned()
            }
          })
          .collect::<Vec<_>>()
          .join(" ")
      })
      .collect::<Vec<_>>()
      .join("\n")
  }

  #[test]
  fn malformed_resolved_inputs_return_redacted_host_errors() {
    let mut host = CliScenarioHost::new([
      fixture_inputs(8, LaneWaveResult::Advanced, 3),
      fixture_inputs(0, LaneWaveResult::Held, 4),
    ]);
    host.apply_line("plan contest").expect("plan staging");
    host.apply_line("commit").expect("commit");
    let error = host
      .apply_line("advance")
      .expect_err("malformed fixture input must fail closed");
    assert_eq!(error, CliHostError::AdvanceRejected);
    let debug = format!("{error:?}");
    assert!(!debug.contains("OpponentDamageExceedsHealth"));
    assert!(!debug.contains("health"));
    assert!(!debug.contains("state_hash"));
  }

  #[test]
  fn identical_fixture_transcripts_have_identical_actor_outputs() {
    let run = |host: &mut CliScenarioHost| {
      [
        "plan contest",
        "commit",
        "advance",
        "plan stabilize",
        "commit",
        "advance",
      ]
      .into_iter()
      .map(|line| host.apply_line(line).expect("deterministic command"))
      .collect::<Vec<_>>()
    };
    assert_eq!(
      run(&mut CliScenarioHost::fixture()),
      run(&mut CliScenarioHost::fixture())
    );
  }
}

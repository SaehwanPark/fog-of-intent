//! Tests for scenario host orchestration, persistence, drafts, and actor protocols.

use super::scenario_host::{CliScenarioHost, fixture_inputs, forced_out_inputs};
use super::types::{
  ActorIllegalCommandPopulationError, ActorIllegalCommandPopulationReport, CLI_HOST_SCHEMA,
  CliHostError, CliHostOutput, CliSessionWindow, MAX_ACTOR_ILLEGAL_COMMAND_POPULATION, SavedRun,
};
use crate::cli::CliRunId;
use crate::host_artifact::CliHostArtifact;
use crate::lane::{
  LaneExecutionRelation, LaneIntent, LaneOutcome, LaneWaveResult, ObservationId, ScenarioWindow,
  observe_player,
};
use crate::protocol::{
  ActorActionDto, ActorActionResultDto, ActorActionResultOutcome, ActorActionResultWindow,
  ActorCommitDto, ActorCommitResultDto, ActorDebriefAttributionLimit, ActorDebriefDto,
  ActorDebriefObjective, ActorDraftClearDto, ActorDraftClearReceiptDto, ActorDraftCommitReceiptDto,
  ActorDraftDto, ActorDraftField, ActorDraftPresence, ActorDraftReceiptDto, ActorDraftStatusDto,
  ActorHistoryDto, ActorHistoryStatus, ActorObservationDto, ActorProtocolError,
  ActorProtocolErrorCode, ActorProtocolRepairHint, ActorReplayDebriefRecordDto,
  ActorReplayRecordDto, ActorReplayVerification,
};
use crate::run_store::CliRunStore;
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
fn session_view_is_actor_safe_and_tracks_draft_commit() {
  let mut host = CliScenarioHost::fixture();
  let empty = host.session_view();
  assert_eq!(empty.window(), CliSessionWindow::First);
  assert!(empty.draft_fields().is_empty());
  assert_eq!(empty.committed_intent(), None);
  assert_eq!(empty.suggested_next(), ["observe", "plan", "commit"]);
  assert!(!format!("{empty:?}").contains("hash"));

  host.apply_line("plan contest").expect("plan");
  let staged = host.session_view();
  assert_eq!(staged.draft_fields(), ["plan"]);
  assert_eq!(staged.suggested_next(), ["commit", "undo", "observe"]);

  host.apply_line("commit").expect("commit");
  let committed = host.session_view();
  assert_eq!(committed.committed_intent(), Some(LaneIntent::Contest));
  assert!(committed.draft_fields().is_empty());
  assert_eq!(committed.suggested_next(), ["advance", "observe", "quit"]);
  assert!(!format!("{committed:?}").contains("hash"));

  let unknown = host.apply_line("help wat").expect_err("unknown topic");
  assert!(matches!(
    unknown,
    CliHostError::UnknownHelpTopic { topic } if topic == "wat"
  ));
}

#[test]
fn actor_observation_projection_matches_host_receipt_without_mutation() {
  let mut host = CliScenarioHost::fixture();
  let initial = host
    .actor_observation()
    .expect("active observation projects");
  assert_eq!(
    initial,
    ActorObservationDto::from_observation(host.observation())
  );
  assert_eq!(initial.schema(), "m5-actor-observation-v1");
  assert!(initial.advertises(crate::protocol::ActorProtocolIntent::Contest));
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.actor_observation(), Ok(initial.clone()));
  assert!(!format!("{initial:?}").contains("hash"));

  host.apply_line("plan contest").expect("plan is staged");
  host.apply_line("commit").expect("plan is committed");
  host.apply_line("advance").expect("first window advances");
  let next = host
    .actor_observation()
    .expect("next active observation projects");
  assert_eq!(
    next,
    ActorObservationDto::from_observation(host.observation())
  );
  assert_ne!(next.observation_id(), initial.observation_id());
  assert_eq!(host.record_count(), 1);

  host
    .apply_line("plan stabilize")
    .expect("second plan is staged");
  host.apply_line("commit").expect("second plan is committed");
  host.apply_line("advance").expect("second window advances");
  assert_eq!(
    host.actor_observation(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut closed = CliScenarioHost::fixture();
  closed.apply_line("quit").expect("host closes");
  assert_eq!(
    closed.actor_observation(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
}

#[test]
fn actor_history_projection_tracks_bounded_lifecycle_without_hidden_state() {
  let mut host = CliScenarioHost::fixture();
  assert_eq!(
    host.actor_history(),
    ActorHistoryDto::new(0, ActorHistoryStatus::Open).expect("open history is bounded")
  );
  assert_eq!(
    host.apply_line("inspect history"),
    Ok(CliHostOutput::History {
      records: 0,
      complete: false,
    })
  );

  for command in ["plan contest", "commit", "advance"] {
    host.apply_line(command).expect("first window advances");
  }
  assert_eq!(
    host.actor_history(),
    ActorHistoryDto::new(1, ActorHistoryStatus::Open).expect("next history is bounded")
  );
  assert!(!format!("{:?}", host.actor_history()).contains("hash"));

  for command in ["plan stabilize", "commit", "advance"] {
    host.apply_line(command).expect("second window advances");
  }
  assert_eq!(
    host.actor_history(),
    ActorHistoryDto::new(2, ActorHistoryStatus::Complete).expect("complete history is bounded")
  );
  host.apply_line("quit").expect("complete host closes");
  assert_eq!(
    host.actor_history(),
    ActorHistoryDto::new(2, ActorHistoryStatus::Closed)
      .expect("closed complete history is bounded")
  );

  let mut partially_closed = CliScenarioHost::fixture();
  for command in ["plan contest", "commit", "advance", "quit"] {
    partially_closed
      .apply_line(command)
      .expect("partial host command succeeds");
  }
  assert_eq!(
    partially_closed.actor_history(),
    ActorHistoryDto::new(1, ActorHistoryStatus::Closed).expect("closed partial history is bounded")
  );

  let mut closed = CliScenarioHost::fixture();
  closed.apply_line("quit").expect("host closes");
  assert_eq!(
    closed.actor_history(),
    ActorHistoryDto::new(0, ActorHistoryStatus::Closed).expect("closed history is bounded")
  );
}

#[test]
fn actor_replay_projection_verifies_history_without_exposing_records() {
  let mut host = CliScenarioHost::fixture();
  let initial_observation = host.observation();
  let initial = host.actor_replay().expect("empty history replays");
  assert_eq!(initial.records(), 0);
  assert_eq!(initial.verification(), ActorReplayVerification::Verified);
  assert_eq!(
    initial.encode(),
    "schema=m5-actor-replay-v1\nrecords=0\nverification=verified\n"
  );
  assert!(!format!("{initial:?}").contains("hash"));
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), initial_observation);

  host.apply_line("plan contest").expect("plan stages");
  host.apply_line("commit").expect("commit stages");
  host.apply_line("advance").expect("first window advances");
  let one = host.actor_replay().expect("one record replays");
  assert_eq!(one.records(), 1);
  host
    .apply_line("plan stabilize")
    .expect("second plan stages");
  host.apply_line("commit").expect("second commit stages");
  host.apply_line("advance").expect("second window advances");
  let complete = host.actor_replay().expect("complete history replays");
  assert_eq!(complete.records(), 2);

  let mut closed = CliScenarioHost::fixture();
  closed.apply_line("quit").expect("host closes");
  assert_eq!(
    closed.actor_replay(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut tampered = CliScenarioHost::fixture();
  tampered.apply_line("plan contest").expect("plan stages");
  tampered.apply_line("commit").expect("commit stages");
  tampered
    .apply_line("advance")
    .expect("first window advances");
  tampered.history.tamper_replay_id_for_test("tampered");
  assert_eq!(
    tampered.actor_replay(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::HostTransitionRejected,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  assert_eq!(tampered.record_count(), 1);
}

#[test]
fn actor_replay_records_are_verified_categorical_projections() {
  let mut host = CliScenarioHost::fixture();
  let initial_observation = host.observation();
  assert_eq!(host.actor_replay_records(), Ok(Vec::new()));
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), initial_observation);

  for command in ["plan contest", "commit", "advance"] {
    host.apply_line(command).expect("first window advances");
  }
  let first = ActorReplayRecordDto::new(
    ActorActionResultWindow::First,
    crate::protocol::ActorProtocolIntent::Contest,
    ActorActionResultOutcome::HeldSpace,
  );
  assert_eq!(host.actor_replay_records(), Ok(vec![first]));
  assert_eq!(first.verification(), ActorReplayVerification::Verified);
  assert!(!format!("{first:?}").contains("StateHash"));
  assert!(!first.encode().contains("execution"));

  for command in ["plan stabilize", "commit", "advance"] {
    host.apply_line(command).expect("second window advances");
  }
  let second = ActorReplayRecordDto::new(
    ActorActionResultWindow::Second,
    crate::protocol::ActorProtocolIntent::Stabilize,
    ActorActionResultOutcome::YieldedSpace,
  );
  assert_eq!(host.actor_replay_records(), Ok(vec![first, second]));
  assert_eq!(host.record_count(), 2);

  let mut closed = CliScenarioHost::fixture();
  closed.apply_line("quit").expect("host closes");
  assert_eq!(
    closed.actor_replay_records(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut tampered = CliScenarioHost::fixture();
  for command in ["plan contest", "commit", "advance"] {
    tampered.apply_line(command).expect("first window advances");
  }
  tampered.history.tamper_replay_id_for_test("tampered");
  assert_eq!(
    tampered.actor_replay_records(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::HostTransitionRejected,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  assert_eq!(tampered.record_count(), 1);
}

#[test]
fn actor_replay_debrief_records_are_complete_and_categorical() {
  let mut host = CliScenarioHost::fixture();
  let initial_observation = host.observation();
  assert_eq!(
    host.actor_replay_debrief_records(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DebriefUnavailable,
      ActorProtocolRepairHint::AwaitCompletion,
    ))
  );
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), initial_observation);

  for command in [
    "plan contest",
    "commit",
    "advance",
    "plan stabilize",
    "commit",
    "advance",
  ] {
    host.apply_line(command).expect("fixture completes");
  }
  let first = ActorReplayDebriefRecordDto::new(
    ActorActionResultWindow::First,
    crate::protocol::ActorProtocolIntent::Contest,
    ActorActionResultOutcome::HeldSpace,
    ActorDebriefObjective::GoalAchieved,
  );
  let second = ActorReplayDebriefRecordDto::new(
    ActorActionResultWindow::Second,
    crate::protocol::ActorProtocolIntent::Stabilize,
    ActorActionResultOutcome::YieldedSpace,
    ActorDebriefObjective::GoalMissed,
  );
  assert_eq!(host.actor_replay_debrief_records(), Ok(vec![first, second]));
  assert_eq!(
    first.attribution(),
    ActorDebriefAttributionLimit::CommittedFactsOnly
  );
  assert_eq!(first.verification(), ActorReplayVerification::Verified);
  assert!(!format!("{first:?}").contains("StateHash"));
  assert!(!format!("{first:?}").contains("trace"));
  assert_eq!(host.record_count(), 2);
  host.apply_line("quit").expect("complete host closes");
  assert_eq!(
    host.actor_replay_debrief_records(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut tampered = CliScenarioHost::fixture();
  for command in [
    "plan contest",
    "commit",
    "advance",
    "plan stabilize",
    "commit",
    "advance",
  ] {
    tampered.apply_line(command).expect("fixture completes");
  }
  tampered.history.tamper_replay_id_for_test("tampered");
  assert_eq!(
    tampered.actor_replay_debrief_records(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::HostTransitionRejected,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  assert_eq!(tampered.record_count(), 2);
}

#[test]
fn actor_action_validation_is_read_only_and_actor_safe() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  let valid = ActorActionDto::new(
    observation.observer().value(),
    observation.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Contest,
  );

  assert_eq!(host.validate_actor_action(valid), Ok(()));
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), observation);

  let cases = [
    (
      ActorActionDto::new(2, observation.observation_id().value(), valid.intent()),
      "actor_mismatch",
      "use_bound_actor",
    ),
    (
      ActorActionDto::new(1, observation.observation_id().value() + 1, valid.intent()),
      "stale_observation",
      "request_fresh_observation",
    ),
    (
      ActorActionDto::new(
        observation.observer().value(),
        observation.observation_id().value(),
        crate::protocol::ActorProtocolIntent::Withdraw,
      ),
      "host_validation_rejected",
      "resend_advertised_action",
    ),
  ];
  for (action, code, repair) in cases {
    let error = host
      .validate_actor_action(action)
      .expect_err("invalid actor action is rejected");
    assert_eq!(error.schema(), "m5-actor-error-v2");
    assert_eq!(error.code().id(), code);
    assert_eq!(error.repair().id(), repair);
    assert!(!format!("{error:?}").contains("hash"));
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.observation(), observation);
  }

  for line in [
    "plan contest",
    "commit",
    "advance",
    "plan stabilize",
    "commit",
    "advance",
  ] {
    host.apply_line(line).expect("fixture action advances");
  }
  let closed_observation = host.observation();
  let error = host
    .validate_actor_action(ActorActionDto::new(
      closed_observation.observer().value(),
      closed_observation.observation_id().value(),
      valid.intent(),
    ))
    .expect_err("complete host rejects actor action");
  assert_eq!(error.code().id(), "window_closed");
  assert_eq!(error.repair().id(), "start_new_session");
  assert_eq!(host.record_count(), 2);
}

#[test]
fn illegal_command_population_is_bounded_and_read_only() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  assert_eq!(MAX_ACTOR_ILLEGAL_COMMAND_POPULATION, 4);
  let draft = ActorDraftDto::new(
    observation.observer().value(),
    observation.observation_id().value(),
    ActorDraftField::Message,
    "keep this draft",
  )
  .expect("draft is bounded");
  host
    .stage_actor_draft(draft)
    .expect("draft staging succeeds");
  let draft_before = host.protocol_draft.clone();
  let observation_before = host.observation();
  let history_before = host.record_count();
  let singleton = ActorIllegalCommandPopulationReport::from_host(&host, 1)
    .expect("lower inclusive invalid-command population succeeds");
  assert_eq!(singleton.attempt_count(), 1);
  let report = ActorIllegalCommandPopulationReport::from_host(&host, 4)
    .expect("bounded invalid-command population succeeds");
  assert_eq!(report.schema(), "m6-actor-illegal-command-population-v1");
  assert_eq!(report.observer(), observation.observer().value());
  assert_eq!(
    report.observation_id(),
    observation.observation_id().value()
  );
  assert_eq!(
    report.rejection_code(),
    ActorProtocolErrorCode::HostValidationRejected
  );
  assert_eq!(report.attempt_count(), 4);
  assert_eq!(
    report,
    ActorIllegalCommandPopulationReport::from_host(&host, 4)
      .expect("repeated construction is deterministic")
  );
  assert_eq!(host.protocol_draft, draft_before);
  assert_eq!(host.record_count(), history_before);
  assert_eq!(host.observation(), observation_before);

  let mut committed_host = CliScenarioHost::fixture();
  let committed_observation = committed_host.observation();
  committed_host
    .commit_actor_draft(ActorCommitDto::new(
      committed_observation.observer().value(),
      committed_observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    ))
    .expect("commit remains local before advance");
  let committed_intent_before = committed_host.committed_intent;
  let committed_observation_before = committed_host.observation();
  let committed_history_before = committed_host.record_count();
  ActorIllegalCommandPopulationReport::from_host(&committed_host, 4)
    .expect("committed host still validates read-only");
  assert_eq!(committed_host.committed_intent, committed_intent_before);
  assert_eq!(committed_host.record_count(), committed_history_before);
  assert_eq!(committed_host.observation(), committed_observation_before);

  let mut closed_host = CliScenarioHost::fixture();
  closed_host
    .apply_line("quit")
    .expect("fixture closes through the host lifecycle");
  assert_eq!(
    ActorIllegalCommandPopulationReport::from_host(&closed_host, 0),
    Err(ActorIllegalCommandPopulationError::EmptyPopulation)
  );
  assert_eq!(
    ActorIllegalCommandPopulationReport::from_host(
      &closed_host,
      MAX_ACTOR_ILLEGAL_COMMAND_POPULATION + 1,
    ),
    Err(ActorIllegalCommandPopulationError::PopulationTooLarge {
      max: MAX_ACTOR_ILLEGAL_COMMAND_POPULATION,
      actual: MAX_ACTOR_ILLEGAL_COMMAND_POPULATION + 1,
    })
  );
}

#[test]
fn actor_action_submission_is_host_owned_and_closes_each_window() {
  let mut host = CliScenarioHost::fixture();
  let first = host.observation();
  let first_action = ActorActionDto::new(
    first.observer().value(),
    first.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Contest,
  );
  assert!(matches!(
    host
      .submit_actor_action(first_action)
      .expect("first actor action submits"),
    CliHostOutput::Advanced {
      window: ScenarioWindow::First,
      ..
    }
  ));
  assert_eq!(host.record_count(), 1);
  assert_eq!(
    host.submit_actor_action(first_action),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::StaleObservation,
      ActorProtocolRepairHint::RequestFreshObservation,
    ))
  );

  let second = host.observation();
  let second_action = ActorActionDto::new(
    second.observer().value(),
    second.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Stabilize,
  );
  assert!(matches!(
    host
      .submit_actor_action(second_action)
      .expect("second actor action submits"),
    CliHostOutput::Advanced {
      window: ScenarioWindow::Second,
      ..
    }
  ));
  assert!(host.is_complete());
  let closed = host.observation();
  assert_eq!(
    host.submit_actor_action(ActorActionDto::new(
      closed.observer().value(),
      closed.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    )),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut malformed = CliScenarioHost::new([
    fixture_inputs(8, LaneWaveResult::Advanced, 1),
    fixture_inputs(0, LaneWaveResult::Held, 2),
  ]);
  let malformed_observation = malformed.observation();
  let transition_error = malformed
    .submit_actor_action(ActorActionDto::new(
      malformed_observation.observer().value(),
      malformed_observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    ))
    .expect_err("malformed execution is redacted");
  assert_eq!(transition_error.code().id(), "host_transition_rejected");
  assert_eq!(transition_error.repair().id(), "start_new_session");
  assert_eq!(malformed.record_count(), 0);
  assert_eq!(malformed.observation(), malformed_observation);
  assert_eq!(
    malformed.apply_line("plan stabilize"),
    Ok(CliHostOutput::DraftStaged { field: "plan" })
  );
  assert!(!format!("{transition_error:?}").contains("health"));
}

#[test]
fn actor_action_result_projection_is_bounded_and_host_owned() {
  let mut host = CliScenarioHost::fixture();
  let first = host.observation();
  let first_action = ActorActionDto::new(
    first.observer().value(),
    first.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Contest,
  );
  let first_result = host
    .submit_actor_action_result(first_action)
    .expect("first action result projects");
  assert_eq!(
    first_result,
    ActorActionResultDto::new(
      ActorActionResultWindow::First,
      ActorActionResultOutcome::HeldSpace,
    )
  );
  assert_eq!(host.record_count(), 1);
  assert_eq!(
    ActorActionResultDto::decode(&first_result.encode()),
    Ok(first_result)
  );
  assert!(!format!("{first_result:?}").contains("hash"));
  assert_eq!(
    host.submit_actor_action_result(first_action),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::StaleObservation,
      ActorProtocolRepairHint::RequestFreshObservation,
    ))
  );

  let second = host.observation();
  let second_result = host
    .submit_actor_action_result(ActorActionDto::new(
      second.observer().value(),
      second.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    ))
    .expect("second action result projects");
  assert_eq!(
    second_result,
    ActorActionResultDto::new(
      ActorActionResultWindow::Second,
      ActorActionResultOutcome::YieldedSpace,
    )
  );
  assert_eq!(host.record_count(), 2);

  let mut forced = CliScenarioHost::new([
    forced_out_inputs(1),
    fixture_inputs(0, LaneWaveResult::Held, 2),
  ]);
  let forced_observation = forced.observation();
  let forced_result = forced
    .submit_actor_action_result(ActorActionDto::new(
      forced_observation.observer().value(),
      forced_observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    ))
    .expect("forced-out result projects");
  assert_eq!(
    forced_result,
    ActorActionResultDto::new(
      ActorActionResultWindow::First,
      ActorActionResultOutcome::ForcedOut,
    )
  );
}

#[test]
fn actor_debrief_projection_is_completion_gated_and_actor_safe() {
  let mut host = CliScenarioHost::fixture();
  let initial_observation = host.observation();
  assert_eq!(
    host.actor_debrief(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DebriefUnavailable,
      ActorProtocolRepairHint::AwaitCompletion,
    ))
  );
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), initial_observation);

  for command in [
    "plan contest",
    "commit",
    "advance",
    "plan stabilize",
    "commit",
    "advance",
  ] {
    host.apply_line(command).expect("fixture completes");
  }
  let debrief = host.actor_debrief().expect("complete host has debrief");
  assert_eq!(debrief.schema(), "m5-actor-debrief-v1");
  assert_eq!(debrief.first().window(), ActorActionResultWindow::First);
  assert_eq!(debrief.first().intent().id(), "contest");
  assert_eq!(
    debrief.first().outcome(),
    ActorActionResultOutcome::HeldSpace
  );
  assert_eq!(
    debrief.first().objective(),
    ActorDebriefObjective::GoalAchieved
  );
  assert_eq!(debrief.second().window(), ActorActionResultWindow::Second);
  assert_eq!(debrief.second().intent().id(), "stabilize");
  assert_eq!(
    debrief.second().outcome(),
    ActorActionResultOutcome::YieldedSpace
  );
  assert_eq!(
    debrief.second().objective(),
    ActorDebriefObjective::GoalMissed
  );
  assert_eq!(debrief.final_objective(), ActorDebriefObjective::GoalMissed);
  assert_eq!(
    debrief.attribution_limit(),
    ActorDebriefAttributionLimit::CommittedFactsOnly
  );
  assert_eq!(ActorDebriefDto::decode(&debrief.encode()), Ok(debrief));
  assert_eq!(host.record_count(), 2);
  assert!(!format!("{debrief:?}").contains("StateHash"));
  assert!(!format!("{debrief:?}").contains("health"));
  assert!(!format!("{debrief:?}").contains("trace"));

  host.apply_line("quit").expect("completed host closes");
  assert_eq!(
    host.actor_debrief(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut closed = CliScenarioHost::fixture();
  closed.apply_line("quit").expect("incomplete host closes");
  assert_eq!(
    closed.actor_debrief(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
}

#[test]
fn actor_debrief_from_store_is_complete_and_does_not_mutate_current_host() {
  let root = temporary_store_root();
  let store = CliRunStore::new(&root);
  let mut source = CliScenarioHost::fixture_with_store(store.clone());
  for command in ["plan contest", "commit", "advance", "save first-window"] {
    source
      .apply_line(command)
      .expect("source first-window command");
  }
  for command in ["plan stabilize", "commit", "advance", "save complete-run"] {
    source
      .apply_line(command)
      .expect("source complete-run command");
  }

  let mut fresh = CliScenarioHost::fixture_with_store(store);
  let before = fresh.observation();
  let first_window = CliRunId::parse("first-window").expect("run ID is valid");
  assert_eq!(
    fresh.actor_debrief_from_run(first_window),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DebriefUnavailable,
      ActorProtocolRepairHint::AwaitCompletion,
    ))
  );
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);

  let complete_run = CliRunId::parse("complete-run").expect("run ID is valid");
  let debrief = fresh
    .actor_debrief_from_run(complete_run)
    .expect("complete saved run has debrief");
  assert_eq!(debrief.schema(), "m5-actor-debrief-v1");
  assert_eq!(debrief.first().intent().id(), "contest");
  assert_eq!(
    debrief.first().outcome(),
    ActorActionResultOutcome::HeldSpace
  );
  assert_eq!(
    debrief.first().objective(),
    ActorDebriefObjective::GoalAchieved
  );
  assert_eq!(debrief.second().intent().id(), "stabilize");
  assert_eq!(
    debrief.second().outcome(),
    ActorActionResultOutcome::YieldedSpace
  );
  assert_eq!(
    debrief.second().objective(),
    ActorDebriefObjective::GoalMissed
  );
  assert_eq!(debrief.final_objective(), ActorDebriefObjective::GoalMissed);
  assert_eq!(
    debrief.attribution_limit(),
    ActorDebriefAttributionLimit::CommittedFactsOnly
  );
  assert_eq!(ActorDebriefDto::decode(&debrief.encode()), Ok(debrief));
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);
  assert!(!format!("{debrief:?}").contains("StateHash"));
  assert!(!format!("{debrief:?}").contains("health"));
  assert!(!format!("{debrief:?}").contains("trace"));

  std::fs::write(root.join("complete-run.foi-artifact"), "malformed")
    .expect("tamper complete artifact");
  assert_eq!(
    fresh.actor_debrief_from_run(complete_run),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::HostTransitionRejected,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);
  fresh.apply_line("quit").expect("fresh host closes");
  assert_eq!(
    fresh.actor_debrief_from_run(complete_run),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  let _ = std::fs::remove_dir_all(root);
}

#[test]
fn actor_commit_is_observation_bound_and_does_not_advance_history() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  let make_draft = |field, value| {
    ActorDraftDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      field,
      value,
    )
    .expect("draft value is bounded")
  };
  for (field, value) in [
    (ActorDraftField::Message, "ping ally"),
    (ActorDraftField::Plan, "contest"),
    (ActorDraftField::Contingency, "retreat if threat"),
  ] {
    host
      .stage_actor_draft(make_draft(field, value))
      .expect("draft stages before commit");
  }

  let first_commit = ActorCommitDto::new(
    observation.observer().value(),
    observation.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Contest,
  );
  let result = host
    .commit_actor_draft(first_commit)
    .expect("matching actor commit succeeds");
  assert_eq!(
    result,
    ActorCommitResultDto::new(crate::protocol::ActorProtocolIntent::Contest)
  );
  assert_eq!(ActorCommitResultDto::decode(&result.encode()), Ok(result));
  assert!(host.draft.is_empty());
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), observation);
  assert_eq!(
    host.stage_actor_draft(make_draft(ActorDraftField::Message, "too late")),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolRepairHint::AwaitNextObservation,
    ))
  );
  assert_eq!(
    host.commit_actor_draft(first_commit),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolRepairHint::AwaitNextObservation,
    ))
  );
  host
    .apply_line("advance")
    .expect("host advances committed intent");

  let second = host.observation();
  assert_eq!(
    host.commit_actor_draft(ActorCommitDto::new(
      second.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    )),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::StaleObservation,
      ActorProtocolRepairHint::RequestFreshObservation,
    ))
  );
  let second_commit = ActorCommitDto::new(
    second.observer().value(),
    second.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Stabilize,
  );
  host
    .commit_actor_draft(second_commit)
    .expect("explicit second intent commits without metadata");
  assert_eq!(host.record_count(), 1);
  assert_eq!(host.observation(), second);
  host.apply_line("advance").expect("second commit advances");
  assert!(host.is_complete());
  assert_eq!(
    host.commit_actor_draft(second_commit),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut mismatch = CliScenarioHost::fixture();
  let mismatch_observation = mismatch.observation();
  let staged = ActorDraftDto::new(
    mismatch_observation.observer().value(),
    mismatch_observation.observation_id().value(),
    ActorDraftField::Plan,
    "contest",
  )
  .expect("staged plan is bounded");
  mismatch
    .stage_actor_draft(staged)
    .expect("plan stages for mismatch test");
  let mismatch_error = mismatch
    .commit_actor_draft(ActorCommitDto::new(
      mismatch_observation.observer().value(),
      mismatch_observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    ))
    .expect_err("staged plan mismatch is rejected");
  assert_eq!(mismatch_error.code().id(), "host_validation_rejected");
  assert_eq!(mismatch_error.repair().id(), "resend_valid_payload");
  assert_eq!(mismatch.record_count(), 0);
  assert_eq!(mismatch.observation(), mismatch_observation);
  assert_eq!(mismatch.draft.plan.as_deref(), Some("contest"));

  let wrong_actor = ActorCommitDto::new(
    mismatch_observation.observer().value().saturating_add(1),
    mismatch_observation.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Contest,
  );
  assert_eq!(
    mismatch.commit_actor_draft(wrong_actor),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ActorMismatch,
      ActorProtocolRepairHint::UseBoundActor,
    ))
  );
  let mut closed = CliScenarioHost::fixture();
  closed.apply_line("quit").expect("host closes");
  assert_eq!(
    closed.commit_actor_draft(first_commit),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
}

#[test]
fn actor_draft_commit_receipt_reports_presence_without_payload_or_history_advance() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  for (field, value) in [
    (ActorDraftField::Message, "ping ally"),
    (ActorDraftField::Plan, "contest"),
    (ActorDraftField::Contingency, "retreat if threat"),
  ] {
    host
      .stage_actor_draft(
        ActorDraftDto::new(
          observation.observer().value(),
          observation.observation_id().value(),
          field,
          value,
        )
        .expect("draft value is bounded"),
      )
      .expect("draft stages before receipt commit");
  }

  let commit = ActorCommitDto::new(
    observation.observer().value(),
    observation.observation_id().value(),
    crate::protocol::ActorProtocolIntent::Contest,
  );
  let receipt = host
    .commit_actor_draft_receipt(commit)
    .expect("matching receipt commit succeeds");
  assert_eq!(receipt.schema(), "m5-actor-draft-commit-receipt-v1");
  assert_eq!(receipt.observer(), observation.observer().value());
  assert_eq!(
    receipt.observation_id(),
    observation.observation_id().value()
  );
  assert_eq!(
    receipt.intent(),
    crate::protocol::ActorProtocolIntent::Contest
  );
  assert_eq!(receipt.message(), ActorDraftPresence::Present);
  assert_eq!(receipt.plan(), ActorDraftPresence::Present);
  assert_eq!(receipt.contingency(), ActorDraftPresence::Present);
  assert_eq!(
    receipt.encode(),
    "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=1\nintent=contest\nmessage=present\nplan=present\ncontingency=present\n"
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(&receipt.encode()),
    Ok(receipt)
  );
  assert!(!format!("{receipt:?}").contains("ping ally"));
  assert!(!format!("{receipt:?}").contains("retreat if threat"));
  assert!(!receipt.encode().contains("ping ally"));
  assert!(!receipt.encode().contains("retreat if threat"));
  assert!(host.draft.is_empty());
  assert_eq!(host.committed_intent, Some(LaneIntent::Contest));
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), observation);

  let mut empty = CliScenarioHost::fixture();
  let empty_observation = empty.observation();
  let empty_receipt = empty
    .commit_actor_draft_receipt(ActorCommitDto::new(
      empty_observation.observer().value(),
      empty_observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    ))
    .expect("commit without metadata succeeds");
  assert_eq!(empty_receipt.message(), ActorDraftPresence::Absent);
  assert_eq!(empty_receipt.plan(), ActorDraftPresence::Absent);
  assert_eq!(empty_receipt.contingency(), ActorDraftPresence::Absent);

  let mut mismatch = CliScenarioHost::fixture();
  let mismatch_observation = mismatch.observation();
  mismatch
    .stage_actor_draft(
      ActorDraftDto::new(
        mismatch_observation.observer().value(),
        mismatch_observation.observation_id().value(),
        ActorDraftField::Plan,
        "contest",
      )
      .expect("mismatch plan is bounded"),
    )
    .expect("mismatch plan stages");
  assert_eq!(
    mismatch.commit_actor_draft_receipt(ActorCommitDto::new(
      mismatch_observation.observer().value(),
      mismatch_observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    )),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::HostValidationRejected,
      ActorProtocolRepairHint::ResendValidPayload,
    ))
  );
  assert_eq!(mismatch.draft.plan.as_deref(), Some("contest"));
  assert_eq!(mismatch.committed_intent, None);
  assert_eq!(mismatch.record_count(), 0);
}

#[test]
fn actor_draft_staging_is_observation_bound_and_replaces_fields() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  let make_draft = |field, value| {
    ActorDraftDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      field,
      value,
    )
    .expect("draft value is bounded")
  };

  for (field, value) in [
    (ActorDraftField::Message, "ping ally"),
    (ActorDraftField::Plan, "contest"),
    (ActorDraftField::Contingency, "retreat if threat"),
  ] {
    assert_eq!(
      host.stage_actor_draft(make_draft(field, value)),
      Ok(CliHostOutput::DraftStaged { field: field.id() })
    );
  }
  assert_eq!(
    host.stage_actor_draft(make_draft(ActorDraftField::Plan, "stabilize")),
    Ok(CliHostOutput::DraftStaged { field: "plan" })
  );
  let stale_before_commit = ActorDraftDto::new(
    observation.observer().value(),
    observation.observation_id().value() + 1,
    ActorDraftField::Message,
    "stale",
  )
  .expect("draft value is bounded");
  assert_eq!(
    host.stage_actor_draft(stale_before_commit),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::StaleObservation,
      ActorProtocolRepairHint::RequestFreshObservation,
    ))
  );
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), observation);
  assert_eq!(
    host.apply_line("commit"),
    Ok(CliHostOutput::Committed {
      intent: LaneIntent::Stabilize,
    })
  );
  assert_eq!(
    host.stage_actor_draft(make_draft(ActorDraftField::Message, "too late")),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolRepairHint::AwaitNextObservation,
    ))
  );

  let wrong_actor = ActorDraftDto::new(
    2,
    observation.observation_id().value(),
    ActorDraftField::Message,
    "ping",
  )
  .expect("draft value is bounded");
  assert_eq!(
    host.stage_actor_draft(wrong_actor),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ActorMismatch,
      ActorProtocolRepairHint::UseBoundActor,
    ))
  );
  let stale = ActorDraftDto::new(
    observation.observer().value(),
    observation.observation_id().value() + 1,
    ActorDraftField::Message,
    "stale",
  )
  .expect("draft value is bounded");
  assert_eq!(
    host.stage_actor_draft(stale),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolRepairHint::AwaitNextObservation,
    ))
  );

  host.apply_line("advance").expect("first window advances");
  let second = host.observation();
  host
    .submit_actor_action(ActorActionDto::new(
      second.observer().value(),
      second.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Stabilize,
    ))
    .expect("second window closes");
  let complete = host.observation();
  let complete_draft = ActorDraftDto::new(
    complete.observer().value(),
    complete.observation_id().value(),
    ActorDraftField::Message,
    "complete",
  )
  .expect("draft value is bounded");
  assert_eq!(
    host.stage_actor_draft(complete_draft),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut closed = CliScenarioHost::fixture();
  let closed_observation = closed.observation();
  closed.apply_line("quit").expect("host closes");
  let closed_draft = ActorDraftDto::new(
    closed_observation.observer().value(),
    closed_observation.observation_id().value(),
    ActorDraftField::Message,
    "closed",
  )
  .expect("draft value is bounded");
  assert_eq!(
    closed.stage_actor_draft(closed_draft),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
}

#[test]
fn actor_draft_status_reports_presence_without_payload_or_mutation() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  let absent = ActorDraftStatusDto::new(
    observation.observer().value(),
    observation.observation_id().value(),
    ActorDraftPresence::Absent,
    ActorDraftPresence::Absent,
    ActorDraftPresence::Absent,
  );
  assert_eq!(host.actor_draft_status(), Ok(absent));
  for (field, value) in [
    (ActorDraftField::Message, "ping ally"),
    (ActorDraftField::Plan, "contest"),
    (ActorDraftField::Contingency, "retreat if threat"),
  ] {
    host
      .stage_actor_draft(
        ActorDraftDto::new(
          observation.observer().value(),
          observation.observation_id().value(),
          field,
          value,
        )
        .expect("draft value is bounded"),
      )
      .expect("draft stages");
  }
  let present = host
    .actor_draft_status()
    .expect("active draft status is available");
  assert_eq!(present.schema(), "m5-actor-draft-status-v1");
  assert_eq!(present.observer(), observation.observer().value());
  assert_eq!(
    present.observation_id(),
    observation.observation_id().value()
  );
  assert_eq!(present.message(), ActorDraftPresence::Present);
  assert_eq!(present.plan(), ActorDraftPresence::Present);
  assert_eq!(present.contingency(), ActorDraftPresence::Present);
  assert_eq!(
    present.encode(),
    "schema=m5-actor-draft-status-v1\nobserver=1\nobservation_id=1\nmessage=present\nplan=present\ncontingency=present\n"
  );
  assert!(!format!("{present:?}").contains("ping ally"));
  assert!(!present.encode().contains("retreat if threat"));
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), observation);

  host
    .commit_actor_draft(ActorCommitDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    ))
    .expect("matching commit succeeds");
  assert_eq!(
    host.actor_draft_status(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolRepairHint::AwaitNextObservation,
    ))
  );
  host.apply_line("advance").expect("first window advances");
  host
    .apply_line("plan stabilize")
    .expect("second plan stages");
  host.apply_line("commit").expect("second commit succeeds");
  host.apply_line("advance").expect("second window advances");
  assert_eq!(
    host.actor_draft_status(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  host.apply_line("quit").expect("complete host closes");
  assert_eq!(
    host.actor_draft_status(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
}

#[test]
fn actor_draft_readback_is_bound_ordered_and_read_only() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  assert_eq!(host.actor_draft(), Ok(Vec::new()));
  for (field, value) in [
    (ActorDraftField::Message, "ping ally"),
    (ActorDraftField::Plan, "contest"),
    (ActorDraftField::Contingency, "retreat if threat"),
  ] {
    host
      .stage_actor_draft(
        ActorDraftDto::new(
          observation.observer().value(),
          observation.observation_id().value(),
          field,
          value,
        )
        .expect("draft value is bounded"),
      )
      .expect("draft stages");
  }
  let expected = vec![
    ActorDraftDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      ActorDraftField::Message,
      "ping ally",
    )
    .expect("message is bounded"),
    ActorDraftDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      ActorDraftField::Plan,
      "contest",
    )
    .expect("plan is bounded"),
    ActorDraftDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      ActorDraftField::Contingency,
      "retreat if threat",
    )
    .expect("contingency is bounded"),
  ];
  assert_eq!(host.actor_draft(), Ok(expected));
  assert_eq!(host.observation(), observation);
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.committed_intent, None);

  let mut cli_draft = CliScenarioHost::fixture();
  assert_eq!(
    cli_draft.apply_line("plan ???"),
    Ok(CliHostOutput::DraftStaged { field: "plan" })
  );
  assert_eq!(cli_draft.actor_draft(), Ok(Vec::new()));
  cli_draft
    .apply_line(&format!("message {}", "x".repeat(257)))
    .expect("CLI accepts legacy draft text");
  assert_eq!(cli_draft.actor_draft(), Ok(Vec::new()));

  let mut mixed = CliScenarioHost::fixture();
  let mixed_observation = mixed.observation();
  mixed
    .stage_actor_draft(
      ActorDraftDto::new(
        mixed_observation.observer().value(),
        mixed_observation.observation_id().value(),
        ActorDraftField::Message,
        "actor message",
      )
      .expect("actor message is bounded"),
    )
    .expect("actor message stages");
  mixed
    .apply_line("plan contest")
    .expect("CLI plan stages alongside actor metadata");
  assert_eq!(
    mixed.actor_draft(),
    Ok(vec![
      ActorDraftDto::new(
        mixed_observation.observer().value(),
        mixed_observation.observation_id().value(),
        ActorDraftField::Message,
        "actor message",
      )
      .expect("actor message is bounded"),
    ])
  );
  assert_eq!(
    mixed.actor_draft_status(),
    Ok(ActorDraftStatusDto::new(
      mixed_observation.observer().value(),
      mixed_observation.observation_id().value(),
      ActorDraftPresence::Present,
      ActorDraftPresence::Present,
      ActorDraftPresence::Absent,
    ))
  );
  let mixed_clear = ActorDraftClearDto::new(
    mixed_observation.observer().value(),
    mixed_observation.observation_id().value(),
  );
  assert_eq!(
    mixed.clear_actor_draft(mixed_clear),
    Ok(ActorDraftClearReceiptDto::new(
      mixed_observation.observer().value(),
      mixed_observation.observation_id().value(),
      ActorDraftPresence::Present,
      ActorDraftPresence::Present,
      ActorDraftPresence::Absent,
    ))
  );
  assert_eq!(mixed.actor_draft(), Ok(Vec::new()));

  host
    .commit_actor_draft(ActorCommitDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    ))
    .expect("draft commits");
  assert_eq!(
    host.actor_draft(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolRepairHint::AwaitNextObservation,
    ))
  );

  let mut complete = CliScenarioHost::fixture();
  for command in [
    "plan contest",
    "commit",
    "advance",
    "plan stabilize",
    "commit",
    "advance",
  ] {
    complete.apply_line(command).expect("fixture completes");
  }
  assert_eq!(
    complete.actor_draft(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );

  let mut closed = CliScenarioHost::fixture();
  closed.apply_line("quit").expect("host closes");
  assert_eq!(
    closed.actor_draft(),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
}

#[test]
fn actor_draft_clear_is_bound_and_reports_pre_clear_presence() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  let clear = ActorDraftClearDto::new(
    observation.observer().value(),
    observation.observation_id().value(),
  );
  let empty = host
    .clear_actor_draft(clear)
    .expect("empty clear is an idempotent no-op");
  assert_eq!(empty.message(), ActorDraftPresence::Absent);
  assert_eq!(empty.plan(), ActorDraftPresence::Absent);
  assert_eq!(empty.contingency(), ActorDraftPresence::Absent);
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), observation);

  for (field, value) in [
    (ActorDraftField::Message, "ping ally"),
    (ActorDraftField::Plan, "contest"),
    (ActorDraftField::Contingency, "retreat if threat"),
  ] {
    host
      .stage_actor_draft(
        ActorDraftDto::new(
          observation.observer().value(),
          observation.observation_id().value(),
          field,
          value,
        )
        .expect("draft value is bounded"),
      )
      .expect("draft stages");
  }
  let receipt = host.clear_actor_draft(clear).expect("active draft clears");
  assert_eq!(receipt.schema(), "m5-actor-draft-clear-receipt-v1");
  assert_eq!(receipt.message(), ActorDraftPresence::Present);
  assert_eq!(receipt.plan(), ActorDraftPresence::Present);
  assert_eq!(receipt.contingency(), ActorDraftPresence::Present);
  assert_eq!(
    ActorDraftClearReceiptDto::decode(&receipt.encode()),
    Ok(receipt)
  );
  assert!(host.draft.is_empty());
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), observation);

  assert_eq!(
    host.clear_actor_draft(ActorDraftClearDto::new(
      observation.observer().value().saturating_add(1),
      observation.observation_id().value(),
    )),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ActorMismatch,
      ActorProtocolRepairHint::UseBoundActor,
    ))
  );
  assert_eq!(
    host.clear_actor_draft(ActorDraftClearDto::new(
      observation.observer().value(),
      observation.observation_id().value() + 1,
    )),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::StaleObservation,
      ActorProtocolRepairHint::RequestFreshObservation,
    ))
  );

  host
    .stage_actor_draft(
      ActorDraftDto::new(
        observation.observer().value(),
        observation.observation_id().value(),
        ActorDraftField::Plan,
        "contest",
      )
      .expect("plan is bounded"),
    )
    .expect("plan stages");
  host
    .commit_actor_draft(ActorCommitDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    ))
    .expect("commit succeeds");
  assert_eq!(
    host.clear_actor_draft(clear),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DraftBoundary,
      ActorProtocolRepairHint::AwaitNextObservation,
    ))
  );
  host.apply_line("advance").expect("first window advances");
  host
    .apply_line("plan stabilize")
    .expect("second plan stages");
  host.apply_line("commit").expect("second commit succeeds");
  host.apply_line("advance").expect("second window advances");
  assert_eq!(
    host.clear_actor_draft(clear),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::WindowClosed,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  host.apply_line("quit").expect("complete host closes");
  assert_eq!(
    host.clear_actor_draft(clear),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
}

#[test]
fn actor_draft_receipt_acknowledges_existing_staging_without_advancing() {
  let mut host = CliScenarioHost::fixture();
  let first = host.observation();
  let draft = ActorDraftDto::new(
    first.observer().value(),
    first.observation_id().value(),
    ActorDraftField::Plan,
    "contest",
  )
  .expect("draft value is bounded");
  let receipt = host
    .stage_actor_draft_receipt(draft)
    .expect("staging receipt succeeds");
  assert_eq!(
    receipt,
    ActorDraftReceiptDto::new(
      first.observer().value(),
      first.observation_id().value(),
      ActorDraftField::Plan,
    )
  );
  assert_eq!(ActorDraftReceiptDto::decode(&receipt.encode()), Ok(receipt));
  assert_eq!(host.record_count(), 0);
  assert_eq!(host.observation(), first);

  host.apply_line("commit").expect("staged plan commits");
  host.apply_line("advance").expect("first window advances");
  let second = host.observation();
  let second_draft = ActorDraftDto::new(
    second.observer().value(),
    second.observation_id().value(),
    ActorDraftField::Contingency,
    "retreat if threat",
  )
  .expect("second draft value is bounded");
  let second_receipt = host
    .stage_actor_draft_receipt(second_draft)
    .expect("second-window receipt succeeds");
  assert_eq!(second_receipt.field(), ActorDraftField::Contingency);
  assert_eq!(
    second_receipt.observation_id(),
    second.observation_id().value()
  );
  assert_eq!(host.record_count(), 1);
  assert_eq!(host.observation(), second);
}

#[test]
fn actor_authorization_and_redaction_matrix_fails_closed() {
  let mut host = CliScenarioHost::fixture();
  let observation = host.observation();
  let observation_id = observation.observation_id().value();
  let wrong_action = ActorActionDto::new(
    2,
    observation_id,
    crate::protocol::ActorProtocolIntent::Contest,
  );
  let wrong_draft = ActorDraftDto::new(2, observation_id, ActorDraftField::Message, "ping")
    .expect("wrong-actor draft is structurally valid");
  let wrong_commit = ActorCommitDto::new(
    2,
    observation_id,
    crate::protocol::ActorProtocolIntent::Contest,
  );

  for error in [
    host
      .validate_actor_action(wrong_action)
      .expect_err("wrong actor action rejects"),
    host
      .stage_actor_draft(wrong_draft.clone())
      .expect_err("wrong actor draft rejects"),
    host
      .commit_actor_draft(wrong_commit)
      .expect_err("wrong actor commit rejects"),
    host
      .stage_actor_draft_receipt(wrong_draft)
      .expect_err("wrong actor draft receipt rejects"),
  ] {
    assert_eq!(error.code(), ActorProtocolErrorCode::ActorMismatch);
    assert_eq!(error.repair(), ActorProtocolRepairHint::UseBoundActor);
    let error_text = format!("{error:?}\n{}", error.encode()).to_ascii_lowercase();
    for marker in [
      "state",
      "hash",
      "health",
      "position",
      "wave",
      "execution",
      "trace",
      "source",
      "provenance",
      "resolved",
    ] {
      assert!(
        !error_text.contains(marker),
        "actor error leaked marker {marker}: {error_text}"
      );
    }
    assert_eq!(host.record_count(), 0);
    assert_eq!(host.observation(), observation);
  }

  let values = [
    (
      format!(
        "{:?}",
        ActorObservationDto::from_observation(
          observe_player(
            &crate::lane::LaneSnapshot::initial(),
            ObservationId::new(observation_id),
          )
          .observation()
        ),
      ),
      ActorObservationDto::from_observation(
        observe_player(
          &crate::lane::LaneSnapshot::initial(),
          ObservationId::new(observation_id),
        )
        .observation(),
      )
      .encode(),
    ),
    (
      format!(
        "{:?}",
        ActorHistoryDto::new(0, ActorHistoryStatus::Open).expect("open history is bounded")
      ),
      ActorHistoryDto::new(0, ActorHistoryStatus::Open)
        .expect("open history is bounded")
        .encode(),
    ),
    (
      format!(
        "{:?}",
        ActorActionResultDto::new(
          ActorActionResultWindow::First,
          ActorActionResultOutcome::HeldSpace,
        )
      ),
      ActorActionResultDto::new(
        ActorActionResultWindow::First,
        ActorActionResultOutcome::HeldSpace,
      )
      .encode(),
    ),
    (
      format!(
        "{:?}",
        ActorCommitResultDto::new(crate::protocol::ActorProtocolIntent::Contest)
      ),
      ActorCommitResultDto::new(crate::protocol::ActorProtocolIntent::Contest).encode(),
    ),
    (
      format!(
        "{:?}",
        ActorDraftReceiptDto::new(1, observation_id, ActorDraftField::Message)
      ),
      ActorDraftReceiptDto::new(1, observation_id, ActorDraftField::Message).encode(),
    ),
  ];
  for (debug, encoded) in values {
    let value = format!("{debug}\n{encoded}").to_ascii_lowercase();
    for marker in [
      "state",
      "hash",
      "health",
      "position",
      "wave",
      "execution",
      "trace",
      "source",
      "provenance",
      "resolved",
    ] {
      assert!(
        !value.contains(marker),
        "actor value leaked marker {marker}: {value}"
      );
    }
  }
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
fn actor_replay_records_load_from_store_without_mutating_current_host() {
  let root = temporary_store_root();
  let store = CliRunStore::new(&root);
  let mut source = CliScenarioHost::fixture_with_store(store.clone());
  for command in ["plan contest", "commit", "advance", "save first-window"] {
    source.apply_line(command).expect("source store command");
  }

  let mut fresh = CliScenarioHost::fixture_with_store(store);
  let before = fresh.observation();
  let run_id = CliRunId::parse("first-window").expect("run ID is valid");
  assert_eq!(
    fresh.actor_replay_records_from_run(run_id),
    Ok(vec![ActorReplayRecordDto::new(
      ActorActionResultWindow::First,
      crate::protocol::ActorProtocolIntent::Contest,
      ActorActionResultOutcome::HeldSpace,
    )])
  );
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);

  std::fs::write(root.join("first-window.foi-artifact"), "malformed").expect("tamper artifact");
  assert_eq!(
    fresh.actor_replay_records_from_run(run_id),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::HostTransitionRejected,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);
  fresh.apply_line("quit").expect("fresh host closes");
  assert_eq!(
    fresh.actor_replay_records_from_run(run_id),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  let _ = std::fs::remove_dir_all(root);
}

#[test]
fn actor_replay_debrief_records_load_from_store_without_mutating_current_host() {
  let root = temporary_store_root();
  let store = CliRunStore::new(&root);
  let mut source = CliScenarioHost::fixture_with_store(store.clone());
  for command in ["plan contest", "commit", "advance", "save first-window"] {
    source
      .apply_line(command)
      .expect("source first-window command");
  }
  for command in ["plan stabilize", "commit", "advance", "save complete-run"] {
    source
      .apply_line(command)
      .expect("source complete-run command");
  }

  let mut fresh = CliScenarioHost::fixture_with_store(store);
  let before = fresh.observation();
  let first_window = CliRunId::parse("first-window").expect("run ID is valid");
  assert_eq!(
    fresh.actor_replay_debrief_records_from_run(first_window),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::DebriefUnavailable,
      ActorProtocolRepairHint::AwaitCompletion,
    ))
  );
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);

  let complete_run = CliRunId::parse("complete-run").expect("run ID is valid");
  let first = ActorReplayDebriefRecordDto::new(
    ActorActionResultWindow::First,
    crate::protocol::ActorProtocolIntent::Contest,
    ActorActionResultOutcome::HeldSpace,
    ActorDebriefObjective::GoalAchieved,
  );
  let second = ActorReplayDebriefRecordDto::new(
    ActorActionResultWindow::Second,
    crate::protocol::ActorProtocolIntent::Stabilize,
    ActorActionResultOutcome::YieldedSpace,
    ActorDebriefObjective::GoalMissed,
  );
  assert_eq!(
    fresh.actor_replay_debrief_records_from_run(complete_run),
    Ok(vec![first, second])
  );
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);
  assert_eq!(
    first.attribution(),
    ActorDebriefAttributionLimit::CommittedFactsOnly
  );
  assert_eq!(first.verification(), ActorReplayVerification::Verified);
  assert!(!format!("{first:?}").contains("StateHash"));
  assert!(!format!("{first:?}").contains("trace"));

  std::fs::write(root.join("complete-run.foi-artifact"), "malformed")
    .expect("tamper complete artifact");
  assert_eq!(
    fresh.actor_replay_debrief_records_from_run(complete_run),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::HostTransitionRejected,
      ActorProtocolRepairHint::StartNewSession,
    ))
  );
  assert_eq!(fresh.record_count(), 0);
  assert_eq!(fresh.observation(), before);
  fresh.apply_line("quit").expect("fresh host closes");
  assert_eq!(
    fresh.actor_replay_debrief_records_from_run(complete_run),
    Err(ActorProtocolError::new(
      ActorProtocolErrorCode::ClosedSession,
      ActorProtocolRepairHint::StartNewSession,
    ))
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
    Err(CliHostError::BranchUnavailable)
  );
}

#[test]
fn branch_review_is_read_only_and_preserves_parent_artifact() {
  let root = temporary_store_root();
  let mut host = CliScenarioHost::fixture_with_store(CliRunStore::new(&root));
  for command in ["plan contest", "commit", "advance", "save parent"] {
    host.apply_line(command).expect("parent command");
  }
  let before_observation = host.observation();
  let before_artifact = host.saved.as_ref().expect("parent saved").artifact.clone();

  host.apply_line("plan yield").expect("alternate plan");
  assert!(matches!(
    host.apply_line("branch first"),
    Ok(CliHostOutput::Branched {
      point_id,
      parent_intent: LaneIntent::Contest,
      branch_intent: LaneIntent::Yield,
      execution_relation: LaneExecutionRelation::Matched,
      ..
    }) if point_id == "first"
  ));
  assert_eq!(host.record_count(), 1);
  assert_eq!(host.observation(), before_observation);
  assert_eq!(
    host.saved.as_ref().expect("parent saved").artifact,
    before_artifact
  );
  assert_eq!(
    host.apply_line("replay"),
    Ok(CliHostOutput::ReplayVerified {
      run_id: None,
      records: 1
    })
  );

  host
    .apply_line("plan stabilize")
    .expect("second alternate plan");
  assert!(matches!(
    host.apply_line("branch"),
    Ok(CliHostOutput::Branched {
      point_id,
      branch_intent: LaneIntent::Stabilize,
      execution_relation: LaneExecutionRelation::Matched,
      ..
    }) if point_id == "first"
  ));
  assert_eq!(
    host.apply_line("load parent"),
    Ok(CliHostOutput::Loaded {
      run_id: "parent".to_owned(),
      records: 1
    })
  );
  let _ = std::fs::remove_dir_all(root);
}

#[test]
fn branch_rejects_missing_invalid_same_and_unsupported_requests() {
  let mut host = CliScenarioHost::fixture();
  assert_eq!(
    host.apply_line("branch first"),
    Err(CliHostError::BranchUnavailable)
  );
  assert_eq!(
    host.apply_line("branch"),
    Err(CliHostError::BranchUnavailable)
  );
  for command in ["plan contest", "commit", "advance"] {
    host.apply_line(command).expect("parent command");
  }
  assert_eq!(
    host.apply_line("branch first"),
    Err(CliHostError::BranchMissingPlan)
  );
  host.apply_line("plan ???").expect("invalid alternate plan");
  assert_eq!(
    host.apply_line("branch first"),
    Err(CliHostError::InvalidPlan {
      text: "???".to_owned(),
    })
  );
  host
    .apply_line("plan contest")
    .expect("same alternate plan");
  assert_eq!(
    host.apply_line("branch first"),
    Err(CliHostError::BranchUnavailable)
  );
  host.apply_line("plan yield").expect("valid alternate plan");
  assert_eq!(
    host.apply_line("branch second"),
    Err(CliHostError::BranchUnavailable)
  );
  assert_eq!(
    host.apply_line("branch 2"),
    Err(CliHostError::BranchUnavailable)
  );
  assert_eq!(
    host.apply_line("branch third"),
    Err(CliHostError::BranchUnavailable)
  );
  host.apply_line("branch first").expect("valid branch");
  host
    .apply_line("plan stabilize")
    .expect("second-window plan");
  host.apply_line("commit").expect("second-window commit");
  host.apply_line("advance").expect("second-window advance");
  assert_eq!(
    host.apply_line("branch third"),
    Err(CliHostError::BranchUnavailable)
  );
  assert_eq!(
    host.apply_line("branch 99"),
    Err(CliHostError::BranchUnavailable)
  );
  assert_eq!(
    host.apply_line("branch second"),
    Err(CliHostError::BranchMissingPlan)
  );
}

#[test]
fn interactive_branch_exploration_across_multiple_windows() {
  let mut host = CliScenarioHost::fixture();
  host.apply_line("plan contest").expect("plan w1");
  host.apply_line("commit").expect("commit w1");
  host.apply_line("advance").expect("advance w1");

  // Exploration at window 1
  host.apply_line("plan yield").expect("alt plan yield w1");
  let b1 = host.apply_line("branch first").expect("branch first");
  assert_eq!(
    b1,
    CliHostOutput::Branched {
      point_id: "first".to_owned(),
      parent_intent: LaneIntent::Contest,
      branch_intent: LaneIntent::Yield,
      parent_outcome: LaneOutcome::HeldSpace,
      branch_outcome: LaneOutcome::YieldedSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  // Default branch when 1 window is committed targets first window
  let b_default = host.apply_line("branch").expect("branch default w1");
  assert_eq!(
    b_default,
    CliHostOutput::Branched {
      point_id: "first".to_owned(),
      parent_intent: LaneIntent::Contest,
      branch_intent: LaneIntent::Yield,
      parent_outcome: LaneOutcome::HeldSpace,
      branch_outcome: LaneOutcome::YieldedSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  // Play through window 2
  host.apply_line("plan stabilize").expect("plan w2");
  host.apply_line("commit").expect("commit w2");
  host.apply_line("advance").expect("advance w2");

  // Exploration at window 2
  host
    .apply_line("plan contest")
    .expect("alt plan contest w2");
  let b2 = host.apply_line("branch second").expect("branch second");
  assert_eq!(
    b2,
    CliHostOutput::Branched {
      point_id: "second".to_owned(),
      parent_intent: LaneIntent::Stabilize,
      branch_intent: LaneIntent::Contest,
      parent_outcome: LaneOutcome::YieldedSpace,
      branch_outcome: LaneOutcome::HeldSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  // Alias forms for window 2
  let b2_alias = host.apply_line("branch 2").expect("branch 2");
  assert_eq!(
    b2_alias,
    CliHostOutput::Branched {
      point_id: "second".to_owned(),
      parent_intent: LaneIntent::Stabilize,
      branch_intent: LaneIntent::Contest,
      parent_outcome: LaneOutcome::YieldedSpace,
      branch_outcome: LaneOutcome::HeldSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  let b2_rec = host.apply_line("branch rec-1").expect("branch rec-1");
  assert_eq!(
    b2_rec,
    CliHostOutput::Branched {
      point_id: "second".to_owned(),
      parent_intent: LaneIntent::Stabilize,
      branch_intent: LaneIntent::Contest,
      parent_outcome: LaneOutcome::YieldedSpace,
      branch_outcome: LaneOutcome::HeldSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  // Default branch when 2 windows are committed targets latest window (second)
  let b2_default = host.apply_line("branch").expect("branch default w2");
  assert_eq!(
    b2_default,
    CliHostOutput::Branched {
      point_id: "second".to_owned(),
      parent_intent: LaneIntent::Stabilize,
      branch_intent: LaneIntent::Contest,
      parent_outcome: LaneOutcome::YieldedSpace,
      branch_outcome: LaneOutcome::HeldSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  // Exploration back to window 1 even after window 2 is complete
  host.apply_line("plan yield").expect("alt plan yield w1");
  let b1_retro = host.apply_line("branch first").expect("branch first retro");
  assert_eq!(
    b1_retro,
    CliHostOutput::Branched {
      point_id: "first".to_owned(),
      parent_intent: LaneIntent::Contest,
      branch_intent: LaneIntent::Yield,
      parent_outcome: LaneOutcome::HeldSpace,
      branch_outcome: LaneOutcome::YieldedSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  let b1_alias = host.apply_line("branch 1").expect("branch 1 retro");
  assert_eq!(
    b1_alias,
    CliHostOutput::Branched {
      point_id: "first".to_owned(),
      parent_intent: LaneIntent::Contest,
      branch_intent: LaneIntent::Yield,
      parent_outcome: LaneOutcome::HeldSpace,
      branch_outcome: LaneOutcome::YieldedSpace,
      execution_relation: LaneExecutionRelation::Matched,
    }
  );

  // Verify that history and replay are unchanged after branching
  assert_eq!(host.record_count(), 2);
  assert_eq!(
    host.apply_line("replay"),
    Ok(CliHostOutput::ReplayVerified {
      run_id: None,
      records: 2,
    })
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
fn cli_and_actor_protocol_paths_preserve_observation_and_action_parity() {
  let mut cli_host = CliScenarioHost::fixture();
  let cli_observation = match cli_host.apply_line("observe").expect("CLI observes") {
    CliHostOutput::Observation(observation) => observation,
    output => panic!("unexpected CLI observation output: {output:?}"),
  };
  let actor_observation = cli_host
    .actor_observation()
    .expect("active host projects actor observation");
  assert_eq!(
    actor_observation.observer(),
    cli_observation.observer().value()
  );
  assert_eq!(actor_observation.turn(), cli_observation.turn().value());
  assert_eq!(
    actor_observation.observation_id(),
    cli_observation.observation_id().value()
  );
  let cli_intents = cli_observation
    .available_intents()
    .iter()
    .map(|intent| match intent {
      LaneIntent::Stabilize => crate::protocol::ActorProtocolIntent::Stabilize,
      LaneIntent::Contest => crate::protocol::ActorProtocolIntent::Contest,
      LaneIntent::Yield => crate::protocol::ActorProtocolIntent::Yield,
      LaneIntent::Recall => crate::protocol::ActorProtocolIntent::Recall,
      LaneIntent::Withdraw => crate::protocol::ActorProtocolIntent::Withdraw,
    })
    .collect::<Vec<_>>();
  assert_eq!(
    actor_observation.available_actions(),
    cli_intents.as_slice()
  );
  assert_eq!(
    actor_observation.visible_threat_response(),
    cli_observation
      .available_threat_response()
      .map(|intent| match intent {
        LaneIntent::Stabilize => crate::protocol::ActorProtocolIntent::Stabilize,
        LaneIntent::Contest => crate::protocol::ActorProtocolIntent::Contest,
        LaneIntent::Yield => crate::protocol::ActorProtocolIntent::Yield,
        LaneIntent::Recall => crate::protocol::ActorProtocolIntent::Recall,
        LaneIntent::Withdraw => crate::protocol::ActorProtocolIntent::Withdraw,
      })
  );

  let cli_advanced = {
    cli_host.apply_line("plan contest").expect("CLI plan");
    cli_host.apply_line("commit").expect("CLI commit");
    match cli_host.apply_line("advance").expect("CLI advance") {
      CliHostOutput::Advanced { window, outcome } => (window, outcome),
      output => panic!("unexpected CLI advance output: {output:?}"),
    }
  };
  let mut actor_host = CliScenarioHost::fixture();
  let observation = actor_host.observation();
  let actor_result = actor_host
    .submit_actor_action_result(ActorActionDto::new(
      observation.observer().value(),
      observation.observation_id().value(),
      crate::protocol::ActorProtocolIntent::Contest,
    ))
    .expect("actor protocol action");
  assert_eq!(cli_advanced.0, ScenarioWindow::First);
  assert_eq!(actor_result.window(), ActorActionResultWindow::First);
  let expected_outcome = match cli_advanced.1 {
    LaneOutcome::HeldSpace => ActorActionResultOutcome::HeldSpace,
    LaneOutcome::YieldedSpace => ActorActionResultOutcome::YieldedSpace,
    LaneOutcome::ForcedOut => ActorActionResultOutcome::ForcedOut,
  };
  assert_eq!(actor_result.outcome(), expected_outcome);
  assert_eq!(cli_host.record_count(), actor_host.record_count());
  assert_eq!(cli_host.observation(), actor_host.observation());
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

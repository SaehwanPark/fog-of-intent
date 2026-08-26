use super::*;
use crate::host::CliScenarioHost;
use crate::kernel::{DrawId, StreamId};
use crate::lane::{
  ALLIED_AUTONOMOUS_ACTOR, JungleThreatTruth, LaneAbortCondition, LaneActorRole, LaneCommitment,
  LaneFallbackBehavior, LaneHealth, LaneIntent, LaneOutcome, LanePingSignal, LaneSnapshot,
  LaneStatus, LaneTargetFocus, M2_LANE_RULESET, ObservationId, PLAYER_LANER, PlayerLaneState,
  WavePressure, WaveState, observe_player, validate_lane_request,
};
use crate::protocol::{
  ActorActionDto, ActorMessageDto, ActorProtocolCodecError, ActorProtocolIntent,
  MAX_ACTOR_DRAFT_VALUE_BYTES,
};

#[test]
fn cautious_agent_uses_initial_actor_visible_candidates_and_legal_request() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(9));
  let agent = ScriptedAgent::cautious_v1();
  let decision = agent.choose(receipt.observation());

  assert_eq!(SCRIPTED_AGENT_SCHEMA, "m4-scripted-agent-v1");
  assert_eq!(decision.profile().profile_id(), SCRIPTED_AGENT_PROFILE_ID);
  assert_eq!(decision.observer(), receipt.observation().observer());
  assert_eq!(
    decision.observation_id(),
    receipt.observation().observation_id()
  );
  assert_eq!(decision.selected_intent(), LaneIntent::Stabilize);
  assert_eq!(decision.candidates().len(), 4);
  assert_eq!(decision.request().intent(), LaneIntent::Stabilize);
  assert!(decision.candidates().iter().any(|candidate| {
    candidate.intent() == LaneIntent::Contest
      && candidate.reason() == ScriptedAgentReason::AvailableAlternative
  }));
  assert_eq!(
    agent
      .evaluate_candidate(receipt.observation(), LaneIntent::Contest)
      .expect("advertised intent evaluates"),
    ScriptedAgentCandidate {
      intent: LaneIntent::Contest,
      score: 60,
      reason: ScriptedAgentReason::AvailableAlternative,
    }
  );
  validate_lane_request(&state, &receipt, &decision.request()).expect("policy request is legal");
}

#[test]
fn experiment_manifest_codec_binds_profiles_rules_and_seed() {
  let seed = ScriptedAgentSeedBundle::new(42, StreamId::new(7), DrawId::new(9));
  let profiles = [
    ScriptedAgentProfile::cautious_v1(),
    ScriptedAgentProfile::risk_taking_v1(),
    ScriptedAgentProfile::yielding_v1(),
  ];
  for profile in profiles {
    let manifest = ScriptedAgentExperimentManifest::new(profile, seed);
    assert_eq!(manifest.schema(), "m6-experiment-manifest-v1");
    assert_eq!(manifest.scenario_id(), "m3-two-window-fixture-v1");
    assert_eq!(manifest.profile().profile_id(), profile.profile_id());
    assert_eq!(
      manifest.profile().evaluation_rule(),
      profile.evaluation_rule()
    );
    assert_eq!(manifest.selection_rule(), "max-score-seeded-tie-v1");
    assert_eq!(manifest.seed_bundle(), seed);
    assert_eq!(
      ScriptedAgentExperimentManifest::decode(&manifest.encode()),
      Ok(manifest)
    );
  }
  assert_eq!(
    ScriptedAgentExperimentManifest::new(profiles[0], seed).encode(),
    "schema=m6-experiment-manifest-v1\nscenario=m3-two-window-fixture-v1\nprofile=cautious-laner-v1\nevaluation_rule=threat-first-pressure-aware-fixed-score-v1\nselection_rule=max-score-seeded-tie-v1\nseed=42\npolicy_stream=7\npolicy_draw=9\n"
  );

  let valid = ScriptedAgentExperimentManifest::new(profiles[0], seed).encode();
  for malformed in [
    (
      valid.replacen("schema=m6-experiment-manifest-v1", "schema=other", 1),
      ScriptedAgentManifestError::UnsupportedSchema,
    ),
    (
      valid.replacen("profile=cautious-laner-v1", "profile=unknown", 1),
      ScriptedAgentManifestError::InvalidValue,
    ),
    (
      valid.replacen(
        "evaluation_rule=threat-first-pressure-aware-fixed-score-v1",
        "evaluation_rule=wrong",
        1,
      ),
      ScriptedAgentManifestError::InvalidValue,
    ),
    (
      valid.replacen(
        "selection_rule=max-score-seeded-tie-v1",
        "selection_rule=wrong",
        1,
      ),
      ScriptedAgentManifestError::InvalidValue,
    ),
    (
      valid.replacen("policy_stream=7", "policy_stream=nope", 1),
      ScriptedAgentManifestError::InvalidValue,
    ),
    (
      valid.replacen("policy_draw=9", "policy_draw=nope", 1),
      ScriptedAgentManifestError::InvalidValue,
    ),
    (
      valid.replacen("seed=42", "seed=nope", 1),
      ScriptedAgentManifestError::InvalidValue,
    ),
    (
      valid.replacen("scenario=m3-two-window-fixture-v1", "scenario=other", 1),
      ScriptedAgentManifestError::InvalidValue,
    ),
    (
      valid.replacen("profile=cautious-laner-v1", "unknown=profile", 1),
      ScriptedAgentManifestError::UnknownField,
    ),
    (
      valid.replacen("profile=cautious-laner-v1\n", "", 1),
      ScriptedAgentManifestError::MissingField,
    ),
    (
      valid.replacen(
        "profile=cautious-laner-v1",
        "schema=m6-experiment-manifest-v1",
        1,
      ),
      ScriptedAgentManifestError::DuplicateField,
    ),
    (
      format!("{valid}extra=value\n"),
      ScriptedAgentManifestError::UnexpectedLineCount {
        expected: 8,
        actual: 9,
      },
    ),
  ] {
    assert_eq!(
      ScriptedAgentExperimentManifest::decode(&malformed.0),
      Err(malformed.1)
    );
  }
  assert_eq!(
    ScriptedAgentExperimentManifest::decode(&"x".repeat(MAX_SCRIPTED_AGENT_MANIFEST_BYTES + 1)),
    Err(ScriptedAgentManifestError::Oversized)
  );
}

#[test]
fn experiment_version_catalog_is_literal_and_deterministic() {
  let catalog = ScriptedAgentExperimentVersionCatalog::current();
  assert_eq!(catalog.schema(), "m6-experiment-version-catalog-v1");
  assert_eq!(catalog.ruleset_id(), "m2-lane-ruleset-v4");
  assert_eq!(M2_LANE_RULESET.value(), 4);
  assert_eq!(catalog.scenario_id(), "m3-two-window-fixture-v1");
  assert_eq!(catalog.policy_schema(), "m4-scripted-agent-v1");
  assert_eq!(
    catalog.profile_ids(),
    [
      "cautious-laner-v1",
      "risk-taking-laner-v1",
      "yielding-laner-v1",
    ]
  );
  assert_eq!(catalog.prompt_version(), "not-applicable");
  assert_eq!(catalog.tool_schema_version(), "not-applicable");
  assert_eq!(catalog.model_version(), "not-applicable");
  assert_eq!(catalog.extractor_version(), "not-applicable");
  assert_eq!(catalog, ScriptedAgentExperimentVersionCatalog::current());
}

#[test]
fn bounded_batch_runner_preserves_order_and_reproducibility() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(44)).observation();
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(1, StreamId::new(2), DrawId::new(3)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(4, StreamId::new(5), DrawId::new(6)),
    ),
  ];
  let first = ScriptedAgentBatchRunner::run(observation, &manifests).expect("batch runs");
  let second = ScriptedAgentBatchRunner::run(observation, &manifests).expect("batch repeats");
  assert_eq!(first, second);
  assert_eq!(first.len(), 2);
  assert_eq!(first[0].profile(), manifests[0].profile());
  assert_eq!(first[1].profile(), manifests[1].profile());
  assert_eq!(first[0].seed_bundle(), Some(manifests[0].seed_bundle()));
  assert_eq!(first[1].seed_bundle(), Some(manifests[1].seed_bundle()));
  assert_eq!(
    ScriptedAgentBatchRunner::run(observation, &[]),
    Err(ScriptedAgentBatchError::EmptyBatch)
  );
  let at_capacity = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS];
  let capacity_decisions =
    ScriptedAgentBatchRunner::run(observation, &at_capacity).expect("inclusive cap runs");
  assert_eq!(capacity_decisions.len(), MAX_SCRIPTED_AGENT_BATCH_MANIFESTS);
  assert!(
    capacity_decisions
      .iter()
      .all(|decision| decision.seed_bundle() == Some(manifests[0].seed_bundle()))
  );
  let too_many = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1];
  assert_eq!(
    ScriptedAgentBatchRunner::run(observation, &too_many),
    Err(ScriptedAgentBatchError::BatchTooLarge {
      max: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS,
      actual: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1,
    })
  );
}

#[test]
fn run_disposition_codec_preserves_all_closed_statuses_and_rejects_malformed_text() {
  let dispositions = [
    (ScriptedAgentRunDisposition::Completed, "completed"),
    (ScriptedAgentRunDisposition::Crashed, "crashed"),
    (ScriptedAgentRunDisposition::TimedOut, "timed_out"),
    (ScriptedAgentRunDisposition::MissingBranch, "missing_branch"),
    (ScriptedAgentRunDisposition::Inconclusive, "inconclusive"),
  ];
  for (disposition, expected_id) in dispositions {
    let record = ScriptedAgentRunDispositionRecord::new(disposition);
    assert_eq!(record.schema(), SCRIPTED_AGENT_RUN_DISPOSITION_SCHEMA);
    assert_eq!(record.disposition(), disposition);
    assert_eq!(disposition.id(), expected_id);
    assert_eq!(
      record.encode(),
      format!("schema=m6-scripted-agent-run-disposition-v1\ndisposition={expected_id}\n")
    );
    assert_eq!(
      ScriptedAgentRunDispositionRecord::decode(&record.encode()),
      Ok(record)
    );
  }
  let valid =
    ScriptedAgentRunDispositionRecord::new(ScriptedAgentRunDisposition::Completed).encode();
  assert_eq!(
    valid,
    "schema=m6-scripted-agent-run-disposition-v1\ndisposition=completed\n"
  );
  for (malformed, expected) in [
    (
      valid.replacen(
        "schema=m6-scripted-agent-run-disposition-v1",
        "schema=other",
        1,
      ),
      ScriptedAgentRunDispositionCodecError::UnsupportedSchema,
    ),
    (
      valid.replacen("disposition=completed", "unknown=completed", 1),
      ScriptedAgentRunDispositionCodecError::UnknownField,
    ),
    (
      valid.replacen("schema=", "disposition=", 1),
      ScriptedAgentRunDispositionCodecError::DuplicateField,
    ),
    (
      valid.replacen("disposition=completed\n", "", 1),
      ScriptedAgentRunDispositionCodecError::MissingField,
    ),
    (
      valid.replacen("disposition=completed", "disposition=unknown", 1),
      ScriptedAgentRunDispositionCodecError::InvalidValue,
    ),
    (
      format!("{valid}extra=value\n"),
      ScriptedAgentRunDispositionCodecError::UnexpectedLineCount {
        expected: 2,
        actual: 3,
      },
    ),
  ] {
    assert_eq!(
      ScriptedAgentRunDispositionRecord::decode(&malformed),
      Err(expected)
    );
  }
  assert_eq!(MAX_SCRIPTED_AGENT_RUN_DISPOSITION_BYTES, 4096);
  assert_eq!(
    ScriptedAgentRunDispositionRecord::decode(&"x".repeat(4096)),
    Err(ScriptedAgentRunDispositionCodecError::InvalidValue)
  );
  assert_eq!(
    ScriptedAgentRunDispositionRecord::decode(&"x".repeat(4097)),
    Err(ScriptedAgentRunDispositionCodecError::Oversized)
  );
}

#[test]
fn operational_log_preserves_closed_ordered_events_without_history_payloads() {
  let events = [
    (ScriptedAgentOperationalEvent::BatchStarted, "batch_started"),
    (
      ScriptedAgentOperationalEvent::ChunkCompleted,
      "chunk_completed",
    ),
    (
      ScriptedAgentOperationalEvent::CheckpointSaved,
      "checkpoint_saved",
    ),
    (ScriptedAgentOperationalEvent::BatchResumed, "batch_resumed"),
    (
      ScriptedAgentOperationalEvent::BatchFinished,
      "batch_finished",
    ),
  ];
  let mut log = ScriptedAgentOperationalLog::new();
  assert_eq!(log.schema(), "m6-scripted-agent-operational-event-v1");
  assert!(log.is_empty());
  assert_eq!(log.len(), 0);
  for (event, expected_id) in events {
    assert_eq!(event.id(), expected_id);
    log.append(event).expect("event fits in operational log");
  }
  assert_eq!(log.len(), events.len());
  assert!(!log.is_empty());
  assert_eq!(log.entries()[0].schema(), log.schema());
  assert_eq!(
    log.entries()[0].event(),
    ScriptedAgentOperationalEvent::BatchStarted
  );
  assert_eq!(
    log.entries()[1].event(),
    ScriptedAgentOperationalEvent::ChunkCompleted
  );
  assert_eq!(
    log.entries()[2].event(),
    ScriptedAgentOperationalEvent::CheckpointSaved
  );
  assert_eq!(
    log.entries()[3].event(),
    ScriptedAgentOperationalEvent::BatchResumed
  );
  assert_eq!(
    log.entries()[4].event(),
    ScriptedAgentOperationalEvent::BatchFinished
  );
  assert_eq!(
    log.encode(),
    "schema=m6-scripted-agent-operational-log-v1\nentries=5\nevent=batch_started\nevent=chunk_completed\nevent=checkpoint_saved\nevent=batch_resumed\nevent=batch_finished\n"
  );
  assert_eq!(
    ScriptedAgentOperationalLog::decode(&log.encode()),
    Ok(log.clone())
  );
  let encoded = log.encode();
  for (malformed, expected) in [
    (
      encoded.replacen(
        "schema=m6-scripted-agent-operational-log-v1",
        "schema=other",
        1,
      ),
      ScriptedAgentOperationalLogCodecError::UnsupportedSchema,
    ),
    (
      encoded.replacen("entries=5", "unknown=5", 1),
      ScriptedAgentOperationalLogCodecError::UnknownField,
    ),
    (
      encoded.replacen(
        "entries=5\n",
        "schema=m6-scripted-agent-operational-log-v1\nentries=5\n",
        1,
      ),
      ScriptedAgentOperationalLogCodecError::DuplicateField,
    ),
    (
      encoded.replacen("entries=5\n", "", 1),
      ScriptedAgentOperationalLogCodecError::MissingField,
    ),
    (
      encoded.replacen("event=batch_finished", "event=unknown", 1),
      ScriptedAgentOperationalLogCodecError::InvalidValue,
    ),
    (
      "not-a-field".to_owned(),
      ScriptedAgentOperationalLogCodecError::InvalidValue,
    ),
    (
      "schema=\nentries=0\n".to_owned(),
      ScriptedAgentOperationalLogCodecError::InvalidValue,
    ),
    (
      encoded.replacen("entries=5", "entries=not-a-number", 1),
      ScriptedAgentOperationalLogCodecError::InvalidValue,
    ),
    (
      encoded.replacen("entries=5", "entries=17", 1),
      ScriptedAgentOperationalLogCodecError::InvalidValue,
    ),
    (
      format!("{encoded}event=batch_started\n"),
      ScriptedAgentOperationalLogCodecError::UnexpectedLineCount {
        expected: 7,
        actual: 8,
      },
    ),
    (
      format!(
        "schema=m6-scripted-agent-operational-log-v1\nentries=16\n{}",
        "event=batch_started\n".repeat(17)
      ),
      ScriptedAgentOperationalLogCodecError::UnexpectedLineCount {
        expected: 18,
        actual: 19,
      },
    ),
  ] {
    assert_eq!(
      ScriptedAgentOperationalLog::decode(&malformed),
      Err(expected)
    );
  }
  let swapped_headers = "entries=5\nschema=m6-scripted-agent-operational-log-v1\nevent=batch_started\nevent=chunk_completed\nevent=checkpoint_saved\nevent=batch_resumed\nevent=batch_finished\n";
  assert_eq!(
    ScriptedAgentOperationalLog::decode(swapped_headers),
    Err(ScriptedAgentOperationalLogCodecError::InvalidValue)
  );
  let event_before_headers = "event=batch_started\nentries=5\nschema=m6-scripted-agent-operational-log-v1\nevent=chunk_completed\nevent=checkpoint_saved\nevent=batch_resumed\nevent=batch_finished\n";
  assert_eq!(
    ScriptedAgentOperationalLog::decode(event_before_headers),
    Err(ScriptedAgentOperationalLogCodecError::InvalidValue)
  );
  assert_eq!(MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_BYTES, 4096);
  assert_eq!(
    SCRIPTED_AGENT_OPERATIONAL_LOG_SCHEMA,
    "m6-scripted-agent-operational-log-v1"
  );
  let inclusive_size_input = format!("{}\n", "x".repeat(4095));
  assert_eq!(
    inclusive_size_input.len(),
    MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_BYTES
  );
  assert_eq!(
    ScriptedAgentOperationalLog::decode(&inclusive_size_input),
    Err(ScriptedAgentOperationalLogCodecError::InvalidValue)
  );
  assert_eq!(
    ScriptedAgentOperationalLog::decode(&"x".repeat(4097)),
    Err(ScriptedAgentOperationalLogCodecError::Oversized)
  );
  assert_eq!(MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS, 16);
  for _ in events.len()..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
    log
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("event fits at inclusive cap");
  }
  assert_eq!(log.len(), MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS);
  let entries_before_overflow = log.entries().to_vec();
  assert_eq!(
    log.append(ScriptedAgentOperationalEvent::BatchFinished),
    Err(ScriptedAgentOperationalLogError::CapacityExceeded {
      max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
    })
  );
  assert_eq!(log.len(), MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS);
  assert_eq!(log.entries(), entries_before_overflow.as_slice());
  assert_eq!(
    ScriptedAgentOperationalLog::decode(&log.encode()),
    Ok(log.clone())
  );
}

#[test]
fn operational_log_sequence_status_is_closed_ordered_and_read_only() {
  let build_log = |events: &[ScriptedAgentOperationalEvent]| {
    let mut log = ScriptedAgentOperationalLog::new();
    for event in events {
      log.append(*event).expect("sequence fixture fits");
    }
    log
  };
  let complete = build_log(&[
    ScriptedAgentOperationalEvent::BatchStarted,
    ScriptedAgentOperationalEvent::ChunkCompleted,
    ScriptedAgentOperationalEvent::BatchFinished,
  ]);
  let before = complete.clone();
  let report = ScriptedAgentOperationalLogSequenceReport::from_log(&complete);
  assert_eq!(
    report.schema(),
    "m6-scripted-agent-operational-log-sequence-v1"
  );
  assert_eq!(report.rule(), "m6-operational-start-chunk-finish-v1");
  assert_eq!(
    report.status(),
    ScriptedAgentOperationalLogSequenceStatus::Complete
  );
  assert_eq!(report.status().id(), "complete");
  assert_eq!(
    ScriptedAgentOperationalLogSequenceReport::from_log(&complete),
    report,
    "repeated sequence classification is deterministic"
  );
  assert_eq!(
    complete, before,
    "status inspection does not mutate the log"
  );

  let optional = build_log(&[
    ScriptedAgentOperationalEvent::BatchStarted,
    ScriptedAgentOperationalEvent::ChunkCompleted,
    ScriptedAgentOperationalEvent::CheckpointSaved,
    ScriptedAgentOperationalEvent::BatchResumed,
    ScriptedAgentOperationalEvent::BatchFinished,
  ]);
  assert_eq!(
    ScriptedAgentOperationalLogSequenceReport::from_log(&optional).status(),
    ScriptedAgentOperationalLogSequenceStatus::Complete
  );

  for (events, expected) in [
    (
      &[][..],
      ScriptedAgentOperationalLogSequenceStatus::MissingStart,
    ),
    (
      &[ScriptedAgentOperationalEvent::BatchStarted][..],
      ScriptedAgentOperationalLogSequenceStatus::MissingChunk,
    ),
    (
      &[
        ScriptedAgentOperationalEvent::BatchStarted,
        ScriptedAgentOperationalEvent::ChunkCompleted,
      ][..],
      ScriptedAgentOperationalLogSequenceStatus::MissingFinish,
    ),
    (
      &[
        ScriptedAgentOperationalEvent::ChunkCompleted,
        ScriptedAgentOperationalEvent::BatchFinished,
      ][..],
      ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
    ),
    (
      &[
        ScriptedAgentOperationalEvent::BatchStarted,
        ScriptedAgentOperationalEvent::CheckpointSaved,
        ScriptedAgentOperationalEvent::ChunkCompleted,
        ScriptedAgentOperationalEvent::BatchFinished,
      ][..],
      ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
    ),
    (
      &[
        ScriptedAgentOperationalEvent::BatchStarted,
        ScriptedAgentOperationalEvent::ChunkCompleted,
        ScriptedAgentOperationalEvent::BatchFinished,
        ScriptedAgentOperationalEvent::BatchStarted,
      ][..],
      ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
    ),
  ] {
    assert_eq!(
      ScriptedAgentOperationalLogSequenceReport::from_log(&build_log(events)).status(),
      expected
    );
  }
  assert_eq!(
    [
      ScriptedAgentOperationalLogSequenceStatus::Complete,
      ScriptedAgentOperationalLogSequenceStatus::MissingStart,
      ScriptedAgentOperationalLogSequenceStatus::MissingChunk,
      ScriptedAgentOperationalLogSequenceStatus::MissingFinish,
      ScriptedAgentOperationalLogSequenceStatus::InvalidOrder,
    ]
    .into_iter()
    .map(ScriptedAgentOperationalLogSequenceStatus::id)
    .collect::<Vec<_>>(),
    vec![
      "complete",
      "missing_start",
      "missing_chunk",
      "missing_finish",
      "invalid_order"
    ]
  );
}

#[test]
fn batch_runner_operational_log_producer_is_ordered_and_preflights_capacity() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(47)).observation();
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(1, StreamId::new(2), DrawId::new(3)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(4, StreamId::new(5), DrawId::new(6)),
    ),
  ];
  let expected = ScriptedAgentBatchRunner::run(observation, &manifests)
    .expect("the direct batch remains the parity reference");
  let mut log = ScriptedAgentOperationalLog::new();
  let produced =
    ScriptedAgentBatchRunner::run_with_operational_log(observation, &manifests, &mut log)
      .expect("the complete batch fits in the operational log");
  assert_eq!(produced, expected);
  assert_eq!(
    log
      .entries()
      .iter()
      .map(|entry| entry.event().id())
      .collect::<Vec<_>>(),
    ["batch_started", "chunk_completed", "batch_finished"]
  );

  let mut invalid_log = ScriptedAgentOperationalLog::new();
  invalid_log
    .append(ScriptedAgentOperationalEvent::CheckpointSaved)
    .expect("one event fits");
  let invalid_before = invalid_log.entries().to_vec();
  assert_eq!(
    ScriptedAgentBatchRunner::run_with_operational_log(observation, &[], &mut invalid_log),
    Err(ScriptedAgentOperationalBatchRunError::Batch(
      ScriptedAgentBatchError::EmptyBatch,
    ))
  );
  assert_eq!(invalid_log.entries(), invalid_before.as_slice());

  assert_eq!(MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS, 16);
  let mut at_capacity_log = ScriptedAgentOperationalLog::new();
  for _ in 0..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS - 3 {
    at_capacity_log
      .append(ScriptedAgentOperationalEvent::CheckpointSaved)
      .expect("inclusive-capacity fixture fits");
  }
  let at_capacity_decisions = ScriptedAgentBatchRunner::run_with_operational_log(
    observation,
    &manifests,
    &mut at_capacity_log,
  )
  .expect("exactly three lifecycle events fit at the inclusive cap");
  assert_eq!(at_capacity_decisions, expected);
  assert_eq!(at_capacity_log.len(), MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS);
  assert_eq!(
    at_capacity_log
      .entries()
      .iter()
      .rev()
      .take(3)
      .map(|entry| entry.event().id())
      .collect::<Vec<_>>()
      .into_iter()
      .rev()
      .collect::<Vec<_>>(),
    ["batch_started", "chunk_completed", "batch_finished"]
  );

  let mut full_log = ScriptedAgentOperationalLog::new();
  for _ in 0..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS - 2 {
    full_log
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("preflight fixture fits");
  }
  let full_before = full_log.entries().to_vec();
  assert_eq!(
    ScriptedAgentBatchRunner::run_with_operational_log(observation, &manifests, &mut full_log),
    Err(ScriptedAgentOperationalBatchRunError::LogCapacityExceeded { max: 16 })
  );
  assert_eq!(full_log.entries(), full_before.as_slice());
}

#[test]
fn batch_checkpoint_codec_and_store_resume_one_chunk() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(45)).observation();
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(7, StreamId::new(8), DrawId::new(9)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(10, StreamId::new(11), DrawId::new(12)),
    ),
  ];
  let checkpoint =
    ScriptedAgentBatchCheckpoint::new(observation, &manifests).expect("checkpoint starts");
  assert_eq!(checkpoint.schema(), SCRIPTED_AGENT_BATCH_RUN_SCHEMA);
  assert_eq!(
    SCRIPTED_AGENT_BATCH_RUN_SCHEMA,
    "m6-scripted-agent-batch-run-v1"
  );
  let encoded = checkpoint.encode();
  assert_eq!(
    encoded,
    format!(
      "schema=m6-scripted-agent-batch-run-v1\nobserver={}\nobservation_id=45\nmanifest_count=2\ncompleted_count=0\ninput_fingerprint={}\n",
      observation.observer().value(),
      12216804097755993549u64,
    )
  );
  assert_eq!(
    ScriptedAgentBatchCheckpoint::decode(&encoded),
    Ok(checkpoint)
  );
  let valid = encoded;
  for (malformed, expected) in [
    (
      valid.replacen("schema=m6-scripted-agent-batch-run-v1", "schema=other", 1),
      ScriptedAgentBatchCheckpointError::UnsupportedSchema,
    ),
    (
      valid.replacen("observer=", "unknown=", 1),
      ScriptedAgentBatchCheckpointError::UnknownField,
    ),
    (
      valid.replacen("observer=", "schema=", 1),
      ScriptedAgentBatchCheckpointError::DuplicateField,
    ),
    (
      valid.replacen("completed_count=0\n", "", 1),
      ScriptedAgentBatchCheckpointError::MissingField,
    ),
    (
      format!("{valid}extra=value\n"),
      ScriptedAgentBatchCheckpointError::UnexpectedLineCount {
        expected: 6,
        actual: 7,
      },
    ),
    (
      valid.replacen("completed_count=0", "completed_count=3", 1),
      ScriptedAgentBatchCheckpointError::InvalidValue,
    ),
  ] {
    assert_eq!(
      ScriptedAgentBatchCheckpoint::decode(&malformed),
      Err(expected)
    );
  }
  assert_eq!(
    ScriptedAgentBatchCheckpoint::decode(&"x".repeat(MAX_SCRIPTED_AGENT_BATCH_RUN_BYTES + 1)),
    Err(ScriptedAgentBatchCheckpointError::Oversized)
  );

  let root = std::env::temp_dir().join(format!("fog-of-intent-agent-batch-{}", std::process::id()));
  let store = crate::agent_batch_store::ScriptedAgentBatchRunStore::new(&root);
  let host_store = crate::run_store::CliRunStore::new(&root);
  let host_artifact = "artifact schema=m3-cli-host-artifact-v1 replay_id=m2-two-window-scenario-v3 run_id=resume records=0";
  host_store
    .save("resume", host_artifact)
    .expect("host artifact saves");
  let mut operational_log = ScriptedAgentOperationalLog::new();
  store
    .save_with_operational_log("resume", checkpoint, &mut operational_log)
    .expect("checkpoint saves with an event");
  assert_eq!(
    operational_log.entries()[0].event(),
    ScriptedAgentOperationalEvent::CheckpointSaved
  );
  assert_eq!(
    host_store.load("resume").expect("host artifact loads"),
    host_artifact
  );
  let loaded = store
    .load_with_operational_log("resume", &mut operational_log)
    .expect("checkpoint loads with an event");
  assert_eq!(
    operational_log.entries()[1].event(),
    ScriptedAgentOperationalEvent::BatchResumed
  );
  let operational_store =
    crate::agent_operational_store::ScriptedAgentOperationalLogStore::new(&root);
  operational_store
    .save("resume", &operational_log)
    .expect("operational log saves beside checkpoint");
  assert_eq!(
    host_store
      .load("resume")
      .expect("host artifact survives log save"),
    host_artifact
  );
  assert_eq!(
    store.load("resume").expect("checkpoint survives log save"),
    checkpoint
  );
  assert!(root.join("resume.foi-operational-log").is_file());
  assert_eq!(
    operational_store
      .load("resume")
      .expect("operational log loads"),
    operational_log
  );
  assert_eq!(
    crate::agent_operational_store::MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS,
    4
  );
  let mut first_segment = ScriptedAgentOperationalLog::new();
  first_segment
    .append(ScriptedAgentOperationalEvent::BatchStarted)
    .expect("first segment fits");
  let mut second_segment = ScriptedAgentOperationalLog::new();
  second_segment
    .append(ScriptedAgentOperationalEvent::BatchFinished)
    .expect("second segment fits");
  operational_store
    .save_segment("resume", 0, &first_segment)
    .expect("first segment saves");
  operational_store
    .save_segment("resume", 1, &second_segment)
    .expect("second segment saves");
  operational_store
    .save_segment("resume", 3, &second_segment)
    .expect("highest segment saves");
  std::fs::write(root.join("resume.foi-operational-log.segment-01"), "bad")
    .expect("leading-zero fixture writes");
  std::fs::write(root.join("resume.foi-operational-log.segment-4"), "bad")
    .expect("out-of-range fixture writes");
  std::fs::write(root.join("resume.foi-operational-log.segment-.tmp0"), "bad")
    .expect("temporary-name fixture writes");
  std::fs::create_dir(root.join("resume.foi-operational-log.segment-2"))
    .expect("non-file fixture creates");
  assert_eq!(
    operational_store
      .load_segment("resume", 0)
      .expect("first segment loads"),
    first_segment
  );
  assert_eq!(
    operational_store
      .load_segment("resume", 1)
      .expect("second segment loads"),
    second_segment
  );
  assert_eq!(
    operational_store
      .load_segment("resume", 3)
      .expect("highest segment loads"),
    second_segment
  );
  assert!(root.join("resume.foi-operational-log.segment-0").is_file());
  assert!(root.join("resume.foi-operational-log.segment-1").is_file());
  assert!(root.join("resume.foi-operational-log.segment-3").is_file());
  assert_eq!(
    operational_store
      .list_segments("resume")
      .expect("segments list"),
    vec![0, 1, 3]
  );
  assert_eq!(
    operational_store
      .load("resume")
      .expect("base log survives segments"),
    operational_log
  );
  let invalid_segment_root = root.join("invalid-segment");
  let invalid_segment_store =
    crate::agent_operational_store::ScriptedAgentOperationalLogStore::new(&invalid_segment_root);
  assert_eq!(
    invalid_segment_store.save_segment("resume", 4, &first_segment),
    Err(
      crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::InvalidSegment {
        max: 4,
      }
    )
  );
  assert!(!invalid_segment_root.exists());
  assert_eq!(
    invalid_segment_store.list_segments("resume"),
    Err(crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::StorageUnavailable)
  );
  assert_eq!(
    operational_store.list_segments("bad/id"),
    Err(crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::StorageUnavailable)
  );
  assert_eq!(
    invalid_segment_store.load_segment("resume", 4),
    Err(
      crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::InvalidSegment {
        max: 4,
      }
    )
  );
  assert!(!invalid_segment_root.exists());
  assert_eq!(
    host_store
      .load("resume")
      .expect("host artifact survives segments"),
    host_artifact
  );
  assert_eq!(
    store.load("resume").expect("checkpoint survives segments"),
    checkpoint
  );
  std::fs::write(root.join("broken.foi-batch-run"), "bad")
    .expect("malformed checkpoint fixture writes");
  let log_before_decode_error = operational_log.entries().to_vec();
  assert_eq!(
    store.load_with_operational_log("broken", &mut operational_log),
    Err(
      crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::Store(
        crate::agent_batch_store::ScriptedAgentBatchStoreError::InvalidCheckpoint {
          error: ScriptedAgentBatchCheckpointError::InvalidValue,
        },
      )
    )
  );
  assert_eq!(
    operational_log.entries(),
    log_before_decode_error.as_slice()
  );
  let invalid_root = root.join("not-a-directory");
  std::fs::write(&invalid_root, "file").expect("invalid storage root fixture writes");
  let invalid_store = crate::agent_batch_store::ScriptedAgentBatchRunStore::new(&invalid_root);
  let mut storage_error_log = ScriptedAgentOperationalLog::new();
  storage_error_log
    .append(ScriptedAgentOperationalEvent::BatchStarted)
    .expect("one event fits");
  let storage_error_before = storage_error_log.entries().to_vec();
  assert_eq!(
    invalid_store.save_with_operational_log("resume", checkpoint, &mut storage_error_log),
    Err(
      crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::Store(
        crate::agent_batch_store::ScriptedAgentBatchStoreError::StorageUnavailable,
      )
    )
  );
  assert_eq!(storage_error_log.entries(), storage_error_before.as_slice());
  assert_eq!(
    invalid_store.load_with_operational_log("resume", &mut storage_error_log),
    Err(
      crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::Store(
        crate::agent_batch_store::ScriptedAgentBatchStoreError::StorageUnavailable,
      )
    )
  );
  assert_eq!(storage_error_log.entries(), storage_error_before.as_slice());
  std::fs::write(root.join("broken.foi-operational-log"), "bad")
    .expect("malformed operational log fixture writes");
  assert_eq!(
    operational_store.load("broken"),
    Err(
      crate::agent_operational_store::ScriptedAgentOperationalLogStoreError::InvalidLog {
        error: ScriptedAgentOperationalLogCodecError::InvalidValue,
      }
    )
  );
  for _ in operational_log.len()..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
    operational_log
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("event log reaches its cap");
  }
  let log_before_capacity_error = operational_log.entries().to_vec();
  assert_eq!(
    store.load_with_operational_log("resume", &mut operational_log),
    Err(
      crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::LogCapacityExceeded {
        max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
      }
    )
  );
  assert_eq!(
    operational_log.entries(),
    log_before_capacity_error.as_slice()
  );
  let mut save_capacity_log = ScriptedAgentOperationalLog::new();
  for _ in 0..MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS {
    save_capacity_log
      .append(ScriptedAgentOperationalEvent::BatchStarted)
      .expect("save capacity fixture reaches its cap");
  }
  let save_capacity_before = save_capacity_log.entries().to_vec();
  assert_eq!(
    store.save_with_operational_log(
      "resume",
      checkpoint.with_completed_count(1),
      &mut save_capacity_log,
    ),
    Err(
      crate::agent_batch_store::ScriptedAgentBatchStoreOperationalError::LogCapacityExceeded {
        max: MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS,
      }
    )
  );
  assert_eq!(save_capacity_log.entries(), save_capacity_before.as_slice());
  assert_eq!(
    store.load("resume").expect("prior checkpoint remains"),
    checkpoint
  );
  let (first, advanced) = ScriptedAgentBatchRunner::run_next(observation, &manifests, loaded, 1)
    .expect("first chunk runs");
  assert_eq!(first.len(), 1);
  assert_eq!(advanced.completed_count(), 1);
  store
    .save("resume", advanced)
    .expect("advanced checkpoint saves");
  let (remaining, complete) = ScriptedAgentBatchRunner::run_next(
    observation,
    &manifests,
    store.load("resume").expect("advanced checkpoint loads"),
    16,
  )
  .expect("remaining chunk runs");
  let full = ScriptedAgentBatchRunner::run(observation, &manifests).expect("full batch runs");
  assert_eq!(remaining, full[1..]);
  assert!(complete.is_complete());
  assert_eq!(complete.completed_count(), 2);
  assert_eq!(
    ScriptedAgentBatchRunner::run_next(observation, &manifests, complete, 1,)
      .expect("completed run is idempotent")
      .0,
    Vec::<ScriptedAgentDecision>::new()
  );
  let mismatched_observation = observe_player(&state, ObservationId::new(46)).observation();
  assert_eq!(
    ScriptedAgentBatchRunner::run_next(mismatched_observation, &manifests, complete, 1),
    Err(ScriptedAgentBatchRunError::InputMismatch)
  );
  let reordered = [manifests[1], manifests[0]];
  assert_eq!(
    ScriptedAgentBatchRunner::run_next(observation, &reordered, complete, 1),
    Err(ScriptedAgentBatchRunError::InputMismatch)
  );
  let _ = std::fs::remove_dir_all(root);
}

#[test]
fn matched_observation_sample_is_stable_and_bounded() {
  let initial = LaneSnapshot::initial();
  let threat = LaneSnapshot::new(
    initial.ruleset(),
    initial.turn(),
    LaneStatus::Open,
    initial.player(),
    initial.opponent(),
    initial.wave(),
    JungleThreatTruth::RiverSide,
  );
  let observations = [
    observe_player(&initial, ObservationId::new(60)).observation(),
    observe_player(&threat, ObservationId::new(61)).observation(),
  ];
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(13, StreamId::new(14), DrawId::new(15)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(16, StreamId::new(17), DrawId::new(18)),
    ),
  ];
  let sample = ScriptedAgentMatchedSample::from_observations(observations, &manifests)
    .expect("matched sample builds");
  assert_eq!(
    SCRIPTED_AGENT_MATCHED_SAMPLE_SCHEMA,
    "m6-scripted-agent-matched-sample-v1"
  );
  assert_eq!(sample.schema(), SCRIPTED_AGENT_MATCHED_SAMPLE_SCHEMA);
  assert_eq!(sample.observer(), observations[0].observer());
  assert_eq!(
    sample.observation_ids(),
    &[ObservationId::new(60), ObservationId::new(61)]
  );
  assert_eq!(sample.entries().len(), 2);
  assert_eq!(sample.entries()[0].profile_id(), SCRIPTED_AGENT_PROFILE_ID);
  assert_eq!(
    sample.entries()[0].evaluation_rule(),
    "threat-first-pressure-aware-fixed-score-v1"
  );
  assert_eq!(
    sample.entries()[0].seed_bundle(),
    manifests[0].seed_bundle()
  );
  assert_eq!(
    sample.entries()[0].selected_intents(),
    [LaneIntent::Stabilize, LaneIntent::Withdraw]
  );
  assert_eq!(
    sample.entries()[1].profile_id(),
    YIELDING_SCRIPTED_AGENT_PROFILE_ID
  );
  assert_eq!(
    sample.entries()[1].evaluation_rule(),
    "yield-first-fixed-score-v1"
  );
  assert_eq!(
    sample.entries()[1].seed_bundle(),
    manifests[1].seed_bundle()
  );
  assert_eq!(
    sample.entries()[1].selected_intents(),
    [LaneIntent::Yield, LaneIntent::Yield]
  );
  assert_eq!(
    sample,
    ScriptedAgentMatchedSample::from_observations(observations, &manifests)
      .expect("matched sample repeats")
  );

  let mixed_observation = observations[1].with_observer(ALLIED_AUTONOMOUS_ACTOR);
  let mixed_actor = [observations[0], mixed_observation];
  assert_eq!(
    ScriptedAgentMatchedSample::from_observations(mixed_actor, &manifests),
    Err(ScriptedAgentMatchedSampleError::MismatchedObserver)
  );
  let duplicate_id = [
    observations[0],
    observe_player(&threat, ObservationId::new(60)).observation(),
  ];
  assert_eq!(
    ScriptedAgentMatchedSample::from_observations(duplicate_id, &manifests),
    Err(ScriptedAgentMatchedSampleError::DuplicateObservationId)
  );
  assert_eq!(
    ScriptedAgentMatchedSample::from_observations(observations, &[]),
    Err(ScriptedAgentMatchedSampleError::Batch(
      ScriptedAgentBatchError::EmptyBatch
    ))
  );
  let too_many = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1];
  assert_eq!(
    ScriptedAgentMatchedSample::from_observations(observations, &too_many),
    Err(ScriptedAgentMatchedSampleError::Batch(
      ScriptedAgentBatchError::BatchTooLarge {
        max: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS,
        actual: MAX_SCRIPTED_AGENT_BATCH_MANIFESTS + 1,
      }
    ))
  );
}

#[test]
fn matched_scenario_sample_set_preserves_order_and_bounds() {
  let initial = LaneSnapshot::initial();
  let threat = LaneSnapshot::new(
    initial.ruleset(),
    initial.turn(),
    LaneStatus::Open,
    initial.player(),
    initial.opponent(),
    initial.wave(),
    JungleThreatTruth::RiverSide,
  );
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(19, StreamId::new(20), DrawId::new(21)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(22, StreamId::new(23), DrawId::new(24)),
    ),
  ];
  let pairs = [
    [
      observe_player(&initial, ObservationId::new(70)).observation(),
      observe_player(&threat, ObservationId::new(71)).observation(),
    ],
    [
      observe_player(&initial, ObservationId::new(72)).observation(),
      observe_player(&threat, ObservationId::new(73)).observation(),
    ],
  ];
  let sample = ScriptedAgentMatchedScenarioSample::from_observations(&pairs, &manifests)
    .expect("matched scenario sample builds");
  assert_eq!(sample.schema(), "m6-scripted-agent-matched-scenarios-v1");
  assert_eq!(sample.observer(), pairs[0][0].observer());
  assert_eq!(sample.samples().len(), 2);
  assert_eq!(sample.samples()[0].entries().len(), 2);
  assert_eq!(
    sample.samples()[0].entries()[0].profile_id(),
    SCRIPTED_AGENT_PROFILE_ID
  );
  assert_eq!(
    sample.samples()[0].entries()[1].profile_id(),
    YIELDING_SCRIPTED_AGENT_PROFILE_ID
  );
  assert_eq!(
    sample.samples()[0].entries()[0].seed_bundle(),
    manifests[0].seed_bundle()
  );
  assert_eq!(
    sample.samples()[0].entries()[1].seed_bundle(),
    manifests[1].seed_bundle()
  );
  assert_eq!(
    sample.samples()[0].observation_ids(),
    &[ObservationId::new(70), ObservationId::new(71)]
  );
  assert_eq!(
    sample.samples()[1].observation_ids(),
    &[ObservationId::new(72), ObservationId::new(73)]
  );
  assert_eq!(
    sample,
    ScriptedAgentMatchedScenarioSample::from_observations(&pairs, &manifests)
      .expect("matched scenario sample repeats")
  );
  let tally = ScriptedAgentMatchedScenarioTallyReport::from_sample(&sample);
  assert_eq!(
    tally.schema(),
    "m6-scripted-agent-matched-scenario-tally-v1"
  );
  assert_eq!(tally.observer(), sample.observer());
  assert_eq!(tally.pair_count(), 2);
  assert_eq!(tally.observation_count(), 4);
  assert_eq!(tally.entries().len(), 2);
  assert_eq!(tally.entries()[0].profile_id(), SCRIPTED_AGENT_PROFILE_ID);
  assert_eq!(
    tally.entries()[0].evaluation_rule(),
    "threat-first-pressure-aware-fixed-score-v1"
  );
  assert_eq!(tally.entries()[0].stabilize_count(), 2);
  assert_eq!(tally.entries()[0].contest_count(), 0);
  assert_eq!(tally.entries()[0].withdraw_count(), 2);
  assert_eq!(
    tally.entries()[0].stabilize_count()
      + tally.entries()[0].contest_count()
      + tally.entries()[0].yield_count()
      + tally.entries()[0].recall_count()
      + tally.entries()[0].withdraw_count(),
    tally.entries()[0].observation_count()
  );
  assert_eq!(
    tally.entries()[1].profile_id(),
    YIELDING_SCRIPTED_AGENT_PROFILE_ID
  );
  assert_eq!(
    tally.entries()[1].evaluation_rule(),
    "yield-first-fixed-score-v1"
  );
  assert_eq!(tally.entries()[1].yield_count(), 4);
  assert_eq!(
    tally.entries()[1].stabilize_count()
      + tally.entries()[1].contest_count()
      + tally.entries()[1].yield_count()
      + tally.entries()[1].recall_count()
      + tally.entries()[1].withdraw_count(),
    tally.entries()[1].observation_count()
  );
  assert_eq!(
    tally,
    ScriptedAgentMatchedScenarioTallyReport::from_sample(&sample)
  );
  let encoded = tally.encode();
  assert_eq!(
    encoded,
    "schema=m6-scripted-agent-matched-scenario-tally-v1\nobserver=1\npair_count=2\nobservation_count=4\nentries=2\nrow=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2\nrow=yielding-laner-v1|yield-first-fixed-score-v1|0|0|4|0|0\n"
  );
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(&encoded, &tally),
    Ok(tally.clone())
  );
  for malformed in [
    (
      encoded.replacen(
        "schema=m6-scripted-agent-matched-scenario-tally-v1",
        "schema=other",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyCodecError::UnsupportedSchema,
    ),
    (
      encoded.replacen("entries=2", "unknown=2", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::UnknownField,
    ),
    (
      encoded.replacen("cautious-laner-v1", "unknown-profile", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen(
        "entries=2",
        "schema=m6-scripted-agent-matched-scenario-tally-v1",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyCodecError::DuplicateField,
    ),
    (
      encoded.replacen("entries=2\n", "", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::MissingField,
    ),
    (
      format!(
        "{encoded}row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2\n"
      ),
      ScriptedAgentMatchedScenarioTallyCodecError::UnexpectedLineCount {
        expected: 7,
        actual: 8,
      },
    ),
    (
      encoded.replacen(
        "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2",
        "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|oops|0|0|0|2",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen(
        "yielding-laner-v1|yield-first-fixed-score-v1",
        "yielding-laner-v1|contest-first-fixed-score-v1",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen(
        "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2",
        "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|1|0|0|0|2",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("pair_count=2", "pair_count=0", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("pair_count=2", "pair_count=5", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("observation_count=4", "observation_count=3", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("entries=2", "entries=0", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("entries=2", "entries=17", 1),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
    (
      encoded.replacen(
        "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0|0|2",
        "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|2|0|0",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyCodecError::InvalidValue,
    ),
  ] {
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyReport::decode(&malformed.0, &tally),
      Err(malformed.1)
    );
  }
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(
      &"x".repeat(MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_BYTES + 1),
      &tally,
    ),
    Err(ScriptedAgentMatchedScenarioTallyCodecError::Oversized)
  );
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(
      &encoded.replacen("observer=1", "observer=255", 1),
      &tally,
    ),
    Err(ScriptedAgentMatchedScenarioTallyCodecError::InputMismatch)
  );
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(
      &encoded.replacen(
        "row=yielding-laner-v1|yield-first-fixed-score-v1|0|0|4|0|0",
        "row=yielding-laner-v1|yield-first-fixed-score-v1|0|0|2|2|0",
        1,
      ),
      &tally,
    ),
    Err(ScriptedAgentMatchedScenarioTallyCodecError::InputMismatch)
  );

  let at_capacity = [
    [
      observe_player(&initial, ObservationId::new(80)).observation(),
      observe_player(&threat, ObservationId::new(81)).observation(),
    ],
    [
      observe_player(&initial, ObservationId::new(82)).observation(),
      observe_player(&threat, ObservationId::new(83)).observation(),
    ],
    [
      observe_player(&initial, ObservationId::new(84)).observation(),
      observe_player(&threat, ObservationId::new(85)).observation(),
    ],
    [
      observe_player(&initial, ObservationId::new(86)).observation(),
      observe_player(&threat, ObservationId::new(87)).observation(),
    ],
  ];
  let capacity_sample =
    ScriptedAgentMatchedScenarioSample::from_observations(&at_capacity, &manifests)
      .expect("inclusive sample cap runs");
  assert_eq!(
    capacity_sample.samples().len(),
    MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES
  );
  assert_eq!(
    capacity_sample.samples()[3].observation_ids(),
    &[ObservationId::new(86), ObservationId::new(87)]
  );
  let capacity_tally = ScriptedAgentMatchedScenarioTallyReport::from_sample(&capacity_sample);
  assert_eq!(capacity_tally.pair_count(), 4);
  assert_eq!(capacity_tally.observation_count(), 8);
  assert_eq!(capacity_tally.entries().len(), 2);
  assert_eq!(capacity_tally.entries()[0].stabilize_count(), 4);
  assert_eq!(capacity_tally.entries()[0].withdraw_count(), 4);
  assert_eq!(capacity_tally.entries()[1].yield_count(), 8);
  for entry in capacity_tally.entries() {
    assert_eq!(
      entry.stabilize_count()
        + entry.contest_count()
        + entry.yield_count()
        + entry.recall_count()
        + entry.withdraw_count(),
      8
    );
  }
  let capacity_encoded = capacity_tally.encode();
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(&capacity_encoded, &capacity_tally),
    Ok(capacity_tally.clone())
  );

  let max_manifest_batch = [manifests[0]; MAX_SCRIPTED_AGENT_BATCH_MANIFESTS];
  let max_entry_sample =
    ScriptedAgentMatchedScenarioSample::from_observations(&[pairs[0]], &max_manifest_batch)
      .expect("inclusive entry cap runs");
  let max_entry_tally = ScriptedAgentMatchedScenarioTallyReport::from_sample(&max_entry_sample);
  assert_eq!(max_entry_tally.pair_count(), 1);
  assert_eq!(max_entry_tally.observation_count(), 2);
  assert_eq!(
    max_entry_tally.entries().len(),
    MAX_SCRIPTED_AGENT_BATCH_MANIFESTS
  );
  let max_entry_encoded = max_entry_tally.encode();
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(&max_entry_encoded, &max_entry_tally),
    Ok(max_entry_tally)
  );

  assert_eq!(
    ScriptedAgentMatchedScenarioSample::from_observations(&[], &manifests),
    Err(ScriptedAgentMatchedScenarioSampleError::EmptySample)
  );
  let too_many = [pairs[0]; MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES + 1];
  assert_eq!(
    ScriptedAgentMatchedScenarioSample::from_observations(&too_many, &manifests),
    Err(ScriptedAgentMatchedScenarioSampleError::SampleTooLarge {
      max: MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES,
      actual: MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_SAMPLES + 1,
    })
  );
  let mut mixed = pairs;
  mixed[1][1] = mixed[1][1].with_observer(ALLIED_AUTONOMOUS_ACTOR);
  assert_eq!(
    ScriptedAgentMatchedScenarioSample::from_observations(&mixed, &manifests),
    Err(ScriptedAgentMatchedScenarioSampleError::MismatchedObserver)
  );
  let duplicate = [
    pairs[0],
    [
      pairs[1][0],
      observe_player(&threat, ObservationId::new(70)).observation(),
    ],
  ];
  assert_eq!(
    ScriptedAgentMatchedScenarioSample::from_observations(&duplicate, &manifests),
    Err(ScriptedAgentMatchedScenarioSampleError::DuplicateObservationId)
  );
}

#[test]
fn fixture_scenario_selection_is_closed_ordered_and_bounded() {
  let scenario_ids = [
    SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
  ];
  let observation_ids = [
    [ObservationId::new(100), ObservationId::new(101)],
    [ObservationId::new(102), ObservationId::new(103)],
    [ObservationId::new(104), ObservationId::new(105)],
    [ObservationId::new(106), ObservationId::new(107)],
  ];
  let selection = ScriptedAgentFixtureScenarioSelection::from_ids(&scenario_ids, &observation_ids)
    .expect("closed fixture selection builds");
  assert_eq!(
    [
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    ["safe-fixture-v1", "river-side-threat-v1"]
  );
  assert_eq!(
    SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA,
    "m6-scripted-agent-fixture-scenarios-v1"
  );
  assert_eq!(
    selection.schema(),
    SCRIPTED_AGENT_FIXTURE_SCENARIO_CATALOG_SCHEMA
  );
  assert_eq!(
    selection
      .scenarios()
      .iter()
      .map(|scenario| scenario.id())
      .collect::<Vec<_>>(),
    scenario_ids
  );
  assert_eq!(selection.observation_ids(), &observation_ids);
  assert_eq!(selection.observations(), selection.observations());
  let observations = selection.observations();
  assert_eq!(observations.len(), 4);
  assert_eq!(observations[0][0].observation_id(), ObservationId::new(100));
  assert_eq!(observations[1][1].observation_id(), ObservationId::new(103));
  assert_eq!(observations[0][1].available_threat_response(), None);
  assert_eq!(
    observations[1][1].available_threat_response(),
    Some(LaneIntent::Withdraw)
  );

  let manifests = [ScriptedAgentExperimentManifest::new(
    ScriptedAgentProfile::cautious_v1(),
    ScriptedAgentSeedBundle::new(31, StreamId::new(32), DrawId::new(33)),
  )];
  let sample = selection
    .matched_sample(&manifests)
    .expect("selected fixture samples compose");
  assert_eq!(sample.samples().len(), 4);
  assert_eq!(
    sample,
    ScriptedAgentFixtureScenarioSelection::from_ids(&scenario_ids, &observation_ids)
      .expect("selection repeats")
      .matched_sample(&manifests)
      .expect("repeated samples compose")
  );

  assert_eq!(MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS, 4);
  let population = ScriptedAgentFixtureScenarioPopulation::generate(4, 200)
    .expect("maximum fixed-fixture population builds");
  assert_eq!(
    SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA,
    "m6-scripted-agent-fixture-population-v1"
  );
  assert_eq!(
    population.schema(),
    SCRIPTED_AGENT_FIXTURE_POPULATION_SCHEMA
  );
  assert_eq!(population.scenarios(), selection.scenarios());
  assert_eq!(
    population.observation_ids(),
    &[
      [ObservationId::new(200), ObservationId::new(201)],
      [ObservationId::new(202), ObservationId::new(203)],
      [ObservationId::new(204), ObservationId::new(205)],
      [ObservationId::new(206), ObservationId::new(207)],
    ]
  );
  assert_eq!(
    population,
    ScriptedAgentFixtureScenarioPopulation::generate(4, 200).expect("repeated population builds")
  );
  assert_eq!(
    population.matched_sample(&manifests),
    population.selection().matched_sample(&manifests)
  );
  let boundary_population = ScriptedAgentFixtureScenarioPopulation::generate(4, u64::MAX - 7)
    .expect("maximum observation IDs fit the population");
  assert_eq!(
    boundary_population.observation_ids().last(),
    Some(&[
      ObservationId::new(u64::MAX - 1),
      ObservationId::new(u64::MAX),
    ])
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioPopulation::generate(0, 200),
    Err(ScriptedAgentFixturePopulationError::EmptyPopulation)
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioPopulation::generate(MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1, 200,),
    Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
      max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
      actual: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1,
    })
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioPopulation::generate(1, u64::MAX),
    Err(ScriptedAgentFixturePopulationError::ObservationIdOverflow)
  );

  assert_eq!(
    ScriptedAgentFixtureScenarioSelection::from_ids(&[], &[]),
    Err(ScriptedAgentFixtureScenarioSelectionError::EmptySelection)
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioSelection::from_ids(
      &["unknown-fixture-v1"],
      &[[ObservationId::new(108), ObservationId::new(109)]],
    ),
    Err(ScriptedAgentFixtureScenarioSelectionError::UnknownScenario)
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioSelection::from_ids(
      &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
      &[],
    ),
    Err(
      ScriptedAgentFixtureScenarioSelectionError::MismatchedObservationPairCount {
        expected: 1,
        actual: 0,
      }
    )
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioSelection::from_ids(
      &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
      &[[ObservationId::new(110), ObservationId::new(110)]],
    ),
    Err(ScriptedAgentFixtureScenarioSelectionError::DuplicateObservationId)
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioSelection::from_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      &[
        [ObservationId::new(112), ObservationId::new(113)],
        [ObservationId::new(114), ObservationId::new(112)],
      ],
    ),
    Err(ScriptedAgentFixtureScenarioSelectionError::DuplicateObservationId)
  );
  let too_many_scenarios =
    [SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID; MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1];
  let too_many_ids = (0..=MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS)
    .map(|index| {
      let offset = u64::try_from(index).expect("fixture index fits in u64") * 2;
      [
        ObservationId::new(120 + offset),
        ObservationId::new(121 + offset),
      ]
    })
    .collect::<Vec<_>>();
  assert_eq!(
    ScriptedAgentFixtureScenarioSelection::from_ids(&too_many_scenarios, &too_many_ids,),
    Err(
      ScriptedAgentFixtureScenarioSelectionError::SelectionTooLarge {
        max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
        actual: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1,
      }
    )
  );
}

#[test]
fn caller_declared_population_composition_preserves_order_and_frequency() {
  let population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    220,
  )
  .expect("caller-declared composition builds");
  assert_eq!(
    population.scenarios(),
    &[
      ScriptedAgentFixtureScenario::Safe,
      ScriptedAgentFixtureScenario::Safe,
      ScriptedAgentFixtureScenario::Safe,
      ScriptedAgentFixtureScenario::RiverSideThreat,
    ]
  );
  assert_eq!(
    population.observation_ids(),
    &[
      [ObservationId::new(220), ObservationId::new(221)],
      [ObservationId::new(222), ObservationId::new(223)],
      [ObservationId::new(224), ObservationId::new(225)],
      [ObservationId::new(226), ObservationId::new(227)],
    ]
  );
  let frequency =
    ScriptedAgentFixtureScenarioFrequencyReport::from_selection(population.selection());
  assert_eq!(frequency.entries()[0].count(), 3);
  assert_eq!(frequency.entries()[1].count(), 1);
  let manifests = [ScriptedAgentExperimentManifest::new(
    ScriptedAgentProfile::cautious_v1(),
    ScriptedAgentSeedBundle::new(41, StreamId::new(42), DrawId::new(43)),
  )];
  assert_eq!(
    population.matched_sample(&manifests),
    population.selection().matched_sample(&manifests)
  );
  let tally = population
    .matched_tally(&manifests)
    .expect("caller-declared population tallies");
  assert_eq!(tally.pair_count(), 4);
  assert_eq!(tally.observation_count(), 8);
  assert_eq!(tally.entries().len(), 1);
  assert_eq!(tally.entries()[0].stabilize_count(), 7);
  assert_eq!(tally.entries()[0].withdraw_count(), 1);
  assert_eq!(
    ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(&[], u64::MAX),
    Err(ScriptedAgentFixturePopulationError::EmptyPopulation)
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &["unknown-fixture-v1"; MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1],
      u64::MAX,
    ),
    Err(ScriptedAgentFixturePopulationError::PopulationTooLarge {
      max: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS,
      actual: MAX_SCRIPTED_AGENT_FIXTURE_SCENARIOS + 1,
    })
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &["unknown-fixture-v1"],
      u64::MAX,
    ),
    Err(ScriptedAgentFixturePopulationError::InvalidSelection(
      ScriptedAgentFixtureScenarioSelectionError::UnknownScenario,
    ))
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
      u64::MAX,
    ),
    Err(ScriptedAgentFixturePopulationError::ObservationIdOverflow)
  );
}

#[test]
fn profile_aware_population_tally_preserves_rows_and_counts() {
  let population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    230,
  )
  .expect("profile-aware population builds");
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(51, StreamId::new(52), DrawId::new(53)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::risk_taking_v1(),
      ScriptedAgentSeedBundle::new(54, StreamId::new(55), DrawId::new(56)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(57, StreamId::new(58), DrawId::new(59)),
    ),
  ];
  let tally = population
    .matched_tally(&manifests)
    .expect("profile-aware population tallies");
  assert_eq!(tally.pair_count(), 4);
  assert_eq!(tally.observation_count(), 8);
  assert_eq!(
    tally
      .entries()
      .iter()
      .map(|entry| entry.profile_id())
      .collect::<Vec<_>>(),
    vec![
      SCRIPTED_AGENT_PROFILE_ID,
      RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
      YIELDING_SCRIPTED_AGENT_PROFILE_ID,
    ]
  );
  assert_eq!(tally.entries()[0].stabilize_count(), 7);
  assert_eq!(tally.entries()[0].withdraw_count(), 1);
  assert_eq!(tally.entries()[1].contest_count(), 8);
  assert_eq!(tally.entries()[2].yield_count(), 8);
  assert_eq!(
    tally.entries()[0].intent_distribution_basis_points(),
    [8_750, 0, 0, 0, 1_250]
  );
  assert_eq!(
    tally.entries()[1].intent_distribution_basis_points(),
    [0, 10_000, 0, 0, 0]
  );
  assert_eq!(
    tally.entries()[2].intent_distribution_basis_points(),
    [0, 0, 10_000, 0, 0]
  );
  assert_eq!(
    tally
      .entries()
      .iter()
      .map(|entry| entry.intent_distribution_basis_points().iter().sum::<u16>())
      .collect::<Vec<_>>(),
    vec![10_000, 10_000, 10_000]
  );
  assert_eq!(
    tally.to_intent_distribution_markdown(),
    "# Profile Intent Distribution\n\n- schema: m6-scripted-agent-matched-scenario-tally-v1\n- observer: 1\n\n| profile_id | evaluation_rule | observation_count | stabilize_bp | contest_bp | yield_bp | recall_bp | withdraw_bp |\n| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |\n| cautious-laner-v1 | threat-first-pressure-aware-fixed-score-v1 | 8 | 8750 | 0 | 0 | 0 | 1250 |\n| risk-taking-laner-v1 | contest-first-fixed-score-v1 | 8 | 0 | 10000 | 0 | 0 | 0 |\n| yielding-laner-v1 | yield-first-fixed-score-v1 | 8 | 0 | 0 | 10000 | 0 | 0 |\n"
  );
  let remainder_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    238,
  )
  .expect("remainder population builds");
  let remainder_manifest = [ScriptedAgentExperimentManifest::new(
    ScriptedAgentProfile::cautious_v1(),
    ScriptedAgentSeedBundle::new(60, StreamId::new(61), DrawId::new(62)),
  )];
  let remainder_tally = remainder_population
    .matched_tally(&remainder_manifest)
    .expect("remainder tally builds");
  assert_eq!(remainder_tally.observation_count(), 6);
  assert_eq!(
    remainder_tally.entries()[0].intent_distribution_basis_points(),
    [8_333, 0, 0, 0, 1_667]
  );
  assert_eq!(
    remainder_tally.entries()[0]
      .intent_distribution_basis_points()
      .iter()
      .sum::<u16>(),
    SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE
  );
  assert_eq!(
    tally
      .entries()
      .iter()
      .map(|entry| {
        u16::from(entry.stabilize_count())
          + u16::from(entry.contest_count())
          + u16::from(entry.yield_count())
          + u16::from(entry.recall_count())
          + u16::from(entry.withdraw_count())
      })
      .collect::<Vec<_>>(),
    vec![8, 8, 8]
  );
}

#[test]
fn scripted_agent_stress_population_catalog_is_closed_and_reproducible() {
  let state = LaneSnapshot::initial();
  let first_receipt = observe_player(&state, ObservationId::new(410));
  let first_observation = first_receipt.observation();
  let host = CliScenarioHost::fixture();
  let host_observation = host.observation();
  let illegal_error = host
    .validate_actor_action(ActorActionDto::new(
      host_observation.observer().value(),
      host_observation.observation_id().value(),
      ActorProtocolIntent::Withdraw,
    ))
    .expect_err("illegal actor command is rejected by host validation");
  assert_eq!(illegal_error.code().id(), "host_validation_rejected");
  let stale_error = host
    .validate_actor_action(ActorActionDto::new(
      host_observation.observer().value(),
      host_observation.observation_id().value() + 1,
      ActorProtocolIntent::Stabilize,
    ))
    .expect_err("stale actor command is rejected by host freshness");
  assert_eq!(stale_error.code().id(), "stale_observation");

  assert_eq!(
    ActorMessageDto::new(
      first_observation.observer().value(),
      ALLIED_AUTONOMOUS_ACTOR.value(),
      first_observation.observation_id().value(),
      &"x".repeat(MAX_ACTOR_DRAFT_VALUE_BYTES + 1),
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );

  let second_receipt = observe_player(&state, ObservationId::new(411));
  let choices = [
    ScriptedAgent::cautious_v1().choose(first_observation),
    ScriptedAgent::cautious_v1().choose(second_receipt.observation()),
  ];
  let degenerate_stabilize_count = u8::try_from(
    choices
      .iter()
      .filter(|choice| choice.selected_intent() == LaneIntent::Stabilize)
      .count(),
  )
  .expect("bounded degenerate count fits in u8");
  assert_eq!(degenerate_stabilize_count, 2);

  let results = [
    ScriptedAgentStressResult::HostValidationRejected,
    ScriptedAgentStressResult::StaleObservation,
    ScriptedAgentStressResult::MessageInvalidValue,
    ScriptedAgentStressResult::RepeatedStabilize,
  ];
  let report =
    ScriptedAgentStressPopulationReport::from_results(results, degenerate_stabilize_count)
      .expect("stress report binds expected results");
  assert_eq!(
    SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA,
    "m6-scripted-agent-stress-population-v1"
  );
  assert_eq!(report.schema(), SCRIPTED_AGENT_STRESS_POPULATION_SCHEMA);
  assert_eq!(report.degenerate_stabilize_count(), 2);
  assert_eq!(
    report
      .entries()
      .iter()
      .map(|entry| (entry.case().id(), entry.result().id()))
      .collect::<Vec<_>>(),
    vec![
      ("illegal-command-v1", "host_validation_rejected"),
      ("exploit-seeking-v1", "stale_observation"),
      ("communication-abuse-v1", "message_invalid_value"),
      ("degenerate-policy-v1", "repeated_stabilize"),
    ]
  );
  assert_eq!(
    report.to_markdown(),
    "# Scripted Agent Stress Population\n\n- schema: m6-scripted-agent-stress-population-v1\n- degenerate_stabilize_count: 2\n\n| case_id | result_id |\n| --- | --- |\n| illegal-command-v1 | host_validation_rejected |\n| exploit-seeking-v1 | stale_observation |\n| communication-abuse-v1 | message_invalid_value |\n| degenerate-policy-v1 | repeated_stabilize |\n"
  );
  assert_eq!(
    ScriptedAgentStressPopulationReport::from_results(results, 2),
    Ok(report.clone())
  );
  assert_eq!(
    ScriptedAgentStressPopulationReport::from_results(
      [
        ScriptedAgentStressResult::RepeatedStabilize,
        ScriptedAgentStressResult::StaleObservation,
        ScriptedAgentStressResult::MessageInvalidValue,
        ScriptedAgentStressResult::RepeatedStabilize,
      ],
      2,
    ),
    Err(ScriptedAgentStressPopulationError::UnexpectedResult)
  );
  assert_eq!(
    ScriptedAgentStressPopulationReport::from_results(results, 0),
    Err(ScriptedAgentStressPopulationError::InvalidDegenerateCount)
  );
  assert!(ScriptedAgentStressPopulationReport::from_results(results, 4).is_ok());
  assert_eq!(
    ScriptedAgentStressPopulationReport::from_results(results, 5),
    Err(ScriptedAgentStressPopulationError::InvalidDegenerateCount)
  );
}

#[test]
fn degenerate_policy_population_is_bounded_and_actor_visible() {
  let state = LaneSnapshot::initial();
  let observations = (0..MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION)
    .map(|offset| {
      observe_player(
        &state,
        ObservationId::new(700 + u64::try_from(offset).expect("offset fits")),
      )
      .observation()
    })
    .collect::<Vec<_>>();
  let report = ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&observations)
    .expect("fixed cautious observations repeat Stabilize");
  assert_eq!(
    SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA,
    "m6-scripted-agent-degenerate-policy-population-v1"
  );
  assert_eq!(
    report.schema(),
    SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION_SCHEMA
  );
  assert_eq!(report.profile_id(), "cautious-laner-v1");
  assert_eq!(
    report.evaluation_rule(),
    "threat-first-pressure-aware-fixed-score-v1"
  );
  assert_eq!(report.observer(), observations[0].observer());
  assert_eq!(report.observation_count(), 4);
  assert_eq!(report.selected_intent(), LaneIntent::Stabilize);
  let singleton =
    ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&observations[..1])
      .expect("inclusive one-member population fits");
  assert_eq!(singleton.observation_count(), 1);
  assert_eq!(singleton.selected_intent(), LaneIntent::Stabilize);
  assert_eq!(
    ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&observations),
    Ok(report)
  );
  assert_eq!(
    ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&[]),
    Err(ScriptedAgentDegeneratePolicyPopulationError::EmptyPopulation)
  );
  assert_eq!(
    ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&[
      observations[0],
      observations[0],
    ]),
    Err(ScriptedAgentDegeneratePolicyPopulationError::DuplicateObservationId)
  );
  let river_observation = ScriptedAgentFixtureScenario::RiverSideThreat
    .observations([ObservationId::new(900), ObservationId::new(901)])[1];
  assert_eq!(
    ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&[river_observation]),
    Err(ScriptedAgentDegeneratePolicyPopulationError::UnexpectedIntent)
  );
  let too_many = (0..=MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION)
    .map(|offset| {
      observe_player(
        &state,
        ObservationId::new(800 + u64::try_from(offset).expect("offset fits")),
      )
      .observation()
    })
    .collect::<Vec<_>>();
  assert_eq!(
    ScriptedAgentDegeneratePolicyPopulationReport::from_observations(&too_many),
    Err(
      ScriptedAgentDegeneratePolicyPopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION,
        actual: MAX_SCRIPTED_AGENT_DEGENERATE_POLICY_POPULATION + 1,
      }
    )
  );
}

#[test]
fn exploit_seeking_population_is_bounded_and_fixed_fixture_only() {
  let state = LaneSnapshot::initial();
  let observations = (0..MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION)
    .map(|offset| {
      observe_player(
        &state,
        ObservationId::new(1_000 + u64::try_from(offset).expect("offset fits")),
      )
      .observation()
    })
    .collect::<Vec<_>>();
  let report = ScriptedAgentExploitSeekingPopulationReport::from_observations(&observations)
    .expect("risk-taking policy selects Contest in the safe fixture");
  assert_eq!(
    SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA,
    "m6-scripted-agent-exploit-seeking-population-v1"
  );
  assert_eq!(
    report.schema(),
    SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION_SCHEMA
  );
  assert_eq!(report.profile_id(), "risk-taking-laner-v1");
  assert_eq!(report.evaluation_rule(), "contest-first-fixed-score-v1");
  assert_eq!(report.observer(), observations[0].observer());
  assert_eq!(report.observation_count(), 4);
  assert_eq!(report.selected_intent(), LaneIntent::Contest);
  let singleton =
    ScriptedAgentExploitSeekingPopulationReport::from_observations(&observations[..1])
      .expect("inclusive one-member population fits");
  assert_eq!(singleton.observation_count(), 1);
  assert_eq!(singleton.selected_intent(), LaneIntent::Contest);
  assert_eq!(
    ScriptedAgentExploitSeekingPopulationReport::from_observations(&observations),
    Ok(report)
  );
  assert_eq!(
    ScriptedAgentExploitSeekingPopulationReport::from_observations(&[]),
    Err(ScriptedAgentExploitSeekingPopulationError::EmptyPopulation)
  );
  assert_eq!(
    ScriptedAgentExploitSeekingPopulationReport::from_observations(&[
      observations[0],
      observations[0],
    ]),
    Err(ScriptedAgentExploitSeekingPopulationError::DuplicateObservationId)
  );
  let allied_observation = observations[0].with_observer(ALLIED_AUTONOMOUS_ACTOR);
  assert_eq!(
    ScriptedAgentExploitSeekingPopulationReport::from_observations(&[
      observations[0],
      allied_observation,
    ]),
    Err(ScriptedAgentExploitSeekingPopulationError::MismatchedObserver)
  );
  let too_many = (0..=MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION)
    .map(|offset| {
      observe_player(
        &state,
        ObservationId::new(1_100 + u64::try_from(offset).expect("offset fits")),
      )
      .observation()
    })
    .collect::<Vec<_>>();
  assert_eq!(
    ScriptedAgentExploitSeekingPopulationReport::from_observations(&too_many),
    Err(
      ScriptedAgentExploitSeekingPopulationError::PopulationTooLarge {
        max: MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION,
        actual: MAX_SCRIPTED_AGENT_EXPLOIT_SEEKING_POPULATION + 1,
      }
    )
  );
}

#[test]
fn profile_aware_population_tally_codec_round_trips_verified_rows() {
  let population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    240,
  )
  .expect("codec population builds");
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(61, StreamId::new(62), DrawId::new(63)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::risk_taking_v1(),
      ScriptedAgentSeedBundle::new(64, StreamId::new(65), DrawId::new(66)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(67, StreamId::new(68), DrawId::new(69)),
    ),
  ];
  let tally = population
    .matched_tally(&manifests)
    .expect("codec tally builds");
  let encoded = tally.encode();
  assert!(encoded.starts_with("schema=m6-scripted-agent-matched-scenario-tally-v1\n"));
  assert!(encoded.contains("entries=3\n"));
  assert!(
    encoded
      .contains("row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1\n")
  );
  assert!(encoded.contains("row=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0\n"));
  assert!(encoded.contains("row=yielding-laner-v1|yield-first-fixed-score-v1|0|0|8|0|0\n"));
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(&encoded, &tally),
    Ok(tally.clone())
  );
  let tampered = encoded.replace("|7|0|0|0|1\n", "|6|0|0|0|2\n");
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyReport::decode(&tampered, &tally),
    Err(ScriptedAgentMatchedScenarioTallyCodecError::InputMismatch)
  );
}

#[test]
fn profile_aware_tally_comparison_preserves_rows_and_signed_deltas() {
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(71, StreamId::new(72), DrawId::new(73)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::risk_taking_v1(),
      ScriptedAgentSeedBundle::new(74, StreamId::new(75), DrawId::new(76)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::yielding_v1(),
      ScriptedAgentSeedBundle::new(77, StreamId::new(78), DrawId::new(79)),
    ),
  ];
  let baseline_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    280,
  )
  .expect("baseline population builds");
  let candidate_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    288,
  )
  .expect("candidate population builds");
  let baseline = baseline_population
    .matched_tally(&manifests)
    .expect("baseline tally builds");
  let candidate = candidate_population
    .matched_tally(&manifests)
    .expect("candidate tally builds");
  let comparison =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
      .expect("matching verified tallies compare");
  assert_eq!(
    SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_SCHEMA,
    "m6-scripted-agent-matched-scenario-tally-compare-v1"
  );
  assert_eq!(
    comparison.schema(),
    "m6-scripted-agent-matched-scenario-tally-compare-v1"
  );
  let encoded = comparison.encode();
  assert_eq!(
    encoded,
    "schema=m6-scripted-agent-matched-scenario-tally-compare-v1\nobserver=1\nbaseline_pair_count=4\nbaseline_observation_count=8\ncandidate_pair_count=4\ncandidate_observation_count=8\nentries=3\nrow=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1|5|0|0|0|3\nrow=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0|0|8|0|0|0\nrow=yielding-laner-v1|yield-first-fixed-score-v1|0|0|8|0|0|0|0|8|0|0\n"
  );
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyComparisonReport::decode(&encoded, &comparison),
    Ok(comparison.clone())
  );
  for (malformed, error) in [
    (
      encoded.replacen(
        "schema=m6-scripted-agent-matched-scenario-tally-compare-v1",
        "schema=wrong-v1",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnsupportedSchema,
    ),
    (
      encoded.replacen("entries=3", "unknown=3", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnknownField,
    ),
    (
      encoded.replacen("observer=1", "schema=wrong-v1", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::DuplicateField,
    ),
    (
      encoded.replacen(
        "observer=1\nbaseline_pair_count=4",
        "baseline_pair_count=4\nobserver=1",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::DuplicateField,
    ),
    (
      encoded.replacen("cautious-laner-v1", "unknown-profile-v1", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
    ),
    (
      encoded.replacen(
        "threat-first-pressure-aware-fixed-score-v1",
        "contest-first-fixed-score-v1",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
    ),
    (
      encoded.replacen("baseline_pair_count=4", "baseline_pair_count=x", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
    ),
    (
      encoded.replacen("|7|0|0|0|1|5|0|0|0|3\n", "|x|0|0|0|1|5|0|0|0|3\n", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
    ),
    (
      encoded.replacen("entries=3", "entries=0", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
    ),
    (
      encoded.replacen("entries=3", "entries=17", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
    ),
    (
      encoded.replacen(
        "row=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1|5|0|0|0|3\nrow=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0|0|8|0|0|0\n",
        "row=risk-taking-laner-v1|contest-first-fixed-score-v1|0|8|0|0|0|0|8|0|0|0\nrow=cautious-laner-v1|threat-first-pressure-aware-fixed-score-v1|7|0|0|0|1|5|0|0|0|3\n",
        1,
      ),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InputMismatch,
    ),
    (
      encoded.lines().take(6).collect::<Vec<_>>().join("\n"),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::MissingField,
    ),
    (
      encoded.replacen("|7|0|0|0|1|5|0|0|0|3\n", "|6|0|0|0|1|5|0|0|0|3\n", 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::InvalidValue,
    ),
    (
      format!("{encoded}extra=x\n"),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::UnexpectedLineCount {
        expected: 10,
        actual: 11,
      },
    ),
    (
      "x".repeat(MAX_SCRIPTED_AGENT_MATCHED_SCENARIO_TALLY_COMPARISON_BYTES + 1),
      ScriptedAgentMatchedScenarioTallyComparisonCodecError::Oversized,
    ),
  ] {
    assert_eq!(
      ScriptedAgentMatchedScenarioTallyComparisonReport::decode(&malformed, &comparison),
      Err(error)
    );
  }
  let tampered = encoded.replacen("|7|0|0|0|1|5|0|0|0|3\n", "|6|0|0|0|2|5|0|0|0|3\n", 1);
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyComparisonReport::decode(&tampered, &comparison),
    Err(ScriptedAgentMatchedScenarioTallyComparisonCodecError::InputMismatch)
  );
  assert_eq!(comparison.observer(), baseline.observer());
  assert_eq!(comparison.baseline_pair_count(), 4);
  assert_eq!(comparison.baseline_observation_count(), 8);
  assert_eq!(comparison.candidate_pair_count(), 4);
  assert_eq!(comparison.candidate_observation_count(), 8);
  assert_eq!(comparison.entries().len(), 3);
  assert_eq!(comparison.entries()[0].profile_id(), "cautious-laner-v1");
  assert_eq!(
    comparison.entries()[0].evaluation_rule(),
    "threat-first-pressure-aware-fixed-score-v1"
  );
  assert_eq!(comparison.entries()[0].baseline_counts(), [7, 0, 0, 0, 1]);
  assert_eq!(comparison.entries()[0].candidate_counts(), [5, 0, 0, 0, 3]);
  assert_eq!(comparison.entries()[0].deltas(), [-2, 0, 0, 0, 2]);
  assert_eq!(comparison.entries()[1].profile_id(), "risk-taking-laner-v1");
  assert_eq!(
    comparison.entries()[1].evaluation_rule(),
    "contest-first-fixed-score-v1"
  );
  assert_eq!(comparison.entries()[1].baseline_counts(), [0, 8, 0, 0, 0]);
  assert_eq!(comparison.entries()[1].candidate_counts(), [0, 8, 0, 0, 0]);
  assert_eq!(comparison.entries()[1].deltas(), [0, 0, 0, 0, 0]);
  assert_eq!(comparison.entries()[2].profile_id(), "yielding-laner-v1");
  assert_eq!(
    comparison.entries()[2].evaluation_rule(),
    "yield-first-fixed-score-v1"
  );
  assert_eq!(comparison.entries()[2].baseline_counts(), [0, 0, 8, 0, 0]);
  assert_eq!(comparison.entries()[2].candidate_counts(), [0, 0, 8, 0, 0]);
  assert_eq!(comparison.entries()[2].deltas(), [0, 0, 0, 0, 0]);
  assert_eq!(
    comparison.regression_rule(),
    "m6-fixed-profile-tally-no-change-v1"
  );
  assert!(!comparison.passes_no_change_gate());
  let unchanged =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
      .expect("unchanged verified tallies compare");
  assert!(unchanged.passes_no_change_gate());
  assert_eq!(
    comparison,
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
      .expect("repeated comparison is stable")
  );
  let reversed =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&candidate, &baseline)
      .expect("reversed verified tallies compare");
  assert_eq!(reversed.entries()[0].deltas(), [2, 0, 0, 0, -2]);

  let smaller_candidate_population =
    ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      296,
    )
    .expect("smaller candidate population builds");
  let smaller_candidate = smaller_candidate_population
    .matched_tally(&manifests)
    .expect("smaller candidate tally builds");
  let changed_total =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &smaller_candidate)
      .expect("changed-total verified tallies compare");
  assert_eq!(changed_total.baseline_pair_count(), 4);
  assert_eq!(changed_total.candidate_pair_count(), 3);
  assert_eq!(changed_total.baseline_observation_count(), 8);
  assert_eq!(changed_total.candidate_observation_count(), 6);
  assert!(!changed_total.passes_no_change_gate());

  let redistributed_population =
    ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
      &[
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
        SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      ],
      302,
    )
    .expect("redistributed population builds");
  let redistributed = redistributed_population
    .matched_tally(&manifests)
    .expect("redistributed tally builds");
  let same_total_redistribution =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &redistributed)
      .expect("same-total verified tallies compare");
  assert_eq!(same_total_redistribution.baseline_pair_count(), 4);
  assert_eq!(same_total_redistribution.candidate_pair_count(), 4);
  assert_eq!(same_total_redistribution.baseline_observation_count(), 8);
  assert_eq!(same_total_redistribution.candidate_observation_count(), 8);
  assert_eq!(
    same_total_redistribution.entries()[0].baseline_counts(),
    [7, 0, 0, 0, 1]
  );
  assert_eq!(
    same_total_redistribution.entries()[0].candidate_counts(),
    [6, 0, 0, 0, 2]
  );
  assert!(!same_total_redistribution.passes_no_change_gate());

  let reordered_candidate = candidate_population
    .matched_tally(&[manifests[1], manifests[0], manifests[2]])
    .expect("reordered candidate tally builds");
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(
      &baseline,
      &reordered_candidate,
    ),
    Err(ScriptedAgentMatchedScenarioTallyComparisonError::MismatchedRows)
  );

  let mut alternate_observations = candidate_population.observations();
  for pair in &mut alternate_observations {
    pair[0] = pair[0].with_observer(ALLIED_AUTONOMOUS_ACTOR);
    pair[1] = pair[1].with_observer(ALLIED_AUTONOMOUS_ACTOR);
  }
  let alternate_sample =
    ScriptedAgentMatchedScenarioSample::from_observations(&alternate_observations, &manifests)
      .expect("alternate observer sample builds");
  let alternate = ScriptedAgentMatchedScenarioTallyReport::from_sample(&alternate_sample);
  assert_eq!(
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &alternate),
    Err(ScriptedAgentMatchedScenarioTallyComparisonError::MismatchedObserver)
  );
}

#[test]
fn profile_aware_tally_largest_delta_candidate_is_stable_and_bounded() {
  let manifests = [
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(81, StreamId::new(82), DrawId::new(83)),
    ),
    ScriptedAgentExperimentManifest::new(
      ScriptedAgentProfile::cautious_v1(),
      ScriptedAgentSeedBundle::new(84, StreamId::new(85), DrawId::new(86)),
    ),
  ];
  let baseline_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    340,
  )
  .expect("baseline candidate builds");
  let candidate_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    ],
    348,
  )
  .expect("candidate population builds");
  let baseline = baseline_population
    .matched_tally(&manifests)
    .expect("baseline tally builds");
  let candidate = candidate_population
    .matched_tally(&manifests)
    .expect("candidate tally builds");
  let comparison =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
      .expect("verified tallies compare");
  let selected = comparison
    .largest_delta_candidate()
    .expect("changed comparison has a candidate");
  assert_eq!(
    selected.schema(),
    "m6-scripted-agent-tally-outlier-candidate-v1"
  );
  assert_eq!(
    selected.selection_rule(),
    "m6-largest-absolute-intent-delta-v1"
  );
  assert_eq!(selected.row_index(), 0);
  assert_eq!(selected.profile_id(), "cautious-laner-v1");
  assert_eq!(
    selected.evaluation_rule(),
    "threat-first-pressure-aware-fixed-score-v1"
  );
  assert_eq!(selected.intent(), LaneIntent::Stabilize);
  assert_eq!(selected.delta(), -2);
  assert_eq!(selected.magnitude(), 2);
  assert_eq!(
    selected.magnitude(),
    selected.delta().unsigned_abs(),
    "magnitude retains the bounded absolute signed delta"
  );
  assert_eq!(
    comparison.largest_delta_candidate(),
    Some(selected),
    "repeated ranking is deterministic"
  );

  let reversed =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&candidate, &baseline)
      .expect("reversed verified tallies compare");
  let reversed_selected = reversed
    .largest_delta_candidate()
    .expect("reversed changed comparison has a candidate");
  assert_eq!(reversed_selected.intent(), LaneIntent::Stabilize);
  assert_eq!(reversed_selected.delta(), 2);
  assert_eq!(reversed_selected.magnitude(), 2);

  let unchanged =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
      .expect("unchanged verified tallies compare");
  assert_eq!(unchanged.largest_delta_candidate(), None);
}

#[test]
fn profile_aware_tally_outlier_threshold_is_provisional_and_bounded() {
  let manifest = [ScriptedAgentExperimentManifest::new(
    ScriptedAgentProfile::cautious_v1(),
    ScriptedAgentSeedBundle::new(91, StreamId::new(92), DrawId::new(93)),
  )];
  let baseline_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    500,
  )
  .expect("baseline population builds");
  let candidate_population = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    506,
  )
  .expect("candidate population builds");
  let baseline = baseline_population
    .matched_tally(&manifest)
    .expect("baseline tally builds");
  let candidate = candidate_population
    .matched_tally(&manifest)
    .expect("candidate tally builds");
  let below =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
      .expect("verified tallies compare");
  assert_eq!(
    below
      .largest_delta_candidate()
      .expect("magnitude-one candidate exists")
      .magnitude(),
    1
  );
  let below_report = ScriptedAgentTallyOutlierThresholdReport::from_comparison(&below);
  assert_eq!(
    below_report.schema(),
    "m6-scripted-agent-tally-outlier-threshold-v1"
  );
  assert_eq!(
    below_report.rule(),
    "m6-fixed-intent-delta-outlier-threshold-v1"
  );
  assert_eq!(below_report.threshold(), 2);
  assert_eq!(
    below_report.status(),
    ScriptedAgentTallyOutlierThresholdStatus::BelowThreshold
  );
  assert_eq!(below_report.status().id(), "below_threshold");

  let above = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    ],
    512,
  )
  .expect("above-threshold population builds")
  .matched_tally(&manifest)
  .expect("above-threshold tally builds");
  let baseline_four = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    520,
  )
  .expect("four-pair baseline population builds")
  .matched_tally(&manifest)
  .expect("four-pair baseline tally builds");
  let above_comparison =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline_four, &above)
      .expect("above-threshold tallies compare");
  assert_eq!(
    above_comparison
      .largest_delta_candidate()
      .expect("magnitude-two candidate exists")
      .magnitude(),
    SCRIPTED_AGENT_TALLY_OUTLIER_THRESHOLD_MAGNITUDE
  );
  let above_report = ScriptedAgentTallyOutlierThresholdReport::from_comparison(&above_comparison);
  assert_eq!(
    above_report.status(),
    ScriptedAgentTallyOutlierThresholdStatus::AboveThreshold
  );
  assert_eq!(above_report.status().id(), "above_threshold");

  let unchanged =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
      .expect("unchanged tallies compare");
  let unchanged_report = ScriptedAgentTallyOutlierThresholdReport::from_comparison(&unchanged);
  assert_eq!(
    unchanged_report.status(),
    ScriptedAgentTallyOutlierThresholdStatus::NoCandidate
  );
  assert_eq!(unchanged_report.status().id(), "no_candidate");
}

#[test]
fn tally_candidate_replay_reference_selects_first_verified_match() {
  let manifest = [ScriptedAgentExperimentManifest::new(
    ScriptedAgentProfile::cautious_v1(),
    ScriptedAgentSeedBundle::new(101, StreamId::new(102), DrawId::new(103)),
  )];
  let baseline = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    540,
  )
  .expect("baseline population builds")
  .matched_tally(&manifest)
  .expect("baseline tally builds");
  let candidate = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    ],
    548,
  )
  .expect("candidate population builds")
  .matched_tally(&manifest)
  .expect("candidate tally builds");
  let comparison =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate)
      .expect("verified tallies compare");
  let candidate = comparison
    .largest_delta_candidate()
    .expect("largest candidate exists");
  let state = LaneSnapshot::initial();
  let first_observation = observe_player(&state, ObservationId::new(600)).observation();
  let selected_observation = observe_player(&state, ObservationId::new(601)).observation();
  let later_observation = observe_player(&state, ObservationId::new(602)).observation();
  let noise = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::risk_taking_v1(),
    first_observation,
    LaneIntent::Contest,
    None,
  );
  let first_match = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    selected_observation,
    LaneIntent::Stabilize,
    None,
  );
  let later_match = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    later_observation,
    LaneIntent::Stabilize,
    None,
  );
  let reference = ScriptedAgentTallyReplayReference::from_candidate_and_records(
    candidate,
    &[noise, first_match.clone(), later_match],
  )
  .expect("first verified matching replay is selected");
  assert_eq!(
    reference.schema(),
    "m6-scripted-agent-tally-replay-reference-v1"
  );
  assert_eq!(
    reference.selection_rule(),
    "m6-first-verified-candidate-replay-v1"
  );
  assert_eq!(reference.row_index(), candidate.row_index());
  assert_eq!(reference.profile_id(), candidate.profile_id());
  assert_eq!(reference.evaluation_rule(), candidate.evaluation_rule());
  assert_eq!(reference.intent(), candidate.intent());
  assert_eq!(reference.delta(), candidate.delta());
  assert_eq!(reference.magnitude(), candidate.magnitude());
  assert_eq!(reference.observation_id(), ObservationId::new(601));

  let mut mismatched = first_match.clone();
  mismatched
    .decision
    .candidates
    .iter_mut()
    .find(|candidate| candidate.intent() == LaneIntent::Stabilize)
    .expect("selected candidate exists")
    .score += 1;
  let later_reference = ScriptedAgentTallyReplayReference::from_candidate_and_records(
    candidate,
    &[mismatched.clone(), first_match.clone()],
  )
  .expect("later verified matching replay is selected");
  assert_eq!(later_reference.observation_id(), ObservationId::new(601));
  assert_eq!(
    ScriptedAgentTallyReplayReference::from_candidate_and_records(candidate, &[mismatched]),
    Err(ScriptedAgentTallyReplayReferenceError::DecisionMismatch)
  );
  let no_match = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::yielding_v1(),
    first_observation,
    LaneIntent::Yield,
    None,
  );
  assert_eq!(
    ScriptedAgentTallyReplayReference::from_candidate_and_records(candidate, &[no_match]),
    Err(ScriptedAgentTallyReplayReferenceError::NoMatchingReplay)
  );
}

#[test]
fn calibrated_outlier_detection_and_representative_replay_is_deterministic() {
  assert_eq!(
    SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA,
    "m6-scripted-agent-calibrated-outlier-replay-v1"
  );
  assert_eq!(
    SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE,
    "m6-calibrated-outlier-representative-replay-v1"
  );
  assert_eq!(SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE, 2);
  assert_eq!(
    ScriptedAgentCalibratedOutlierReplayStatus::Qualified.id(),
    "qualified"
  );
  assert_eq!(
    ScriptedAgentCalibratedOutlierReplayStatus::BelowThreshold.id(),
    "below_threshold"
  );
  assert_eq!(
    ScriptedAgentCalibratedOutlierReplayStatus::NoCandidate.id(),
    "no_candidate"
  );
  assert_eq!(
    ScriptedAgentCalibratedOutlierReplayStatus::NoMatchingReplay.id(),
    "no_matching_replay"
  );
  assert_eq!(
    ScriptedAgentCalibratedOutlierReplayStatus::DecisionMismatch.id(),
    "decision_mismatch"
  );

  let manifest = [ScriptedAgentExperimentManifest::new(
    ScriptedAgentProfile::cautious_v1(),
    ScriptedAgentSeedBundle::new(101, StreamId::new(102), DrawId::new(103)),
  )];

  // 1. Qualified Outlier: delta >= 2 with matching verified replay record
  let baseline = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    540,
  )
  .expect("baseline population builds")
  .matched_tally(&manifest)
  .expect("baseline tally builds");
  let candidate_tally = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    ],
    548,
  )
  .expect("candidate population builds")
  .matched_tally(&manifest)
  .expect("candidate tally builds");
  let comparison =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &candidate_tally)
      .expect("verified tallies compare");
  let candidate = comparison
    .largest_delta_candidate()
    .expect("largest candidate exists");
  assert!(candidate.magnitude() >= SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE);

  let state = LaneSnapshot::initial();
  let noise_observation = observe_player(&state, ObservationId::new(700)).observation();
  let match_observation = observe_player(&state, ObservationId::new(701)).observation();
  let noise_record = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::risk_taking_v1(),
    noise_observation,
    LaneIntent::Contest,
    None,
  );
  let match_record = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    match_observation,
    LaneIntent::Stabilize,
    None,
  );

  let qualified_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
    &comparison,
    &[noise_record.clone(), match_record.clone()],
  );
  assert_eq!(
    qualified_report.schema(),
    SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA
  );
  assert_eq!(
    qualified_report.rule(),
    SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE
  );
  assert_eq!(
    qualified_report.threshold(),
    SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE
  );
  assert_eq!(
    qualified_report.status(),
    ScriptedAgentCalibratedOutlierReplayStatus::Qualified
  );
  assert_eq!(qualified_report.candidate(), Some(candidate));
  assert_eq!(
    qualified_report.observation_id(),
    Some(ObservationId::new(701))
  );

  // 2. Below Threshold: delta = 1
  let below_candidate = ScriptedAgentFixtureScenarioPopulation::generate_from_scenario_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    548,
  )
  .expect("below candidate population builds")
  .matched_tally(&manifest)
  .expect("below candidate tally builds");
  let below_comparison =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &below_candidate)
      .expect("below tallies compare");
  let below_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
    &below_comparison,
    std::slice::from_ref(&match_record),
  );
  assert_eq!(
    below_report.status(),
    ScriptedAgentCalibratedOutlierReplayStatus::BelowThreshold
  );
  assert!(below_report.candidate().is_some());
  assert_eq!(below_report.candidate().unwrap().magnitude(), 1);
  assert_eq!(below_report.observation_id(), None);

  // 3. No Candidate: unchanged baseline vs baseline
  let unchanged_comparison =
    ScriptedAgentMatchedScenarioTallyComparisonReport::from_reports(&baseline, &baseline)
      .expect("unchanged tallies compare");
  let no_cand_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
    &unchanged_comparison,
    std::slice::from_ref(&match_record),
  );
  assert_eq!(
    no_cand_report.status(),
    ScriptedAgentCalibratedOutlierReplayStatus::NoCandidate
  );
  assert_eq!(no_cand_report.candidate(), None);
  assert_eq!(no_cand_report.observation_id(), None);

  // 4. No Matching Replay: delta >= 2 but no matching record
  let no_matching_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
    &comparison,
    std::slice::from_ref(&noise_record),
  );
  assert_eq!(
    no_matching_report.status(),
    ScriptedAgentCalibratedOutlierReplayStatus::NoMatchingReplay
  );
  assert_eq!(no_matching_report.candidate(), Some(candidate));
  assert_eq!(no_matching_report.observation_id(), None);

  // 5. Decision Mismatch: delta >= 2 with corrupted replay record
  let mut corrupted = match_record;
  corrupted
    .decision
    .candidates
    .iter_mut()
    .find(|c| c.intent() == LaneIntent::Stabilize)
    .expect("candidate exists")
    .score += 1;
  let mismatch_report = ScriptedAgentCalibratedOutlierReplayReport::from_comparison_and_records(
    &comparison,
    std::slice::from_ref(&corrupted),
  );
  assert_eq!(
    mismatch_report.status(),
    ScriptedAgentCalibratedOutlierReplayStatus::DecisionMismatch
  );
  assert_eq!(mismatch_report.candidate(), Some(candidate));
  assert_eq!(
    mismatch_report.observation_id(),
    Some(ObservationId::new(701))
  );
}

#[test]
fn fixture_scenario_frequency_report_counts_ordered_selection() {
  let selection = ScriptedAgentFixtureScenarioSelection::from_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    ],
    &[
      [ObservationId::new(130), ObservationId::new(131)],
      [ObservationId::new(132), ObservationId::new(133)],
      [ObservationId::new(134), ObservationId::new(135)],
      [ObservationId::new(136), ObservationId::new(137)],
    ],
  )
  .expect("frequency selection builds");
  let report = ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&selection);
  assert_eq!(
    SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA,
    "m6-scripted-agent-fixture-frequency-v1"
  );
  assert_eq!(
    report.schema(),
    SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_SCHEMA
  );
  assert_eq!(report.selection_count(), 4);
  assert_eq!(report.entries()[0].scenario_id(), "safe-fixture-v1");
  assert_eq!(report.entries()[0].count(), 2);
  assert_eq!(report.entries()[1].scenario_id(), "river-side-threat-v1");
  assert_eq!(report.entries()[1].count(), 2);
  assert_eq!(
    report.entries()[0].count() + report.entries()[1].count(),
    report.selection_count()
  );
  assert_eq!(SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE, 10_000);
  assert_eq!(report.distribution_basis_points(), [5_000, 5_000]);
  assert_eq!(
    report.distribution_basis_points().iter().sum::<u16>(),
    10_000
  );
  assert_eq!(
    report,
    ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&selection)
  );
  let encoded = report.encode();
  assert_eq!(
    encoded,
    "schema=m6-scripted-agent-fixture-frequency-v1\nselection_count=4\nentries=2\nrow=safe-fixture-v1|2\nrow=river-side-threat-v1|2\n"
  );
  assert_eq!(
    report.to_markdown(),
    "# Scenario Frequency\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n\n| scenario_id | count |\n| --- | ---: |\n| safe-fixture-v1 | 2 |\n| river-side-threat-v1 | 2 |\n"
  );
  assert_eq!(
    report.to_distribution_markdown(),
    "# Scenario Distribution\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n- share_scale_basis_points: 10000\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| safe-fixture-v1 | 2 | 5000 |\n| river-side-threat-v1 | 2 | 5000 |\n"
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioFrequencyReport::decode(&encoded, &report),
    Ok(report.clone())
  );

  let singleton = ScriptedAgentFixtureScenarioSelection::from_ids(
    &[SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID],
    &[[ObservationId::new(140), ObservationId::new(141)]],
  )
  .expect("singleton selection builds");
  let singleton_report = ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&singleton);
  assert_eq!(singleton_report.selection_count(), 1);
  assert_eq!(
    singleton_report.entries()[0].scenario_id(),
    "safe-fixture-v1"
  );
  assert_eq!(singleton_report.entries()[0].count(), 1);
  assert_eq!(
    singleton_report.entries()[1].scenario_id(),
    "river-side-threat-v1"
  );
  assert_eq!(singleton_report.entries()[1].count(), 0);
  assert_eq!(
    singleton_report.entries()[0].count() + singleton_report.entries()[1].count(),
    singleton_report.selection_count()
  );
  assert_eq!(singleton_report.distribution_basis_points(), [10_000, 0]);
  let singleton_encoded = singleton_report.encode();
  assert_eq!(
    singleton_report.to_markdown(),
    "# Scenario Frequency\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 1\n\n| scenario_id | count |\n| --- | ---: |\n| safe-fixture-v1 | 1 |\n| river-side-threat-v1 | 0 |\n"
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioFrequencyReport::decode(&singleton_encoded, &singleton_report,),
    Ok(singleton_report.clone())
  );

  let skewed_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    &[
      [ObservationId::new(142), ObservationId::new(143)],
      [ObservationId::new(144), ObservationId::new(145)],
      [ObservationId::new(146), ObservationId::new(147)],
      [ObservationId::new(148), ObservationId::new(149)],
    ],
  )
  .expect("skewed selection builds");
  let skewed_report =
    ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&skewed_selection);
  assert_eq!(skewed_report.selection_count(), 4);
  assert_eq!(
    skewed_report
      .entries()
      .iter()
      .map(|entry| entry.count())
      .collect::<Vec<_>>(),
    vec![1, 3]
  );
  assert_eq!(skewed_report.distribution_basis_points(), [2_500, 7_500]);
  assert_eq!(
    skewed_report
      .distribution_basis_points()
      .iter()
      .sum::<u16>(),
    SCRIPTED_AGENT_SCENARIO_DISTRIBUTION_SCALE
  );
  assert_eq!(
    skewed_report.to_distribution_markdown(),
    "# Scenario Distribution\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n- share_scale_basis_points: 10000\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| safe-fixture-v1 | 1 | 2500 |\n| river-side-threat-v1 | 3 | 7500 |\n"
  );

  let all_safe_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    ],
    &[
      [ObservationId::new(150), ObservationId::new(151)],
      [ObservationId::new(152), ObservationId::new(153)],
      [ObservationId::new(154), ObservationId::new(155)],
      [ObservationId::new(156), ObservationId::new(157)],
    ],
  )
  .expect("all-safe selection builds");
  let all_safe_report =
    ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&all_safe_selection);
  assert_eq!(all_safe_report.distribution_basis_points(), [10_000, 0]);
  assert_eq!(
    all_safe_report.to_distribution_markdown(),
    "# Scenario Distribution\n\n- schema: m6-scripted-agent-fixture-frequency-v1\n- selection_count: 4\n- share_scale_basis_points: 10000\n\n| scenario_id | count | share_basis_points |\n| --- | ---: | ---: |\n| safe-fixture-v1 | 4 | 10000 |\n| river-side-threat-v1 | 0 | 0 |\n"
  );
  for malformed in [
    (
      encoded.replacen(
        "schema=m6-scripted-agent-fixture-frequency-v1",
        "schema=other",
        1,
      ),
      ScriptedAgentFixtureScenarioFrequencyCodecError::UnsupportedSchema,
    ),
    (
      encoded.replacen("entries=2", "unknown=2", 1),
      ScriptedAgentFixtureScenarioFrequencyCodecError::UnknownField,
    ),
    (
      encoded.replacen(
        "entries=2",
        "schema=m6-scripted-agent-fixture-frequency-v1",
        1,
      ),
      ScriptedAgentFixtureScenarioFrequencyCodecError::DuplicateField,
    ),
    (
      encoded.replacen("entries=2\n", "", 1),
      ScriptedAgentFixtureScenarioFrequencyCodecError::MissingField,
    ),
    (
      format!("{encoded}row=safe-fixture-v1|2\n"),
      ScriptedAgentFixtureScenarioFrequencyCodecError::UnexpectedLineCount {
        expected: 5,
        actual: 6,
      },
    ),
    (
      encoded.replacen("row=safe-fixture-v1|2", "row=unknown-fixture-v1|2", 1),
      ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("row=safe-fixture-v1|2", "row=safe-fixture-v1|oops", 1),
      ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("row=safe-fixture-v1|2", "row=safe-fixture-v1|1", 1),
      ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("row=safe-fixture-v1|2", "row=safe-fixture-v1|255", 1),
      ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
    ),
    (
      encoded.replacen("entries=2", "entries=3", 1),
      ScriptedAgentFixtureScenarioFrequencyCodecError::InvalidValue,
    ),
  ] {
    assert_eq!(
      ScriptedAgentFixtureScenarioFrequencyReport::decode(&malformed.0, &report),
      Err(malformed.1)
    );
  }
  assert_eq!(
    ScriptedAgentFixtureScenarioFrequencyReport::decode(
      &encoded.replacen(
        "row=safe-fixture-v1|2\nrow=river-side-threat-v1|2",
        "row=safe-fixture-v1|1\nrow=river-side-threat-v1|3",
        1,
      ),
      &report,
    ),
    Err(ScriptedAgentFixtureScenarioFrequencyCodecError::InputMismatch)
  );
  assert_eq!(
    ScriptedAgentFixtureScenarioFrequencyReport::decode(
      &"x".repeat(MAX_SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_BYTES + 1),
      &report,
    ),
    Err(ScriptedAgentFixtureScenarioFrequencyCodecError::Oversized)
  );
}

#[test]
fn fixture_frequency_report_comparison_preserves_declared_order_and_deltas() {
  let baseline_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    &[
      [ObservationId::new(150), ObservationId::new(151)],
      [ObservationId::new(152), ObservationId::new(153)],
    ],
  )
  .expect("baseline selection builds");
  let candidate_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_RIVER_SIDE_FIXTURE_SCENARIO_ID,
    ],
    &[
      [ObservationId::new(154), ObservationId::new(155)],
      [ObservationId::new(156), ObservationId::new(157)],
      [ObservationId::new(158), ObservationId::new(159)],
      [ObservationId::new(160), ObservationId::new(161)],
    ],
  )
  .expect("candidate selection builds");
  let baseline = ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&baseline_selection);
  let candidate = ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&candidate_selection);
  let comparison =
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&baseline, &candidate);
  assert_eq!(
    SCRIPTED_AGENT_FIXTURE_SCENARIO_FREQUENCY_COMPARISON_SCHEMA,
    "m6-scripted-agent-fixture-frequency-compare-v1"
  );
  assert_eq!(
    comparison.schema(),
    "m6-scripted-agent-fixture-frequency-compare-v1"
  );
  assert_eq!(comparison.baseline_build_id(), None);
  assert_eq!(comparison.candidate_build_id(), None);
  assert_eq!(comparison.baseline_selection_count(), 2);
  assert_eq!(comparison.candidate_selection_count(), 4);
  assert_eq!(comparison.entries()[0].scenario_id(), "safe-fixture-v1");
  assert_eq!(comparison.entries()[0].baseline_count(), 1);
  assert_eq!(comparison.entries()[0].candidate_count(), 2);
  assert_eq!(comparison.entries()[0].delta(), 1);
  assert_eq!(
    comparison.entries()[1].scenario_id(),
    "river-side-threat-v1"
  );
  assert_eq!(comparison.entries()[1].baseline_count(), 1);
  assert_eq!(comparison.entries()[1].candidate_count(), 2);
  assert_eq!(comparison.entries()[1].delta(), 1);
  assert_eq!(
    comparison.regression_rule(),
    "m6-fixed-frequency-no-change-v1"
  );
  assert!(!comparison.passes_no_change_gate());
  assert_eq!(
    comparison,
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&baseline, &candidate)
  );
  let reversed =
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&candidate, &baseline);
  assert_eq!(reversed.entries()[0].delta(), -1);
  assert_eq!(reversed.entries()[1].delta(), -1);
  let unchanged =
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&baseline, &baseline);
  assert!(unchanged.passes_no_change_gate());
  let baseline_build = ScriptedAgentBuildId::new(140);
  let candidate_build = ScriptedAgentBuildId::new(141);
  assert_eq!(baseline_build.schema(), "m6-scripted-agent-build-id-v1");
  assert_eq!(baseline_build.value(), 140);
  let labeled = ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
    &baseline,
    &candidate,
    baseline_build,
    candidate_build,
  )
  .expect("distinct build labels compare");
  assert_eq!(labeled.baseline_build_id(), Some(baseline_build));
  assert_eq!(labeled.candidate_build_id(), Some(candidate_build));
  assert_eq!(labeled.entries(), comparison.entries());
  assert_eq!(
    labeled.baseline_selection_count(),
    comparison.baseline_selection_count()
  );
  assert_eq!(
    labeled.candidate_selection_count(),
    comparison.candidate_selection_count()
  );
  assert_eq!(
    labeled.passes_no_change_gate(),
    comparison.passes_no_change_gate()
  );
  assert_eq!(
    labeled,
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
      &baseline,
      &candidate,
      baseline_build,
      candidate_build,
    )
    .expect("repeated labeled comparison is stable")
  );
  let labeled_unchanged =
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
      &baseline,
      &baseline,
      baseline_build,
      candidate_build,
    )
    .expect("distinct labels retain unchanged comparison");
  assert_eq!(labeled_unchanged.baseline_selection_count(), 2);
  assert_eq!(labeled_unchanged.candidate_selection_count(), 2);
  assert!(labeled_unchanged.passes_no_change_gate());
  assert_eq!(
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports_with_build_ids(
      &baseline,
      &candidate,
      baseline_build,
      baseline_build,
    ),
    Err(ScriptedAgentBuildComparisonError::MatchingBuildIds)
  );
  let redistributed_selection = ScriptedAgentFixtureScenarioSelection::from_ids(
    &[
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
      SCRIPTED_AGENT_SAFE_FIXTURE_SCENARIO_ID,
    ],
    &[
      [ObservationId::new(162), ObservationId::new(163)],
      [ObservationId::new(164), ObservationId::new(165)],
    ],
  )
  .expect("redistributed selection builds");
  let redistributed =
    ScriptedAgentFixtureScenarioFrequencyReport::from_selection(&redistributed_selection);
  let same_total_redistribution =
    ScriptedAgentFixtureScenarioFrequencyComparisonReport::from_reports(&baseline, &redistributed);
  assert_eq!(same_total_redistribution.baseline_selection_count(), 2);
  assert_eq!(same_total_redistribution.candidate_selection_count(), 2);
  assert_eq!(same_total_redistribution.entries()[0].candidate_count(), 2);
  assert_eq!(same_total_redistribution.entries()[1].candidate_count(), 0);
  assert!(!same_total_redistribution.passes_no_change_gate());
}

#[test]
fn evaluation_rejects_intents_outside_the_actor_visible_candidate_set() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(13)).observation();

  assert_eq!(
    ScriptedAgent::cautious_v1().evaluate_candidate(observation, LaneIntent::Withdraw),
    Err(ScriptedAgentEvaluationError::UnavailableIntent)
  );
}

#[test]
fn cautious_agent_prioritizes_visible_threat_response_without_hidden_state() {
  let initial = LaneSnapshot::initial();
  let state = LaneSnapshot::new(
    initial.ruleset(),
    initial.turn(),
    LaneStatus::Open,
    initial.player(),
    initial.opponent(),
    initial.wave(),
    JungleThreatTruth::RiverSide,
  );
  let observation = observe_player(&state, ObservationId::new(10)).observation();
  let decision = ScriptedAgent::cautious_v1().choose(observation);

  assert_eq!(decision.selected_intent(), LaneIntent::Withdraw);
  assert!(decision.candidates().iter().any(|candidate| {
    candidate.intent() == LaneIntent::Withdraw
      && candidate.reason() == ScriptedAgentReason::ThreatResponse
      && candidate.score() == 100
  }));
  assert_eq!(
    ScriptedAgent::cautious_v1()
      .evaluate_candidate(observation, LaneIntent::Withdraw)
      .expect("visible threat response evaluates"),
    ScriptedAgentCandidate {
      intent: LaneIntent::Withdraw,
      score: 100,
      reason: ScriptedAgentReason::ThreatResponse,
    }
  );
}

#[test]
fn cautious_agent_decision_is_reproducible_for_identical_observation() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(11)).observation();
  let agent = ScriptedAgent::cautious_v1();

  assert_eq!(agent.choose(observation), agent.choose(observation));
}

#[test]
fn seeded_decision_records_bundle_and_repeats_for_identical_inputs() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(19)).observation();
  let seed = ScriptedAgentSeedBundle::new(42, StreamId::new(21), DrawId::new(3));
  let decision = ScriptedAgent::cautious_v1().choose_with_seed(observation, seed);

  assert_eq!(seed.schema(), "m4-scripted-agent-random-v1");
  assert_eq!(seed.seed(), 42);
  assert_eq!(seed.policy_trace().stream().value(), 21);
  assert_eq!(seed.policy_trace().draw().value(), 3);
  assert_eq!(decision.seed_bundle(), Some(seed));
  assert_eq!(decision.selection_rule(), "max-score-seeded-tie-v1");
  assert_eq!(decision.selected_intent(), LaneIntent::Stabilize);
  assert_eq!(
    decision,
    ScriptedAgent::cautious_v1().choose_with_seed(observation, seed)
  );
  validate_lane_request(
    &state,
    &observe_player(&state, ObservationId::new(19)),
    &decision.request(),
  )
  .expect("seeded policy request is legal");
}

#[test]
fn seeded_tie_selection_is_reproducible_and_stream_scoped() {
  let candidates = [
    ScriptedAgentCandidate {
      intent: LaneIntent::Contest,
      score: 70,
      reason: ScriptedAgentReason::AvailableAlternative,
    },
    ScriptedAgentCandidate {
      intent: LaneIntent::Stabilize,
      score: 70,
      reason: ScriptedAgentReason::StableDefault,
    },
  ];
  let first_seed = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(3));
  let same_seed = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(3));
  let next_draw = ScriptedAgentSeedBundle::new(1, StreamId::new(21), DrawId::new(4));

  let first = ScriptedAgent::select_candidate_with_seed(&candidates, first_seed);
  assert_eq!(
    first,
    ScriptedAgent::select_candidate_with_seed(&candidates, same_seed)
  );
  assert_ne!(
    first,
    ScriptedAgent::select_candidate_with_seed(&candidates, next_draw)
  );
}

#[test]
fn decision_replay_classifies_expected_and_declared_anomalous_cases() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(20)).observation();
  let expected = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    observation,
    LaneIntent::Stabilize,
    None,
  );
  let anomalous = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    observation,
    LaneIntent::Contest,
    None,
  );
  let seeded = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    observation,
    LaneIntent::Stabilize,
    Some(ScriptedAgentSeedBundle::new(
      42,
      StreamId::new(21),
      DrawId::new(3),
    )),
  );

  assert_eq!(expected.schema(), "m4-scripted-agent-replay-v1");
  assert_eq!(expected.profile().profile_id(), SCRIPTED_AGENT_PROFILE_ID);
  assert_eq!(
    expected.disposition(),
    ScriptedAgentReplayDisposition::Expected
  );
  assert_eq!(expected.expected_intent(), LaneIntent::Stabilize);
  assert_eq!(expected.selected_intent(), LaneIntent::Stabilize);
  assert_eq!(expected.replay(), Ok(expected.decision().clone()));
  assert_eq!(
    anomalous.disposition(),
    ScriptedAgentReplayDisposition::Anomalous
  );
  assert_eq!(anomalous.expected_intent(), LaneIntent::Contest);
  assert_eq!(anomalous.selected_intent(), LaneIntent::Stabilize);
  assert_eq!(anomalous.replay(), Ok(anomalous.decision().clone()));
  assert_eq!(
    seeded.disposition(),
    ScriptedAgentReplayDisposition::Expected
  );
  assert!(seeded.seed_bundle().is_some());
  assert_eq!(seeded.replay(), Ok(seeded.decision().clone()));
}

#[test]
fn decision_replay_rejects_tampered_recorded_decision() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(22)).observation();
  let mut record = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    observation,
    LaneIntent::Stabilize,
    None,
  );
  record.decision.selected_intent = LaneIntent::Contest;

  assert_eq!(
    record.replay(),
    Err(ScriptedAgentReplayError::DecisionMismatch)
  );

  let mut seeded_record = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    observation,
    LaneIntent::Stabilize,
    Some(ScriptedAgentSeedBundle::new(
      42,
      StreamId::new(21),
      DrawId::new(3),
    )),
  );
  seeded_record.decision.seed_bundle = Some(ScriptedAgentSeedBundle::new(
    99,
    StreamId::new(22),
    DrawId::new(4),
  ));

  assert_eq!(
    seeded_record.replay(),
    Err(ScriptedAgentReplayError::DecisionMismatch)
  );
}

#[test]
fn replay_sequence_evidence_binds_decision_identity_and_log_status() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(23)).observation();
  let expected = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    observation,
    LaneIntent::Stabilize,
    None,
  );
  let mut complete = ScriptedAgentOperationalLog::new();
  for event in [
    ScriptedAgentOperationalEvent::BatchStarted,
    ScriptedAgentOperationalEvent::ChunkCompleted,
    ScriptedAgentOperationalEvent::BatchFinished,
  ] {
    complete.append(event).expect("sequence fixture fits");
  }
  let evidence =
    ScriptedAgentReplaySequenceEvidenceReport::from_record_and_log(&expected, &complete);
  assert_eq!(
    evidence.schema(),
    "m6-scripted-agent-replay-sequence-evidence-v1"
  );
  assert_eq!(
    evidence.rule(),
    "m6-replay-identity-operational-sequence-v1"
  );
  assert_eq!(
    evidence.replay_identity(),
    ScriptedAgentReplayIdentityStatus::Verified
  );
  assert_eq!(evidence.replay_identity().id(), "verified");
  assert_eq!(
    evidence.sequence_status(),
    ScriptedAgentOperationalLogSequenceStatus::Complete
  );

  let mut incomplete = ScriptedAgentOperationalLog::new();
  incomplete
    .append(ScriptedAgentOperationalEvent::BatchStarted)
    .expect("sequence fixture fits");
  assert_eq!(
    ScriptedAgentReplaySequenceEvidenceReport::from_record_and_log(&expected, &incomplete,)
      .sequence_status(),
    ScriptedAgentOperationalLogSequenceStatus::MissingChunk
  );

  let mut tampered = expected.clone();
  tampered.decision.selected_intent = LaneIntent::Contest;
  let mismatch =
    ScriptedAgentReplaySequenceEvidenceReport::from_record_and_log(&tampered, &complete);
  assert_eq!(
    mismatch.replay_identity(),
    ScriptedAgentReplayIdentityStatus::DecisionMismatch
  );
  assert_eq!(mismatch.replay_identity().id(), "decision_mismatch");
  assert_eq!(
    mismatch.sequence_status(),
    ScriptedAgentOperationalLogSequenceStatus::Complete
  );
}

#[test]
fn scenario_replay_identity_verifies_sequence_and_rejects_malformed_input() {
  let state = LaneSnapshot::initial();
  let obs1 = observe_player(&state, ObservationId::new(101)).observation();
  let obs2 = observe_player(&state, ObservationId::new(102)).observation();
  let obs3 = observe_player(&state, ObservationId::new(103)).observation();

  let rec1 = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    obs1,
    LaneIntent::Stabilize,
    None,
  );
  let rec2 = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::risk_taking_v1(),
    obs2,
    LaneIntent::Contest,
    None,
  );
  let rec3 =
    ScriptedAgentReplayRecord::capture(ScriptedAgent::yielding_v1(), obs3, LaneIntent::Yield, None);

  let report = ScriptedAgentScenarioReplayIdentityReport::from_records(&[
    rec1.clone(),
    rec2.clone(),
    rec3.clone(),
  ])
  .expect("valid sequence verifies");

  assert_eq!(
    report.schema(),
    "m6-scripted-agent-scenario-replay-identity-v1"
  );
  assert_eq!(report.rule(), "m6-scenario-replay-identity-v1");
  assert_eq!(report.record_count(), 3);
  assert_eq!(report.verified_count(), 3);
  assert_eq!(
    report.status(),
    ScriptedAgentScenarioReplayIdentityStatus::AllVerified
  );
  assert_eq!(report.status().id(), "all_verified");
  assert_eq!(report.start_observation_id(), ObservationId::new(101));
  assert_eq!(report.end_observation_id(), ObservationId::new(103));

  // Decision mismatch in one record
  let mut tampered = rec2.clone();
  tampered.decision.selected_intent = LaneIntent::Stabilize;
  let mismatch_report = ScriptedAgentScenarioReplayIdentityReport::from_records(&[
    rec1.clone(),
    tampered,
    rec3.clone(),
  ])
  .expect("evaluates with mismatch");
  assert_eq!(mismatch_report.record_count(), 3);
  assert_eq!(mismatch_report.verified_count(), 2);
  assert_eq!(
    mismatch_report.status(),
    ScriptedAgentScenarioReplayIdentityStatus::DecisionMismatch
  );
  assert_eq!(mismatch_report.status().id(), "decision_mismatch");

  // Empty input fails closed
  assert_eq!(
    ScriptedAgentScenarioReplayIdentityReport::from_records(&[]),
    Err(ScriptedAgentScenarioReplayIdentityError::Empty)
  );

  // Duplicate observation ID fails closed
  let duplicate_obs = observe_player(&state, ObservationId::new(101)).observation();
  let duplicate_rec = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::yielding_v1(),
    duplicate_obs,
    LaneIntent::Yield,
    None,
  );
  assert_eq!(
    ScriptedAgentScenarioReplayIdentityReport::from_records(&[rec1.clone(), duplicate_rec]),
    Err(ScriptedAgentScenarioReplayIdentityError::DuplicateObservationId)
  );

  // Oversized input fails closed
  let mut oversized = Vec::new();
  for i in 0..=MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS {
    let obs_id = u64::try_from(i.saturating_add(200)).expect("fits in u64");
    let obs = observe_player(&state, ObservationId::new(obs_id)).observation();
    oversized.push(ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      obs,
      LaneIntent::Stabilize,
      None,
    ));
  }
  assert_eq!(
    ScriptedAgentScenarioReplayIdentityReport::from_records(&oversized),
    Err(ScriptedAgentScenarioReplayIdentityError::Oversized)
  );
}

#[test]
fn scenario_causal_trace_completeness_verifies_sequence_and_rejects_malformed_input() {
  let state = LaneSnapshot::initial();
  let obs1 = observe_player(&state, ObservationId::new(201)).observation();
  let obs2 = observe_player(&state, ObservationId::new(202)).observation();
  let obs3 = observe_player(&state, ObservationId::new(203)).observation();

  let rec1 = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::cautious_v1(),
    obs1,
    LaneIntent::Stabilize,
    None,
  );
  let rec2 = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::risk_taking_v1(),
    obs2,
    LaneIntent::Contest,
    None,
  );
  let rec3 =
    ScriptedAgentReplayRecord::capture(ScriptedAgent::yielding_v1(), obs3, LaneIntent::Yield, None);

  let report = ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[
    rec1.clone(),
    rec2.clone(),
    rec3.clone(),
  ])
  .expect("valid sequence verifies causal completeness");

  assert_eq!(
    report.schema(),
    "m6-scripted-agent-scenario-causal-trace-completeness-v1"
  );
  assert_eq!(report.rule(), "m6-scenario-causal-trace-completeness-v1");
  assert_eq!(report.record_count(), 3);
  assert_eq!(report.traced_count(), 3);
  assert_eq!(
    report.status(),
    ScriptedAgentScenarioCausalTraceCompletenessStatus::AllComplete
  );
  assert_eq!(report.status().id(), "all_complete");
  assert_eq!(report.start_observation_id(), ObservationId::new(201));
  assert_eq!(report.end_observation_id(), ObservationId::new(203));

  // Decision mismatch in one record makes it incomplete
  let mut tampered = rec2.clone();
  tampered.decision.selected_intent = LaneIntent::Stabilize;
  let incomplete_report = ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[
    rec1.clone(),
    tampered,
    rec3.clone(),
  ])
  .expect("evaluates with incomplete trace");
  assert_eq!(incomplete_report.record_count(), 3);
  assert_eq!(incomplete_report.traced_count(), 2);
  assert_eq!(
    incomplete_report.status(),
    ScriptedAgentScenarioCausalTraceCompletenessStatus::IncompleteTrace
  );
  assert_eq!(incomplete_report.status().id(), "incomplete_trace");

  // Empty input fails closed
  assert_eq!(
    ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[]),
    Err(ScriptedAgentScenarioCausalTraceCompletenessError::Empty)
  );

  // Duplicate observation ID fails closed
  let duplicate_obs = observe_player(&state, ObservationId::new(201)).observation();
  let duplicate_rec = ScriptedAgentReplayRecord::capture(
    ScriptedAgent::yielding_v1(),
    duplicate_obs,
    LaneIntent::Yield,
    None,
  );
  assert_eq!(
    ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&[
      rec1.clone(),
      duplicate_rec
    ]),
    Err(ScriptedAgentScenarioCausalTraceCompletenessError::DuplicateObservationId)
  );

  // Oversized input fails closed
  let mut oversized = Vec::new();
  for i in 0..=MAX_SCRIPTED_AGENT_SCENARIO_REPLAY_RECORDS {
    let obs_id = u64::try_from(i.saturating_add(300)).expect("fits in u64");
    let obs = observe_player(&state, ObservationId::new(obs_id)).observation();
    oversized.push(ScriptedAgentReplayRecord::capture(
      ScriptedAgent::cautious_v1(),
      obs,
      LaneIntent::Stabilize,
      None,
    ));
  }
  assert_eq!(
    ScriptedAgentScenarioCausalTraceCompletenessReport::from_records(&oversized),
    Err(ScriptedAgentScenarioCausalTraceCompletenessError::Oversized)
  );
}

#[test]
fn cautious_agent_stabilize_score_rises_with_observed_wave_pressure() {
  let initial = LaneSnapshot::initial();
  let low_pressure = LaneSnapshot::new(
    initial.ruleset(),
    initial.turn(),
    LaneStatus::Open,
    initial.player(),
    initial.opponent(),
    WaveState::new(WavePressure::new(0).expect("bounded pressure")),
    initial.jungle_threat(),
  );
  let high_pressure = LaneSnapshot::new(
    initial.ruleset(),
    initial.turn(),
    LaneStatus::Open,
    initial.player(),
    initial.opponent(),
    WaveState::new(WavePressure::new(3).expect("bounded pressure")),
    initial.jungle_threat(),
  );
  let low_receipt = observe_player(&low_pressure, ObservationId::new(17));
  let high_receipt = observe_player(&high_pressure, ObservationId::new(17));
  let agent = ScriptedAgent::cautious_v1();
  let low = agent
    .evaluate_candidate(low_receipt.observation(), LaneIntent::Stabilize)
    .expect("stabilize is advertised at low pressure");
  let high = agent
    .evaluate_candidate(high_receipt.observation(), LaneIntent::Stabilize)
    .expect("stabilize is advertised at high pressure");

  assert_eq!(low.score(), 80);
  assert_eq!(high.score(), 83);
  assert!(high.score() > low.score());
  assert_eq!(
    agent.choose(low_receipt.observation()).selected_intent(),
    LaneIntent::Stabilize
  );
  assert_eq!(
    agent.choose(high_receipt.observation()).selected_intent(),
    LaneIntent::Stabilize
  );
  validate_lane_request(
    &low_pressure,
    &low_receipt,
    &agent.choose(low_receipt.observation()).request(),
  )
  .expect("low-pressure request is legal");
  validate_lane_request(
    &high_pressure,
    &high_receipt,
    &agent.choose(high_receipt.observation()).request(),
  )
  .expect("high-pressure request is legal");
}

#[test]
fn candidate_breadth_tracks_only_actor_visible_advertisements() {
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
  let safe = observe_player(&initial, ObservationId::new(18)).observation();
  let threat = observe_player(&threat_state, ObservationId::new(18)).observation();
  let agent = ScriptedAgent::cautious_v1();
  let safe_candidates = agent.generate_candidates(safe);
  let threat_candidates = agent.generate_candidates(threat);

  assert_eq!(safe_candidates.len(), 4);
  assert_eq!(threat_candidates.len(), 5);
  assert_eq!(safe_candidates, safe.available_intents().to_vec());
  assert!(safe_candidates.iter().all(|intent| {
    safe.available_intents().contains(intent) || safe.available_threat_response() == Some(*intent)
  }));
  assert!(threat_candidates.iter().all(|intent| {
    threat.available_intents().contains(intent)
      || threat.available_threat_response() == Some(*intent)
  }));
  assert_eq!(
    threat_candidates
      .iter()
      .filter(|intent| **intent == LaneIntent::Withdraw)
      .count(),
    1
  );
  for candidates in [safe_candidates, threat_candidates] {
    for (index, candidate) in candidates.iter().enumerate() {
      assert!(!candidates[index + 1..].contains(candidate));
    }
  }
  assert_eq!(agent.choose(safe).selected_intent(), LaneIntent::Stabilize);
  assert_eq!(agent.choose(threat).selected_intent(), LaneIntent::Withdraw);
}

#[test]
fn stable_selection_keeps_the_first_advertised_maximum() {
  let candidates = [
    ScriptedAgentCandidate {
      intent: LaneIntent::Contest,
      score: 70,
      reason: ScriptedAgentReason::AvailableAlternative,
    },
    ScriptedAgentCandidate {
      intent: LaneIntent::Stabilize,
      score: 70,
      reason: ScriptedAgentReason::StableDefault,
    },
    ScriptedAgentCandidate {
      intent: LaneIntent::Yield,
      score: 60,
      reason: ScriptedAgentReason::AvailableAlternative,
    },
  ];

  assert_eq!(
    ScriptedAgent::select_candidate(&candidates).intent(),
    LaneIntent::Contest
  );
}

#[test]
fn matched_observation_distinguishes_three_profiles() {
  let state = LaneSnapshot::initial();
  let receipt = observe_player(&state, ObservationId::new(12));
  let cautious = ScriptedAgent::cautious_v1().choose(receipt.observation());
  let risk_taking = ScriptedAgent::risk_taking_v1().choose(receipt.observation());
  let yielding = ScriptedAgent::yielding_v1().choose(receipt.observation());

  assert_eq!(cautious.selected_intent(), LaneIntent::Stabilize);
  assert_eq!(risk_taking.selected_intent(), LaneIntent::Contest);
  assert_eq!(yielding.selected_intent(), LaneIntent::Yield);
  assert_eq!(cautious.profile().role(), ScriptedAgentRole::Anchor);
  assert_eq!(risk_taking.profile().role(), ScriptedAgentRole::Duelist);
  assert_eq!(yielding.profile().role(), ScriptedAgentRole::Pacer);
  assert_eq!(cautious.profile().role().id(), "anchor-v1");
  assert_eq!(risk_taking.profile().role().id(), "duelist-v1");
  assert_eq!(yielding.profile().role().id(), "pacer-v1");
  assert_eq!(
    cautious.profile().selection_rule(),
    "max-score-stable-order-v1"
  );
  assert_eq!(
    risk_taking.profile().selection_rule(),
    "max-score-stable-order-v1"
  );
  assert_eq!(
    yielding.profile().selection_rule(),
    "max-score-stable-order-v1"
  );
  assert_eq!(cautious.profile().preferred_intent(), LaneIntent::Stabilize);
  assert_eq!(
    risk_taking.profile().preferred_intent(),
    LaneIntent::Contest
  );
  assert_eq!(yielding.profile().preferred_intent(), LaneIntent::Yield);
  assert_eq!(
    cautious.profile().evaluation_rule(),
    "threat-first-pressure-aware-fixed-score-v1"
  );
  assert_eq!(
    risk_taking.profile().profile_id(),
    RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID
  );
  assert_eq!(
    risk_taking.profile().evaluation_rule(),
    "contest-first-fixed-score-v1"
  );
  assert_eq!(
    yielding.profile().profile_id(),
    YIELDING_SCRIPTED_AGENT_PROFILE_ID
  );
  assert_eq!(
    yielding.profile().evaluation_rule(),
    "yield-first-fixed-score-v1"
  );
  assert_eq!(
    cautious
      .candidates()
      .iter()
      .map(|candidate| candidate.intent())
      .collect::<Vec<_>>(),
    risk_taking
      .candidates()
      .iter()
      .map(|candidate| candidate.intent())
      .collect::<Vec<_>>()
  );
  assert_eq!(
    cautious
      .candidates()
      .iter()
      .map(|candidate| candidate.intent())
      .collect::<Vec<_>>(),
    yielding
      .candidates()
      .iter()
      .map(|candidate| candidate.intent())
      .collect::<Vec<_>>()
  );
  assert!(risk_taking.candidates().iter().any(|candidate| {
    candidate.intent() == LaneIntent::Contest
      && candidate.reason() == ScriptedAgentReason::RiskPreference
      && candidate.score() == 100
  }));
  assert!(yielding.candidates().iter().any(|candidate| {
    candidate.intent() == LaneIntent::Yield
      && candidate.reason() == ScriptedAgentReason::YieldPreference
      && candidate.score() == 100
  }));
  assert_eq!(
    risk_taking,
    ScriptedAgent::risk_taking_v1().choose(receipt.observation())
  );
  assert_eq!(
    yielding,
    ScriptedAgent::yielding_v1().choose(receipt.observation())
  );
  validate_lane_request(&state, &receipt, &cautious.request()).expect("cautious is legal");
  validate_lane_request(&state, &receipt, &risk_taking.request()).expect("risk-taking is legal");
  validate_lane_request(&state, &receipt, &yielding.request()).expect("yielding is legal");
}

#[test]
fn comparison_report_is_versioned_bounded_and_reproducible() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(16)).observation();
  let report = ScriptedAgentComparisonReport::from_observation(observation);

  assert_eq!(
    SCRIPTED_AGENT_METRICS_SCHEMA,
    "m4-scripted-agent-metrics-v1"
  );
  assert_eq!(report.schema(), SCRIPTED_AGENT_METRICS_SCHEMA);
  assert_eq!(report.observer(), observation.observer());
  assert_eq!(report.observation_id(), observation.observation_id());
  assert_eq!(report.entries().len(), 3);
  assert_eq!(
    report
      .entries()
      .iter()
      .map(|entry| entry.profile_id())
      .collect::<Vec<_>>(),
    vec![
      SCRIPTED_AGENT_PROFILE_ID,
      RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
      YIELDING_SCRIPTED_AGENT_PROFILE_ID
    ]
  );
  assert_eq!(
    report
      .entries()
      .iter()
      .map(|entry| entry.evaluation_rule())
      .collect::<Vec<_>>(),
    vec![
      "threat-first-pressure-aware-fixed-score-v1",
      "contest-first-fixed-score-v1",
      "yield-first-fixed-score-v1"
    ]
  );
  assert_eq!(
    report
      .entries()
      .iter()
      .map(|entry| entry.selected_intent())
      .collect::<Vec<_>>(),
    vec![
      LaneIntent::Stabilize,
      LaneIntent::Contest,
      LaneIntent::Yield
    ]
  );
  assert_eq!(
    report
      .entries()
      .iter()
      .map(|entry| entry.selected_score())
      .collect::<Vec<_>>(),
    vec![81, 100, 100]
  );
  assert!(
    report
      .entries()
      .iter()
      .all(|entry| entry.candidate_count() == 4)
  );
  assert_eq!(
    report,
    ScriptedAgentComparisonReport::from_observation(observation)
  );
}

#[test]
fn action_tally_reports_bounded_profile_counts_and_rejects_mixed_observers() {
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
  let safe_receipt = observe_player(&initial, ObservationId::new(14));
  let threat_receipt = observe_player(&threat_state, ObservationId::new(15));
  let report = ScriptedAgentActionTallyReport::from_observations([
    safe_receipt.observation(),
    threat_receipt.observation(),
  ])
  .expect("matched player observations tally");
  assert_eq!(
    report,
    ScriptedAgentActionTallyReport::from_observations([
      safe_receipt.observation(),
      threat_receipt.observation(),
    ])
    .expect("repeated matched observations tally")
  );

  assert_eq!(
    SCRIPTED_AGENT_ACTION_TALLY_SCHEMA,
    "m4-scripted-agent-action-tally-v2"
  );
  assert_eq!(report.schema(), SCRIPTED_AGENT_ACTION_TALLY_SCHEMA);
  assert_eq!(report.observer(), safe_receipt.observation().observer());
  assert_eq!(
    report.observation_ids(),
    &[ObservationId::new(14), ObservationId::new(15)]
  );
  assert_eq!(report.entries().len(), 3);
  assert_eq!(
    report
      .entries()
      .iter()
      .map(|entry| entry.profile_id())
      .collect::<Vec<_>>(),
    vec![
      SCRIPTED_AGENT_PROFILE_ID,
      RISK_TAKING_SCRIPTED_AGENT_PROFILE_ID,
      YIELDING_SCRIPTED_AGENT_PROFILE_ID
    ]
  );
  assert_eq!(
    report
      .entries()
      .iter()
      .map(|entry| entry.evaluation_rule())
      .collect::<Vec<_>>(),
    vec![
      "threat-first-pressure-aware-fixed-score-v1",
      "contest-first-fixed-score-v1",
      "yield-first-fixed-score-v1"
    ]
  );
  let cautious = report.entries()[0];
  assert_eq!(cautious.observation_count(), 2);
  assert_eq!(cautious.stabilize_count(), 1);
  assert_eq!(cautious.withdraw_count(), 1);
  assert_eq!(cautious.contest_count(), 0);
  assert_eq!(cautious.yield_count(), 0);
  assert_eq!(cautious.recall_count(), 0);
  let risk_taking = report.entries()[1];
  assert_eq!(risk_taking.contest_count(), 2);
  assert_eq!(risk_taking.stabilize_count(), 0);
  assert_eq!(risk_taking.withdraw_count(), 0);
  let yielding = report.entries()[2];
  assert_eq!(yielding.yield_count(), 2);
  assert_eq!(yielding.stabilize_count(), 0);
  assert_eq!(yielding.withdraw_count(), 0);

  for agent in [
    ScriptedAgent::cautious_v1(),
    ScriptedAgent::risk_taking_v1(),
    ScriptedAgent::yielding_v1(),
  ] {
    validate_lane_request(
      &initial,
      &safe_receipt,
      &agent.choose(safe_receipt.observation()).request(),
    )
    .expect("safe tally request is legal");
    validate_lane_request(
      &threat_state,
      &threat_receipt,
      &agent.choose(threat_receipt.observation()).request(),
    )
    .expect("threat tally request is legal");
  }

  let mixed_observer = safe_receipt
    .observation()
    .with_observer(ALLIED_AUTONOMOUS_ACTOR);
  assert_eq!(
    ScriptedAgentActionTallyReport::from_observations(
      [safe_receipt.observation(), mixed_observer,]
    ),
    Err(ScriptedAgentActionTallyError::MismatchedObserver)
  );
  assert_eq!(
    ScriptedAgentActionTallyReport::from_observations([
      safe_receipt.observation(),
      observe_player(&threat_state, ObservationId::new(14)).observation(),
    ]),
    Err(ScriptedAgentActionTallyError::DuplicateObservationId)
  );
}

#[test]
fn visible_threat_changes_only_the_cautious_profile_selection() {
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
  let safe_receipt = observe_player(&initial, ObservationId::new(14));
  let threat_receipt = observe_player(&threat_state, ObservationId::new(14));

  let cautious = ScriptedAgent::cautious_v1();
  let risk_taking = ScriptedAgent::risk_taking_v1();
  let yielding = ScriptedAgent::yielding_v1();
  let cautious_safe = cautious.choose(safe_receipt.observation());
  let cautious_threat = cautious.choose(threat_receipt.observation());
  let risk_safe = risk_taking.choose(safe_receipt.observation());
  let risk_threat = risk_taking.choose(threat_receipt.observation());
  let yielding_safe = yielding.choose(safe_receipt.observation());
  let yielding_threat = yielding.choose(threat_receipt.observation());

  assert_eq!(cautious_safe.selected_intent(), LaneIntent::Stabilize);
  assert_eq!(cautious_threat.selected_intent(), LaneIntent::Withdraw);
  assert_eq!(risk_safe.selected_intent(), LaneIntent::Contest);
  assert_eq!(risk_threat.selected_intent(), LaneIntent::Contest);
  assert_eq!(yielding_safe.selected_intent(), LaneIntent::Yield);
  assert_eq!(yielding_threat.selected_intent(), LaneIntent::Yield);
  assert!(cautious_threat.candidates().iter().any(|candidate| {
    candidate.intent() == LaneIntent::Withdraw
      && candidate.reason() == ScriptedAgentReason::ThreatResponse
  }));
  assert!(risk_threat.candidates().iter().any(|candidate| {
    candidate.intent() == LaneIntent::Withdraw
      && candidate.reason() == ScriptedAgentReason::ThreatResponse
  }));
  assert!(yielding_threat.candidates().iter().any(|candidate| {
    candidate.intent() == LaneIntent::Withdraw
      && candidate.reason() == ScriptedAgentReason::ThreatResponse
  }));
  validate_lane_request(&initial, &safe_receipt, &cautious_safe.request())
    .expect("cautious safe request is legal");
  validate_lane_request(&threat_state, &threat_receipt, &cautious_threat.request())
    .expect("cautious threat request is legal");
  validate_lane_request(&initial, &safe_receipt, &risk_safe.request())
    .expect("risk safe request is legal");
  validate_lane_request(&threat_state, &threat_receipt, &risk_threat.request())
    .expect("risk threat request is legal");
  validate_lane_request(&initial, &safe_receipt, &yielding_safe.request())
    .expect("yielding safe request is legal");
  validate_lane_request(&threat_state, &threat_receipt, &yielding_threat.request())
    .expect("yielding threat request is legal");
}

#[test]
fn semantic_profile_dimensions_round_trip_and_reject_invalid() {
  for (val, label) in [
    (SemanticRiskTolerance::Cautious, "cautious"),
    (SemanticRiskTolerance::Balanced, "balanced"),
    (SemanticRiskTolerance::RiskSeeking, "risk-seeking"),
  ] {
    assert_eq!(val.as_str(), label);
    assert_eq!(SemanticRiskTolerance::parse(label), Some(val));
  }
  assert_eq!(SemanticRiskTolerance::parse("unknown"), None);

  for (val, label) in [
    (SemanticDeference::Autonomous, "autonomous"),
    (SemanticDeference::Compliant, "compliant"),
    (SemanticDeference::Yielding, "yielding"),
  ] {
    assert_eq!(val.as_str(), label);
    assert_eq!(SemanticDeference::parse(label), Some(val));
  }
  assert_eq!(SemanticDeference::parse("unknown"), None);

  for (val, label) in [
    (SemanticFocus::Patience, "patience"),
    (SemanticFocus::Opportunity, "opportunity"),
    (SemanticFocus::Urgency, "urgency"),
  ] {
    assert_eq!(val.as_str(), label);
    assert_eq!(SemanticFocus::parse(label), Some(val));
  }
  assert_eq!(SemanticFocus::parse("unknown"), None);

  for (val, label) in [
    (SemanticCommunicationClarity::Terse, "terse"),
    (SemanticCommunicationClarity::Standard, "standard"),
    (SemanticCommunicationClarity::Verbose, "verbose"),
  ] {
    assert_eq!(val.as_str(), label);
    assert_eq!(SemanticCommunicationClarity::parse(label), Some(val));
  }
  assert_eq!(SemanticCommunicationClarity::parse("unknown"), None);
}

#[test]
fn semantic_profile_definitions_and_vocabulary_lookup_are_canonical() {
  let cautious = SemanticProfileDefinition::cautious_v1();
  let risk_taking = SemanticProfileDefinition::risk_taking_v1();
  let yielding = SemanticProfileDefinition::yielding_v1();

  assert_eq!(cautious.schema(), SEMANTIC_PROFILE_VOCABULARY_SCHEMA);
  assert_eq!(risk_taking.schema(), SEMANTIC_PROFILE_VOCABULARY_SCHEMA);
  assert_eq!(yielding.schema(), SEMANTIC_PROFILE_VOCABULARY_SCHEMA);

  assert_eq!(cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(risk_taking.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert_eq!(yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);

  assert_eq!(cautious.risk_tolerance(), SemanticRiskTolerance::Cautious);
  assert_eq!(cautious.deference(), SemanticDeference::Autonomous);
  assert_eq!(cautious.focus(), SemanticFocus::Patience);
  assert_eq!(
    cautious.communication_clarity(),
    SemanticCommunicationClarity::Terse
  );
  assert!(!cautious.description().is_empty());

  assert_eq!(
    risk_taking.risk_tolerance(),
    SemanticRiskTolerance::RiskSeeking
  );
  assert_eq!(risk_taking.deference(), SemanticDeference::Autonomous);
  assert_eq!(risk_taking.focus(), SemanticFocus::Opportunity);
  assert_eq!(
    risk_taking.communication_clarity(),
    SemanticCommunicationClarity::Standard
  );
  assert!(!risk_taking.description().is_empty());

  assert_eq!(yielding.risk_tolerance(), SemanticRiskTolerance::Cautious);
  assert_eq!(yielding.deference(), SemanticDeference::Yielding);
  assert_eq!(yielding.focus(), SemanticFocus::Patience);
  assert_eq!(
    yielding.communication_clarity(),
    SemanticCommunicationClarity::Terse
  );
  assert!(!yielding.description().is_empty());

  let all = SemanticProfileVocabulary::all_profiles();
  assert_eq!(all.len(), 3);
  assert_eq!(all[0], cautious);
  assert_eq!(all[1], risk_taking);
  assert_eq!(all[2], yielding);

  assert_eq!(
    SemanticProfileVocabulary::lookup(CAUTIOUS_SEMANTIC_PROFILE_ID),
    Some(cautious)
  );
  assert_eq!(
    SemanticProfileVocabulary::lookup(RISK_TAKING_SEMANTIC_PROFILE_ID),
    Some(risk_taking)
  );
  assert_eq!(
    SemanticProfileVocabulary::lookup(YIELDING_SEMANTIC_PROFILE_ID),
    Some(yielding)
  );
  assert_eq!(SemanticProfileVocabulary::lookup("unknown-profile"), None);

  assert_eq!(
    SemanticProfileVocabulary::validate_profile_id(CAUTIOUS_SEMANTIC_PROFILE_ID),
    Ok(cautious)
  );
  assert_eq!(
    SemanticProfileVocabulary::validate_profile_id(RISK_TAKING_SEMANTIC_PROFILE_ID),
    Ok(risk_taking)
  );
  assert_eq!(
    SemanticProfileVocabulary::validate_profile_id(YIELDING_SEMANTIC_PROFILE_ID),
    Ok(yielding)
  );
  assert_eq!(
    SemanticProfileVocabulary::validate_profile_id("unknown-profile"),
    Err(SemanticProfileVocabularyError::UnknownProfile)
  );
}

#[test]
fn diagnostic_choice_domains_and_catalog_are_canonical() {
  for (domain, label) in [
    (DiagnosticChoiceDomain::ContestConcede, "contest-concede"),
    (DiagnosticChoiceDomain::FollowReject, "follow-reject"),
    (DiagnosticChoiceDomain::FarmAssist, "farm-assist"),
    (DiagnosticChoiceDomain::RecallTiming, "recall-timing"),
    (DiagnosticChoiceDomain::Sacrifice, "sacrifice"),
    (DiagnosticChoiceDomain::Surprise, "surprise"),
    (
      DiagnosticChoiceDomain::ResponseToFailure,
      "response-to-failure",
    ),
  ] {
    assert_eq!(domain.as_str(), label);
    assert_eq!(DiagnosticChoiceDomain::parse(label), Some(domain));
  }
  assert_eq!(DiagnosticChoiceDomain::parse("unknown"), None);

  let cc = DiagnosticChoiceDefinition::contest_concede_v1();
  let fr = DiagnosticChoiceDefinition::follow_reject_v1();
  let fa = DiagnosticChoiceDefinition::farm_assist_v1();
  let rt = DiagnosticChoiceDefinition::recall_timing_v1();
  let sc = DiagnosticChoiceDefinition::sacrifice_v1();
  let sp = DiagnosticChoiceDefinition::surprise_v1();
  let rf = DiagnosticChoiceDefinition::response_to_failure_v1();

  for choice in [cc, fr, fa, rt, sc, sp, rf] {
    assert_eq!(choice.schema(), DIAGNOSTIC_CHOICE_CATALOG_SCHEMA);
    assert!(!choice.choice_id().is_empty());
    assert!(!choice.intended_contrast().is_empty());
    assert!(!choice.description().is_empty());
    assert_ne!(choice.primary_intent(), choice.alternative_intent());
  }

  assert_eq!(cc.choice_id(), CHOICE_CONTEST_CONCEDE_ID);
  assert_eq!(cc.domain(), DiagnosticChoiceDomain::ContestConcede);
  assert_eq!(cc.primary_intent(), LaneIntent::Contest);
  assert_eq!(cc.alternative_intent(), LaneIntent::Yield);

  assert_eq!(fr.choice_id(), CHOICE_FOLLOW_REJECT_ID);
  assert_eq!(fr.domain(), DiagnosticChoiceDomain::FollowReject);
  assert_eq!(fr.primary_intent(), LaneIntent::Contest);
  assert_eq!(fr.alternative_intent(), LaneIntent::Stabilize);

  assert_eq!(fa.choice_id(), CHOICE_FARM_ASSIST_ID);
  assert_eq!(fa.domain(), DiagnosticChoiceDomain::FarmAssist);
  assert_eq!(fa.primary_intent(), LaneIntent::Stabilize);
  assert_eq!(fa.alternative_intent(), LaneIntent::Contest);

  assert_eq!(rt.choice_id(), CHOICE_RECALL_TIMING_ID);
  assert_eq!(rt.domain(), DiagnosticChoiceDomain::RecallTiming);
  assert_eq!(rt.primary_intent(), LaneIntent::Recall);
  assert_eq!(rt.alternative_intent(), LaneIntent::Stabilize);

  assert_eq!(sc.choice_id(), CHOICE_SACRIFICE_ID);
  assert_eq!(sc.domain(), DiagnosticChoiceDomain::Sacrifice);
  assert_eq!(sc.primary_intent(), LaneIntent::Contest);
  assert_eq!(sc.alternative_intent(), LaneIntent::Withdraw);

  assert_eq!(sp.choice_id(), CHOICE_SURPRISE_ID);
  assert_eq!(sp.domain(), DiagnosticChoiceDomain::Surprise);
  assert_eq!(sp.primary_intent(), LaneIntent::Withdraw);
  assert_eq!(sp.alternative_intent(), LaneIntent::Stabilize);

  assert_eq!(rf.choice_id(), CHOICE_RESPONSE_TO_FAILURE_ID);
  assert_eq!(rf.domain(), DiagnosticChoiceDomain::ResponseToFailure);
  assert_eq!(rf.primary_intent(), LaneIntent::Yield);
  assert_eq!(rf.alternative_intent(), LaneIntent::Contest);

  let all = DiagnosticChoiceCatalog::all_choices();
  assert_eq!(all.len(), 7);
  assert_eq!(all[0], cc);
  assert_eq!(all[1], fr);
  assert_eq!(all[2], fa);
  assert_eq!(all[3], rt);
  assert_eq!(all[4], sc);
  assert_eq!(all[5], sp);
  assert_eq!(all[6], rf);

  for choice in [cc, fr, fa, rt, sc, sp, rf] {
    assert_eq!(
      DiagnosticChoiceCatalog::lookup(choice.choice_id()),
      Some(choice)
    );
    assert_eq!(
      DiagnosticChoiceCatalog::validate_choice_id(choice.choice_id()),
      Ok(choice)
    );
    assert_eq!(
      DiagnosticChoiceCatalog::choice_for_domain(choice.domain()),
      choice
    );
  }

  assert_eq!(DiagnosticChoiceCatalog::lookup("unknown-choice"), None);
  assert_eq!(
    DiagnosticChoiceCatalog::validate_choice_id("unknown-choice"),
    Err(DiagnosticChoiceCatalogError::UnknownChoice)
  );
}

#[test]
fn m7_model_prompt_and_repeated_sampling_protocols_are_bounded_and_fail_closed() {
  let std_prompt = ModelPromptProtocolDefinition::reference_standard_v1();
  let diag_prompt = ModelPromptProtocolDefinition::reference_diagnostic_v1();
  let alt_prompt = ModelPromptProtocolDefinition::alternative_diagnostic_v1();

  assert_eq!(std_prompt.schema(), MODEL_PROMPT_PROTOCOL_SCHEMA);
  assert_eq!(std_prompt.protocol_id(), MODEL_PROMPT_REFERENCE_STANDARD_ID);
  assert_eq!(std_prompt.model_family_id(), "model-family-reference-v1");
  assert_eq!(
    std_prompt.prompt_template_id(),
    "prompt-template-lane-standard-v1"
  );
  assert_eq!(
    std_prompt.system_prompt_version(),
    "sysprompt-actor-contract-v1"
  );
  assert_eq!(std_prompt.temperature_centiperc(), 70);
  assert_eq!(std_prompt.top_p_centiperc(), 95);
  assert!(std_prompt.requires_structured_output());
  assert!(!std_prompt.chain_of_thought_required());
  assert_eq!(std_prompt.validate(), Ok(()));

  assert_eq!(diag_prompt.schema(), MODEL_PROMPT_PROTOCOL_SCHEMA);
  assert_eq!(
    diag_prompt.protocol_id(),
    MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
  );
  assert_eq!(diag_prompt.model_family_id(), "model-family-reference-v1");
  assert_eq!(
    diag_prompt.prompt_template_id(),
    "prompt-template-lane-diagnostic-v1"
  );
  assert_eq!(diag_prompt.temperature_centiperc(), 50);
  assert_eq!(diag_prompt.top_p_centiperc(), 90);
  assert!(diag_prompt.requires_structured_output());
  assert!(!diag_prompt.chain_of_thought_required());
  assert_eq!(diag_prompt.validate(), Ok(()));

  assert_eq!(alt_prompt.schema(), MODEL_PROMPT_PROTOCOL_SCHEMA);
  assert_eq!(
    alt_prompt.protocol_id(),
    MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID
  );
  assert_eq!(alt_prompt.model_family_id(), "model-family-alternative-v1");
  assert_eq!(
    alt_prompt.prompt_template_id(),
    "prompt-template-lane-diagnostic-v1"
  );
  assert_eq!(alt_prompt.temperature_centiperc(), 50);
  assert_eq!(alt_prompt.top_p_centiperc(), 90);
  assert!(alt_prompt.requires_structured_output());
  assert!(!alt_prompt.chain_of_thought_required());
  assert_eq!(alt_prompt.validate(), Ok(()));

  let all_prompts = ModelPromptProtocolCatalog::all_protocols();
  assert_eq!(all_prompts.len(), 3);
  assert_eq!(all_prompts[0], std_prompt);
  assert_eq!(all_prompts[1], diag_prompt);
  assert_eq!(all_prompts[2], alt_prompt);

  for p in [std_prompt, diag_prompt, alt_prompt] {
    assert_eq!(ModelPromptProtocolCatalog::lookup(p.protocol_id()), Some(p));
    assert_eq!(
      ModelPromptProtocolCatalog::validate_protocol_id(p.protocol_id()),
      Ok(p)
    );
  }
  assert_eq!(
    ModelPromptProtocolCatalog::lookup("unknown-model-prompt"),
    None
  );
  assert_eq!(
    ModelPromptProtocolCatalog::validate_protocol_id("unknown-model-prompt"),
    Err(ModelPromptProtocolError::UnknownProtocol)
  );

  let mut invalid_temp = std_prompt;
  invalid_temp.temperature_centiperc = 201;
  assert_eq!(
    invalid_temp.validate(),
    Err(ModelPromptProtocolError::InvalidTemperature)
  );

  let mut invalid_top_p = std_prompt;
  invalid_top_p.top_p_centiperc = 101;
  assert_eq!(
    invalid_top_p.validate(),
    Err(ModelPromptProtocolError::InvalidTopP)
  );

  let mut invalid_cot = std_prompt;
  invalid_cot.chain_of_thought_required = true;
  assert_eq!(
    invalid_cot.validate(),
    Err(ModelPromptProtocolError::PrivateChainOfThoughtForbidden)
  );

  let std_samp = RepeatedSamplingProtocolDefinition::standard_repeat_10_v1();
  let diag_samp = RepeatedSamplingProtocolDefinition::diagnostic_repeat_30_v1();
  let quick_samp = RepeatedSamplingProtocolDefinition::quick_check_5_v1();

  assert_eq!(std_samp.schema(), REPEATED_SAMPLING_PROTOCOL_SCHEMA);
  assert_eq!(std_samp.protocol_id(), SAMPLING_STANDARD_REPEAT_10_ID);
  assert_eq!(std_samp.sample_count(), 10);
  assert_eq!(std_samp.seed_offset_step(), 1);
  assert_eq!(std_samp.max_repair_retries(), 3);
  assert!(std_samp.fail_closed_on_unrepaired());
  assert_eq!(std_samp.validate(), Ok(()));

  assert_eq!(diag_samp.schema(), REPEATED_SAMPLING_PROTOCOL_SCHEMA);
  assert_eq!(diag_samp.protocol_id(), SAMPLING_DIAGNOSTIC_REPEAT_30_ID);
  assert_eq!(diag_samp.sample_count(), 30);
  assert_eq!(diag_samp.seed_offset_step(), 1);
  assert_eq!(diag_samp.max_repair_retries(), 3);
  assert!(diag_samp.fail_closed_on_unrepaired());
  assert_eq!(diag_samp.validate(), Ok(()));

  assert_eq!(quick_samp.schema(), REPEATED_SAMPLING_PROTOCOL_SCHEMA);
  assert_eq!(quick_samp.protocol_id(), SAMPLING_QUICK_CHECK_5_ID);
  assert_eq!(quick_samp.sample_count(), 5);
  assert_eq!(quick_samp.seed_offset_step(), 1);
  assert_eq!(quick_samp.max_repair_retries(), 2);
  assert!(quick_samp.fail_closed_on_unrepaired());
  assert_eq!(quick_samp.validate(), Ok(()));

  let all_samps = RepeatedSamplingProtocolCatalog::all_protocols();
  assert_eq!(all_samps.len(), 3);
  assert_eq!(all_samps[0], std_samp);
  assert_eq!(all_samps[1], diag_samp);
  assert_eq!(all_samps[2], quick_samp);

  for s in [std_samp, diag_samp, quick_samp] {
    assert_eq!(
      RepeatedSamplingProtocolCatalog::lookup(s.protocol_id()),
      Some(s)
    );
    assert_eq!(
      RepeatedSamplingProtocolCatalog::validate_protocol_id(s.protocol_id()),
      Ok(s)
    );
  }
  assert_eq!(
    RepeatedSamplingProtocolCatalog::lookup("unknown-sampling-protocol"),
    None
  );
  assert_eq!(
    RepeatedSamplingProtocolCatalog::validate_protocol_id("unknown-sampling-protocol"),
    Err(RepeatedSamplingProtocolError::UnknownProtocol)
  );

  let mut zero_samples = std_samp;
  zero_samples.sample_count = 0;
  assert_eq!(
    zero_samples.validate(),
    Err(RepeatedSamplingProtocolError::InvalidSampleCount)
  );

  let mut excessive_samples = std_samp;
  excessive_samples.sample_count = 101;
  assert_eq!(
    excessive_samples.validate(),
    Err(RepeatedSamplingProtocolError::InvalidSampleCount)
  );

  let mut zero_step = std_samp;
  zero_step.seed_offset_step = 0;
  assert_eq!(
    zero_step.validate(),
    Err(RepeatedSamplingProtocolError::InvalidSeedOffsetStep)
  );

  let mut excessive_retries = std_samp;
  excessive_retries.max_repair_retries = 11;
  assert_eq!(
    excessive_retries.validate(),
    Err(RepeatedSamplingProtocolError::InvalidMaxRetries)
  );
}

#[test]
fn m7_empirical_action_and_communication_distribution_estimates_are_bounded_and_exact() {
  let cautious_profile = CAUTIOUS_SEMANTIC_PROFILE_ID;
  let cc_choice = CHOICE_CONTEST_CONCEDE_ID;

  let valid_action =
    DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 10, 2, 8, 0)
      .expect("valid action distribution");

  assert_eq!(valid_action.schema(), EMPIRICAL_ACTION_DISTRIBUTION_SCHEMA);
  assert_eq!(valid_action.choice_id(), cc_choice);
  assert_eq!(valid_action.profile_id(), cautious_profile);
  assert_eq!(valid_action.primary_intent(), LaneIntent::Contest);
  assert_eq!(valid_action.alternative_intent(), LaneIntent::Yield);
  assert_eq!(valid_action.sample_count(), 10);
  assert_eq!(valid_action.primary_count(), 2);
  assert_eq!(valid_action.alternative_count(), 8);
  assert_eq!(valid_action.other_count(), 0);
  assert_eq!(valid_action.basis_points(), [2_000, 8_000, 0]);
  assert_eq!(valid_action.primary_share_basis_points(), 2_000);
  assert_eq!(valid_action.alternative_share_basis_points(), 8_000);
  assert_eq!(valid_action.other_share_basis_points(), 0);
  assert_eq!(
    valid_action.basis_points().iter().sum::<u16>(),
    EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
  );
  assert!(valid_action.to_markdown().contains(cc_choice));

  // Remainder handling in basis points
  let odd_action = DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 7, 2, 4, 1)
    .expect("valid odd sample distribution");
  let bp = odd_action.basis_points();
  assert_eq!(bp[0], 2857); // 2 * 10000 / 7
  assert_eq!(bp[1], 5714); // 4 * 10000 / 7
  assert_eq!(bp[2], 1429); // 10000 - (2857 + 5714) = 1429
  assert_eq!(
    bp.iter().sum::<u16>(),
    EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
  );

  // Validation errors
  assert_eq!(
    DiagnosticChoiceActionDistribution::new("unknown-choice", cautious_profile, 10, 2, 8, 0,),
    Err(EmpiricalDistributionEstimationError::UnknownChoice)
  );
  assert_eq!(
    DiagnosticChoiceActionDistribution::new(cc_choice, "unknown-profile", 10, 2, 8, 0,),
    Err(EmpiricalDistributionEstimationError::UnknownProfile)
  );
  assert_eq!(
    DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 0, 0, 0, 0,),
    Err(EmpiricalDistributionEstimationError::InvalidSampleCount)
  );
  assert_eq!(
    DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 101, 50, 50, 1,),
    Err(EmpiricalDistributionEstimationError::InvalidSampleCount)
  );
  assert_eq!(
    DiagnosticChoiceActionDistribution::new(cc_choice, cautious_profile, 10, 2, 7, 0,),
    Err(EmpiricalDistributionEstimationError::CountSumMismatch)
  );

  // Communication distribution
  let valid_comm = DiagnosticChoiceCommunicationDistribution::new(
    cc_choice,
    cautious_profile,
    10,
    [8, 1, 1, 0, 0],
  )
  .expect("valid comm distribution");
  assert_eq!(
    valid_comm.schema(),
    EMPIRICAL_COMMUNICATION_DISTRIBUTION_SCHEMA
  );
  assert_eq!(valid_comm.choice_id(), cc_choice);
  assert_eq!(valid_comm.profile_id(), cautious_profile);
  assert_eq!(valid_comm.sample_count(), 10);
  assert_eq!(valid_comm.signal_counts(), [8, 1, 1, 0, 0]);
  assert_eq!(valid_comm.basis_points(), [8_000, 1_000, 1_000, 0, 0]);
  assert_eq!(
    valid_comm.signal_share_basis_points(LanePingSignal::None),
    8_000
  );
  assert_eq!(
    valid_comm.signal_share_basis_points(LanePingSignal::Danger),
    1_000
  );
  assert_eq!(
    valid_comm.signal_share_basis_points(LanePingSignal::OnMyWay),
    1_000
  );
  assert_eq!(
    valid_comm.signal_share_basis_points(LanePingSignal::Assist),
    0
  );
  assert_eq!(
    valid_comm.signal_share_basis_points(LanePingSignal::EnemyMissing),
    0
  );
  assert_eq!(
    valid_comm.basis_points().iter().sum::<u16>(),
    EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
  );
  assert!(valid_comm.to_markdown().contains(cc_choice));

  // Communication validation errors
  assert_eq!(
    DiagnosticChoiceCommunicationDistribution::new(
      "unknown-choice",
      cautious_profile,
      10,
      [10, 0, 0, 0, 0],
    ),
    Err(EmpiricalDistributionEstimationError::UnknownChoice)
  );
  assert_eq!(
    DiagnosticChoiceCommunicationDistribution::new(
      cc_choice,
      "unknown-profile",
      10,
      [10, 0, 0, 0, 0],
    ),
    Err(EmpiricalDistributionEstimationError::UnknownProfile)
  );
  assert_eq!(
    DiagnosticChoiceCommunicationDistribution::new(cc_choice, cautious_profile, 0, [0, 0, 0, 0, 0],),
    Err(EmpiricalDistributionEstimationError::InvalidSampleCount)
  );
  assert_eq!(
    DiagnosticChoiceCommunicationDistribution::new(
      cc_choice,
      cautious_profile,
      10,
      [9, 0, 0, 0, 0],
    ),
    Err(EmpiricalDistributionEstimationError::CountSumMismatch)
  );

  // Canonical baseline reports
  let cautious_rep = EmpiricalDistributionEstimateReport::cautious_v1();
  let risk_rep = EmpiricalDistributionEstimateReport::risk_taking_v1();
  let yielding_rep = EmpiricalDistributionEstimateReport::yielding_v1();

  for rep in [&cautious_rep, &risk_rep, &yielding_rep] {
    assert_eq!(rep.schema(), EMPIRICAL_DISTRIBUTION_ESTIMATION_SCHEMA);
    assert_eq!(rep.validate(), Ok(()));
    assert_eq!(rep.action_distributions().len(), 7);
    assert_eq!(rep.communication_distributions().len(), 7);

    for action_dist in rep.action_distributions() {
      assert_eq!(
        action_dist.basis_points().iter().sum::<u16>(),
        EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
      );
    }
    for comm_dist in rep.communication_distributions() {
      assert_eq!(
        comm_dist.basis_points().iter().sum::<u16>(),
        EMPIRICAL_DISTRIBUTION_SCALE_BASIS_POINTS
      );
    }
    let md = rep.to_markdown();
    assert!(md.contains("# Empirical Distribution Estimate Report"));
    assert!(md.contains("## Action Distributions"));
    assert!(md.contains("## Communication Distributions"));
  }

  assert_eq!(cautious_rep.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(risk_rep.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert_eq!(yielding_rep.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);

  // Verify report validation failure on mismatched profile inside action dist
  let mut bad_rep = cautious_rep.clone();
  bad_rep.action_distributions[0] = DiagnosticChoiceActionDistribution::new(
    CHOICE_CONTEST_CONCEDE_ID,
    RISK_TAKING_SEMANTIC_PROFILE_ID,
    10,
    9,
    1,
    0,
  )
  .expect("valid dist");
  assert_eq!(
    bad_rep.validate(),
    Err(EmpiricalDistributionEstimationError::MismatchedProfile)
  );

  // Verify report validation failure on mismatched choice order
  let mut unordered_rep = cautious_rep.clone();
  unordered_rep.action_distributions[0] = cautious_rep.action_distributions[1];
  assert_eq!(
    unordered_rep.validate(),
    Err(EmpiricalDistributionEstimationError::MismatchedChoice)
  );
}

#[test]
fn behavioral_measures_evaluate_distance_entropy_sensitivity_consistency_and_adaptation() {
  let cautious_rep = EmpiricalDistributionEstimateReport::cautious_v1();
  let risk_rep = EmpiricalDistributionEstimateReport::risk_taking_v1();
  let yielding_rep = EmpiricalDistributionEstimateReport::yielding_v1();

  // 1. Behavioral Distance (TVD)
  let dist_cautious_cautious = BehavioralDistanceMeasure::action_tvd(
    cautious_rep.action_distributions()[0],
    cautious_rep.action_distributions()[0],
  );
  assert_eq!(dist_cautious_cautious, 0);

  let dist_cautious_risk = BehavioralDistanceMeasure::action_tvd(
    cautious_rep.action_distributions()[0],
    risk_rep.action_distributions()[0],
  );
  let dist_risk_cautious = BehavioralDistanceMeasure::action_tvd(
    risk_rep.action_distributions()[0],
    cautious_rep.action_distributions()[0],
  );
  assert_eq!(dist_cautious_risk, dist_risk_cautious);
  // Cautious contest-concede is [2000, 8000, 0]; Risk is [9000, 1000, 0].
  // diff = |2000-9000| + |8000-1000| + |0-0| = 7000 + 7000 = 14000. TVD = 7000 bp.
  assert_eq!(dist_cautious_risk, 7000);

  // Triangle inequality on contest-concede: TVD(A, C) <= TVD(A, B) + TVD(B, C)
  let dist_cautious_yielding = BehavioralDistanceMeasure::action_tvd(
    cautious_rep.action_distributions()[0],
    yielding_rep.action_distributions()[0],
  );
  let dist_risk_yielding = BehavioralDistanceMeasure::action_tvd(
    risk_rep.action_distributions()[0],
    yielding_rep.action_distributions()[0],
  );
  assert!(dist_cautious_risk <= dist_cautious_yielding + dist_risk_yielding);

  // Distance Report
  let dist_rep = BehavioralDistanceReport::from_reports(&cautious_rep, &risk_rep);
  assert_eq!(dist_rep.schema(), BEHAVIORAL_DISTANCE_SCHEMA);
  assert_eq!(dist_rep.baseline_profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(
    dist_rep.candidate_profile_id(),
    RISK_TAKING_SEMANTIC_PROFILE_ID
  );
  assert_eq!(dist_rep.action_choice_distances().len(), 7);
  assert_eq!(dist_rep.communication_choice_distances().len(), 7);
  assert!(dist_rep.mean_action_distance_bp() > 0);
  let dist_md = dist_rep.to_markdown();
  assert!(dist_md.contains("# Behavioral Distance Report"));
  assert!(dist_md.contains("contest-concede"));

  // 2. Behavioral Entropy (Gini diversity index)
  // Deterministic distribution ([10, 0, 0]) in Surprise for Cautious -> entropy == 0
  let surprise_cautious = cautious_rep.action_distributions()[5];
  assert_eq!(
    BehavioralEntropyMeasure::action_entropy(surprise_cautious),
    0
  );

  // Mixed distribution in ContestConcede for Cautious ([2, 8, 0] -> [2000, 8000, 0])
  // sum_sq = 2000^2 + 8000^2 = 4_000_000 + 64_000_000 = 68_000_000. conc = 6800.
  // entropy = 10000 - 6800 = 3200 bp.
  let contest_cautious = cautious_rep.action_distributions()[0];
  assert_eq!(
    BehavioralEntropyMeasure::action_entropy(contest_cautious),
    3200
  );

  let mean_action_entropy = BehavioralEntropyMeasure::mean_action_entropy(&cautious_rep);
  assert!(mean_action_entropy > 0);
  assert!(mean_action_entropy < 5000);

  // 3. Behavioral Sensitivity
  // Cautious: ContestConcede primary_bp = 2000; Surprise primary_bp (Withdraw) = 10000.
  // |2000 - 10000| = 8000 bp sensitivity.
  assert_eq!(
    BehavioralSensitivityMeasure::surprise_sensitivity(&cautious_rep),
    8000
  );

  // 4. Behavioral Consistency
  // In Surprise, Cautious is [10, 0, 0] -> consistency is 10,000 bp (100% modal adherence).
  assert_eq!(
    BehavioralConsistencyMeasure::action_consistency(surprise_cautious),
    10000
  );
  let mean_consistency = BehavioralConsistencyMeasure::mean_action_consistency(&cautious_rep);
  assert!(mean_consistency >= 8000); // High modal adherence for baseline

  // 5. Behavioral Adaptation
  assert_eq!(
    BehavioralAdaptationMeasure::surprise_adaptation_bp(&cautious_rep),
    10000
  );
  assert_eq!(
    BehavioralAdaptationMeasure::failure_adaptation_bp(&cautious_rep),
    9000
  );
  assert_eq!(
    BehavioralAdaptationMeasure::composite_adaptation_bp(&cautious_rep),
    9500
  );

  // RiskTaking should have low defensive adaptation
  assert_eq!(
    BehavioralAdaptationMeasure::surprise_adaptation_bp(&risk_rep),
    2000
  );
  assert_eq!(
    BehavioralAdaptationMeasure::failure_adaptation_bp(&risk_rep),
    1000
  );
  assert_eq!(
    BehavioralAdaptationMeasure::composite_adaptation_bp(&risk_rep),
    1500
  );

  // 6. Unified Behavioral Measures Report
  let measures_cautious = BehavioralMeasuresReport::from_report(&cautious_rep);
  assert_eq!(measures_cautious.schema(), BEHAVIORAL_MEASURES_SCHEMA);
  assert_eq!(measures_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(measures_cautious.surprise_sensitivity_bp(), 8000);
  assert_eq!(measures_cautious.composite_adaptation_bp(), 9500);

  let md = measures_cautious.to_markdown();
  assert!(md.contains("# Behavioral Measures Report"));
  assert!(md.contains("composite_adaptation_bp: 9500"));
}

#[test]
fn parametric_policy_fitting_and_regularization_evaluate_correctly() {
  let cc_choice = CHOICE_CONTEST_CONCEDE_ID;

  // 1. ParametricActionWeights creation and validation
  let act_w = ParametricActionWeights::new(
    cc_choice,
    LaneIntent::Contest,
    LaneIntent::Yield,
    2000,
    8000,
    0,
  )
  .expect("valid action weights");
  assert_eq!(act_w.choice_id(), cc_choice);
  assert_eq!(act_w.primary_intent(), LaneIntent::Contest);
  assert_eq!(act_w.alternative_intent(), LaneIntent::Yield);
  assert_eq!(act_w.primary_weight_bp(), 2000);
  assert_eq!(act_w.alternative_weight_bp(), 8000);
  assert_eq!(act_w.residual_weight_bp(), 0);
  assert_eq!(act_w.basis_points(), [2000, 8000, 0]);
  assert_eq!(act_w.predicted_intent(), LaneIntent::Yield);
  let act_md = act_w.to_markdown();
  assert!(act_md.contains(cc_choice));
  assert!(act_md.contains("yield"));

  // Action weight error handling
  assert_eq!(
    ParametricActionWeights::new(
      "unknown-choice",
      LaneIntent::Contest,
      LaneIntent::Yield,
      2000,
      8000,
      0,
    ),
    Err(ParametricPolicyError::UnknownChoice)
  );
  assert_eq!(
    ParametricActionWeights::new(
      cc_choice,
      LaneIntent::Stabilize,
      LaneIntent::Yield,
      2000,
      8000,
      0,
    ),
    Err(ParametricPolicyError::MismatchedChoice)
  );
  assert_eq!(
    ParametricActionWeights::new(
      cc_choice,
      LaneIntent::Contest,
      LaneIntent::Yield,
      2000,
      7000,
      0,
    ),
    Err(ParametricPolicyError::WeightSumMismatch)
  );

  // 2. ParametricCommunicationWeights creation and validation
  let comm_w = ParametricCommunicationWeights::new(cc_choice, [5000, 3000, 1000, 1000, 0])
    .expect("valid communication weights");
  assert_eq!(comm_w.choice_id(), cc_choice);
  assert_eq!(comm_w.none_bp(), 5000);
  assert_eq!(comm_w.danger_bp(), 3000);
  assert_eq!(comm_w.on_my_way_bp(), 1000);
  assert_eq!(comm_w.assist_bp(), 1000);
  assert_eq!(comm_w.enemy_missing_bp(), 0);
  assert_eq!(comm_w.predicted_signal(), LanePingSignal::None);
  let comm_md = comm_w.to_markdown();
  assert!(comm_md.contains(cc_choice));
  assert!(comm_md.contains("none"));

  // Communication weight error handling
  assert_eq!(
    ParametricCommunicationWeights::new("unknown-choice", [2000; 5]),
    Err(ParametricPolicyError::UnknownChoice)
  );
  assert_eq!(
    ParametricCommunicationWeights::new(cc_choice, [2000, 2000, 2000, 2000, 1000]),
    Err(ParametricPolicyError::WeightSumMismatch)
  );

  // 3. Regularized Fitting across lambda values
  let cautious_rep = EmpiricalDistributionEstimateReport::cautious_v1();
  let risk_rep = EmpiricalDistributionEstimateReport::risk_taking_v1();
  let yielding_rep = EmpiricalDistributionEstimateReport::yielding_v1();

  // Lambda = 0 (unregularized / MLE)
  let unreg_cautious =
    ParametricPolicyFitter::fit_unregularized(&cautious_rep).expect("unregularized fit succeeds");
  assert_eq!(unreg_cautious.regularization_bp(), 0);
  assert_eq!(unreg_cautious.mean_fit_loss_bp(), 0);
  assert_eq!(
    unreg_cautious.action_weights()[0].basis_points(),
    cautious_rep.action_distributions()[0].basis_points()
  );
  assert_eq!(
    unreg_cautious.communication_weights()[0].signal_weights_bp(),
    cautious_rep.communication_distributions()[0].basis_points()
  );

  let unreg_risk =
    ParametricPolicyFitter::fit_unregularized(&risk_rep).expect("unregularized risk fit succeeds");
  assert_eq!(unreg_risk.regularization_bp(), 0);
  assert_eq!(unreg_risk.mean_fit_loss_bp(), 0);

  let unreg_yielding = ParametricPolicyFitter::fit_unregularized(&yielding_rep)
    .expect("unregularized yielding fit succeeds");
  assert_eq!(unreg_yielding.regularization_bp(), 0);
  assert_eq!(unreg_yielding.mean_fit_loss_bp(), 0);

  // Lambda = 10,000 (fully regularized prior)
  let fully_reg =
    ParametricPolicyFitter::fit(&cautious_rep, 10_000).expect("fully regularized fit succeeds");
  assert_eq!(fully_reg.regularization_bp(), 10_000);
  for w in fully_reg.action_weights() {
    assert_eq!(w.primary_weight_bp(), 5000);
    assert_eq!(w.alternative_weight_bp(), 5000);
    assert_eq!(w.residual_weight_bp(), 0);
  }
  for w in fully_reg.communication_weights() {
    assert_eq!(w.signal_weights_bp(), [2000, 2000, 2000, 2000, 2000]);
  }

  // Monotonic shrinkage as lambda increases from 0 to 10,000
  // In cautious ContestConcede, empirical primary is 2000 bp. Prior is 5000 bp.
  // As lambda increases, primary weight must monotonically increase towards 5000 bp.
  let fit_0 = ParametricPolicyFitter::fit(&cautious_rep, 0).expect("fit 0");
  let fit_1k = ParametricPolicyFitter::fit(&cautious_rep, 1000).expect("fit 1k");
  let fit_5k = ParametricPolicyFitter::fit(&cautious_rep, 5000).expect("fit 5k");
  let fit_10k = ParametricPolicyFitter::fit(&cautious_rep, 10000).expect("fit 10k");

  let p0 = fit_0.action_weights()[0].primary_weight_bp();
  let p1k = fit_1k.action_weights()[0].primary_weight_bp();
  let p5k = fit_5k.action_weights()[0].primary_weight_bp();
  let p10k = fit_10k.action_weights()[0].primary_weight_bp();

  assert_eq!(p0, 2000);
  assert_eq!(p1k, 2300); // 0.9 * 2000 + 0.1 * 5000 = 1800 + 500 = 2300
  assert_eq!(p5k, 3500); // 0.5 * 2000 + 0.5 * 5000 = 1000 + 2500 = 3500
  assert_eq!(p10k, 5000);
  assert!(p0 < p1k && p1k < p5k && p5k < p10k);

  // Sum conservation check for all fitted policies
  for fit in [&fit_0, &fit_1k, &fit_5k, &fit_10k] {
    for act in fit.action_weights() {
      assert_eq!(
        u32::from(act.primary_weight_bp())
          + u32::from(act.alternative_weight_bp())
          + u32::from(act.residual_weight_bp()),
        10_000
      );
    }
    for comm in fit.communication_weights() {
      let sum: u32 = comm.signal_weights_bp().iter().map(|&w| u32::from(w)).sum();
      assert_eq!(sum, 10_000);
    }
  }

  // 4. Canonical baseline fitted policies
  let policy_cautious = ParametricPolicyDefinition::cautious_v1();
  let policy_risk = ParametricPolicyDefinition::risk_taking_v1();
  let policy_yielding = ParametricPolicyDefinition::yielding_v1();

  for policy in [&policy_cautious, &policy_risk, &policy_yielding] {
    assert_eq!(policy.schema(), PARAMETRIC_POLICY_SCHEMA);
    assert_eq!(
      policy.regularization_bp(),
      DEFAULT_PARAMETRIC_REGULARIZATION_BASIS_POINTS
    );
    assert_eq!(policy.validate(), Ok(()));
    assert_eq!(policy.action_weights().len(), 7);
    assert_eq!(policy.communication_weights().len(), 7);

    let md = policy.to_markdown();
    assert!(md.contains("# Parametric Policy Definition"));
    assert!(md.contains("## Action Parameter Weights"));
    assert!(md.contains("## Communication Parameter Weights"));
  }

  // Trait bounds and predicted intents
  // Cautious: ContestConcede -> Yield; Surprise -> Withdraw
  let cc_cautious = policy_cautious
    .action_weights_for_domain(DiagnosticChoiceDomain::ContestConcede)
    .expect("choice found");
  assert_eq!(cc_cautious.predicted_intent(), LaneIntent::Yield);

  let surprise_cautious = policy_cautious
    .action_weights_for_domain(DiagnosticChoiceDomain::Surprise)
    .expect("choice found");
  assert_eq!(surprise_cautious.predicted_intent(), LaneIntent::Withdraw);

  // RiskTaking: ContestConcede -> Contest; FarmAssist -> Contest
  let cc_risk = policy_risk
    .action_weights_for_domain(DiagnosticChoiceDomain::ContestConcede)
    .expect("choice found");
  assert_eq!(cc_risk.predicted_intent(), LaneIntent::Contest);

  let farm_risk = policy_risk
    .action_weights_for_domain(DiagnosticChoiceDomain::FarmAssist)
    .expect("choice found");
  assert_eq!(farm_risk.predicted_intent(), LaneIntent::Contest);

  // Yielding: ResponseToFailure -> Yield
  let failure_yielding = policy_yielding
    .action_weights_for_domain(DiagnosticChoiceDomain::ResponseToFailure)
    .expect("choice found");
  assert_eq!(failure_yielding.predicted_intent(), LaneIntent::Yield);

  // 5. Error cases
  assert_eq!(
    ParametricPolicyFitter::fit(&cautious_rep, 10_001),
    Err(ParametricPolicyError::InvalidRegularization)
  );
}

#[test]
fn held_out_scenario_evaluation_and_counterfactual_perturbations_are_verified() {
  // 1. Schema constants and thresholds
  assert_eq!(HELD_OUT_SCENARIO_SCHEMA, "m7-held-out-scenario-v1");
  assert_eq!(
    HELD_OUT_SCENARIO_CATALOG_SCHEMA,
    "m7-held-out-scenario-catalog-v1"
  );
  assert_eq!(
    HELD_OUT_EVALUATION_SCHEMA,
    "m7-held-out-scenario-evaluation-v1"
  );
  assert_eq!(
    COUNTERFACTUAL_PERTURBATION_SCHEMA,
    "m7-counterfactual-perturbation-v1"
  );
  assert_eq!(
    COUNTERFACTUAL_SENSITIVITY_SCHEMA,
    "m7-counterfactual-sensitivity-v1"
  );
  assert_eq!(CALIBRATION_HELD_OUT_SCHEMA, "m7-calibration-held-out-v1");
  assert_eq!(MAX_ACCEPTABLE_HELD_OUT_LOSS_BP, 2_500);
  assert_eq!(MIN_ACCEPTABLE_MODAL_ACCURACY_BP, 7_000);
  assert_eq!(COUNTERFACTUAL_TOLERANCE_BP, 200);

  // 2. Held-out scenario suites for all 3 canonical profiles
  let cautious_scenarios =
    HeldOutScenarioCatalog::scenarios_for_profile(CAUTIOUS_SEMANTIC_PROFILE_ID)
      .expect("cautious scenarios found");
  let risk_scenarios =
    HeldOutScenarioCatalog::scenarios_for_profile(RISK_TAKING_SEMANTIC_PROFILE_ID)
      .expect("risk scenarios found");
  let yielding_scenarios =
    HeldOutScenarioCatalog::scenarios_for_profile(YIELDING_SEMANTIC_PROFILE_ID)
      .expect("yielding scenarios found");

  assert_eq!(cautious_scenarios.len(), 7);
  assert_eq!(risk_scenarios.len(), 7);
  assert_eq!(yielding_scenarios.len(), 7);

  assert_eq!(
    HeldOutScenarioCatalog::scenarios_for_profile("unknown-profile-v1"),
    Err(HeldOutEvaluationError::UnknownProfile)
  );

  // Verify all 7 diagnostic domains are represented in each suite
  for suite in [&cautious_scenarios, &risk_scenarios, &yielding_scenarios] {
    let domains: Vec<_> = suite.iter().map(|s| s.domain()).collect();
    assert_eq!(
      domains,
      vec![
        DiagnosticChoiceDomain::ContestConcede,
        DiagnosticChoiceDomain::FollowReject,
        DiagnosticChoiceDomain::FarmAssist,
        DiagnosticChoiceDomain::RecallTiming,
        DiagnosticChoiceDomain::Sacrifice,
        DiagnosticChoiceDomain::Surprise,
        DiagnosticChoiceDomain::ResponseToFailure,
      ]
    );
    for s in *suite {
      assert_eq!(s.schema(), HELD_OUT_SCENARIO_SCHEMA);
      assert!(!s.description().is_empty());
      assert_eq!(
        u32::from(s.held_out_distribution().primary_count())
          + u32::from(s.held_out_distribution().alternative_count())
          + u32::from(s.held_out_distribution().other_count()),
        100
      );
    }
  }

  // 3. Held-out scenario evaluation report for regularized policies
  let policy_cautious = ParametricPolicyDefinition::cautious_v1();
  let policy_risk = ParametricPolicyDefinition::risk_taking_v1();
  let policy_yielding = ParametricPolicyDefinition::yielding_v1();

  let rep_cautious = HeldOutScenarioEvaluationReport::from_policy(&policy_cautious)
    .expect("cautious held-out evaluation succeeds");
  assert_eq!(rep_cautious.schema(), HELD_OUT_EVALUATION_SCHEMA);
  assert_eq!(rep_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert!(rep_cautious.mean_held_out_loss_bp() <= MAX_ACCEPTABLE_HELD_OUT_LOSS_BP);
  assert!(rep_cautious.modal_accuracy_bp() >= MIN_ACCEPTABLE_MODAL_ACCURACY_BP);
  assert!(rep_cautious.passed_generalization_threshold());

  let rep_risk = HeldOutScenarioEvaluationReport::from_policy(&policy_risk)
    .expect("risk-taking held-out evaluation succeeds");
  assert_eq!(rep_risk.schema(), HELD_OUT_EVALUATION_SCHEMA);
  assert_eq!(rep_risk.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert!(rep_risk.mean_held_out_loss_bp() <= MAX_ACCEPTABLE_HELD_OUT_LOSS_BP);
  assert!(rep_risk.modal_accuracy_bp() >= MIN_ACCEPTABLE_MODAL_ACCURACY_BP);
  assert!(rep_risk.passed_generalization_threshold());

  let rep_yielding = HeldOutScenarioEvaluationReport::from_policy(&policy_yielding)
    .expect("yielding held-out evaluation succeeds");
  assert_eq!(rep_yielding.schema(), HELD_OUT_EVALUATION_SCHEMA);
  assert_eq!(rep_yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert!(rep_yielding.mean_held_out_loss_bp() <= MAX_ACCEPTABLE_HELD_OUT_LOSS_BP);
  assert!(rep_yielding.modal_accuracy_bp() >= MIN_ACCEPTABLE_MODAL_ACCURACY_BP);
  assert!(rep_yielding.passed_generalization_threshold());

  let md_held_out = rep_cautious.to_markdown();
  assert!(md_held_out.contains("# Held-Out Scenario Evaluation Report"));
  assert!(md_held_out.contains("passed_generalization_threshold: true"));

  // 4. Counterfactual Perturbation Catalog
  let perturbations = CounterfactualPerturbationCatalog::all_perturbations();
  assert_eq!(perturbations.len(), 4);
  assert_eq!(perturbations[0].perturbation_id(), CF_THREAT_ESCALATION_ID);
  assert_eq!(perturbations[1].perturbation_id(), CF_ALLIED_RETREAT_ID);
  assert_eq!(perturbations[2].perturbation_id(), CF_HEALTH_ATTRITION_ID);
  assert_eq!(perturbations[3].perturbation_id(), CF_FAVORABLE_OPENING_ID);

  for cf in perturbations {
    assert_eq!(cf.schema(), COUNTERFACTUAL_PERTURBATION_SCHEMA);
    assert_eq!(
      CounterfactualPerturbationCatalog::lookup(cf.perturbation_id()),
      Some(cf)
    );
    assert_eq!(
      CounterfactualPerturbationCatalog::validate_perturbation_id(cf.perturbation_id()),
      Ok(cf)
    );
  }
  assert_eq!(
    CounterfactualPerturbationCatalog::validate_perturbation_id("unknown-cf-id"),
    Err(HeldOutEvaluationError::UnknownPerturbation)
  );

  // 5. Counterfactual Sensitivity Reports
  let cf_cautious = CounterfactualSensitivityReport::from_policy(&policy_cautious)
    .expect("cautious counterfactual sensitivity succeeds");
  assert_eq!(cf_cautious.schema(), COUNTERFACTUAL_SENSITIVITY_SCHEMA);
  assert_eq!(cf_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert!(cf_cautious.all_coherent());
  for ev in cf_cautious.evaluations() {
    assert_eq!(ev.status(), DirectionalCoherenceStatus::Coherent);
  }

  let cf_risk = CounterfactualSensitivityReport::from_policy(&policy_risk)
    .expect("risk counterfactual sensitivity succeeds");
  assert_eq!(cf_risk.schema(), COUNTERFACTUAL_SENSITIVITY_SCHEMA);
  assert_eq!(cf_risk.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert!(cf_risk.all_coherent());
  for ev in cf_risk.evaluations() {
    assert_eq!(ev.status(), DirectionalCoherenceStatus::Coherent);
  }

  let cf_yielding = CounterfactualSensitivityReport::from_policy(&policy_yielding)
    .expect("yielding counterfactual sensitivity succeeds");
  assert_eq!(cf_yielding.schema(), COUNTERFACTUAL_SENSITIVITY_SCHEMA);
  assert_eq!(cf_yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert!(cf_yielding.all_coherent());
  for ev in cf_yielding.evaluations() {
    assert_eq!(ev.status(), DirectionalCoherenceStatus::Coherent);
  }

  let md_cf = cf_cautious.to_markdown();
  assert!(md_cf.contains("# Counterfactual Sensitivity Report"));
  assert!(md_cf.contains("all_coherent: true"));

  // 6. Integrated Calibration Held-Out Report
  let cal_cautious = CalibrationHeldOutReport::from_policy(&policy_cautious)
    .expect("cautious calibration report succeeds");
  assert_eq!(cal_cautious.schema(), CALIBRATION_HELD_OUT_SCHEMA);
  assert_eq!(cal_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert!(cal_cautious.meets_calibration_gate());

  let cal_risk =
    CalibrationHeldOutReport::from_policy(&policy_risk).expect("risk calibration report succeeds");
  assert_eq!(cal_risk.schema(), CALIBRATION_HELD_OUT_SCHEMA);
  assert_eq!(cal_risk.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert!(cal_risk.meets_calibration_gate());

  let cal_yielding = CalibrationHeldOutReport::from_policy(&policy_yielding)
    .expect("yielding calibration report succeeds");
  assert_eq!(cal_yielding.schema(), CALIBRATION_HELD_OUT_SCHEMA);
  assert_eq!(cal_yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert!(cal_yielding.meets_calibration_gate());

  let md_cal = cal_cautious.to_markdown();
  assert!(md_cal.contains("# Calibration Held-Out & Counterfactual Report"));
  assert!(md_cal.contains("meets_calibration_gate: true"));

  // 7. Error handling & boundary checks
  let invalid_dist = DiagnosticChoiceActionDistribution::new(
    CHOICE_CONTEST_CONCEDE_ID,
    CAUTIOUS_SEMANTIC_PROFILE_ID,
    100,
    50,
    50,
    0,
  )
  .expect("valid dist");

  assert_eq!(
    HeldOutScenarioDefinition::new(
      "test-scenario",
      DiagnosticChoiceDomain::ContestConcede,
      CHOICE_FOLLOW_REJECT_ID, // Mismatched choice domain
      invalid_dist,
      LaneIntent::Contest,
      "test description",
    ),
    Err(HeldOutEvaluationError::MismatchedChoice)
  );
}

#[test]
fn test_multi_model_family_comparison() {
  // 1. Cautious profile comparison across model/prompting families
  let rep_cautious = MultiModelComparisonReport::cautious_comparison_v1();
  assert_eq!(rep_cautious.schema(), MULTI_MODEL_COMPARISON_SCHEMA);
  assert_eq!(rep_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(
    rep_cautious.reference_model_prompt_protocol_id(),
    MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
  );
  assert_eq!(
    rep_cautious.alternative_model_prompt_protocol_id(),
    MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID
  );
  assert_eq!(
    rep_cautious.alignment_status(),
    ModelFamilyAlignmentStatus::Aligned
  );
  assert_eq!(rep_cautious.modal_agreement_count(), 7);
  assert!(rep_cautious.mean_action_tvd_bp() <= ALIGNMENT_THRESHOLD_ALIGNED_TVD_BP);
  assert_eq!(rep_cautious.entries().len(), 7);

  for entry in rep_cautious.entries() {
    assert!(entry.modal_agreement());
    assert_eq!(entry.ref_modal_intent(), entry.alt_modal_intent());
    let md_row = entry.to_markdown();
    assert!(md_row.starts_with("| choice-"));
  }

  let md_cautious = rep_cautious.to_markdown();
  assert!(md_cautious.contains("# Multi-Model & Prompting Family Comparison Report"));
  assert!(md_cautious.contains("alignment_status: aligned"));
  assert!(md_cautious.contains("modal_agreement_count: 7/7"));

  // 2. Risk-taking profile comparison across model/prompting families
  let rep_risk = MultiModelComparisonReport::risk_taking_comparison_v1();
  assert_eq!(rep_risk.schema(), MULTI_MODEL_COMPARISON_SCHEMA);
  assert_eq!(rep_risk.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert_eq!(
    rep_risk.reference_model_prompt_protocol_id(),
    MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
  );
  assert_eq!(
    rep_risk.alternative_model_prompt_protocol_id(),
    MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID
  );
  assert_eq!(
    rep_risk.alignment_status(),
    ModelFamilyAlignmentStatus::Aligned
  );
  assert_eq!(rep_risk.modal_agreement_count(), 7);
  assert!(rep_risk.mean_action_tvd_bp() <= ALIGNMENT_THRESHOLD_ALIGNED_TVD_BP);

  // 3. Yielding profile comparison across model/prompting families
  let rep_yielding = MultiModelComparisonReport::yielding_comparison_v1();
  assert_eq!(rep_yielding.schema(), MULTI_MODEL_COMPARISON_SCHEMA);
  assert_eq!(rep_yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert_eq!(
    rep_yielding.reference_model_prompt_protocol_id(),
    MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
  );
  assert_eq!(
    rep_yielding.alternative_model_prompt_protocol_id(),
    MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID
  );
  assert_eq!(
    rep_yielding.alignment_status(),
    ModelFamilyAlignmentStatus::Aligned
  );
  assert_eq!(rep_yielding.modal_agreement_count(), 7);
  assert!(rep_yielding.mean_action_tvd_bp() <= ALIGNMENT_THRESHOLD_ALIGNED_TVD_BP);

  // 4. Identical protocol self-comparison
  let ref_cautious_emp = EmpiricalDistributionEstimateReport::cautious_v1();
  let ref_cautious_pol =
    ParametricPolicyFitter::fit_standard_regularized(&ref_cautious_emp).expect("fit");
  let self_rep = MultiModelComparisonReport::compare(
    &ref_cautious_emp,
    &ref_cautious_emp,
    &ref_cautious_pol,
    &ref_cautious_pol,
  )
  .expect("self comparison");

  assert_eq!(self_rep.mean_action_tvd_bp(), 0);
  assert_eq!(self_rep.mean_communication_tvd_bp(), 0);
  assert_eq!(self_rep.modal_agreement_count(), 7);
  assert_eq!(
    self_rep.alignment_status(),
    ModelFamilyAlignmentStatus::Aligned
  );

  // 5. Error handling and status enum tests
  let ref_risk_emp = EmpiricalDistributionEstimateReport::risk_taking_v1();
  let ref_risk_pol = ParametricPolicyFitter::fit_standard_regularized(&ref_risk_emp).expect("fit");

  assert_eq!(
    MultiModelComparisonReport::compare(
      &ref_cautious_emp,
      &ref_risk_emp,
      &ref_cautious_pol,
      &ref_risk_pol,
    ),
    Err(MultiModelComparisonError::MismatchedProfile)
  );

  assert_eq!(ModelFamilyAlignmentStatus::Aligned.as_str(), "aligned");
  assert_eq!(ModelFamilyAlignmentStatus::Shifted.as_str(), "shifted");
  assert_eq!(ModelFamilyAlignmentStatus::Divergent.as_str(), "divergent");
  assert_eq!(
    ModelFamilyAlignmentStatus::parse("aligned"),
    Some(ModelFamilyAlignmentStatus::Aligned)
  );
  assert_eq!(
    ModelFamilyAlignmentStatus::parse("shifted"),
    Some(ModelFamilyAlignmentStatus::Shifted)
  );
  assert_eq!(
    ModelFamilyAlignmentStatus::parse("divergent"),
    Some(ModelFamilyAlignmentStatus::Divergent)
  );
  assert_eq!(ModelFamilyAlignmentStatus::parse("unknown"), None);
}

#[test]
fn parameter_identifiability_report_evaluates_and_bounds_traits_correctly() {
  assert_eq!(
    PARAMETER_IDENTIFIABILITY_SCHEMA,
    "m7-parameter-identifiability-v1"
  );
  assert_eq!(IDENTIFIABILITY_THRESHOLD_IDENTIFIED_BP, 1_500);
  assert_eq!(IDENTIFIABILITY_THRESHOLD_WEAK_BP, 500);
  assert_eq!(IDENTIFIABILITY_MAX_CONFOUNDING_RISK_BP, 3_000);

  // Status enum and trait dimension string parsing
  assert_eq!(
    SemanticTraitDimension::RiskTolerance.as_str(),
    "risk-tolerance"
  );
  assert_eq!(SemanticTraitDimension::Deference.as_str(), "deference");
  assert_eq!(SemanticTraitDimension::Focus.as_str(), "focus");
  assert_eq!(
    SemanticTraitDimension::CommunicationClarity.as_str(),
    "communication-clarity"
  );
  assert_eq!(
    SemanticTraitDimension::parse("risk-tolerance"),
    Some(SemanticTraitDimension::RiskTolerance)
  );
  assert_eq!(
    SemanticTraitDimension::parse("deference"),
    Some(SemanticTraitDimension::Deference)
  );
  assert_eq!(
    SemanticTraitDimension::parse("focus"),
    Some(SemanticTraitDimension::Focus)
  );
  assert_eq!(
    SemanticTraitDimension::parse("communication-clarity"),
    Some(SemanticTraitDimension::CommunicationClarity)
  );
  assert_eq!(SemanticTraitDimension::parse("unknown"), None);
  assert_eq!(SemanticTraitDimension::all_dimensions().len(), 4);

  assert_eq!(
    ParameterIdentifiabilityStatus::Identifiable.as_str(),
    "identifiable"
  );
  assert_eq!(
    ParameterIdentifiabilityStatus::WeaklyIdentified.as_str(),
    "weakly-identified"
  );
  assert_eq!(
    ParameterIdentifiabilityStatus::Unidentifiable.as_str(),
    "unidentifiable"
  );
  assert_eq!(
    ParameterIdentifiabilityStatus::parse("identifiable"),
    Some(ParameterIdentifiabilityStatus::Identifiable)
  );
  assert_eq!(
    ParameterIdentifiabilityStatus::parse("weakly-identified"),
    Some(ParameterIdentifiabilityStatus::WeaklyIdentified)
  );
  assert_eq!(
    ParameterIdentifiabilityStatus::parse("unidentifiable"),
    Some(ParameterIdentifiabilityStatus::Unidentifiable)
  );
  assert_eq!(ParameterIdentifiabilityStatus::parse("unknown"), None);

  // 1. Cautious identifiability report
  let rep_cautious = ParameterIdentifiabilityReport::cautious_identifiability_v1();
  assert_eq!(rep_cautious.schema(), PARAMETER_IDENTIFIABILITY_SCHEMA);
  assert_eq!(rep_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(rep_cautious.entries().len(), 4);
  assert!(rep_cautious.identifiable_count() >= 3);
  assert_eq!(rep_cautious.unidentifiable_count(), 0);
  assert!(rep_cautious.mean_sensitivity_bp() >= IDENTIFIABILITY_THRESHOLD_IDENTIFIED_BP);

  for entry in rep_cautious.entries() {
    assert!(entry.sensitivity_bp() > 0);
    assert!(entry.confounding_risk_bp() <= IDENTIFIABILITY_MAX_CONFOUNDING_RISK_BP);
    assert!(entry.salient_dilemma_count() > 0);
    assert!(!entry.justification().is_empty());
    let md = entry.to_markdown();
    assert!(md.starts_with("| "));
  }

  let md_cautious = rep_cautious.to_markdown();
  assert!(md_cautious.contains("# Parameter Identifiability Report"));
  assert!(md_cautious.contains("schema: m7-parameter-identifiability-v1"));
  assert!(md_cautious.contains("identifiable_traits:"));

  // 2. Risk-taking identifiability report
  let rep_risk = ParameterIdentifiabilityReport::risk_taking_identifiability_v1();
  assert_eq!(rep_risk.schema(), PARAMETER_IDENTIFIABILITY_SCHEMA);
  assert_eq!(rep_risk.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert!(rep_risk.identifiable_count() >= 3);

  // 3. Yielding identifiability report
  let rep_yielding = ParameterIdentifiabilityReport::yielding_identifiability_v1();
  assert_eq!(rep_yielding.schema(), PARAMETER_IDENTIFIABILITY_SCHEMA);
  assert_eq!(rep_yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert_eq!(rep_yielding.identifiable_count(), 2);
  assert_eq!(rep_yielding.weakly_identified_count(), 2);
  assert_eq!(rep_yielding.unidentifiable_count(), 0);

  // 4. Error handling
  assert_eq!(
    SemanticProfileVocabulary::validate_profile_id("unknown-profile-v1"),
    Err(SemanticProfileVocabularyError::UnknownProfile)
  );
  assert_eq!(ParameterIdentifiabilityStatus::parse("unknown"), None);
  assert_eq!(SemanticTraitDimension::parse("unknown"), None);
}

#[test]
fn semantic_label_stability_report_evaluates_cross_model_shifts_correctly() {
  assert_eq!(
    SEMANTIC_LABEL_STABILITY_SCHEMA,
    "m7-semantic-label-stability-v1"
  );
  assert_eq!(STABILITY_THRESHOLD_STABLE_TVD_BP, 1_000);
  assert_eq!(STABILITY_THRESHOLD_SENSITIVE_TVD_BP, 3_000);

  assert_eq!(SemanticLabelStabilityStatus::Stable.as_str(), "stable");
  assert_eq!(
    SemanticLabelStabilityStatus::Sensitive.as_str(),
    "sensitive"
  );
  assert_eq!(SemanticLabelStabilityStatus::Unstable.as_str(), "unstable");
  assert_eq!(
    SemanticLabelStabilityStatus::parse("stable"),
    Some(SemanticLabelStabilityStatus::Stable)
  );
  assert_eq!(
    SemanticLabelStabilityStatus::parse("sensitive"),
    Some(SemanticLabelStabilityStatus::Sensitive)
  );
  assert_eq!(
    SemanticLabelStabilityStatus::parse("unstable"),
    Some(SemanticLabelStabilityStatus::Unstable)
  );
  assert_eq!(SemanticLabelStabilityStatus::parse("unknown"), None);

  // 1. Cautious stability report
  let rep_cautious = SemanticLabelStabilityReport::cautious_stability_v1();
  assert_eq!(rep_cautious.schema(), SEMANTIC_LABEL_STABILITY_SCHEMA);
  assert_eq!(rep_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(rep_cautious.entries().len(), 4);
  assert_eq!(rep_cautious.stable_count(), 3);
  assert_eq!(rep_cautious.sensitive_count(), 1);
  assert_eq!(rep_cautious.unstable_count(), 0);
  assert!(rep_cautious.mean_stability_score_bp() >= 9_000);

  for entry in rep_cautious.entries() {
    assert!(entry.modal_agreement());
    assert!(entry.cross_model_tvd_bp() <= STABILITY_THRESHOLD_SENSITIVE_TVD_BP);
    assert_eq!(
      entry.stability_score_bp(),
      10_000 - entry.cross_model_tvd_bp()
    );
    let md = entry.to_markdown();
    assert!(md.starts_with("| "));
  }

  let md_cautious = rep_cautious.to_markdown();
  assert!(md_cautious.contains("# Semantic Label Stability Report"));
  assert!(md_cautious.contains("stable_labels: 3/4"));

  // 2. Risk-taking stability report
  let rep_risk = SemanticLabelStabilityReport::risk_taking_stability_v1();
  assert_eq!(rep_risk.schema(), SEMANTIC_LABEL_STABILITY_SCHEMA);
  assert_eq!(rep_risk.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert_eq!(rep_risk.stable_count(), 4);

  // 3. Yielding stability report
  let rep_yielding = SemanticLabelStabilityReport::yielding_stability_v1();
  assert_eq!(rep_yielding.schema(), SEMANTIC_LABEL_STABILITY_SCHEMA);
  assert_eq!(rep_yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert_eq!(rep_yielding.stable_count(), 4);
}

#[test]
fn calibration_uncertainty_report_integrates_identifiability_and_stability() {
  assert_eq!(
    CALIBRATION_UNCERTAINTY_SCHEMA,
    "m7-calibration-uncertainty-v1"
  );
  assert!(CALIBRATION_UNCERTAINTY_DISCLAIMER.contains("reference policy distribution"));
  assert!(CALIBRATION_UNCERTAINTY_DISCLAIMER.contains("not human ground truth"));

  // 1. Cautious calibration uncertainty report
  let rep_cautious = CalibrationUncertaintyReport::cautious_uncertainty_v1();
  assert_eq!(rep_cautious.schema(), CALIBRATION_UNCERTAINTY_SCHEMA);
  assert_eq!(rep_cautious.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(
    rep_cautious.disclaimer(),
    CALIBRATION_UNCERTAINTY_DISCLAIMER
  );
  assert!(!rep_cautious.unidentifiable_parameters_present());
  assert!(!rep_cautious.unstable_labels_present());
  assert!(rep_cautious.overall_uncertainty_score_bp() <= 5_000);

  let md_cautious = rep_cautious.to_markdown();
  assert!(md_cautious.contains("# Calibration Uncertainty Report"));
  assert!(md_cautious.contains("schema: m7-calibration-uncertainty-v1"));
  assert!(md_cautious.contains(CALIBRATION_UNCERTAINTY_DISCLAIMER));
  assert!(md_cautious.contains("# Parameter Identifiability Report"));
  assert!(md_cautious.contains("# Semantic Label Stability Report"));

  // 2. Risk-taking calibration uncertainty report
  let rep_risk = CalibrationUncertaintyReport::risk_taking_uncertainty_v1();
  assert_eq!(rep_risk.schema(), CALIBRATION_UNCERTAINTY_SCHEMA);
  assert_eq!(rep_risk.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert!(!rep_risk.unidentifiable_parameters_present());
  assert!(!rep_risk.unstable_labels_present());

  // 3. Yielding calibration uncertainty report
  let rep_yielding = CalibrationUncertaintyReport::yielding_uncertainty_v1();
  assert_eq!(rep_yielding.schema(), CALIBRATION_UNCERTAINTY_SCHEMA);
  assert_eq!(rep_yielding.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert!(!rep_yielding.unidentifiable_parameters_present());
  assert!(!rep_yielding.unstable_labels_present());

  // 4. Mismatched profile error
  let ident_cautious = ParameterIdentifiabilityReport::cautious_identifiability_v1();
  let stab_risk = SemanticLabelStabilityReport::risk_taking_stability_v1();
  assert_eq!(
    CalibrationUncertaintyReport::evaluate(ident_cautious, stab_risk),
    Err(CalibrationUncertaintyError::MismatchedProfile)
  );
}

#[test]
fn reference_output_records_and_rationales_validate_and_bound_correctly() {
  assert_eq!(REFERENCE_OUTPUT_SCHEMA, "m7-reference-output-v1");
  assert_eq!(
    REFERENCE_OUTPUT_PRESERVATION_SCHEMA,
    "m7-reference-output-preservation-v1"
  );
  assert_eq!(MAX_STRUCTURED_RATIONALE_LEN, 128);

  // 1. StructuredRationaleCategory string mappings
  let categories = StructuredRationaleCategory::all_categories();
  assert_eq!(categories.len(), 6);
  assert_eq!(
    StructuredRationaleCategory::ThreatMitigation.as_str(),
    "threat-mitigation"
  );
  assert_eq!(
    StructuredRationaleCategory::ResourcePreservation.as_str(),
    "resource-preservation"
  );
  assert_eq!(
    StructuredRationaleCategory::ObjectiveContest.as_str(),
    "objective-contest"
  );
  assert_eq!(
    StructuredRationaleCategory::TeamCoordination.as_str(),
    "team-coordination"
  );
  assert_eq!(
    StructuredRationaleCategory::FallbackContingency.as_str(),
    "fallback-contingency"
  );
  assert_eq!(
    StructuredRationaleCategory::PacingAdjustment.as_str(),
    "pacing-adjustment"
  );

  for cat in categories {
    assert_eq!(StructuredRationaleCategory::parse(cat.as_str()), Some(cat));
  }
  assert_eq!(StructuredRationaleCategory::parse("unknown"), None);

  // 2. StructuredRationale bounds and validation
  let valid_rat = StructuredRationale::new(
    StructuredRationaleCategory::ThreatMitigation,
    "Mitigate wave pressure and avoid lethal trade",
  )
  .expect("valid rationale");
  assert_eq!(
    valid_rat.category(),
    StructuredRationaleCategory::ThreatMitigation
  );
  assert_eq!(
    valid_rat.summary(),
    "Mitigate wave pressure and avoid lethal trade"
  );

  assert_eq!(
    StructuredRationale::new(StructuredRationaleCategory::ThreatMitigation, ""),
    Err(ReferenceOutputError::EmptyRationaleSummary)
  );

  let long_summary = "a".repeat(129);
  let static_long: &'static str = Box::leak(long_summary.into_boxed_str());
  assert_eq!(
    StructuredRationale::new(StructuredRationaleCategory::ThreatMitigation, static_long),
    Err(ReferenceOutputError::RationaleSummaryTooLong)
  );

  assert_eq!(
    StructuredRationale::new(
      StructuredRationaleCategory::ThreatMitigation,
      "Invalid\x00summary"
    ),
    Err(ReferenceOutputError::InvalidRationaleSummary)
  );

  // 3. ReferenceOutputRecord validation and fail-closed CoT rejection
  let valid_rec = ReferenceOutputRecord::new(
    CAUTIOUS_SEMANTIC_PROFILE_ID,
    CHOICE_CONTEST_CONCEDE_ID,
    DiagnosticChoiceDomain::ContestConcede,
    "model-family-reference-v1",
    MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
    LaneIntent::Yield,
    LaneTargetFocus::Minions,
    LaneCommitment::Cautious,
    LanePingSignal::Danger,
    Some(valid_rat),
    false,
  )
  .expect("valid reference record");

  assert_eq!(valid_rec.schema(), REFERENCE_OUTPUT_SCHEMA);
  assert_eq!(valid_rec.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(valid_rec.choice_id(), CHOICE_CONTEST_CONCEDE_ID);
  assert_eq!(
    valid_rec.dilemma_domain(),
    DiagnosticChoiceDomain::ContestConcede
  );
  assert_eq!(valid_rec.model_family_id(), "model-family-reference-v1");
  assert_eq!(
    valid_rec.prompt_protocol_id(),
    MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
  );
  assert_eq!(valid_rec.selected_intent(), LaneIntent::Yield);
  assert_eq!(valid_rec.target_focus(), LaneTargetFocus::Minions);
  assert_eq!(valid_rec.commitment(), LaneCommitment::Cautious);
  assert_eq!(valid_rec.ping_signal(), LanePingSignal::Danger);
  assert_eq!(valid_rec.structured_rationale(), Some(valid_rat));
  assert!(!valid_rec.chain_of_thought_present());

  // Fail-closed on private chain-of-thought
  assert_eq!(
    ReferenceOutputRecord::new(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      CHOICE_CONTEST_CONCEDE_ID,
      DiagnosticChoiceDomain::ContestConcede,
      "model-family-reference-v1",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      LaneIntent::Yield,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LanePingSignal::Danger,
      Some(valid_rat),
      true,
    ),
    Err(ReferenceOutputError::PrivateChainOfThoughtForbidden)
  );

  // Fail-closed on unknown profile
  assert_eq!(
    ReferenceOutputRecord::new(
      "unknown-profile-v1",
      CHOICE_CONTEST_CONCEDE_ID,
      DiagnosticChoiceDomain::ContestConcede,
      "model-family-reference-v1",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      LaneIntent::Yield,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LanePingSignal::Danger,
      Some(valid_rat),
      false,
    ),
    Err(ReferenceOutputError::UnknownProfile)
  );

  // Fail-closed on unknown choice
  assert_eq!(
    ReferenceOutputRecord::new(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      "unknown-choice-v1",
      DiagnosticChoiceDomain::ContestConcede,
      "model-family-reference-v1",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      LaneIntent::Yield,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LanePingSignal::Danger,
      Some(valid_rat),
      false,
    ),
    Err(ReferenceOutputError::UnknownChoice)
  );

  // Fail-closed on domain mismatch
  assert_eq!(
    ReferenceOutputRecord::new(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      CHOICE_CONTEST_CONCEDE_ID,
      DiagnosticChoiceDomain::FollowReject,
      "model-family-reference-v1",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      LaneIntent::Yield,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LanePingSignal::Danger,
      Some(valid_rat),
      false,
    ),
    Err(ReferenceOutputError::DomainMismatch)
  );

  // Fail-closed on unknown protocol or mismatched model family
  assert_eq!(
    ReferenceOutputRecord::new(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      CHOICE_CONTEST_CONCEDE_ID,
      DiagnosticChoiceDomain::ContestConcede,
      "model-family-alternative-v1",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      LaneIntent::Yield,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LanePingSignal::Danger,
      Some(valid_rat),
      false,
    ),
    Err(ReferenceOutputError::UnknownProtocol)
  );

  // Error as_str tests
  assert_eq!(
    ReferenceOutputError::UnknownProfile.as_str(),
    "unknown-profile"
  );
  assert_eq!(
    ReferenceOutputError::UnknownChoice.as_str(),
    "unknown-choice"
  );
  assert_eq!(
    ReferenceOutputError::UnknownProtocol.as_str(),
    "unknown-protocol"
  );
  assert_eq!(
    ReferenceOutputError::DomainMismatch.as_str(),
    "domain-mismatch"
  );
  assert_eq!(
    ReferenceOutputError::PrivateChainOfThoughtForbidden.as_str(),
    "private-chain-of-thought-forbidden"
  );
  assert_eq!(
    ReferenceOutputError::EmptyRationaleSummary.as_str(),
    "empty-rationale-summary"
  );
  assert_eq!(
    ReferenceOutputError::RationaleSummaryTooLong.as_str(),
    "rationale-summary-too-long"
  );
  assert_eq!(
    ReferenceOutputError::InvalidRationaleSummary.as_str(),
    "invalid-rationale-summary"
  );
  assert_eq!(
    ReferenceOutputError::InvalidRecordOrder.as_str(),
    "invalid-record-order"
  );
  assert_eq!(
    ReferenceOutputError::DuplicateDomainRecord.as_str(),
    "duplicate-domain-record"
  );
}

#[test]
fn reference_output_preservation_reports_and_catalog_verify_all_canonical_suites() {
  // 1. All 6 canonical suites in the catalog
  let suites = ReferenceOutputCatalog::canonical_reference_suites();
  assert_eq!(suites.len(), 6);

  for suite in &suites {
    assert_eq!(suite.schema(), REFERENCE_OUTPUT_PRESERVATION_SCHEMA);
    assert!(suite.chain_of_thought_free());
    assert_eq!(suite.structured_rationale_count(), 7);
    assert_eq!(suite.records().len(), 7);

    let expected_domains = ReferenceOutputPreservationReport::canonical_domains();
    for (i, rec) in suite.records().iter().enumerate() {
      assert_eq!(rec.schema(), REFERENCE_OUTPUT_SCHEMA);
      assert_eq!(rec.profile_id(), suite.profile_id());
      assert_eq!(rec.model_family_id(), suite.model_family_id());
      assert_eq!(rec.prompt_protocol_id(), suite.prompt_protocol_id());
      assert_eq!(rec.dilemma_domain(), expected_domains[i]);
      assert!(!rec.chain_of_thought_present());
      assert!(rec.structured_rationale().is_some());
    }

    let md = suite.to_markdown();
    assert!(md.contains("# Reference Output Preservation Report"));
    assert!(md.contains("**Schema:** `m7-reference-output-preservation-v1`"));
    assert!(md.contains("**Private Chain-of-Thought Free:** `true`"));
    assert!(md.contains("**Structured Rationales Count:** `7/7`"));
    assert!(md.contains("| Dilemma Domain | Choice ID | Selected Intent | Target Focus | Commitment | Ping Signal | Structured Rationale | CoT Free |"));
    assert!(md.contains("Observable reference outputs preserved without storing or requiring private chain-of-thought."));
  }

  // 2. Individual profile validation
  let cautious_ref = ReferenceOutputPreservationReport::cautious_reference_diagnostic_v1();
  assert_eq!(cautious_ref.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(
    cautious_ref.records()[0].selected_intent(),
    LaneIntent::Yield
  );
  assert_eq!(
    cautious_ref.records()[1].selected_intent(),
    LaneIntent::Stabilize
  );

  let risk_ref = ReferenceOutputPreservationReport::risk_taking_reference_diagnostic_v1();
  assert_eq!(risk_ref.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert_eq!(risk_ref.records()[0].selected_intent(), LaneIntent::Contest);
  assert_eq!(risk_ref.records()[1].selected_intent(), LaneIntent::Contest);

  let yielding_ref = ReferenceOutputPreservationReport::yielding_reference_diagnostic_v1();
  assert_eq!(yielding_ref.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert_eq!(
    yielding_ref.records()[0].selected_intent(),
    LaneIntent::Yield
  );
  assert_eq!(
    yielding_ref.records()[1].selected_intent(),
    LaneIntent::Yield
  );

  // 3. Alternative protocol suites
  let cautious_alt = ReferenceOutputPreservationReport::cautious_alternative_diagnostic_v1();
  assert_eq!(
    cautious_alt.model_family_id(),
    "model-family-alternative-v1"
  );
  assert_eq!(
    cautious_alt.prompt_protocol_id(),
    MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID
  );

  // 4. Catalog lookup
  assert_eq!(
    ReferenceOutputCatalog::find_by_profile_and_protocol(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
    ),
    Some(cautious_ref)
  );
  assert_eq!(
    ReferenceOutputCatalog::find_by_profile_and_protocol(
      "unknown-profile",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
    ),
    None
  );
  assert_eq!(
    ReferenceOutputCatalog::find_by_profile_and_protocol(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      "unknown-protocol"
    ),
    None
  );

  // 5. Custom report constructor validation
  let mut swapped_records = cautious_ref.records();
  swapped_records.swap(0, 1);
  assert_eq!(
    ReferenceOutputPreservationReport::new(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      "model-family-reference-v1",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      swapped_records
    ),
    Err(ReferenceOutputError::InvalidRecordOrder)
  );

  assert_eq!(
    ReferenceOutputPreservationReport::new(
      "unknown-profile-v1",
      "model-family-reference-v1",
      MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID,
      cautious_ref.records()
    ),
    Err(ReferenceOutputError::UnknownProfile)
  );

  assert_eq!(
    ReferenceOutputPreservationReport::new(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      "model-family-reference-v1",
      "unknown-protocol-v1",
      cautious_ref.records()
    ),
    Err(ReferenceOutputError::UnknownProtocol)
  );
}

#[test]
fn recalibration_triggers_and_calibration_model_card_contract_holds() {
  // 1. Schemas, thresholds, and constants
  assert_eq!(RECALIBRATION_TRIGGER_SCHEMA, "m7-recalibration-trigger-v1");
  assert_eq!(
    RECALIBRATION_EVALUATION_SCHEMA,
    "m7-recalibration-evaluation-v1"
  );
  assert_eq!(
    CALIBRATION_MODEL_CARD_SCHEMA,
    "m7-calibration-model-card-v1"
  );
  assert_eq!(DEFAULT_RECALIBRATION_TVD_THRESHOLD_BP, 1_500);
  assert_eq!(DEFAULT_RECALIBRATION_MAX_MODAL_DISAGREEMENTS, 1);
  assert_eq!(DEFAULT_RECALIBRATION_HELD_OUT_LOSS_MAX_BP, 2_500);
  assert_eq!(DEFAULT_RECALIBRATION_HELD_OUT_ACCURACY_MIN_BP, 7_000);
  assert!(
    RECALIBRATION_DISCLAIMER
      .contains("AI-agent behavior serves solely as a reference policy distribution")
  );

  // 2. Trigger reasons enum and parsing
  let all_reasons = RecalibrationTriggerReason::all_reasons();
  assert_eq!(all_reasons.len(), 9);
  for reason in all_reasons {
    let s = reason.as_str();
    assert_eq!(RecalibrationTriggerReason::parse(s), Some(reason));
  }
  assert_eq!(RecalibrationTriggerReason::parse("unknown-reason"), None);

  // 3. Urgency enum and parsing
  for urgency in [
    RecalibrationUrgency::Immediate,
    RecalibrationUrgency::Scheduled,
    RecalibrationUrgency::None,
  ] {
    let s = urgency.as_str();
    assert_eq!(RecalibrationUrgency::parse(s), Some(urgency));
  }
  assert_eq!(RecalibrationUrgency::parse("unknown-urgency"), None);

  // 4. Trigger condition construction & validation
  let cond = RecalibrationTriggerCondition::new(
    RecalibrationTriggerReason::TotalVariationDistanceBreach,
    RecalibrationUrgency::Immediate,
    "Distribution drift detected",
    Some(2_100),
    Some(1_500),
  )
  .expect("valid condition");
  assert_eq!(
    cond.reason(),
    RecalibrationTriggerReason::TotalVariationDistanceBreach
  );
  assert_eq!(cond.urgency(), RecalibrationUrgency::Immediate);
  assert_eq!(cond.detail(), "Distribution drift detected");
  assert_eq!(cond.metric_value_bp(), Some(2_100));
  assert_eq!(cond.threshold_bp(), Some(1_500));

  // Condition validation errors
  assert_eq!(
    RecalibrationTriggerCondition::new(
      RecalibrationTriggerReason::ModelVersionChanged,
      RecalibrationUrgency::Scheduled,
      "",
      None,
      None,
    ),
    Err(RecalibrationError::InvalidConditionDetail)
  );
  assert_eq!(
    RecalibrationTriggerCondition::new(
      RecalibrationTriggerReason::TotalVariationDistanceBreach,
      RecalibrationUrgency::Immediate,
      "Valid detail",
      Some(10_001),
      Some(1_500),
    ),
    Err(RecalibrationError::InvalidThreshold)
  );

  // 5. Policy construction & validation
  let default_policy = RecalibrationPolicy::default();
  assert_eq!(default_policy.schema(), RECALIBRATION_TRIGGER_SCHEMA);
  assert_eq!(default_policy.tvd_threshold_bp(), 1_500);
  assert_eq!(default_policy.max_modal_disagreements(), 1);
  assert_eq!(default_policy.max_held_out_loss_bp(), 2_500);
  assert_eq!(default_policy.min_held_out_accuracy_bp(), 7_000);

  let custom_policy =
    RecalibrationPolicy::new(1_200, 0, 2_000, 8_000).expect("valid custom policy");
  assert_eq!(custom_policy.tvd_threshold_bp(), 1_200);
  assert_eq!(custom_policy.max_modal_disagreements(), 0);

  assert_eq!(
    RecalibrationPolicy::new(10_001, 1, 2_500, 7_000),
    Err(RecalibrationError::InvalidThreshold)
  );
  assert_eq!(
    RecalibrationPolicy::new(1_500, 8, 2_500, 7_000),
    Err(RecalibrationError::InvalidThreshold)
  );

  // 6. Baseline evaluations
  let cautious_eval = RecalibrationEvaluationReport::cautious_baseline_v1();
  assert_eq!(cautious_eval.profile_id(), CAUTIOUS_SEMANTIC_PROFILE_ID);
  assert_eq!(
    cautious_eval.reference_model_family(),
    "model-family-reference-v1"
  );
  assert_eq!(
    cautious_eval.candidate_model_family(),
    "model-family-alternative-v1"
  );
  assert_eq!(
    cautious_eval.reference_prompt_protocol(),
    MODEL_PROMPT_REFERENCE_DIAGNOSTIC_ID
  );
  assert_eq!(
    cautious_eval.candidate_prompt_protocol(),
    MODEL_PROMPT_ALTERNATIVE_DIAGNOSTIC_ID
  );
  assert!(cautious_eval.is_recalibration_required());
  // Because prompt protocol & model family differ between reference and alternative diagnostic suites, scheduled recalibration is active
  assert_eq!(cautious_eval.urgency(), RecalibrationUrgency::Scheduled);
  assert!(
    cautious_eval
      .active_triggers()
      .iter()
      .any(|t| t.reason() == RecalibrationTriggerReason::ModelVersionChanged)
  );
  assert!(
    cautious_eval
      .active_triggers()
      .iter()
      .any(|t| t.reason() == RecalibrationTriggerReason::PromptProtocolChanged)
  );

  let risk_eval = RecalibrationEvaluationReport::risk_taking_baseline_v1();
  assert_eq!(risk_eval.profile_id(), RISK_TAKING_SEMANTIC_PROFILE_ID);
  assert_eq!(risk_eval.urgency(), RecalibrationUrgency::Scheduled);

  let yielding_eval = RecalibrationEvaluationReport::yielding_baseline_v1();
  assert_eq!(yielding_eval.profile_id(), YIELDING_SEMANTIC_PROFILE_ID);
  assert_eq!(yielding_eval.urgency(), RecalibrationUrgency::Scheduled);

  // Markdown rendering of evaluation report
  let md = cautious_eval.to_markdown();
  assert!(md.contains("# Recalibration Trigger Evaluation Report — `cautious-laner-semantic-v1`"));
  assert!(md.contains("**Recalibration Urgency:** `scheduled`"));
  assert!(md.contains("| `model-version-changed` | `scheduled` |"));
  assert!(md.contains(RECALIBRATION_DISCLAIMER));

  // 7. Policy evaluation error paths
  let comp = MultiModelComparisonReport::cautious_comparison_v1();
  let unc = CalibrationUncertaintyReport::cautious_uncertainty_v1();
  let param_pol = ParametricPolicyDefinition::cautious_v1();
  let ho = CalibrationHeldOutReport::from_policy(&param_pol).expect("valid held out");
  let pres = ReferenceOutputPreservationReport::cautious_reference_diagnostic_v1();

  assert_eq!(
    default_policy.evaluate("unknown-profile-v1", &comp, &unc, &ho, Some(&pres)),
    Err(RecalibrationError::UnknownProfile)
  );

  let risk_comp = MultiModelComparisonReport::risk_taking_comparison_v1();
  assert_eq!(
    default_policy.evaluate(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      &risk_comp,
      &unc,
      &ho,
      Some(&pres)
    ),
    Err(RecalibrationError::MismatchedProfile)
  );

  let risk_unc = CalibrationUncertaintyReport::risk_taking_uncertainty_v1();
  assert_eq!(
    default_policy.evaluate(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      &comp,
      &risk_unc,
      &ho,
      Some(&pres)
    ),
    Err(RecalibrationError::MismatchedProfile)
  );

  let risk_pres = ReferenceOutputPreservationReport::risk_taking_reference_diagnostic_v1();
  assert_eq!(
    default_policy.evaluate(
      CAUTIOUS_SEMANTIC_PROFILE_ID,
      &comp,
      &unc,
      &ho,
      Some(&risk_pres)
    ),
    Err(RecalibrationError::MismatchedProfile)
  );

  // Strict policy triggering immediate recalibration on modal disagreement or TVD breach
  let strict_policy = RecalibrationPolicy::new(100, 0, 100, 9_999).expect("strict policy");
  let strict_eval = strict_policy
    .evaluate(CAUTIOUS_SEMANTIC_PROFILE_ID, &comp, &unc, &ho, Some(&pres))
    .expect("strict evaluation succeeds");
  assert_eq!(strict_eval.urgency(), RecalibrationUrgency::Immediate);
  assert!(strict_eval.is_recalibration_required());
  assert!(
    strict_eval
      .active_triggers()
      .iter()
      .any(|t| t.reason() == RecalibrationTriggerReason::TotalVariationDistanceBreach)
  );
  assert!(
    strict_eval
      .active_triggers()
      .iter()
      .any(|t| t.reason() == RecalibrationTriggerReason::HeldOutLossBreach)
  );

  // 8. Calibration Model Card
  let model_card = CalibrationModelCardReport::canonical_m7();
  assert_eq!(model_card.schema(), CALIBRATION_MODEL_CARD_SCHEMA);
  assert_eq!(
    model_card.title(),
    "Fog of Intent M7 Semantic-to-Parametric Calibration Model Card"
  );
  assert_eq!(model_card.profiles_evaluated().len(), 3);
  assert!(
    model_card
      .intended_use()
      .contains("parametric policy proxies")
  );
  assert!(
    model_card
      .evidence_limits()
      .contains("not represent human ground truth")
  );
  assert!(
    model_card
      .held_out_generalization_status()
      .contains("<= 25.00% mean TVD loss")
  );
  assert!(
    model_card
      .uncertainty_and_identifiability_status()
      .contains("identifiability")
  );
  assert!(
    model_card
      .recalibration_policy_summary()
      .contains("Deterministic recalibration triggers monitor")
  );
  assert!(
    model_card
      .chain_of_thought_policy()
      .contains("Zero private chain-of-thought")
  );

  let card_md = model_card.to_markdown();
  assert!(card_md.contains("# Fog of Intent M7 Semantic-to-Parametric Calibration Model Card"));
  assert!(card_md.contains("## Held-Out Generalization Status"));
  assert!(card_md.contains("## Uncertainty and Identifiability Findings"));
  assert!(card_md.contains("## Recalibration Trigger Policy"));
  assert!(card_md.contains("## Observability and Chain-of-Thought Policy"));
  assert!(card_md.contains("## Evidence and Claim Limits"));
}

#[test]
fn team_communication_speech_acts_and_vocabularies_are_canonical_and_parseable() {
  // 1. Schema constants
  assert_eq!(TEAM_COMMUNICATION_SCHEMA, "m8-team-communication-v1");
  assert_eq!(TEAM_SPEECH_ACT_SCHEMA, "m8-team-speech-act-v1");
  assert_eq!(TEAM_MESSAGE_ENVELOPE_SCHEMA, "m8-team-message-envelope-v1");

  // 2. Speech Acts: all 8 variants
  let speech_acts = TeamSpeechAct::all();
  assert_eq!(speech_acts.len(), 8);
  for act in speech_acts {
    let label = act.as_str();
    assert_eq!(TeamSpeechAct::parse(label), Some(act));
  }
  assert_eq!(TeamSpeechAct::parse("invalid-act"), None);

  // 3. Recipients
  assert_eq!(TeamRecipient::Broadcast.as_str(), "broadcast");
  assert_eq!(
    TeamRecipient::parse("broadcast"),
    Some(TeamRecipient::Broadcast)
  );
  assert_eq!(TeamRecipient::parse("all"), Some(TeamRecipient::Broadcast));

  let direct_human = TeamRecipient::Direct(LaneActorRole::HumanLaner);
  assert_eq!(direct_human.as_str(), "direct:human-laner");
  assert_eq!(
    TeamRecipient::parse("direct:human-laner"),
    Some(direct_human)
  );
  assert!(direct_human.matches_recipient(LaneActorRole::HumanLaner));
  assert!(!direct_human.matches_recipient(LaneActorRole::AlliedAutonomous));
  assert!(TeamRecipient::Broadcast.matches_recipient(LaneActorRole::HumanLaner));
  assert!(TeamRecipient::Broadcast.matches_recipient(LaneActorRole::OpposingLaner));
  assert_eq!(TeamRecipient::parse("direct:unknown-actor"), None);

  // 4. Urgency
  let urgencies = TeamMessageUrgency::all();
  assert_eq!(urgencies.len(), 3);
  for u in urgencies {
    let label = u.as_str();
    assert_eq!(TeamMessageUrgency::parse(label), Some(u));
  }
  assert_eq!(TeamMessageUrgency::parse("super-critical"), None);

  // 5. Confidence
  let confidences = TeamConfidenceLevel::all();
  assert_eq!(confidences.len(), 3);
  for c in confidences {
    let label = c.as_str();
    assert_eq!(TeamConfidenceLevel::parse(label), Some(c));
  }
  assert_eq!(TeamConfidenceLevel::parse("unsure"), None);

  // 6. Conditions
  let conditions = TeamMessageCondition::all();
  assert_eq!(conditions.len(), 5);
  for cond in conditions {
    let label = cond.as_str();
    assert_eq!(TeamMessageCondition::parse(label), Some(cond));
  }
  assert_eq!(TeamMessageCondition::parse("weather-clear"), None);

  // 7. Visibility
  assert_eq!(TeamMessageVisibility::TeamOnly.as_str(), "team-only");
  assert_eq!(
    TeamMessageVisibility::parse("team-only"),
    Some(TeamMessageVisibility::TeamOnly)
  );
  assert_eq!(TeamMessageVisibility::DirectOnly.as_str(), "direct-only");
  assert_eq!(
    TeamMessageVisibility::parse("direct-only"),
    Some(TeamMessageVisibility::DirectOnly)
  );
  assert_eq!(TeamMessageVisibility::Public.as_str(), "public");
  assert_eq!(
    TeamMessageVisibility::parse("public"),
    Some(TeamMessageVisibility::Public)
  );
  assert_eq!(TeamMessageVisibility::parse("secret"), None);

  // Visibility checks
  let sender = LaneActorRole::HumanLaner;
  let ally = LaneActorRole::AlliedAutonomous;
  let opponent = LaneActorRole::OpposingLaner;
  let recipient_ally = TeamRecipient::Direct(ally);

  // Public: visible to all
  assert!(TeamMessageVisibility::Public.is_visible_to(sender, sender, recipient_ally, true));
  assert!(TeamMessageVisibility::Public.is_visible_to(ally, sender, recipient_ally, true));
  assert!(TeamMessageVisibility::Public.is_visible_to(opponent, sender, recipient_ally, false));

  // TeamOnly: visible to teammates, hidden from opponents
  assert!(TeamMessageVisibility::TeamOnly.is_visible_to(sender, sender, recipient_ally, true));
  assert!(TeamMessageVisibility::TeamOnly.is_visible_to(ally, sender, recipient_ally, true));
  assert!(!TeamMessageVisibility::TeamOnly.is_visible_to(opponent, sender, recipient_ally, false));

  // DirectOnly: visible only to sender and recipient
  assert!(TeamMessageVisibility::DirectOnly.is_visible_to(sender, sender, recipient_ally, true));
  assert!(TeamMessageVisibility::DirectOnly.is_visible_to(ally, sender, recipient_ally, true));
  assert!(!TeamMessageVisibility::DirectOnly.is_visible_to(
    opponent,
    sender,
    recipient_ally,
    false
  ));
}

#[test]
fn team_message_envelope_validation_and_rejection_paths() {
  let valid = TeamMessageEnvelope::new(
    "msg-test-valid-v1",
    LaneActorRole::AlliedAutonomous,
    TeamRecipient::Direct(LaneActorRole::HumanLaner),
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::ThreatAbsent,
    TeamMessageVisibility::TeamOnly,
    1,
    "Valid proposal test message.",
  );

  assert_eq!(valid.validate(), Ok(()));
  assert_eq!(valid.schema(), TEAM_MESSAGE_ENVELOPE_SCHEMA);
  assert_eq!(valid.message_id(), "msg-test-valid-v1");
  assert_eq!(valid.sender(), LaneActorRole::AlliedAutonomous);
  assert_eq!(
    valid.recipient(),
    TeamRecipient::Direct(LaneActorRole::HumanLaner)
  );
  assert_eq!(valid.speech_act(), TeamSpeechAct::Proposal);
  assert_eq!(valid.proposed_intent(), Some(LaneIntent::Contest));
  assert_eq!(valid.urgency(), TeamMessageUrgency::Standard);
  assert_eq!(valid.confidence(), TeamConfidenceLevel::Confident);
  assert_eq!(valid.condition(), TeamMessageCondition::ThreatAbsent);
  assert_eq!(valid.visibility(), TeamMessageVisibility::TeamOnly);
  assert_eq!(valid.turn(), 1);
  assert_eq!(valid.content_summary(), "Valid proposal test message.");
  assert!(!valid.chain_of_thought_present());

  // Markdown formatting
  let md = valid.format_markdown();
  assert!(md.contains("### Message `msg-test-valid-v1`"));
  assert!(md.contains("**Sender:** allied-autonomous"));
  assert!(md.contains("**Recipient:** direct:human-laner"));
  assert!(md.contains("**Speech Act:** proposal"));
  assert!(md.contains("**Proposed Intent:** contest"));
  assert!(md.contains("**Chain-of-Thought Free:** true"));

  // Fail-closed rejection: Chain-of-Thought present
  let cot_invalid = TeamMessageEnvelope::with_chain_of_thought_flag(
    "msg-test-cot-v1",
    LaneActorRole::AlliedAutonomous,
    TeamRecipient::Direct(LaneActorRole::HumanLaner),
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::ThreatAbsent,
    TeamMessageVisibility::TeamOnly,
    1,
    "CoT test",
    true,
  );
  assert_eq!(
    cot_invalid.validate(),
    Err(TeamCommunicationError::ChainOfThoughtForbidden)
  );

  // Fail-closed rejection: Empty message ID
  let empty_id = TeamMessageEnvelope::new(
    "",
    LaneActorRole::AlliedAutonomous,
    TeamRecipient::Direct(LaneActorRole::HumanLaner),
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::ThreatAbsent,
    TeamMessageVisibility::TeamOnly,
    1,
    "Empty ID test",
  );
  assert_eq!(
    empty_id.validate(),
    Err(TeamCommunicationError::EmptyMessageId)
  );

  // Fail-closed rejection: Self-addressing direct recipient
  let self_addressed = TeamMessageEnvelope::new(
    "msg-self-addressed-v1",
    LaneActorRole::HumanLaner,
    TeamRecipient::Direct(LaneActorRole::HumanLaner),
    TeamSpeechAct::Clarification,
    None,
    TeamMessageUrgency::Low,
    TeamConfidenceLevel::Tentative,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::DirectOnly,
    1,
    "Self addressing test",
  );
  assert_eq!(
    self_addressed.validate(),
    Err(TeamCommunicationError::SelfAddressingForbidden)
  );
}

#[test]
fn team_communication_catalog_is_complete_and_validates_all_speech_acts() {
  let envelopes = TeamCommunicationCatalog::all_envelopes();
  assert_eq!(envelopes.len(), 8);

  // Every canonical envelope must validate cleanly
  for env in envelopes {
    assert_eq!(
      env.validate(),
      Ok(()),
      "envelope {} must validate",
      env.message_id()
    );
    assert_eq!(
      TeamCommunicationCatalog::lookup(env.message_id()),
      Some(env),
      "lookup must match envelope {}",
      env.message_id()
    );
    assert_eq!(
      TeamCommunicationCatalog::validate_message_id(env.message_id()),
      Ok(env)
    );
  }

  // Check that all 8 speech acts are covered by the catalog
  let speech_acts_represented = envelopes
    .iter()
    .map(|env| env.speech_act())
    .collect::<Vec<_>>();
  for act in TeamSpeechAct::all() {
    assert!(
      speech_acts_represented.contains(&act),
      "speech act {:?} must be present in canonical catalog",
      act
    );
  }

  // Unknown ID lookup fails closed
  assert_eq!(
    TeamCommunicationCatalog::lookup("non-existent-msg-id"),
    None
  );
  assert_eq!(
    TeamCommunicationCatalog::validate_message_id("non-existent-msg-id"),
    Err(TeamCommunicationError::UnknownEnvelopeId)
  );
}

#[test]
fn team_dialogue_statuses_reasons_and_condition_evaluators_are_canonical() {
  // 1. Schema
  assert_eq!(TEAM_DIALOGUE_SCHEMA, "m8-team-dialogue-v1");

  // 2. Dialogue Statuses
  let statuses = TeamDialogueStatus::all();
  assert_eq!(statuses.len(), 8);
  for status in statuses {
    let label = status.as_str();
    assert_eq!(TeamDialogueStatus::parse(label), Some(status));
  }
  assert_eq!(TeamDialogueStatus::parse("invalid-status"), None);
  assert!(!TeamDialogueStatus::Idle.is_terminal());
  assert!(!TeamDialogueStatus::Proposed.is_terminal());
  assert!(!TeamDialogueStatus::Clarifying.is_terminal());
  assert!(!TeamDialogueStatus::Negotiating.is_terminal());
  assert!(TeamDialogueStatus::Agreed.is_terminal());
  assert!(TeamDialogueStatus::Diverged.is_terminal());
  assert!(TeamDialogueStatus::Aborted.is_terminal());
  assert!(TeamDialogueStatus::Failed.is_terminal());

  // 3. Dissent Reasons
  let reasons = TeamDissentReason::all();
  assert_eq!(reasons.len(), 6);
  for reason in reasons {
    let label = reason.as_str();
    assert_eq!(TeamDissentReason::parse(label), Some(reason));
  }
  assert_eq!(TeamDissentReason::parse("invalid-reason"), None);

  // 4. Condition Evaluator
  assert!(TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::Unconditional,
    1,
    true,
    false,
    0
  ));
  assert!(TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::HealthAboveThreshold,
    3,
    true,
    false,
    0
  ));
  assert!(!TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::HealthAboveThreshold,
    2,
    false,
    true,
    5
  ));
  assert!(TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::ThreatAbsent,
    5,
    false,
    false,
    0
  ));
  assert!(!TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::ThreatAbsent,
    5,
    true,
    true,
    5
  ));
  assert!(TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::AlliedPresence,
    1,
    true,
    true,
    0
  ));
  assert!(!TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::AlliedPresence,
    5,
    false,
    false,
    5
  ));
  assert!(TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::ResourceSufficient,
    1,
    true,
    false,
    2
  ));
  assert!(!TeamConditionEvaluator::is_condition_satisfied(
    TeamMessageCondition::ResourceSufficient,
    5,
    false,
    true,
    1
  ));
}

#[test]
fn team_speech_act_profile_evaluation_is_posture_consistent() {
  // Cautious: dissents under threat, dissents on low health, counters contest when wave is low
  let cautious = TeamSpeechActProfile::Cautious;
  assert_eq!(
    cautious.evaluate_proposal(LaneIntent::Contest, 5, true, 3, 3),
    TeamEvaluationOutcome::Dissent(TeamDissentReason::ThreatDetected)
  );
  assert_eq!(
    cautious.evaluate_proposal(LaneIntent::Contest, 2, false, 3, 3),
    TeamEvaluationOutcome::Dissent(TeamDissentReason::LowHealth)
  );
  assert_eq!(
    cautious.evaluate_proposal(LaneIntent::Contest, 5, false, 1, 3),
    TeamEvaluationOutcome::Counter(LaneIntent::Stabilize)
  );
  assert_eq!(
    cautious.evaluate_proposal(LaneIntent::Contest, 5, false, 3, 3),
    TeamEvaluationOutcome::Conditional(LaneIntent::Contest, TeamMessageCondition::ThreatAbsent)
  );
  assert_eq!(
    cautious.evaluate_proposal(LaneIntent::Stabilize, 5, false, 3, 3),
    TeamEvaluationOutcome::Accept(LaneIntent::Stabilize)
  );

  // RiskTaking: counters stabilize when healthy, accepts contest
  let risk_taking = TeamSpeechActProfile::RiskTaking;
  assert_eq!(
    risk_taking.evaluate_proposal(LaneIntent::Stabilize, 5, false, 2, 3),
    TeamEvaluationOutcome::Counter(LaneIntent::Contest)
  );
  assert_eq!(
    risk_taking.evaluate_proposal(LaneIntent::Contest, 4, true, 3, 2),
    TeamEvaluationOutcome::Accept(LaneIntent::Contest)
  );
  assert_eq!(
    risk_taking.evaluate_proposal(LaneIntent::Contest, 1, false, 3, 2),
    TeamEvaluationOutcome::Dissent(TeamDissentReason::LowHealth)
  );

  // Yielding: defers readily, conditional on presence under threat
  let yielding = TeamSpeechActProfile::Yielding;
  assert_eq!(
    yielding.evaluate_proposal(LaneIntent::Contest, 4, false, 2, 2),
    TeamEvaluationOutcome::Accept(LaneIntent::Contest)
  );
  assert_eq!(
    yielding.evaluate_proposal(LaneIntent::Contest, 4, true, 2, 2),
    TeamEvaluationOutcome::Conditional(LaneIntent::Contest, TeamMessageCondition::AlliedPresence)
  );
  assert_eq!(
    yielding.evaluate_proposal(LaneIntent::Stabilize, 1, false, 2, 2),
    TeamEvaluationOutcome::Dissent(TeamDissentReason::LowHealth)
  );
}

#[test]
fn team_dialogue_session_state_transitions_and_catalog_validation() {
  // Test all canonical dialogues from catalog
  let dialogues = TeamDialogueCatalog::all_dialogues();
  assert_eq!(dialogues.len(), 7);

  // 1. Agreed Contest
  let agreed = TeamDialogueCatalog::dialogue_agreed_contest_v1().unwrap();
  assert_eq!(agreed.status(), TeamDialogueStatus::Agreed);
  assert_eq!(agreed.active_intent(), Some(LaneIntent::Contest));
  assert_eq!(agreed.history().len(), 2);
  assert_eq!(agreed.schema(), TEAM_DIALOGUE_SCHEMA);
  assert_eq!(agreed.session_id(), "dialogue-agreed-contest-v1");
  assert_eq!(agreed.round(), 0);

  // Markdown format test
  let md = agreed.format_markdown();
  assert!(md.contains("## Team Dialogue `dialogue-agreed-contest-v1`"));
  assert!(md.contains("**Status:** agreed"));
  assert!(md.contains("**Active Intent:** contest"));
  assert!(md.contains("1. [proposal]"));
  assert!(md.contains("2. [confirmation]"));

  // 2. Dissent Threat
  let dissent = TeamDialogueCatalog::dialogue_dissent_threat_v1().unwrap();
  assert_eq!(dissent.status(), TeamDialogueStatus::Diverged);
  assert_eq!(dissent.history().len(), 2);

  // 3. Counter Negotiation
  let counter = TeamDialogueCatalog::dialogue_counter_negotiation_v1().unwrap();
  assert_eq!(counter.status(), TeamDialogueStatus::Agreed);
  assert_eq!(counter.active_intent(), Some(LaneIntent::Stabilize));
  assert_eq!(counter.round(), 1);
  assert_eq!(counter.history().len(), 3);

  // 4. Clarification
  let clarification = TeamDialogueCatalog::dialogue_clarification_v1().unwrap();
  assert_eq!(clarification.status(), TeamDialogueStatus::Agreed);
  assert_eq!(clarification.history().len(), 3);

  // 5. Conditional Commitment
  let cond = TeamDialogueCatalog::dialogue_conditional_commitment_v1().unwrap();
  assert_eq!(cond.status(), TeamDialogueStatus::Agreed);
  assert_eq!(cond.round(), 1);
  assert_eq!(cond.history().len(), 3);

  // 6. Withdrawal
  let withdrawal = TeamDialogueCatalog::dialogue_withdrawal_v1().unwrap();
  assert_eq!(withdrawal.status(), TeamDialogueStatus::Aborted);
  assert_eq!(withdrawal.history().len(), 3);

  // 7. Failure Recovery
  let failure = TeamDialogueCatalog::dialogue_failure_recovery_v1().unwrap();
  assert_eq!(failure.status(), TeamDialogueStatus::Failed);
  assert_eq!(failure.history().len(), 3);

  // Lookup validation
  for d in &dialogues {
    assert_eq!(TeamDialogueCatalog::lookup(d.session_id()), Some(d.clone()));
    assert_eq!(
      TeamDialogueCatalog::validate_dialogue(d.session_id()),
      Ok(d.clone())
    );
  }
  assert_eq!(TeamDialogueCatalog::lookup("non-existent-dialogue"), None);
  assert_eq!(
    TeamDialogueCatalog::validate_dialogue("non-existent-dialogue"),
    Err(TeamCommunicationError::UnknownDialogueId)
  );
}

#[test]
fn team_dialogue_session_fail_closed_rejections_and_invariants() {
  let mut session = TeamDialogueSession::new(
    "dialogue-test-invariants-v1",
    LaneActorRole::HumanLaner,
    LaneActorRole::AlliedAutonomous,
    1,
  );

  // 1. Invalid start with confirmation
  let confirm = TeamCommunicationCatalog::confirmation_contest_v1();
  assert_eq!(
    session.step(confirm),
    Err(TeamCommunicationError::InvalidTransition)
  );

  // 2. Actor mismatch: message from OpposingLaner
  let opposing_msg = TeamMessageEnvelope::new(
    "msg-opposing-leak-v1",
    LaneActorRole::OpposingLaner,
    TeamRecipient::Direct(LaneActorRole::HumanLaner),
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::Public,
    1,
    "Opponent message",
  );
  assert_eq!(
    session.step(opposing_msg),
    Err(TeamCommunicationError::ActorMismatch)
  );

  // 3. Step valid proposal
  let prop = TeamMessageEnvelope::new(
    "msg-test-prop-v1",
    LaneActorRole::HumanLaner,
    TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Valid proposal",
  );
  assert_eq!(session.step(prop), Ok(TeamDialogueStatus::Proposed));

  // 4. Multiple counter-proposals exceeding MAX_DIALOGUE_ROUNDS (4)
  let mut neg_session = session.clone();
  for i in 0..4 {
    let counter_ally = TeamMessageEnvelope::new(
      "msg-counter-ally-v1",
      LaneActorRole::AlliedAutonomous,
      TeamRecipient::Direct(LaneActorRole::HumanLaner),
      TeamSpeechAct::CounterProposal,
      Some(if i % 2 == 0 {
        LaneIntent::Stabilize
      } else {
        LaneIntent::Contest
      }),
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      TeamMessageVisibility::TeamOnly,
      1,
      "Counter back and forth",
    );
    assert_eq!(
      neg_session.step(counter_ally),
      Ok(TeamDialogueStatus::Negotiating)
    );
  }
  assert_eq!(neg_session.round(), 4);
  // 5th round exceeds limit
  let counter_over = TeamMessageEnvelope::new(
    "msg-counter-over-v1",
    LaneActorRole::HumanLaner,
    TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
    TeamSpeechAct::CounterProposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Over limit counter",
  );
  assert_eq!(
    neg_session.step(counter_over),
    Err(TeamCommunicationError::MaxRoundsExceeded)
  );

  // 5. Session already closed rejection
  let mut closed_session = TeamDialogueCatalog::dialogue_dissent_threat_v1().unwrap();
  assert_eq!(closed_session.status(), TeamDialogueStatus::Diverged);
  let post_close_msg = TeamMessageEnvelope::new(
    "msg-post-close-v1",
    LaneActorRole::AlliedAutonomous,
    TeamRecipient::Direct(LaneActorRole::HumanLaner),
    TeamSpeechAct::Confirmation,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Post close attempt",
  );
  assert_eq!(
    closed_session.step(post_close_msg),
    Err(TeamCommunicationError::SessionAlreadyClosed)
  );
}

#[test]
fn team_plan_and_individual_plan_schemas_and_objective_enums() {
  assert_eq!(TEAM_PLAN_SCHEMA, "m8-team-plan-v1");
  assert_eq!(INDIVIDUAL_PLAN_SCHEMA, "m8-individual-plan-v1");
  assert_eq!(
    TEAM_PLAN_RELATIONSHIP_SCHEMA,
    "m8-team-plan-relationship-v1"
  );

  // Objectives
  let objectives = TeamStrategicObjective::all();
  assert_eq!(objectives.len(), 6);
  for obj in &objectives {
    assert_eq!(TeamStrategicObjective::parse(obj.as_str()), Some(*obj));
    assert_eq!(format!("{}", obj), obj.as_str());
  }
  assert_eq!(TeamStrategicObjective::parse("unknown-obj"), None);

  // Phases
  let phases = TeamPlanPhase::all();
  assert_eq!(phases.len(), 4);
  for phase in &phases {
    assert_eq!(TeamPlanPhase::parse(phase.as_str()), Some(*phase));
    assert_eq!(format!("{}", phase), phase.as_str());
  }
  assert_eq!(TeamPlanPhase::parse("unknown-phase"), None);

  // Alignment Types
  let alignments = [
    TeamPlanAlignmentType::Aligned,
    TeamPlanAlignmentType::Divergent,
    TeamPlanAlignmentType::ConditionalCompliance,
    TeamPlanAlignmentType::Independent,
    TeamPlanAlignmentType::Conflicted,
  ];
  for align in &alignments {
    assert_eq!(TeamPlanAlignmentType::parse(align.as_str()), Some(*align));
    assert_eq!(format!("{}", align), align.as_str());
  }
  assert_eq!(TeamPlanAlignmentType::parse("unknown-align"), None);

  // Errors
  let errors = [
    TeamPlanError::SchemaMismatch,
    TeamPlanError::InvalidPlanId,
    TeamPlanError::EmptyPlanAssignments,
    TeamPlanError::DuplicateRoleAssignment,
    TeamPlanError::ChainOfThoughtPresent,
  ];
  for err in &errors {
    assert_eq!(format!("{}", err), err.as_str());
  }
}

#[test]
fn team_plan_catalog_and_definition_validation() {
  let plans = TeamPlanCatalog::all();
  assert_eq!(plans.len(), 6);

  for plan in plans {
    assert_eq!(plan.validate(), Ok(()));
    assert_eq!(TeamPlanCatalog::lookup(plan.plan_id), Some(plan));
    assert!(!plan.chain_of_thought_present);
    assert!(!plan.plan_id.is_empty());
    assert!(!plan.assignments.is_empty());
  }

  assert_eq!(TeamPlanCatalog::lookup("non-existent-plan"), None);

  let gank_plans: Vec<_> =
    TeamPlanCatalog::all_for_objective(TeamStrategicObjective::GankSetup).collect();
  assert_eq!(gank_plans.len(), 1);
  assert_eq!(gank_plans[0].plan_id, "plan-gank-setup-v1");

  // Invariant error cases
  let mut invalid_cot_plan = plans[0];
  invalid_cot_plan.chain_of_thought_present = true;
  assert_eq!(
    invalid_cot_plan.validate(),
    Err(TeamPlanError::ChainOfThoughtPresent)
  );

  let mut invalid_empty_id_plan = plans[0];
  invalid_empty_id_plan.plan_id = "";
  assert_eq!(
    invalid_empty_id_plan.validate(),
    Err(TeamPlanError::InvalidPlanId)
  );

  let mut invalid_empty_assign_plan = plans[0];
  invalid_empty_assign_plan.assignments = &[];
  assert_eq!(
    invalid_empty_assign_plan.validate(),
    Err(TeamPlanError::EmptyPlanAssignments)
  );

  static DUPLICATE_ASSIGNMENTS: [RolePlanAssignment; 2] = [
    RolePlanAssignment::new(
      LaneActorRole::HumanLaner,
      LaneIntent::Contest,
      LaneTargetFocus::OpposingLaner,
      LaneCommitment::Standard,
      LaneFallbackBehavior::RetreatToTower,
    ),
    RolePlanAssignment::new(
      LaneActorRole::HumanLaner,
      LaneIntent::Stabilize,
      LaneTargetFocus::Minions,
      LaneCommitment::Cautious,
      LaneFallbackBehavior::MaintainPlan,
    ),
  ];
  let mut duplicate_plan = plans[0];
  duplicate_plan.assignments = &DUPLICATE_ASSIGNMENTS;
  assert_eq!(
    duplicate_plan.validate(),
    Err(TeamPlanError::DuplicateRoleAssignment)
  );
}

#[test]
fn individual_plan_validation_and_chain_of_thought_rejection() {
  let valid_plan = IndividualPlanDefinition {
    plan_id: "plan-indiv-human-contest-v1",
    actor: LaneActorRole::HumanLaner,
    selected_intent: LaneIntent::Contest,
    target_focus: LaneTargetFocus::OpposingLaner,
    commitment: LaneCommitment::Standard,
    abort_condition: LaneAbortCondition::HealthThreshold,
    fallback_behavior: LaneFallbackBehavior::RetreatToTower,
    ping_signal: LanePingSignal::Assist,
    chain_of_thought_present: false,
  };
  assert_eq!(valid_plan.validate(), Ok(()));

  let mut invalid_cot_plan = valid_plan;
  invalid_cot_plan.chain_of_thought_present = true;
  assert_eq!(
    invalid_cot_plan.validate(),
    Err(TeamPlanError::ChainOfThoughtPresent)
  );

  let mut invalid_id_plan = valid_plan;
  invalid_id_plan.plan_id = "";
  assert_eq!(
    invalid_id_plan.validate(),
    Err(TeamPlanError::InvalidPlanId)
  );
}

#[test]
fn team_plan_evaluator_evaluates_aligned_and_divergent_individual_plans() {
  let gank_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1").unwrap();

  // 1. Aligned human plan
  let aligned_human_plan = IndividualPlanDefinition {
    plan_id: "indiv-human-gank-aligned",
    actor: LaneActorRole::HumanLaner,
    selected_intent: LaneIntent::Contest,
    target_focus: LaneTargetFocus::OpposingLaner,
    commitment: LaneCommitment::Standard,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::RetreatToTower,
    ping_signal: LanePingSignal::OnMyWay,
    chain_of_thought_present: false,
  };
  let eval = TeamPlanEvaluator::evaluate_alignment(gank_plan, &aligned_human_plan, None).unwrap();
  assert_eq!(eval.alignment_type, TeamPlanAlignmentType::Aligned);
  assert!(eval.intent_match);
  assert!(eval.focus_match);
  assert!(eval.condition_satisfied);
  assert_eq!(eval.dissent_reason, None);

  // 2. Divergent human plan (selecting Recall instead of Contest)
  let divergent_human_plan = IndividualPlanDefinition {
    plan_id: "indiv-human-gank-divergent",
    actor: LaneActorRole::HumanLaner,
    selected_intent: LaneIntent::Recall,
    target_focus: LaneTargetFocus::Minions,
    commitment: LaneCommitment::Cautious,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::RetreatToTower,
    ping_signal: LanePingSignal::Danger,
    chain_of_thought_present: false,
  };
  let eval_div =
    TeamPlanEvaluator::evaluate_alignment(gank_plan, &divergent_human_plan, None).unwrap();
  assert_eq!(eval_div.alignment_type, TeamPlanAlignmentType::Divergent);
  assert!(!eval_div.intent_match);
  assert_eq!(
    eval_div.dissent_reason,
    Some(TeamDissentReason::PostureIncompatible)
  );

  // 3. Independent actor (OpposingLaner not in team plan assignments)
  let independent_plan = IndividualPlanDefinition {
    plan_id: "indiv-opponent-plan",
    actor: LaneActorRole::OpposingLaner,
    selected_intent: LaneIntent::Contest,
    target_focus: LaneTargetFocus::Minions,
    commitment: LaneCommitment::Standard,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::MaintainPlan,
    ping_signal: LanePingSignal::None,
    chain_of_thought_present: false,
  };
  let eval_indep =
    TeamPlanEvaluator::evaluate_alignment(gank_plan, &independent_plan, None).unwrap();
  assert_eq!(
    eval_indep.alignment_type,
    TeamPlanAlignmentType::Independent
  );
  assert!(!eval_indep.intent_match);
}

#[test]
fn team_plan_evaluator_evaluates_conditional_compliance_with_observation() {
  let siege_plan = TeamPlanCatalog::lookup("plan-lane-siege-v1").unwrap();
  // Siege plan requires HealthAboveThreshold (>= 3 hp)

  let human_plan = IndividualPlanDefinition {
    plan_id: "indiv-human-siege",
    actor: LaneActorRole::HumanLaner,
    selected_intent: LaneIntent::Contest,
    target_focus: LaneTargetFocus::Minions,
    commitment: LaneCommitment::Standard,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::MaintainPlan,
    ping_signal: LanePingSignal::Assist,
    chain_of_thought_present: false,
  };

  // High health observation (initial = 8 hp >= 3) -> Condition satisfied -> Aligned
  let high_state = LaneSnapshot::initial();
  let high_health_obs = observe_player(&high_state, ObservationId::new(901)).observation();
  let eval_high =
    TeamPlanEvaluator::evaluate_alignment(siege_plan, &human_plan, Some(&high_health_obs)).unwrap();
  assert_eq!(eval_high.alignment_type, TeamPlanAlignmentType::Aligned);
  assert!(eval_high.condition_satisfied);

  // Low health observation (2 hp < 3) -> Condition not satisfied -> ConditionalCompliance
  let low_state = LaneSnapshot::new(
    high_state.ruleset(),
    high_state.turn(),
    high_state.status(),
    PlayerLaneState::new(
      PLAYER_LANER,
      LaneHealth::new(2).unwrap(),
      high_state.player().resources(),
      high_state.player().position(),
    ),
    high_state.opponent(),
    high_state.wave(),
    high_state.jungle_threat(),
  );
  let low_health_obs = observe_player(&low_state, ObservationId::new(902)).observation();
  let eval_low =
    TeamPlanEvaluator::evaluate_alignment(siege_plan, &human_plan, Some(&low_health_obs)).unwrap();
  assert_eq!(
    eval_low.alignment_type,
    TeamPlanAlignmentType::ConditionalCompliance
  );
  assert!(!eval_low.condition_satisfied);
}

#[test]
fn team_plan_alignment_report_and_cohesion_scoring() {
  let gank_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1").unwrap();

  let human_plan = IndividualPlanDefinition {
    plan_id: "indiv-human-gank-v1",
    actor: LaneActorRole::HumanLaner,
    selected_intent: LaneIntent::Contest,
    target_focus: LaneTargetFocus::OpposingLaner,
    commitment: LaneCommitment::Standard,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::RetreatToTower,
    ping_signal: LanePingSignal::OnMyWay,
    chain_of_thought_present: false,
  };

  let ally_aligned_plan = IndividualPlanDefinition {
    plan_id: "indiv-ally-gank-v1",
    actor: LaneActorRole::AlliedAutonomous,
    selected_intent: LaneIntent::Contest,
    target_focus: LaneTargetFocus::OpposingLaner,
    commitment: LaneCommitment::Standard,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::RetreatToTower,
    ping_signal: LanePingSignal::Assist,
    chain_of_thought_present: false,
  };

  let ally_divergent_plan = IndividualPlanDefinition {
    plan_id: "indiv-ally-gank-div-v1",
    actor: LaneActorRole::AlliedAutonomous,
    selected_intent: LaneIntent::Stabilize,
    target_focus: LaneTargetFocus::Minions,
    commitment: LaneCommitment::Cautious,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::RetreatToTower,
    ping_signal: LanePingSignal::Danger,
    chain_of_thought_present: false,
  };

  // Case 1: 100% aligned team (2 out of 2 = 10,000 bp)
  let report_aligned =
    TeamPlanEvaluator::evaluate_team_alignment(gank_plan, &[human_plan, ally_aligned_plan], None)
      .unwrap();
  assert_eq!(
    report_aligned.overall_alignment,
    TeamPlanAlignmentType::Aligned
  );
  assert_eq!(report_aligned.aligned_actors_count, 2);
  assert_eq!(report_aligned.divergent_actors_count, 0);
  assert_eq!(report_aligned.cohesion_score_bp, 10_000);
  assert_eq!(report_aligned.schema, TEAM_PLAN_RELATIONSHIP_SCHEMA);

  // Markdown rendering
  let md = report_aligned.render_markdown();
  assert!(md.contains("# Team Plan Alignment Report"));
  assert!(md.contains("- **Schema:** `m8-team-plan-relationship-v1`"));
  assert!(md.contains("- **Team Plan ID:** `plan-gank-setup-v1`"));
  assert!(md.contains("- **Strategic Objective:** `gank-setup`"));
  assert!(md.contains("- **Overall Alignment:** `aligned`"));
  assert!(md.contains("- **Cohesion Score:** 10000 bp (2 aligned, 0 divergent)"));
  assert!(md.contains("| HumanLaner | `aligned` | yes | yes | satisfied | `none` |"));
  assert!(md.contains("| AlliedAutonomous | `aligned` | yes | yes | satisfied | `none` |"));

  // Case 2: 50% aligned team (1 out of 2 = 5,000 bp)
  let report_mixed =
    TeamPlanEvaluator::evaluate_team_alignment(gank_plan, &[human_plan, ally_divergent_plan], None)
      .unwrap();
  assert_eq!(
    report_mixed.overall_alignment,
    TeamPlanAlignmentType::ConditionalCompliance
  );
  assert_eq!(report_mixed.aligned_actors_count, 1);
  assert_eq!(report_mixed.divergent_actors_count, 1);
  assert_eq!(report_mixed.cohesion_score_bp, 5_000);
}

#[test]
fn test_team_trust_level_classification_and_labels() {
  assert_eq!(
    TeamTrustLevel::from_basis_points(10_000),
    TeamTrustLevel::HighTrust
  );
  assert_eq!(
    TeamTrustLevel::from_basis_points(7_500),
    TeamTrustLevel::HighTrust
  );
  assert_eq!(
    TeamTrustLevel::from_basis_points(7_499),
    TeamTrustLevel::StandardTrust
  );
  assert_eq!(
    TeamTrustLevel::from_basis_points(4_000),
    TeamTrustLevel::StandardTrust
  );
  assert_eq!(
    TeamTrustLevel::from_basis_points(3_999),
    TeamTrustLevel::LowTrust
  );
  assert_eq!(
    TeamTrustLevel::from_basis_points(1_500),
    TeamTrustLevel::LowTrust
  );
  assert_eq!(
    TeamTrustLevel::from_basis_points(1_499),
    TeamTrustLevel::Distrusted
  );
  assert_eq!(
    TeamTrustLevel::from_basis_points(0),
    TeamTrustLevel::Distrusted
  );

  for level in TeamTrustLevel::all() {
    let label = level.as_str();
    assert_eq!(TeamTrustLevel::parse(label), Some(level));
    assert_eq!(format!("{}", level), label);
  }
  assert_eq!(TeamTrustLevel::parse("unknown"), None);
}

#[test]
fn test_call_outcome_labels_and_parsing() {
  for outcome in CallOutcome::all() {
    let label = outcome.as_str();
    assert_eq!(CallOutcome::parse(label), Some(outcome));
    assert_eq!(format!("{}", outcome), label);
  }
  assert_eq!(CallOutcome::parse("invalid"), None);
}

#[test]
fn test_caller_reputation_record_updates_and_thresholds() {
  let mut record = CallerReputationRecord::new(LaneActorRole::HumanLaner);
  assert_eq!(record.caller, LaneActorRole::HumanLaner);
  assert_eq!(record.reputation_bp, 5_000);
  assert_eq!(record.trust_level(), TeamTrustLevel::StandardTrust);
  assert_eq!(record.total_calls(), 0);
  assert_eq!(record.success_rate_bp(), 5_000);
  assert_eq!(record.compliance_threshold_bp(), 5_000);
  assert!(record.validate().is_ok());

  // Record 3 successes: 5,000 -> 6,000 -> 7,000 -> 8,000
  record.record_outcome(CallOutcome::SuccessfulExecution);
  assert_eq!(record.reputation_bp, 6_000);
  record.record_outcome(CallOutcome::SuccessfulExecution);
  assert_eq!(record.reputation_bp, 7_000);
  record.record_outcome(CallOutcome::SuccessfulExecution);
  assert_eq!(record.reputation_bp, 8_000);
  assert_eq!(record.trust_level(), TeamTrustLevel::HighTrust);
  assert_eq!(record.compliance_threshold_bp(), 2_000);
  assert_eq!(record.total_calls(), 3);
  assert_eq!(record.successful_calls, 3);
  assert_eq!(record.success_rate_bp(), 10_000);

  // Record 4 failures: 8,000 -> 6,500 -> 5,000 -> 3,500 -> 2,000
  record.record_outcome(CallOutcome::FailedExecution);
  assert_eq!(record.reputation_bp, 6_500);
  record.record_outcome(CallOutcome::FailedExecution);
  assert_eq!(record.reputation_bp, 5_000);
  record.record_outcome(CallOutcome::FailedExecution);
  assert_eq!(record.reputation_bp, 3_500);
  assert_eq!(record.trust_level(), TeamTrustLevel::LowTrust);
  assert_eq!(record.compliance_threshold_bp(), 8_000);
  record.record_outcome(CallOutcome::FailedExecution);
  assert_eq!(record.reputation_bp, 2_000);

  // Record abandoned call: 2,000 -> 1,500 -> 1,000
  record.record_outcome(CallOutcome::AbandonedCall);
  assert_eq!(record.reputation_bp, 1_500);
  record.record_outcome(CallOutcome::AbandonedCall);
  assert_eq!(record.reputation_bp, 1_000);
  assert_eq!(record.trust_level(), TeamTrustLevel::Distrusted);
  assert_eq!(record.compliance_threshold_bp(), 10_000);

  // Error cases
  assert_eq!(
    CallerReputationRecord::with_reputation(LaneActorRole::HumanLaner, 10_001),
    Err(TeamTrustError::ReputationOutOfBounds {
      reputation_bp: 10_001,
      max: 10_000,
    })
  );

  let mut invalid_cot = record;
  invalid_cot.chain_of_thought_present = true;
  assert_eq!(
    invalid_cot.validate(),
    Err(TeamTrustError::ChainOfThoughtPresent)
  );
}

#[test]
fn test_team_trust_matrix_multi_role_tracking() {
  let mut matrix = TeamTrustMatrix::new();
  assert!(matrix.validate().is_ok());
  assert_eq!(matrix.allied_average_reputation_bp(), 5_000);

  matrix.record_outcome(LaneActorRole::HumanLaner, CallOutcome::SuccessfulExecution);
  assert_eq!(matrix.get(LaneActorRole::HumanLaner).reputation_bp, 6_000);
  assert_eq!(matrix.allied_average_reputation_bp(), 5_500);

  matrix.record_outcome(
    LaneActorRole::AlliedAutonomous,
    CallOutcome::FailedExecution,
  );
  assert_eq!(
    matrix.get(LaneActorRole::AlliedAutonomous).reputation_bp,
    3_500
  );
  assert_eq!(matrix.allied_average_reputation_bp(), 4_750);
}

#[test]
fn test_communication_clarity_and_transmission_delay() {
  for clarity in CommunicationClarity::all() {
    let label = clarity.as_str();
    assert_eq!(CommunicationClarity::parse(label), Some(clarity));
    assert_eq!(format!("{}", clarity), label);
  }
  assert_eq!(CommunicationClarity::parse("noise"), None);
  assert_eq!(CommunicationClarity::Crisp.clarity_modifier_bp(), 10_000);
  assert_eq!(CommunicationClarity::Ambiguous.clarity_modifier_bp(), 7_000);
  assert_eq!(CommunicationClarity::Degraded.clarity_modifier_bp(), 4_000);
  assert_eq!(CommunicationClarity::Garbled.clarity_modifier_bp(), 1_000);
  assert!(CommunicationClarity::Crisp.is_intelligible());
  assert!(CommunicationClarity::Ambiguous.is_intelligible());
  assert!(CommunicationClarity::Degraded.is_intelligible());
  assert!(!CommunicationClarity::Garbled.is_intelligible());

  for delay in TransmissionDelay::all() {
    let label = delay.as_str();
    assert_eq!(TransmissionDelay::parse(label), Some(delay));
    assert_eq!(format!("{}", delay), label);
  }
  assert_eq!(TransmissionDelay::parse("infinite"), None);
  assert_eq!(TransmissionDelay::Immediate.delay_beats(), 0);
  assert_eq!(TransmissionDelay::OneBeat.delay_beats(), 1);
  assert_eq!(TransmissionDelay::TwoBeats.delay_beats(), 2);
}

#[test]
fn test_delivery_status_and_packet_lifecycle() {
  let delivered = DeliveryStatus::Delivered;
  assert_eq!(delivered.as_str(), "delivered");
  assert!(delivered.is_delivered());
  assert!(!delivered.is_dropped_or_suppressed());

  let delayed = DeliveryStatus::Delayed { remaining_beats: 2 };
  assert_eq!(delayed.as_str(), "delayed");
  assert!(!delayed.is_delivered());
  assert!(!delayed.is_dropped_or_suppressed());
  assert_eq!(format!("{}", delayed), "delayed(2 beats remaining)");

  let missing = DeliveryStatus::DroppedMissing;
  assert_eq!(missing.as_str(), "dropped-missing");
  assert!(!missing.is_delivered());
  assert!(missing.is_dropped_or_suppressed());

  let overload = DeliveryStatus::DroppedOverload;
  assert_eq!(overload.as_str(), "dropped-overload");
  assert!(overload.is_dropped_or_suppressed());

  let suppressed = DeliveryStatus::SuppressedDistrusted;
  assert_eq!(suppressed.as_str(), "suppressed-distrusted");
  assert!(suppressed.is_dropped_or_suppressed());
}

#[test]
fn test_team_communication_channel_queue_and_delivery() {
  let mut channel = TeamCommunicationChannel::new(3);
  let mut trust_matrix = TeamTrustMatrix::new();

  let envelope_immediate = TeamMessageEnvelope::new(
    "msg-001",
    LaneActorRole::HumanLaner,
    TeamRecipient::Broadcast,
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Definite,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Initiate lane contest",
  );

  // 1. Immediate transmission
  let status1 = channel
    .enqueue(
      envelope_immediate,
      CommunicationClarity::Crisp,
      TransmissionDelay::Immediate,
      &trust_matrix,
    )
    .unwrap();
  assert_eq!(status1, DeliveryStatus::Delivered);
  assert_eq!(channel.active_packet_count(), 1);
  assert_eq!(channel.total_delivered, 1);

  // 2. Delayed transmission (1 beat)
  let envelope_delayed = TeamMessageEnvelope::new(
    "msg-002",
    LaneActorRole::HumanLaner,
    TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::DirectOnly,
    1,
    "Incoming gank support",
  );
  let status2 = channel
    .enqueue(
      envelope_delayed,
      CommunicationClarity::Crisp,
      TransmissionDelay::OneBeat,
      &trust_matrix,
    )
    .unwrap();
  assert_eq!(status2, DeliveryStatus::Delayed { remaining_beats: 1 });
  assert_eq!(channel.total_delivered, 1);

  // Recipient sees only msg-001
  let visible_before = channel.drain_delivered_for_recipient(LaneActorRole::AlliedAutonomous, true);
  assert_eq!(visible_before.len(), 1);
  assert_eq!(visible_before[0].message_id(), "msg-001");

  // Advance turn beat
  channel.tick_turn();
  assert_eq!(channel.current_turn(), 2);
  assert_eq!(channel.total_delivered, 2);

  // Recipient now sees msg-001 and msg-002
  let visible_after = channel.drain_delivered_for_recipient(LaneActorRole::AlliedAutonomous, true);
  assert_eq!(visible_after.len(), 2);
  assert_eq!(visible_after[1].message_id(), "msg-002");

  // Opposing laner sees 0 messages due to visibility rules
  let visible_opposing = channel.drain_delivered_for_recipient(LaneActorRole::OpposingLaner, false);
  assert_eq!(visible_opposing.len(), 0);

  // 3. Garbled non-critical message is dropped missing
  let envelope_garbled = TeamMessageEnvelope::new(
    "msg-003",
    LaneActorRole::HumanLaner,
    TeamRecipient::Broadcast,
    TeamSpeechAct::Clarification,
    None,
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Tentative,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Unclear signal",
  );
  let status3 = channel
    .enqueue(
      envelope_garbled,
      CommunicationClarity::Garbled,
      TransmissionDelay::Immediate,
      &trust_matrix,
    )
    .unwrap();
  assert_eq!(status3, DeliveryStatus::DroppedMissing);
  assert_eq!(channel.total_dropped, 1);

  // 4. Overload capacity drop (capacity = 3, buffer has 3 packets)
  let envelope_overload = TeamMessageEnvelope::new(
    "msg-004",
    LaneActorRole::HumanLaner,
    TeamRecipient::Broadcast,
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Critical,
    TeamConfidenceLevel::Definite,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Critical push",
  );
  let status4 = channel
    .enqueue(
      envelope_overload,
      CommunicationClarity::Crisp,
      TransmissionDelay::Immediate,
      &trust_matrix,
    )
    .unwrap();
  assert_eq!(status4, DeliveryStatus::DroppedOverload);
  assert_eq!(channel.total_dropped, 2);

  // 5. Suppressed distrusted sender on Low-urgency message
  let mut distrusted_channel = TeamCommunicationChannel::new(5);
  trust_matrix
    .get_mut(LaneActorRole::HumanLaner)
    .reputation_bp = 1_000;
  let envelope_low_urgency = TeamMessageEnvelope::new(
    "msg-005",
    LaneActorRole::HumanLaner,
    TeamRecipient::Broadcast,
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Stabilize),
    TeamMessageUrgency::Low,
    TeamConfidenceLevel::Tentative,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Optional farm",
  );
  let status5 = distrusted_channel
    .enqueue(
      envelope_low_urgency,
      CommunicationClarity::Crisp,
      TransmissionDelay::Immediate,
      &trust_matrix,
    )
    .unwrap();
  assert_eq!(status5, DeliveryStatus::SuppressedDistrusted);
  assert_eq!(distrusted_channel.total_suppressed, 1);
}

#[test]
fn test_team_trust_evaluator_deterministic_compliance() {
  let initial = LaneSnapshot::initial();
  let safe_obs = observe_player(&initial, ObservationId::new(999)).observation();
  let threat_state = LaneSnapshot::new(
    initial.ruleset(),
    initial.turn(),
    LaneStatus::Open,
    initial.player(),
    initial.opponent(),
    initial.wave(),
    JungleThreatTruth::RiverSide,
  );
  let threat_obs = observe_player(&threat_state, ObservationId::new(1000)).observation();

  let proposal = TeamMessageEnvelope::new(
    "prop-001",
    LaneActorRole::HumanLaner,
    TeamRecipient::Broadcast,
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::ThreatAbsent,
    TeamMessageVisibility::TeamOnly,
    1,
    "Allied gank engagement",
  );

  // 1. High-trust caller -> Comply
  let high_caller = TeamTrustCatalog::HIGH_TRUST_CALLER;
  let report_high = TeamTrustEvaluator::evaluate_proposal(
    &proposal,
    &high_caller,
    CommunicationClarity::Crisp,
    &safe_obs,
    LaneActorRole::AlliedAutonomous,
  )
  .unwrap();
  assert_eq!(report_high.decision, TrustComplianceDecision::Comply);
  assert_eq!(report_high.caller_trust_level, TeamTrustLevel::HighTrust);
  assert!(report_high.validate().is_ok());

  // Markdown rendering check
  let md = report_high.render_markdown();
  assert!(md.contains("### Trust Evaluation Report"));
  assert!(md.contains("- **Caller Reputation**: 8500 bp (high-trust)"));
  assert!(md.contains("- **Decision**: comply"));

  // 2. Standard-trust caller with Degraded clarity -> Clarify
  let std_caller = TeamTrustCatalog::STANDARD_TRUST_CALLER;
  let report_std_degraded = TeamTrustEvaluator::evaluate_proposal(
    &proposal,
    &std_caller,
    CommunicationClarity::Degraded,
    &safe_obs,
    LaneActorRole::AlliedAutonomous,
  )
  .unwrap();
  assert_eq!(
    report_std_degraded.decision,
    TrustComplianceDecision::Clarify
  );

  // 3. Low-trust caller with Ambiguous clarity -> Clarify
  let low_caller = TeamTrustCatalog::LOW_TRUST_CALLER;
  let report_low_ambiguous = TeamTrustEvaluator::evaluate_proposal(
    &proposal,
    &low_caller,
    CommunicationClarity::Ambiguous,
    &safe_obs,
    LaneActorRole::AlliedAutonomous,
  )
  .unwrap();
  assert_eq!(
    report_low_ambiguous.decision,
    TrustComplianceDecision::Clarify
  );

  // 4. Low-trust caller with Crisp clarity and satisfied condition -> Comply
  let report_low_crisp = TeamTrustEvaluator::evaluate_proposal(
    &proposal,
    &low_caller,
    CommunicationClarity::Crisp,
    &safe_obs,
    LaneActorRole::AlliedAutonomous,
  )
  .unwrap();
  assert_eq!(report_low_crisp.decision, TrustComplianceDecision::Comply);

  // 5. Distrusted caller -> Dissent(PostureIncompatible)
  let distrusted_caller = TeamTrustCatalog::DISTRUSTED_CALLER;
  let report_distrusted = TeamTrustEvaluator::evaluate_proposal(
    &proposal,
    &distrusted_caller,
    CommunicationClarity::Crisp,
    &safe_obs,
    LaneActorRole::AlliedAutonomous,
  )
  .unwrap();
  assert_eq!(
    report_distrusted.decision,
    TrustComplianceDecision::Dissent(TeamDissentReason::PostureIncompatible)
  );

  // 6. Prerequisite condition failure on standard trust -> Dissent(ThreatDetected)
  let report_unmet = TeamTrustEvaluator::evaluate_proposal(
    &proposal,
    &std_caller,
    CommunicationClarity::Crisp,
    &threat_obs,
    LaneActorRole::AlliedAutonomous,
  )
  .unwrap();
  assert_eq!(
    report_unmet.decision,
    TrustComplianceDecision::Dissent(TeamDissentReason::ThreatDetected)
  );
}

#[test]
fn test_team_trust_catalog_and_reference_profiles() {
  assert_eq!(
    TeamTrustCatalog::get_reference_caller("high-trust-caller"),
    Some(TeamTrustCatalog::HIGH_TRUST_CALLER)
  );
  assert_eq!(
    TeamTrustCatalog::get_reference_caller("standard-trust-caller"),
    Some(TeamTrustCatalog::STANDARD_TRUST_CALLER)
  );
  assert_eq!(
    TeamTrustCatalog::get_reference_caller("low-trust-caller"),
    Some(TeamTrustCatalog::LOW_TRUST_CALLER)
  );
  assert_eq!(
    TeamTrustCatalog::get_reference_caller("distrusted-caller"),
    Some(TeamTrustCatalog::DISTRUSTED_CALLER)
  );
  assert_eq!(TeamTrustCatalog::get_reference_caller("non-existent"), None);
}

#[test]
fn test_leadership_enums_and_display() {
  // ConsensusRule
  for rule in ConsensusRule::all() {
    assert_eq!(ConsensusRule::parse(rule.as_str()), Some(rule));
    assert_eq!(format!("{}", rule), rule.as_str());
  }
  assert_eq!(ConsensusRule::parse("invalid"), None);

  // FallbackLeadershipMode
  for mode in FallbackLeadershipMode::all() {
    assert_eq!(FallbackLeadershipMode::parse(mode.as_str()), Some(mode));
    assert_eq!(format!("{}", mode), mode.as_str());
  }
  assert_eq!(FallbackLeadershipMode::parse("invalid"), None);

  // LeadershipStructure
  let designated = LeadershipStructure::DesignatedShotCaller {
    caller: LaneActorRole::HumanLaner,
    fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
  };
  assert_eq!(designated.as_str(), "designated-shot-caller");
  assert_eq!(designated.caller_role(), Some(LaneActorRole::HumanLaner));

  let decentralized = LeadershipStructure::Decentralized {
    consensus_rule: ConsensusRule::UnanimousConsensus,
    min_cohesion_bp: 10_000,
  };
  assert_eq!(decentralized.as_str(), "decentralized-coordination");
  assert_eq!(decentralized.caller_role(), None);

  let shared = LeadershipStructure::SharedLeadership {
    primary_caller: LaneActorRole::HumanLaner,
    secondary_caller: LaneActorRole::AlliedAutonomous,
    fallback_mode: FallbackLeadershipMode::FallbackToSecondaryCaller,
  };
  assert_eq!(shared.as_str(), "shared-leadership");
  assert_eq!(shared.caller_role(), Some(LaneActorRole::HumanLaner));

  // LeadershipResolutionOutcome
  let res_consensus = LeadershipResolutionOutcome::ConsensusAchieved {
    agreed_plan_id: "plan-gank-setup-v1",
    objective: TeamStrategicObjective::GankSetup,
    cohesion_bp: 10_000,
    compliance_bp: 10_000,
  };
  assert_eq!(res_consensus.as_str(), "consensus-achieved");
  assert_eq!(res_consensus.compliance_bp(), 10_000);
  assert!(res_consensus.is_consensus());

  let res_split = LeadershipResolutionOutcome::SplitDecision {
    primary_plan_id: "plan-lane-siege-v1",
    objective: TeamStrategicObjective::LaneSiege,
    dissenting_roles: vec![(
      LaneActorRole::AlliedAutonomous,
      TeamDissentReason::LowHealth,
    )],
    compliance_bp: 5_000,
  };
  assert_eq!(res_split.as_str(), "split-decision");
  assert_eq!(res_split.compliance_bp(), 5_000);
  assert!(!res_split.is_consensus());

  let res_fallback = LeadershipResolutionOutcome::FallbackIndividualPlans {
    trigger_reason: TeamDissentReason::ThreatDetected,
    compliance_bp: 0,
  };
  assert_eq!(res_fallback.as_str(), "fallback-individual-plans");
  assert_eq!(res_fallback.compliance_bp(), 0);
  assert!(!res_fallback.is_consensus());

  let res_deadlock = LeadershipResolutionOutcome::ConflictedDeadlock {
    colliding_objectives: vec![
      TeamStrategicObjective::LaneSiege,
      TeamStrategicObjective::ObjectiveContest,
    ],
    compliance_bp: 0,
  };
  assert_eq!(res_deadlock.as_str(), "conflicted-deadlock");
  assert_eq!(res_deadlock.compliance_bp(), 0);
  assert!(!res_deadlock.is_consensus());
}

#[test]
fn test_shot_caller_directive_and_policy() {
  let directive = ShotCallerDirective::new(
    LaneActorRole::HumanLaner,
    "plan-lane-siege-v1",
    TeamStrategicObjective::LaneSiege,
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::HealthAboveThreshold,
    "Siege tower while healthy.",
  )
  .unwrap();

  assert_eq!(directive.schema, SHOT_CALLER_POLICY_SCHEMA);
  assert_eq!(directive.caller, LaneActorRole::HumanLaner);
  assert_eq!(directive.plan_id, "plan-lane-siege-v1");

  let envelope = directive.to_message_envelope(1).unwrap();
  assert_eq!(envelope.sender(), LaneActorRole::HumanLaner);
  assert_eq!(envelope.recipient(), TeamRecipient::Broadcast);
  assert_eq!(envelope.speech_act(), TeamSpeechAct::Proposal);
  assert_eq!(envelope.proposed_intent(), Some(LaneIntent::Contest));

  // Fail-closed on empty or unknown plan
  assert_eq!(
    ShotCallerDirective::new(
      LaneActorRole::HumanLaner,
      "",
      TeamStrategicObjective::LaneSiege,
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      "Rationale",
    ),
    Err(TeamLeadershipError::EmptyPlanId)
  );
  assert_eq!(
    ShotCallerDirective::new(
      LaneActorRole::HumanLaner,
      "unknown-plan-id",
      TeamStrategicObjective::LaneSiege,
      TeamMessageUrgency::Standard,
      TeamConfidenceLevel::Confident,
      TeamMessageCondition::Unconditional,
      "Rationale",
    ),
    Err(TeamLeadershipError::CatalogPlanNotFound("unknown-plan-id"))
  );

  // Policy evaluation
  let policy = ShotCallerPolicy::new(
    LaneActorRole::HumanLaner,
    TeamStrategicObjective::LaneSiege,
    "plan-lane-siege-v1",
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::HealthAboveThreshold,
  )
  .unwrap();

  let initial = LaneSnapshot::initial();
  let safe_obs = observe_player(&initial, ObservationId::new(999)).observation();
  let evaluated_dir = policy.evaluate_directive(&safe_obs).unwrap();
  assert_eq!(evaluated_dir.plan_id, "plan-lane-siege-v1");
  assert_eq!(evaluated_dir.objective, TeamStrategicObjective::LaneSiege);
}

#[test]
fn test_peer_plan_proposal_and_decentralized_coordinator() {
  let proposal1 = PeerPlanProposal::new(
    LaneActorRole::HumanLaner,
    "plan-gank-setup-v1",
    TeamStrategicObjective::GankSetup,
    TeamMessageUrgency::Critical,
    TeamConfidenceLevel::Definite,
    TeamMessageCondition::ThreatAbsent,
    8_500,
  )
  .unwrap();

  let proposal2 = PeerPlanProposal::new(
    LaneActorRole::AlliedAutonomous,
    "plan-gank-setup-v1",
    TeamStrategicObjective::GankSetup,
    TeamMessageUrgency::Critical,
    TeamConfidenceLevel::Definite,
    TeamMessageCondition::ThreatAbsent,
    7_500,
  )
  .unwrap();

  let initial = LaneSnapshot::initial();
  let safe_obs = observe_player(&initial, ObservationId::new(999)).observation();

  // 1. Unanimous consensus success
  let res_unanimous = DecentralizedCoordinator::arbitrate_proposals(
    ConsensusRule::UnanimousConsensus,
    &[proposal1.clone(), proposal2.clone()],
    10_000,
    &safe_obs,
  )
  .unwrap();
  assert!(res_unanimous.is_consensus());
  assert_eq!(res_unanimous.compliance_bp(), 10_000);

  // 2. Highest reputation lead
  let proposal_diff = PeerPlanProposal::new(
    LaneActorRole::AlliedAutonomous,
    "plan-defensive-hold-v1",
    TeamStrategicObjective::DefensiveHold,
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Tentative,
    TeamMessageCondition::Unconditional,
    4_000,
  )
  .unwrap();

  let res_rep = DecentralizedCoordinator::arbitrate_proposals(
    ConsensusRule::HighestReputationLead,
    &[proposal1.clone(), proposal_diff.clone()],
    5_000,
    &safe_obs,
  )
  .unwrap();
  assert_eq!(res_rep.as_str(), "consensus-achieved");
  assert_eq!(res_rep.compliance_bp(), 5_000);

  // 3. Urgency first
  let res_urgency = DecentralizedCoordinator::arbitrate_proposals(
    ConsensusRule::UrgencyFirst,
    &[proposal1.clone(), proposal_diff.clone()],
    5_000,
    &safe_obs,
  )
  .unwrap();
  assert_eq!(res_urgency.as_str(), "consensus-achieved");

  // 4. Majority tie deadlock
  let res_deadlock = DecentralizedCoordinator::arbitrate_proposals(
    ConsensusRule::MajoritySupport,
    &[proposal1.clone(), proposal_diff.clone()],
    5_000,
    &safe_obs,
  )
  .unwrap();
  assert_eq!(res_deadlock.as_str(), "conflicted-deadlock");

  // 5. Chain of thought fail-closed rejection
  let mut cot_prop = proposal1.clone();
  cot_prop.chain_of_thought_present = true;
  assert_eq!(
    cot_prop.to_message_envelope(1),
    Err(TeamLeadershipError::ChainOfThoughtForbidden)
  );
  assert_eq!(
    DecentralizedCoordinator::arbitrate_proposals(
      ConsensusRule::UnanimousConsensus,
      &[cot_prop],
      10_000,
      &safe_obs,
    ),
    Err(TeamLeadershipError::ChainOfThoughtForbidden)
  );
}

#[test]
fn test_team_leadership_evaluator_and_reports() {
  let initial = LaneSnapshot::initial();
  let safe_obs = observe_player(&initial, ObservationId::new(999)).observation();
  let mut trust_matrix = TeamTrustMatrix::new();
  trust_matrix
    .get_mut(LaneActorRole::HumanLaner)
    .reputation_bp = 8_500;

  let directive = ShotCallerDirective::new(
    LaneActorRole::HumanLaner,
    "plan-lane-siege-v1",
    TeamStrategicObjective::LaneSiege,
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::HealthAboveThreshold,
    "Siege tower while healthy.",
  )
  .unwrap();

  let designated = LeadershipStructure::DesignatedShotCaller {
    caller: LaneActorRole::HumanLaner,
    fallback_mode: FallbackLeadershipMode::FallbackToDefaultHold,
  };

  let report = TeamLeadershipEvaluator::evaluate_leadership(
    &designated,
    core::slice::from_ref(&directive),
    &[],
    &trust_matrix,
    &safe_obs,
  )
  .unwrap();

  assert_eq!(report.schema, LEADERSHIP_EVALUATION_REPORT_SCHEMA);
  assert_eq!(report.overall_compliance_bp, 10_000);
  assert!(report.resolution.is_consensus());
  assert_eq!(report.role_decisions.len(), 2);

  // Markdown formatting verification
  let md = report.to_markdown_summary();
  assert!(md.contains("# Team Leadership & Coordination Evaluation Report"));
  assert!(md.contains("**Overall Compliance:** 10000 bp (100.00%)"));
  assert!(md.contains("`human-laner`"));
  assert!(md.contains("`allied-autonomous`"));

  // Shared leadership evaluation
  let shared = LeadershipStructure::SharedLeadership {
    primary_caller: LaneActorRole::HumanLaner,
    secondary_caller: LaneActorRole::AlliedAutonomous,
    fallback_mode: FallbackLeadershipMode::FallbackToSecondaryCaller,
  };
  let shared_report = TeamLeadershipEvaluator::evaluate_leadership(
    &shared,
    core::slice::from_ref(&directive),
    &[],
    &trust_matrix,
    &safe_obs,
  )
  .unwrap();
  assert_eq!(shared_report.overall_compliance_bp, 10_000);
  assert!(shared_report.resolution.is_consensus());
}

#[test]
fn test_leadership_catalog_validation() {
  LeadershipCatalog::validate_catalog().unwrap();
  assert!(LeadershipCatalog::all().len() >= 6);

  let anchor_lead = LeadershipCatalog::get("leader-designated-anchor-v1").unwrap();
  assert_eq!(anchor_lead.name, "Designated Human Laner Anchor Lead");
  assert_eq!(anchor_lead.expected_compliance_bp, 8_500);

  let jungler_lead = LeadershipCatalog::get("leader-designated-jungler-v1").unwrap();
  assert_eq!(jungler_lead.name, "Designated Allied Jungler Lead");
  assert_eq!(jungler_lead.expected_compliance_bp, 7_500);

  let decentralized_unanimous =
    LeadershipCatalog::get("leader-decentralized-unanimous-v1").unwrap();
  assert_eq!(
    decentralized_unanimous.name,
    "Decentralized Unanimous Consensus"
  );

  assert_eq!(LeadershipCatalog::get("unknown-leader-id"), None);
}

#[test]
fn test_leadership_error_formatting() {
  let err_cot = TeamLeadershipError::ChainOfThoughtForbidden;
  assert_eq!(
    format!("{}", err_cot),
    "private chain-of-thought is strictly forbidden in leadership contracts"
  );

  let err_empty = TeamLeadershipError::EmptyPlanId;
  assert_eq!(format!("{}", err_empty), "target plan ID cannot be empty");

  let err_plan = TeamLeadershipError::CatalogPlanNotFound("test-plan");
  assert_eq!(
    format!("{}", err_plan),
    "team plan `test-plan` not found in catalog"
  );

  let err_cat = TeamLeadershipError::CatalogEntryNotFound;
  assert_eq!(format!("{}", err_cat), "leadership catalog entry not found");

  let err_bp = TeamLeadershipError::BasisPointOutOfRange {
    bp: 12_000,
    max: 10_000,
  };
  assert_eq!(
    format!("{}", err_bp),
    "basis points 12000 exceeded maximum allowed bound 10000"
  );

  let err_missing = TeamLeadershipError::CallerDirectiveMissing(LaneActorRole::HumanLaner);
  assert_eq!(
    format!("{}", err_missing),
    "shot-caller directive missing for designated role `human-laner`"
  );
}

#[test]
fn test_attribution_evaluator_with_simultaneous_resolution() {
  let mut window = TeamSimultaneousWindow::new_two_role(
    LaneActorRole::HumanLaner,
    LaneActorRole::AlliedAutonomous,
    100,
    1,
  )
  .unwrap();

  let team_plan = TeamPlanCatalog::lookup("plan-gank-setup-v1").unwrap();

  let msg = TeamMessageEnvelope::new(
    "msg-gank-prop-attr-v1",
    LaneActorRole::HumanLaner,
    TeamRecipient::Direct(LaneActorRole::AlliedAutonomous),
    TeamSpeechAct::Proposal,
    Some(LaneIntent::Contest),
    TeamMessageUrgency::Standard,
    TeamConfidenceLevel::Confident,
    TeamMessageCondition::Unconditional,
    TeamMessageVisibility::TeamOnly,
    1,
    "Gank proposal for attribution test",
  );

  let indiv_human = IndividualPlanDefinition {
    plan_id: "plan-human-attr-gank-v1",
    actor: LaneActorRole::HumanLaner,
    selected_intent: LaneIntent::Contest,
    target_focus: LaneTargetFocus::OpposingLaner,
    commitment: LaneCommitment::Standard,
    abort_condition: LaneAbortCondition::None,
    fallback_behavior: LaneFallbackBehavior::RetreatToTower,
    ping_signal: LanePingSignal::OnMyWay,
    chain_of_thought_present: false,
  };

  let indiv_ally = IndividualPlanDefinition {
    plan_id: "plan-ally-attr-gank-v1",
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
    Some(indiv_human),
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
    Some(indiv_ally),
    false,
  )
  .unwrap();

  window.submit(sub1).unwrap();
  window.submit(sub2).unwrap();

  let initial = LaneSnapshot::initial();
  let safe_obs = observe_player(&initial, ObservationId::new(100)).observation();

  let resolution = TeamSimultaneousResolver::resolve(
    &mut window,
    Some(team_plan),
    None,
    &[],
    &[],
    None,
    &safe_obs,
  )
  .unwrap();

  assert_eq!(
    resolution.coordination_outcome(),
    TeamCoordinationOutcome::FullyCoordinated
  );

  // Evaluate triumph
  let input = AttributionEvaluationInput {
    lane_outcome: LaneOutcome::HeldSpace,
    execution_score_bp: 8_500,
    primary_coord_factor: CoordinationCausalFactor::DirectiveCompliance,
    secondary_coord_factors: [Some(CoordinationCausalFactor::UnanimousAlignment), None],
    primary_exec_factor: ExecutionCausalFactor::DecisiveDamageAdvantage,
    secondary_exec_factors: [Some(ExecutionCausalFactor::ObjectiveSecured), None],
    weights: AttributionWeights::new(5_000, 4_000, 1_000).unwrap(),
    summary_notes: "Coordinated gank executed to plan",
    chain_of_thought_present: false,
  };
  let report = TeamAttributionEvaluator::evaluate(&resolution, input).unwrap();

  assert_eq!(
    report.attribution().quadrant(),
    AttributionQuadrant::CoordinatedTriumph
  );
  assert!(report.attribution().quadrant().is_coordination_effective());
  assert!(report.attribution().quadrant().is_execution_effective());
  assert_eq!(
    report.attribution().coordination_assessment().rating(),
    CoordinationRating::HighCoordination
  );
  assert_eq!(
    report.attribution().execution_assessment().rating(),
    ExecutionRating::FlawlessExecution
  );

  let md = report.to_markdown();
  assert!(md.contains("coordinated-triumph"));
  assert!(md.contains("directive-compliance"));
  assert!(md.contains("decisive-damage-advantage"));
}

#[test]
fn test_attribution_scenarios_comprehensive_matrix() {
  let scenarios = CoordinationAttributionCatalog::all_scenarios();
  assert_eq!(scenarios.len(), 6);

  for sc in scenarios {
    assert!(sc.validate().is_ok());

    let coord = CoordinationAssessment::new(
      sc.coordination_outcome,
      sc.cohesion_bp,
      sc.primary_coord_factor,
      [None, None],
    )
    .unwrap();

    let exec = ExecutionAssessment::new(
      sc.lane_outcome,
      sc.execution_score_bp,
      sc.primary_exec_factor,
      [None, None],
    )
    .unwrap();

    let weights = AttributionWeights::new(
      sc.coordination_contribution_bp,
      sc.execution_contribution_bp,
      sc.exogenous_variance_bp,
    )
    .unwrap();

    let attr =
      CoordinationExecutionAttribution::new(coord, exec, weights, sc.summary_notes).unwrap();

    assert_eq!(attr.quadrant(), sc.expected_quadrant);
    assert_eq!(
      attr.coordination_assessment().rating(),
      sc.expected_coordination_rating
    );
    assert_eq!(
      attr.execution_assessment().rating(),
      sc.expected_execution_rating
    );

    let report = CoordinationExecutionAttributionReport::new(1, 123, attr, false).unwrap();
    let md = report.to_markdown();
    assert!(md.contains(sc.expected_quadrant.as_str()));
  }
}

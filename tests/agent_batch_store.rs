//! Integration tests for file-backed batch store and operational log store.

use fog_of_intent::agent::{
  MAX_SCRIPTED_AGENT_OPERATIONAL_EVENTS, ScriptedAgentBatchCheckpoint,
  ScriptedAgentBatchCheckpointError, ScriptedAgentBatchRunner, ScriptedAgentExperimentManifest,
  ScriptedAgentOperationalEvent, ScriptedAgentOperationalLog,
  ScriptedAgentOperationalLogCodecError, ScriptedAgentProfile, ScriptedAgentSeedBundle,
};
use fog_of_intent::agent_batch_store::{
  ScriptedAgentBatchRunStore, ScriptedAgentBatchStoreError, ScriptedAgentBatchStoreOperationalError,
};
use fog_of_intent::agent_operational_store::{
  MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS, ScriptedAgentOperationalLogStore,
  ScriptedAgentOperationalLogStoreError,
};
use fog_of_intent::host::CliScenarioHost;
use fog_of_intent::kernel::{DrawId, StreamId};
use fog_of_intent::lane::{ALLIED_AUTONOMOUS_ACTOR, LaneSnapshot, ObservationId, observe_player};
use fog_of_intent::protocol::{
  ActorActionDto, ActorMessageDto, ActorProtocolCodecError, ActorProtocolIntent,
  MAX_ACTOR_DRAFT_VALUE_BYTES,
};
use fog_of_intent::run_store::CliRunStore;

#[test]
fn batch_checkpoint_and_operational_store_integration() {
  let state = LaneSnapshot::initial();
  let observation = observe_player(&state, ObservationId::new(42)).observation();
  let seed = ScriptedAgentSeedBundle::new(42, StreamId::new(7), DrawId::new(9));
  let manifests = [
    ScriptedAgentExperimentManifest::new(ScriptedAgentProfile::cautious_v1(), seed),
    ScriptedAgentExperimentManifest::new(ScriptedAgentProfile::risk_taking_v1(), seed),
  ];
  let checkpoint = ScriptedAgentBatchCheckpoint::new(observation, &manifests)
    .expect("manifests create checkpoint");

  let root = std::env::temp_dir().join(format!("fog-of-intent-agent-batch-{}", std::process::id()));
  let store = ScriptedAgentBatchRunStore::new(&root);
  let host_store = CliRunStore::new(&root);
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
  let operational_store = ScriptedAgentOperationalLogStore::new(&root);
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
  assert_eq!(MAX_SCRIPTED_AGENT_OPERATIONAL_LOG_SEGMENTS, 4);
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
  let invalid_segment_store = ScriptedAgentOperationalLogStore::new(&invalid_segment_root);
  assert_eq!(
    invalid_segment_store.save_segment("resume", 4, &first_segment),
    Err(ScriptedAgentOperationalLogStoreError::InvalidSegment { max: 4 })
  );
  assert!(!invalid_segment_root.exists());
  assert_eq!(
    invalid_segment_store.list_segments("resume"),
    Err(ScriptedAgentOperationalLogStoreError::StorageUnavailable)
  );
  assert_eq!(
    operational_store.list_segments("bad/id"),
    Err(ScriptedAgentOperationalLogStoreError::StorageUnavailable)
  );
  assert_eq!(
    invalid_segment_store.load_segment("resume", 4),
    Err(ScriptedAgentOperationalLogStoreError::InvalidSegment { max: 4 })
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
    Err(ScriptedAgentBatchStoreOperationalError::Store(
      ScriptedAgentBatchStoreError::InvalidCheckpoint {
        error: ScriptedAgentBatchCheckpointError::InvalidValue,
      },
    ))
  );
  assert_eq!(
    operational_log.entries(),
    log_before_decode_error.as_slice()
  );
  let invalid_root = root.join("not-a-directory");
  std::fs::write(&invalid_root, "file").expect("invalid storage root fixture writes");
  let invalid_store = ScriptedAgentBatchRunStore::new(&invalid_root);
  let mut storage_error_log = ScriptedAgentOperationalLog::new();
  storage_error_log
    .append(ScriptedAgentOperationalEvent::BatchStarted)
    .expect("one event fits");
  let storage_error_before = storage_error_log.entries().to_vec();
  assert_eq!(
    invalid_store.save_with_operational_log("resume", checkpoint, &mut storage_error_log),
    Err(ScriptedAgentBatchStoreOperationalError::Store(
      ScriptedAgentBatchStoreError::StorageUnavailable,
    ))
  );
  assert_eq!(storage_error_log.entries(), storage_error_before.as_slice());
  assert_eq!(
    invalid_store.load_with_operational_log("resume", &mut storage_error_log),
    Err(ScriptedAgentBatchStoreOperationalError::Store(
      ScriptedAgentBatchStoreError::StorageUnavailable,
    ))
  );
  assert_eq!(storage_error_log.entries(), storage_error_before.as_slice());
  std::fs::write(root.join("broken.foi-operational-log"), "bad")
    .expect("malformed operational log fixture writes");
  assert_eq!(
    operational_store.load("broken"),
    Err(ScriptedAgentOperationalLogStoreError::InvalidLog {
      error: ScriptedAgentOperationalLogCodecError::InvalidValue,
    })
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
      ScriptedAgentBatchStoreOperationalError::LogCapacityExceeded {
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
      ScriptedAgentBatchStoreOperationalError::LogCapacityExceeded {
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
    Vec::new()
  );
  let _ = std::fs::remove_dir_all(root);
}

#[test]
fn cross_crate_stress_population_rejection_parity() {
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
}

//! Tests for actor protocol DTOs, codecs, and communication reports.

use super::action::{
  ActorActionDto, ActorActionResultDto, ActorActionResultOutcome, ActorActionResultWindow,
};
use super::codec::{
  ACTOR_PROTOCOL_CODEC_SCHEMA, ActorProtocolCodecError, MAX_ACTOR_PROTOCOL_BYTES,
};
use super::commit::{ActorCommitDto, ActorCommitResultDto};
use super::debrief::{
  ActorDebriefAttributionLimit, ActorDebriefDto, ActorDebriefObjective, ActorDebriefWindow,
};
use super::draft::{
  ActorDraftClearDto, ActorDraftClearReceiptDto, ActorDraftCommitReceiptDto, ActorDraftDto,
  ActorDraftField, ActorDraftPresence, ActorDraftReceiptDto, ActorDraftStatusDto,
  MAX_ACTOR_DRAFT_VALUE_BYTES,
};
use super::error::{
  ACTOR_PROTOCOL_ERROR_SCHEMA, ACTOR_PROTOCOL_ERROR_SCHEMA_V1, ActorProtocolError,
  ActorProtocolErrorCode, ActorProtocolRepairHint,
};
use super::history::{ActorHistoryDto, ActorHistoryStatus};
use super::intents::{ACTOR_PROTOCOL_SCHEMA, ActorProtocolIntent};
use super::message::{
  ACTOR_COMMUNICATION_ABUSE_POPULATION_SCHEMA, ActorCommunicationAbusePopulationError,
  ActorCommunicationAbusePopulationReport, ActorMessageDto,
  MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION,
};
use super::observation::ActorObservationDto;
use super::replay::{
  ActorReplayDebriefRecordDto, ActorReplayDto, ActorReplayRecordDto, ActorReplayVerification,
};
use super::transcript::{
  ActorToolAuthority, ActorTranscriptDto, ActorTranscriptResult, ActorTranscriptTool,
  actor_tool_capabilities,
};
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
fn actor_draft_commit_receipt_codec_is_exact_and_payload_free() {
  let receipt = ActorDraftCommitReceiptDto::new(
    1,
    41,
    ActorProtocolIntent::Contest,
    ActorDraftPresence::Present,
    ActorDraftPresence::Absent,
    ActorDraftPresence::Present,
  );
  assert_eq!(receipt.schema(), "m5-actor-draft-commit-receipt-v1");
  assert_eq!(receipt.message(), ActorDraftPresence::Present);
  assert_eq!(receipt.plan(), ActorDraftPresence::Absent);
  assert_eq!(receipt.contingency(), ActorDraftPresence::Present);
  assert_eq!(
    receipt.encode(),
    "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=41\nintent=contest\nmessage=present\nplan=absent\ncontingency=present\n"
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(&receipt.encode()),
    Ok(receipt)
  );
  assert!(!format!("{receipt:?}").contains("ping ally"));
  assert!(!receipt.encode().contains("retreat if threat"));

  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=41\nintent=contest\nunknown=present\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::UnknownField)
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=41\nintent=contest\nmessage=present\nmessage=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::DuplicateField)
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=41\nintent=contest\nmessage=present\nplan=absent\n"
    ),
    Err(ActorProtocolCodecError::MissingField)
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v0\nobserver=1\nobservation_id=41\nintent=contest\nmessage=present\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v1\nobserver=nope\nobservation_id=41\nintent=contest\nmessage=present\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=41\nintent=contest\nmessage=unknown\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=nope\nintent=contest\nmessage=present\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftCommitReceiptDto::decode(
      "schema=m5-actor-draft-commit-receipt-v1\nobserver=1\nobservation_id=41\nintent=contest\nmessage=present\nplan=absent\ncontingency=present\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 7,
      actual: 8,
    })
  );
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
fn actor_message_codec_is_recipient_bound_and_closed() {
  let message = ActorMessageDto::new(1, 3, 36, "ping ally").expect("message is bounded");
  assert_eq!(message.schema(), "m5-actor-message-v1");
  assert_eq!(message.sender(), 1);
  assert_eq!(message.recipient(), 3);
  assert_eq!(message.observation_id(), 36);
  assert_eq!(message.message(), "ping ally");
  assert_eq!(
    message.encode(),
    "schema=m5-actor-message-v1\nsender=1\nrecipient=3\nobservation_id=36\nmessage=ping ally\n"
  );
  assert_eq!(
    ActorMessageDto::decode(&message.encode()),
    Ok(message.clone())
  );
  assert!(!format!("{message:?}").contains("StateHash"));
  assert!(!format!("{message:?}").contains("health"));

  for invalid in [
    (0, 3, 36, "ping ally"),
    (1, 0, 36, "ping ally"),
    (1, 1, 36, "ping ally"),
    (1, 3, 0, "ping ally"),
    (1, 3, 36, ""),
    (1, 3, 36, "line\nfeed"),
  ] {
    assert_eq!(
      ActorMessageDto::new(invalid.0, invalid.1, invalid.2, invalid.3),
      Err(ActorProtocolCodecError::InvalidValue)
    );
  }
  assert_eq!(
    ActorMessageDto::new(1, 3, 36, &"x".repeat(MAX_ACTOR_DRAFT_VALUE_BYTES + 1)),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  let max_message = ActorMessageDto::new(1, 3, 36, &"x".repeat(MAX_ACTOR_DRAFT_VALUE_BYTES))
    .expect("the inclusive message bound is accepted");
  assert_eq!(
    ActorMessageDto::decode(&max_message.encode()),
    Ok(max_message)
  );

  let valid = message.encode();
  for malformed in [
    (
      valid.replacen("schema=m5-actor-message-v1", "schema=other", 1),
      ActorProtocolCodecError::UnsupportedSchema,
    ),
    (
      valid.replacen("recipient=3", "unknown=3", 1),
      ActorProtocolCodecError::UnknownField,
    ),
    (
      valid.replacen("recipient=3", "sender=3", 1),
      ActorProtocolCodecError::DuplicateField,
    ),
    (
      valid.replacen("recipient=3\n", "", 1),
      ActorProtocolCodecError::MissingField,
    ),
    (
      valid.replacen("sender=1", "sender=nope", 1),
      ActorProtocolCodecError::InvalidValue,
    ),
    (
      format!("{valid}extra=line\n"),
      ActorProtocolCodecError::UnexpectedLineCount {
        expected: 5,
        actual: 6,
      },
    ),
  ] {
    assert_eq!(ActorMessageDto::decode(&malformed.0), Err(malformed.1));
  }
}

#[test]
fn actor_draft_receipt_codec_is_bounded_and_payload_free() {
  for field in [
    ActorDraftField::Message,
    ActorDraftField::Plan,
    ActorDraftField::Contingency,
  ] {
    let receipt = ActorDraftReceiptDto::new(1, 36, field);
    assert_eq!(receipt.schema(), "m5-actor-draft-receipt-v1");
    assert_eq!(receipt.observer(), 1);
    assert_eq!(receipt.observation_id(), 36);
    assert_eq!(receipt.field(), field);
    if field == ActorDraftField::Message {
      assert_eq!(
        receipt.encode(),
        "schema=m5-actor-draft-receipt-v1\nobserver=1\nobservation_id=36\nfield=message\n"
      );
    }
    assert_eq!(ActorDraftReceiptDto::decode(&receipt.encode()), Ok(receipt));
    assert!(!format!("{receipt:?}").contains("value"));
    assert!(!format!("{receipt:?}").contains("StateHash"));
  }
  assert_eq!(
    ActorDraftReceiptDto::decode(
      "schema=m5-actor-draft-receipt-v1\nobserver=1\nobservation_id=36\nunknown=message\n"
    ),
    Err(ActorProtocolCodecError::UnknownField)
  );
  assert_eq!(
    ActorDraftReceiptDto::decode(
      "schema=m5-actor-draft-receipt-v1\nobserver=1\nobserver=1\nfield=message\n"
    ),
    Err(ActorProtocolCodecError::DuplicateField)
  );
  assert_eq!(
    ActorDraftReceiptDto::decode(
      "schema=m5-actor-draft-receipt-v1\nobserver=1\nobservation_id=36\n"
    ),
    Err(ActorProtocolCodecError::MissingField)
  );
  assert_eq!(
    ActorDraftReceiptDto::decode(
      "schema=m5-actor-draft-receipt-v0\nobserver=1\nobservation_id=36\nfield=message\n"
    ),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  assert_eq!(
    ActorDraftReceiptDto::decode(
      "schema=m5-actor-draft-receipt-v1\nobserver=nope\nobservation_id=36\nfield=message\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftReceiptDto::decode(
      "schema=m5-actor-draft-receipt-v1\nobserver=1\nobservation_id=36\nfield=unknown\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftReceiptDto::decode(
      "schema=m5-actor-draft-receipt-v1\nobserver=1\nobservation_id=36\nfield=message\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 4,
      actual: 5,
    })
  );
}

#[test]
fn actor_draft_status_codec_is_bounded_and_payload_free() {
  let status = ActorDraftStatusDto::new(
    1,
    36,
    ActorDraftPresence::Present,
    ActorDraftPresence::Absent,
    ActorDraftPresence::Present,
  );
  assert_eq!(status.schema(), "m5-actor-draft-status-v1");
  assert_eq!(status.observer(), 1);
  assert_eq!(status.observation_id(), 36);
  assert_eq!(status.message(), ActorDraftPresence::Present);
  assert_eq!(status.plan(), ActorDraftPresence::Absent);
  assert_eq!(status.contingency(), ActorDraftPresence::Present);
  assert_eq!(
    status.encode(),
    "schema=m5-actor-draft-status-v1\nobserver=1\nobservation_id=36\nmessage=present\nplan=absent\ncontingency=present\n"
  );
  assert_eq!(ActorDraftStatusDto::decode(&status.encode()), Ok(status));
  assert!(!format!("{status:?}").contains("ping ally"));
  assert!(!status.encode().contains("ping ally"));

  assert_eq!(
    ActorDraftStatusDto::decode(
      "schema=m5-actor-draft-status-v1\nobserver=1\nobservation_id=36\nmessage=present\nplan=absent\nunknown=present\n"
    ),
    Err(ActorProtocolCodecError::UnknownField)
  );
  assert_eq!(
    ActorDraftStatusDto::decode(
      "schema=m5-actor-draft-status-v1\nobserver=1\nobservation_id=36\nmessage=present\nmessage=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::DuplicateField)
  );
  assert_eq!(
    ActorDraftStatusDto::decode(
      "schema=m5-actor-draft-status-v1\nobserver=1\nobservation_id=36\nmessage=present\nplan=absent\n"
    ),
    Err(ActorProtocolCodecError::MissingField)
  );
  assert_eq!(
    ActorDraftStatusDto::decode(
      "schema=m5-actor-draft-status-v0\nobserver=1\nobservation_id=36\nmessage=present\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  assert_eq!(
    ActorDraftStatusDto::decode(
      "schema=m5-actor-draft-status-v1\nobserver=nope\nobservation_id=36\nmessage=present\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftStatusDto::decode(
      "schema=m5-actor-draft-status-v1\nobserver=1\nobservation_id=36\nmessage=unknown\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftStatusDto::decode(
      "schema=m5-actor-draft-status-v1\nobserver=1\nobservation_id=36\nmessage=present\nplan=absent\ncontingency=present\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 6,
      actual: 7,
    })
  );
}

#[test]
fn actor_draft_clear_codecs_are_observation_bound_and_payload_free() {
  let clear = ActorDraftClearDto::new(1, 36);
  assert_eq!(clear.schema(), "m5-actor-draft-clear-v1");
  assert_eq!(
    clear.encode(),
    "schema=m5-actor-draft-clear-v1\nobserver=1\nobservation_id=36\n"
  );
  assert_eq!(ActorDraftClearDto::decode(&clear.encode()), Ok(clear));
  for input in [
    "schema=m5-actor-draft-clear-v1\nobserver=1\nobserver=1\nobservation_id=36\n",
    "schema=m5-actor-draft-clear-v1\nobserver=1\n",
    "schema=m5-actor-draft-clear-v0\nobserver=1\nobservation_id=36\n",
    "schema=m5-actor-draft-clear-v1\nobserver=nope\nobservation_id=36\n",
    "schema=m5-actor-draft-clear-v1\nobserver=1\nobservation_id=36\nextra=x\n",
  ] {
    assert!(ActorDraftClearDto::decode(input).is_err());
  }

  let receipt = ActorDraftClearReceiptDto::new(
    1,
    36,
    ActorDraftPresence::Present,
    ActorDraftPresence::Absent,
    ActorDraftPresence::Present,
  );
  assert_eq!(receipt.schema(), "m5-actor-draft-clear-receipt-v1");
  assert_eq!(
    receipt.encode(),
    "schema=m5-actor-draft-clear-receipt-v1\nobserver=1\nobservation_id=36\nmessage=present\nplan=absent\ncontingency=present\n"
  );
  assert_eq!(
    ActorDraftClearReceiptDto::decode(&receipt.encode()),
    Ok(receipt)
  );
  assert!(!format!("{receipt:?}").contains("ping ally"));
  assert!(!receipt.encode().contains("retreat if threat"));
  assert_eq!(
    ActorDraftClearReceiptDto::decode(
      "schema=m5-actor-draft-clear-receipt-v1\nobserver=1\nobservation_id=36\nmessage=unknown\nplan=absent\ncontingency=present\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorDraftClearReceiptDto::decode(
      "schema=m5-actor-draft-clear-receipt-v1\nobserver=1\nobservation_id=36\nmessage=present\nplan=absent\ncontingency=present\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 6,
      actual: 7,
    })
  );
}

#[test]
fn actor_transcript_codec_binds_closed_tools_and_results() {
  let tools = [
    (
      ActorTranscriptTool::Observation,
      "observation",
      "m5-actor-observation-v1",
    ),
    (ActorTranscriptTool::Draft, "draft", "m5-actor-draft-v1"),
    (
      ActorTranscriptTool::DraftReceipt,
      "draft_receipt",
      "m5-actor-draft-receipt-v1",
    ),
    (ActorTranscriptTool::Commit, "commit", "m5-actor-commit-v1"),
    (ActorTranscriptTool::Action, "action", "m5-actor-action-v1"),
  ];
  for (tool, expected_tool_id, expected_schema) in tools {
    for result in [
      ActorTranscriptResult::Accepted,
      ActorTranscriptResult::Rejected,
    ] {
      let transcript = ActorTranscriptDto::new(1, 42, tool, result);
      assert_eq!(transcript.schema(), "m5-actor-transcript-v1");
      assert_eq!(transcript.tool(), tool);
      assert_eq!(tool.id(), expected_tool_id);
      assert_eq!(tool.schema_id(), expected_schema);
      assert_eq!(transcript.tool_schema(), expected_schema);
      assert_eq!(transcript.result(), result);
      assert_eq!(
        transcript.encode(),
        format!(
          "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool={expected_tool_id}\ntool_schema={expected_schema}\nresult={}\n",
          result.id()
        )
      );
      assert_eq!(
        ActorTranscriptDto::decode(&transcript.encode()),
        Ok(transcript)
      );
      let visible = format!("{:?}\n{}", transcript, transcript.encode()).to_ascii_lowercase();
      for marker in [
        "payload",
        "state",
        "hash",
        "execution",
        "trace",
        "source",
        "provenance",
        "transport",
        "prompt",
        "model",
      ] {
        assert!(
          !visible.contains(marker),
          "transcript leaked marker {marker}: {visible}"
        );
      }
    }
  }
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool=unknown\ntool_schema=m5-actor-observation-v1\nresult=accepted\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-action-v1\nresult=accepted\n"
    ),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v0\nobserver=1\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-observation-v1\nresult=accepted\n"
    ),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-observation-v1\nunknown=accepted\n"
    ),
    Err(ActorProtocolCodecError::UnknownField)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-observation-v1\ntool=observation\n"
    ),
    Err(ActorProtocolCodecError::DuplicateField)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-observation-v1\n"
    ),
    Err(ActorProtocolCodecError::MissingField)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=nope\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-observation-v1\nresult=accepted\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-observation-v1\nresult=maybe\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorTranscriptDto::decode(
      "schema=m5-actor-transcript-v1\nobserver=1\nobservation_id=42\ntool=observation\ntool_schema=m5-actor-observation-v1\nresult=accepted\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 6,
      actual: 7,
    })
  );
}

#[test]
fn actor_tool_capability_catalog_is_stable_and_ordinary_only() {
  let expected = [
    (
      ActorTranscriptTool::Observation,
      "observation",
      "m5-actor-observation-v1",
    ),
    (ActorTranscriptTool::Draft, "draft", "m5-actor-draft-v1"),
    (
      ActorTranscriptTool::DraftReceipt,
      "draft_receipt",
      "m5-actor-draft-receipt-v1",
    ),
    (ActorTranscriptTool::Commit, "commit", "m5-actor-commit-v1"),
    (ActorTranscriptTool::Action, "action", "m5-actor-action-v1"),
  ];
  let catalog = actor_tool_capabilities();
  assert_eq!(catalog.len(), expected.len());
  for (capability, (tool, tool_id, schema)) in catalog.into_iter().zip(expected) {
    assert_eq!(capability.tool(), tool);
    assert_eq!(capability.tool().id(), tool_id);
    assert_eq!(capability.tool().schema_id(), schema);
    assert_eq!(capability.authority(), ActorToolAuthority::OrdinaryActor);
    assert_eq!(capability.authority().id(), "ordinary_actor");
  }
  assert_eq!(
    ActorToolAuthority::PrivilegedExperimentController.id(),
    "privileged_experiment_controller"
  );
  assert!(
    actor_tool_capabilities().into_iter().all(
      |capability| capability.authority() != ActorToolAuthority::PrivilegedExperimentController
    )
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
fn actor_replay_codec_round_trips_bounded_verification_status() {
  for records in [0, 1, 2] {
    let dto = ActorReplayDto::new(records).expect("replay count is bounded");
    assert_eq!(dto.schema(), "m5-actor-replay-v1");
    assert_eq!(dto.records(), records);
    assert_eq!(dto.verification(), ActorReplayVerification::Verified);
    assert_eq!(
      dto.encode(),
      format!("schema=m5-actor-replay-v1\nrecords={records}\nverification=verified\n")
    );
    assert_eq!(ActorReplayDto::decode(&dto.encode()), Ok(dto));
  }
  assert_eq!(
    ActorReplayDto::new(3),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorReplayDto::decode("schema=m5-actor-replay-v1\nrecords=3\nverification=verified\n"),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorReplayDto::decode("schema=m5-actor-replay-v1\nrecords=0\nverification=unknown\n"),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorReplayDto::decode("schema=m5-actor-replay-v1\nrecords=0\nrecords=1\n"),
    Err(ActorProtocolCodecError::DuplicateField)
  );
  assert_eq!(
    ActorReplayDto::decode("schema=m5-actor-replay-v1\nrecords=0\n"),
    Err(ActorProtocolCodecError::MissingField)
  );
  assert_eq!(
    ActorReplayDto::decode("schema=m5-actor-replay-v0\nrecords=0\nverification=verified\n"),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  assert_eq!(
    ActorReplayDto::decode(
      "schema=m5-actor-replay-v1\nrecords=0\nverification=verified\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 3,
      actual: 4,
    })
  );
}

#[test]
fn actor_replay_record_codec_is_categorical_and_verification_bound() {
  let cases = [
    (
      ActorActionResultWindow::First,
      ActorProtocolIntent::Contest,
      ActorActionResultOutcome::HeldSpace,
    ),
    (
      ActorActionResultWindow::Second,
      ActorProtocolIntent::Stabilize,
      ActorActionResultOutcome::YieldedSpace,
    ),
  ];
  for (window, intent, outcome) in cases {
    let dto = ActorReplayRecordDto::new(window, intent, outcome);
    assert_eq!(dto.schema(), "m5-actor-replay-record-v1");
    assert_eq!(dto.window(), window);
    assert_eq!(dto.intent(), intent);
    assert_eq!(dto.outcome(), outcome);
    assert_eq!(dto.verification(), ActorReplayVerification::Verified);
    assert_eq!(
      dto.encode(),
      format!(
        "schema=m5-actor-replay-record-v1\nwindow={}\nintent={}\noutcome={}\nverification=verified\n",
        window.id(),
        intent.id(),
        outcome.id()
      )
    );
    assert_eq!(ActorReplayRecordDto::decode(&dto.encode()), Ok(dto));
    assert!(!format!("{dto:?}").contains("StateHash"));
    assert!(!dto.encode().contains("trace"));
  }
  assert_eq!(
    ActorReplayRecordDto::decode(
      "schema=m5-actor-replay-record-v1\nwindow=first\nintent=contest\nunknown=held_space\nverification=verified\n"
    ),
    Err(ActorProtocolCodecError::UnknownField)
  );
  assert_eq!(
    ActorReplayRecordDto::decode(
      "schema=m5-actor-replay-record-v1\nwindow=first\nintent=contest\nintent=yield\nverification=verified\n"
    ),
    Err(ActorProtocolCodecError::DuplicateField)
  );
  assert_eq!(
    ActorReplayRecordDto::decode(
      "schema=m5-actor-replay-record-v1\nwindow=first\nintent=contest\noutcome=held_space\n"
    ),
    Err(ActorProtocolCodecError::MissingField)
  );
  assert_eq!(
    ActorReplayRecordDto::decode(
      "schema=m5-actor-replay-record-v0\nwindow=first\nintent=contest\noutcome=held_space\nverification=verified\n"
    ),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  for (field, value) in [
    ("window", "third"),
    ("intent", "unknown"),
    ("outcome", "unknown"),
  ] {
    let input = format!(
      "schema=m5-actor-replay-record-v1\nwindow={}\nintent={}\noutcome={}\nverification=verified\n",
      if field == "window" { value } else { "first" },
      if field == "intent" { value } else { "contest" },
      if field == "outcome" {
        value
      } else {
        "held_space"
      },
    );
    assert_eq!(
      ActorReplayRecordDto::decode(&input),
      Err(ActorProtocolCodecError::InvalidValue)
    );
  }
  assert_eq!(
    ActorReplayRecordDto::decode(
      "schema=m5-actor-replay-record-v1\nwindow=first\nintent=contest\noutcome=held_space\nverification=unknown\n"
    ),
    Err(ActorProtocolCodecError::InvalidValue)
  );
  assert_eq!(
    ActorReplayRecordDto::decode(
      "schema=m5-actor-replay-record-v1\nwindow=first\nintent=contest\noutcome=held_space\nverification=verified\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 5,
      actual: 6,
    })
  );
}

#[test]
fn actor_replay_debrief_record_codec_is_verified_and_committed_facts_only() {
  let cases = [
    (
      ActorActionResultWindow::First,
      ActorProtocolIntent::Contest,
      ActorActionResultOutcome::HeldSpace,
      ActorDebriefObjective::GoalAchieved,
    ),
    (
      ActorActionResultWindow::Second,
      ActorProtocolIntent::Stabilize,
      ActorActionResultOutcome::YieldedSpace,
      ActorDebriefObjective::GoalPartiallyAchieved,
    ),
  ];
  for (window, intent, outcome, objective) in cases {
    let dto = ActorReplayDebriefRecordDto::new(window, intent, outcome, objective);
    assert_eq!(dto.schema(), "m5-actor-replay-debrief-record-v1");
    assert_eq!(
      dto.attribution(),
      ActorDebriefAttributionLimit::CommittedFactsOnly
    );
    assert_eq!(dto.verification(), ActorReplayVerification::Verified);
    assert_eq!(
      dto.encode(),
      format!(
        "schema=m5-actor-replay-debrief-record-v1\nwindow={}\nintent={}\noutcome={}\nobjective={}\nattribution=committed_facts_only\nverification=verified\n",
        window.id(),
        intent.id(),
        outcome.id(),
        objective.id()
      )
    );
    assert_eq!(ActorReplayDebriefRecordDto::decode(&dto.encode()), Ok(dto));
    assert!(!format!("{dto:?}").contains("StateHash"));
    assert!(!dto.encode().contains("trace"));
    assert!(!dto.encode().contains("health"));
  }
  assert_eq!(
    ActorReplayDebriefRecordDto::decode(
      "schema=m5-actor-replay-debrief-record-v1\nwindow=first\nintent=contest\noutcome=held_space\nunknown=goal_achieved\nattribution=committed_facts_only\nverification=verified\n"
    ),
    Err(ActorProtocolCodecError::UnknownField)
  );
  assert_eq!(
    ActorReplayDebriefRecordDto::decode(
      "schema=m5-actor-replay-debrief-record-v1\nwindow=first\nintent=contest\nintent=yield\noutcome=held_space\nattribution=committed_facts_only\nverification=verified\n"
    ),
    Err(ActorProtocolCodecError::DuplicateField)
  );
  assert_eq!(
    ActorReplayDebriefRecordDto::decode(
      "schema=m5-actor-replay-debrief-record-v1\nwindow=first\nintent=contest\noutcome=held_space\nobjective=goal_achieved\nattribution=committed_facts_only\n"
    ),
    Err(ActorProtocolCodecError::MissingField)
  );
  assert_eq!(
    ActorReplayDebriefRecordDto::decode(
      "schema=m5-actor-replay-debrief-record-v0\nwindow=first\nintent=contest\noutcome=held_space\nobjective=goal_achieved\nattribution=committed_facts_only\nverification=verified\n"
    ),
    Err(ActorProtocolCodecError::UnsupportedSchema)
  );
  for (field, value) in [
    ("window", "third"),
    ("intent", "unknown"),
    ("outcome", "unknown"),
    ("objective", "unknown"),
    ("attribution", "other"),
    ("verification", "unknown"),
  ] {
    let input = format!(
      "schema=m5-actor-replay-debrief-record-v1\nwindow={}\nintent={}\noutcome={}\nobjective={}\nattribution={}\nverification={}\n",
      if field == "window" { value } else { "first" },
      if field == "intent" { value } else { "contest" },
      if field == "outcome" {
        value
      } else {
        "held_space"
      },
      if field == "objective" {
        value
      } else {
        "goal_achieved"
      },
      if field == "attribution" {
        value
      } else {
        "committed_facts_only"
      },
      if field == "verification" {
        value
      } else {
        "verified"
      },
    );
    assert_eq!(
      ActorReplayDebriefRecordDto::decode(&input),
      Err(ActorProtocolCodecError::InvalidValue)
    );
  }
  assert_eq!(
    ActorReplayDebriefRecordDto::decode(
      "schema=m5-actor-replay-debrief-record-v1\nwindow=first\nintent=contest\noutcome=held_space\nobjective=goal_achieved\nattribution=committed_facts_only\nverification=verified\nextra=x\n"
    ),
    Err(ActorProtocolCodecError::UnexpectedLineCount {
      expected: 7,
      actual: 8,
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

#[test]
fn actor_communication_abuse_population_is_bounded_and_pure() {
  assert_eq!(MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION, 4);
  assert_eq!(
    ACTOR_COMMUNICATION_ABUSE_POPULATION_SCHEMA,
    "m6-actor-communication-abuse-population-v1"
  );

  let invalid_payloads = ["", "line\nbreak", "null\0byte", "cr\rreturn"];

  for invalid_payload in invalid_payloads {
    for attempts in 1..=MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION {
      let report = ActorCommunicationAbusePopulationReport::from_invalid_payload(
        1,
        2,
        55,
        invalid_payload,
        attempts,
      )
      .expect("bounded invalid payload population succeeds");

      assert_eq!(
        report.schema(),
        "m6-actor-communication-abuse-population-v1"
      );
      assert_eq!(report.sender(), 1);
      assert_eq!(report.recipient(), 2);
      assert_eq!(report.observation_id(), 55);
      assert_eq!(
        report.rejection_error(),
        ActorProtocolCodecError::InvalidValue
      );
      assert_eq!(
        report.attempt_count(),
        u8::try_from(attempts).expect("fits in u8")
      );

      let debug_str = format!("{report:?}");
      assert!(!debug_str.contains("break"));
      assert!(!debug_str.contains("payload"));
      assert!(!debug_str.contains("StateHash"));
    }
  }

  let oversized = "x".repeat(MAX_ACTOR_DRAFT_VALUE_BYTES + 1);
  let report =
    ActorCommunicationAbusePopulationReport::from_invalid_payload(1, 2, 55, &oversized, 3)
      .expect("oversized invalid payload population succeeds");
  assert_eq!(report.attempt_count(), 3);

  // Self-delivery is also an invalid message value
  let report_self =
    ActorCommunicationAbusePopulationReport::from_invalid_payload(1, 1, 55, "valid text", 2)
      .expect("self-delivery invalid payload population succeeds");
  assert_eq!(report_self.sender(), 1);
  assert_eq!(report_self.recipient(), 1);
  assert_eq!(report_self.attempt_count(), 2);

  assert_eq!(
    ActorCommunicationAbusePopulationReport::from_invalid_payload(1, 2, 55, "", 0),
    Err(ActorCommunicationAbusePopulationError::EmptyPopulation)
  );

  assert_eq!(
    ActorCommunicationAbusePopulationReport::from_invalid_payload(
      1,
      2,
      55,
      "",
      MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION + 1
    ),
    Err(ActorCommunicationAbusePopulationError::PopulationTooLarge { max: 4, actual: 5 })
  );

  assert_eq!(
    ActorCommunicationAbusePopulationReport::from_invalid_payload(1, 2, 55, "valid ping", 1),
    Err(ActorCommunicationAbusePopulationError::UnexpectedError {
      actual: ActorProtocolCodecError::InvalidValue,
    })
  );

  assert_eq!(
    ActorCommunicationAbusePopulationReport::from_invalid_payload(0, 2, 55, "", 1),
    Err(ActorCommunicationAbusePopulationError::InvalidTarget)
  );
  assert_eq!(
    ActorCommunicationAbusePopulationReport::from_invalid_payload(1, 0, 55, "", 1),
    Err(ActorCommunicationAbusePopulationError::InvalidTarget)
  );
  assert_eq!(
    ActorCommunicationAbusePopulationReport::from_invalid_payload(1, 2, 0, "", 1),
    Err(ActorCommunicationAbusePopulationError::InvalidTarget)
  );
}

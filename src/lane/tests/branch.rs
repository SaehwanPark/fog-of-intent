#[test]
    fn matched_branch_replays_and_preserves_parent() {
        let (parent, receipt) = committed_parent(LaneIntent::Contest);
        let parent_records = parent.records.clone();
        let parent_current = parent.current_state();
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );
        let branch = branch_from_window(
            &parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("matched branch");

        assert_eq!(
            branch.identity().replay_id(),
            "m2-one-lane-window-branch-v1"
        );
        assert_eq!(
            branch.identity().execution_mode(),
            BranchExecutionMode::MatchedParent
        );
        assert_eq!(branch.record().inputs(), parent.records()[0].inputs());
        assert_eq!(branch.record().command().intent(), LaneIntent::Stabilize);
        branch.verify_replay(&parent).expect("branch replay");
        let review = branch.review(&parent).expect("counterfactual review");
        assert_eq!(review.parent_outcome(), LaneOutcome::HeldSpace);
        assert_eq!(review.branch_outcome(), LaneOutcome::YieldedSpace);
        assert_eq!(
            review.attribution_limit(),
            LaneAttributionLimit::MatchedDecisionOnly
        );
        assert_eq!(parent.records, parent_records);
        assert_eq!(parent.current_state(), parent_current);
        assert_eq!(parent.verify_replay(), Ok(parent_current));
    }

#[test]
    fn matched_branch_clears_contest_only_mana_for_non_contest_and_binds_identity() {
        let state = LaneSnapshot::initial();
        let (receipt, parent_request) = request(&state, LaneIntent::Contest);
        let spent = LaneMana::new(1).expect("bounded spend");
        let spent_inputs = inputs(1, 1, LaneWaveResult::Held).with_mana_spent(spent);
        let mut spent_parent = LaneHistory::new(state).expect("valid");
        spent_parent
            .append(&receipt, &parent_request, spent_inputs)
            .expect("append");
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );
        let spent_branch = branch_from_window(
            &spent_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("matched branch normalizes intent-scoped spend");
        assert_eq!(
            spent_branch.record().inputs().execution().mana_spent(),
            LaneMana::zero()
        );
        assert_eq!(
            spent_branch.identity().mana_policy(),
            LaneBranchManaPolicy::NonContestSpendCleared
        );
        let spent_review = spent_branch.review(&spent_parent).expect("review");
        assert_eq!(
            spent_review.execution_relation(),
            LaneExecutionRelation::MatchedWithResourceNormalization
        );
        assert_eq!(
            spent_review.attribution_limit(),
            LaneAttributionLimit::DecisionAndResourceChanged
        );
        spent_branch
            .verify_replay(&spent_parent)
            .expect("branch replay");

        let mut no_spend_parent = LaneHistory::new(state).expect("valid");
        no_spend_parent
            .append(
                &receipt,
                &parent_request,
                inputs(1, 1, LaneWaveResult::Held),
            )
            .expect("append");
        let no_spend_branch = branch_from_window(
            &no_spend_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("matched branch");
        assert_eq!(
            no_spend_branch.identity().mana_policy(),
            LaneBranchManaPolicy::ParentSpendPreserved
        );
        assert_ne!(
            spent_branch.identity().parent_record_identity(),
            no_spend_branch.identity().parent_record_identity()
        );
    }

#[test]
    fn regenerated_branch_uses_a_stable_branch_trace() {
        let (parent, receipt) = committed_parent(LaneIntent::Stabilize);
        let branch_id = BranchId::new(7).expect("branch id is bounded");
        let execution = LaneExecutionInputs::new(
            trace(135, 0),
            LaneDamage::new(0).expect("bounded"),
            LaneDamage::new(2).expect("bounded"),
            LaneWaveResult::Advanced,
        );
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Contest,
        );
        let branch = branch_from_window(
            &parent,
            &alternate,
            BranchExecutionSelection::regenerated(branch_id, execution),
        )
        .expect("regenerated branch");
        assert_eq!(branch.identity().branch_id(), Some(branch_id));
        assert_eq!(branch.identity().execution_trace(), trace(135, 0));
        assert_eq!(
            branch.record().inputs().environment(),
            parent.records()[0].inputs().environment()
        );
        assert_eq!(branch.record().inputs().execution().trace(), trace(135, 0));
        assert_eq!(
            branch.review(&parent).expect("review").attribution_limit(),
            LaneAttributionLimit::DecisionAndExecutionChanged
        );
        branch.verify_replay(&parent).expect("branch replay");
    }

#[test]
    fn parent_record_identity_preserves_neutral_input_provenance() {
        let state = LaneSnapshot::initial();
        let (receipt, parent_request) = request(&state, LaneIntent::Contest);
        let parent_inputs = inputs(1, 1, LaneWaveResult::Held);
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );

        let mut first_parent = LaneHistory::new(state).expect("valid");
        first_parent
            .append(&receipt, &parent_request, parent_inputs)
            .expect("append");
        let changed_neutral_inputs = LaneResolvedInputs::new(
            trace(101, 101),
            trace(102, 102),
            trace(103, 103),
            trace(104, 104),
            parent_inputs.execution(),
        );
        let mut second_parent = LaneHistory::new(state).expect("valid");
        second_parent
            .append(&receipt, &parent_request, changed_neutral_inputs)
            .expect("append");

        let first_branch = branch_from_window(
            &first_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("branch");
        let second_branch = branch_from_window(
            &second_parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("branch");
        assert_eq!(
            first_branch.record().result(),
            second_branch.record().result()
        );
        assert_ne!(
            first_branch.identity().parent_record_identity(),
            second_branch.identity().parent_record_identity()
        );
    }

#[test]
    fn branch_rejects_invalid_parent_or_selection_and_detects_tampering() {
        let state = LaneSnapshot::initial();
        let (_receipt, request) = request(&state, LaneIntent::Contest);
        let empty = LaneHistory::new(state).expect("initial state is valid");
        assert!(matches!(
            branch_from_window(&empty, &request, BranchExecutionSelection::matched_parent()),
            Err(LaneBranchError::ParentNotExactlyOneWindow)
        ));

        let (parent, receipt) = committed_parent(LaneIntent::Contest);
        let same_intent = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Contest,
        );
        assert!(matches!(
            branch_from_window(
                &parent,
                &same_intent,
                BranchExecutionSelection::matched_parent()
            ),
            Err(LaneBranchError::NotAnAlternateIntent)
        ));
        assert!(matches!(
            BranchId::new(128),
            Err(LaneBranchError::InvalidBranchId { value: 128 })
        ));

        let bad_execution = LaneExecutionInputs::new(
            trace(5, 0),
            LaneDamage::new(0).expect("bounded"),
            LaneDamage::new(0).expect("bounded"),
            LaneWaveResult::Held,
        );
        let alternate = LaneIntentRequest::new(
            PLAYER_LANER,
            receipt.observation().observation_id(),
            LaneIntent::Stabilize,
        );
        assert!(matches!(
            branch_from_window(
                &parent,
                &alternate,
                BranchExecutionSelection::regenerated(
                    BranchId::new(1).expect("bounded"),
                    bad_execution,
                )
            ),
            Err(LaneBranchError::InvalidBranchExecutionIdentity)
        ));

        let mut tampered = branch_from_window(
            &parent,
            &alternate,
            BranchExecutionSelection::matched_parent(),
        )
        .expect("branch");
        tampered.record.command = LaneIntentCommand::new(
            PLAYER_LANER,
            state.turn(),
            M2_LANE_RULESET,
            receipt.observation().observation_id(),
            StateHash::from_raw(0),
            LaneIntent::Stabilize,
        );
        assert_eq!(
            tampered.verify_replay(&parent),
            Err(LaneBranchError::BranchReplayMismatch)
        );
    }

#[test]
fn branch_parent_record_identity_hash_remains_stable() {
    let state = LaneSnapshot::initial();
    let (receipt, request) = request(&state, LaneIntent::Contest);
    let mut parent = LaneHistory::new(state).expect("valid initial state");
    parent
        .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
        .expect("append");
    let alternate = LaneIntentRequest::new(
        PLAYER_LANER,
        receipt.observation().observation_id(),
        LaneIntent::Stabilize,
    );
    let branch = branch_from_window(
        &parent,
        &alternate,
        BranchExecutionSelection::matched_parent(),
    )
    .expect("matched branch");

    assert_eq!(
        branch.identity().parent_record_identity().value(),
        15_128_027_512_774_469_724
    );
}

#[test]
    fn observation_redacts_latent_state() {
        let first = LaneSnapshot::initial();
        let second = LaneSnapshot::new(
            M2_LANE_RULESET,
            first.turn(),
            LanePhase::Open,
            first.player(),
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(1).expect("bounded"),
                LanePosition::FarSide,
                OpponentPosture::Passive,
            ),
            first.wave(),
            JungleThreatTruth::Absent,
            None,
        );
        let second_observation = observe_player(&second, ObservationId::new(1)).observation();
        assert_eq!(
            observe_player(&first, ObservationId::new(1))
                .observation()
                .opponent()
                .last_known_position(),
            None
        );
        assert_eq!(
            second_observation.opponent().last_known_position(),
            Some(LanePosition::FarSide)
        );
        assert_eq!(second_observation.opponent().health(), HiddenValue::Unknown);
        assert_eq!(
            second_observation.opponent().posture(),
            HiddenValue::Unknown
        );
        assert_eq!(second_observation.jungle_threat(), ThreatReport::Unknown);

        let same_report_different_hidden_state = LaneSnapshot::new(
            M2_LANE_RULESET,
            second.turn(),
            LanePhase::Open,
            second.player(),
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(9).expect("bounded"),
                LanePosition::FarSide,
                OpponentPosture::Aggressive,
            ),
            second.wave(),
            second.jungle_threat(),
            None,
        );
        assert_eq!(
            second_observation,
            observe_player(&same_report_different_hidden_state, ObservationId::new(1))
                .observation()
        );
    }

#[test]
    fn far_side_opponent_report_replays_and_remains_allied_unknown() {
        let initial = LaneSnapshot::initial();
        let state = LaneSnapshot::new(
            initial.ruleset(),
            Turn::new(2),
            LanePhase::Open,
            initial.player(),
            OpponentTruth::new(
                OPPONENT_LANER,
                initial.opponent().health(),
                LanePosition::FarSide,
                initial.opponent().posture(),
            ),
            initial.wave(),
            initial.jungle_threat(),
            None,
        );
        let player_observation = observe_player(&state, ObservationId::new(12)).observation();
        assert_eq!(
            player_observation.opponent().last_known_position(),
            Some(LanePosition::FarSide)
        );
        assert_eq!(
            player_observation.opponent().last_seen_turn(),
            Some(Turn::new(2))
        );
        let allied_observation = observe_allied(&state, ObservationId::new(12)).observation();
        assert_eq!(allied_observation.opponent().last_known_position(), None);
        assert_eq!(allied_observation.opponent().last_seen_turn(), None);

        let (receipt, request) = request(&state, LaneIntent::Stabilize);
        let mut history = LaneHistory::new(state).expect("valid initial state");
        history
            .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
            .expect("append");
        assert_eq!(
            history.records()[0].observation().opponent(),
            player_observation.opponent()
        );
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
    }

#[test]
    fn observation_reports_only_the_bounded_river_side_last_known_threat() {
        let state = LaneSnapshot::initial();
        let river_side = LaneSnapshot::new(
            state.ruleset(),
            Turn::new(3),
            LanePhase::Open,
            state.player(),
            state.opponent(),
            state.wave(),
            JungleThreatTruth::RiverSide,
            None,
        );
        let river_observation = observe_player(&river_side, ObservationId::new(1)).observation();
        assert_eq!(
            river_observation.jungle_threat(),
            ThreatReport::LastKnown {
                region: JungleThreatRegion::RiverSide,
                last_seen_turn: Turn::new(3),
            }
        );
        assert_eq!(
            river_observation.jungle_threat().last_known_region(),
            Some(JungleThreatRegion::RiverSide)
        );
        assert_eq!(
            river_observation.jungle_threat().last_seen_turn(),
            Some(Turn::new(3))
        );
        assert_eq!(
            observe_player(&state, ObservationId::new(1))
                .observation()
                .jungle_threat(),
            ThreatReport::Unknown
        );
    }

#[test]
    fn river_side_observation_replays_without_changing_transition_authority() {
        let initial = LaneSnapshot::new(
            M2_LANE_RULESET,
            Turn::new(0),
            LanePhase::Open,
            PlayerLaneState::new(
                PLAYER_LANER,
                LaneHealth::new(8).expect("bounded"),
                LanePosition::Center,
            ),
            OpponentTruth::new(
                OPPONENT_LANER,
                LaneHealth::new(7).expect("bounded"),
                LanePosition::Center,
                OpponentPosture::Aggressive,
            ),
            WaveState::new(WavePressure::new(1).expect("bounded")),
            JungleThreatTruth::RiverSide,
            None,
        );
        let (receipt, request) = request(&initial, LaneIntent::Contest);
        assert_eq!(
            receipt.observation().jungle_threat(),
            ThreatReport::LastKnown {
                region: JungleThreatRegion::RiverSide,
                last_seen_turn: Turn::new(0),
            }
        );
        let mut history = LaneHistory::new(initial).expect("valid initial state");
        history
            .append(&receipt, &request, inputs(0, 0, LaneWaveResult::Held))
            .expect("append");
        assert_eq!(history.verify_replay(), Ok(history.current_state()));
        assert_eq!(
            history.current_state().jungle_threat(),
            JungleThreatTruth::RiverSide
        );
    }

#[test]
    fn receipt_debug_does_not_reveal_the_host_state_hash() {
        let state = LaneSnapshot::initial();
        let receipt = observe_player(&state, ObservationId::new(1));
        let debug = format!("{receipt:?}");
        assert!(!debug.contains("source_state_hash"));
        assert!(!debug.contains(&state.hash().value().to_string()));
    }

#[test]
    fn withdraw_is_advertised_only_for_a_river_side_last_known_report() {
        let unknown = LaneSnapshot::initial();
        assert_eq!(
            observe_player(&unknown, ObservationId::new(9))
                .observation()
                .available_threat_response(),
            None
        );

        let river_side = river_side_state();
        let observation = observe_player(&river_side, ObservationId::new(9)).observation();
        assert_eq!(
            observation.available_threat_response(),
            Some(LaneIntent::Withdraw)
        );
        assert_eq!(
            observation.available_intents(),
            [
                LaneIntent::Stabilize,
                LaneIntent::Contest,
                LaneIntent::Yield,
                LaneIntent::Recall
            ]
        );
    }

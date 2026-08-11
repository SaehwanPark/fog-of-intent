# M6 Calibrated Outlier Detection and Representative Replay Handoff

## Summary
Delivered `ScriptedAgentCalibratedOutlierReplayReport` under schema `m6-scripted-agent-calibrated-outlier-replay-v1` and rule `m6-calibrated-outlier-representative-replay-v1`. It calibrates outlier detection from a verified profile-aware comparison report against an explicit threshold magnitude (2) and deterministically traces a qualified candidate to a verified committed decision replay record.

## Artifacts & Evidence
- `src/agent.rs`:
  - `SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_SCHEMA`
  - `SCRIPTED_AGENT_CALIBRATED_OUTLIER_REPLAY_RULE`
  - `SCRIPTED_AGENT_CALIBRATED_OUTLIER_THRESHOLD_MAGNITUDE`
  - `ScriptedAgentCalibratedOutlierReplayStatus`
  - `ScriptedAgentCalibratedOutlierReplayReport`
  - `calibrated_outlier_detection_and_representative_replay_is_deterministic` test.
- `Cargo.toml` / `Cargo.lock`: Version bumped to `0.1.171`.
- `CHANGELOG.md`: Added release notes for `0.1.171`.
- `ROADMAP.md`: Marked `Calibrate outlier detection and select representative replays deterministically` complete.
- `SPEC.md`, `ARCHITECTURE.md`, `LESSONS.md`: Reconciled and updated.

## Verification
- `cargo fmt --all -- --check` passed.
- `cargo clippy --locked --all-targets --all-features -- -D warnings` passed.
- `cargo test --locked` passed (257 library tests, 7 binary integration tests, 3 doc-tests).
- `python3 scripts/check_repository.py` passed.

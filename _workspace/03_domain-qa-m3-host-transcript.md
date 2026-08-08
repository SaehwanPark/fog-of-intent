# Domain QA — M3 Host-Backed Transcript

## Scope

Reviewed the `m3-cli-host-v1` application-edge fixture against the M2/M3
authority and information-boundary contracts.

## Findings

- `CliScenarioHost` owns lifecycle, draft, in-memory snapshot, and replay/
  debrief coordination; the lane contract remains the transition authority.
- Resolved execution inputs are supplied at construction. The fixture does not
  create randomness or read hidden state through actor-facing values.
- `CliHostOutput` returns observations, bounded history counts, window outcomes,
  saved/load identifiers, replay status, and the redacted debrief report. It
  does not return `LaneSnapshot`, transition records, hidden opponent truth, or
  terminal state hashes.
- Plan text is explicitly bounded to existing `LaneIntent` names. Message and
  contingency text is staged metadata and is not silently converted into lane
  state.
- Replay with a run ID verifies the saved snapshot identified by that ID;
  replay without an ID verifies current history. Save/load is in-memory only.
- Branch execution, terminal rendering, persistent storage, and
  keyboard/screen-reader evidence remain explicitly open.

## Evidence

- Three focused host tests passed.
- Full pinned Rust suite: 122 tests passed; one compile-fail Rustdoc test
  passed.
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

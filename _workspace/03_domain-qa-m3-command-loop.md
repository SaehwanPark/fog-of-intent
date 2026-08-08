# Domain QA — M3 Command Loop

## Scope

Reviewed `m3-cli-command-loop-v1` as the outer stdin/stdout adapter around the
bounded host and pure terminal-text projection.

## Findings

- The loop passes each input line to `CliScenarioHost` and renders only the
  resulting actor-valid output or bounded error.
- Malformed commands emit one actionable error and the loop continues; no
  retry, prompt, ANSI styling, or hidden-state access is introduced.
- `quit` is rendered before the loop returns `Quit`; clean end-of-input returns
  `EndOfInput` without fabricating a command result.
- The binary now exposes the deterministic two-window fixture only. Scenario
  selection, persistent storage, branch execution, and human accessibility
  evidence remain open.

## Evidence

- Three focused command-loop tests passed.
- Full pinned Rust suite: 129 tests passed; one compile-fail RustDoc test
  passed.
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

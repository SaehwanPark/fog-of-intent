# Domain QA — M3 Terminal Text Projection

## Scope

Reviewed `m3-cli-terminal-text-v1` as a pure presentation edge over the
actor-valid host contract.

## Findings

- `render_output` consumes only `CliHostOutput`; it never receives a snapshot,
  transition record, state hash, or resolved input.
- Observation text distinguishes reported opponent/threat values from
  unknown values and lists only actor-advertised intents.
- Debrief text uses the redacted `ScenarioDebriefReport` projection and omits
  source hashes and execution provenance not present in that DTO.
- `render_error` keeps host domain failures in bounded categories and echoes
  only sanitized parser/user context; control characters become `?`.
- Empty history has a stable `records=0 status=open` line, and all output is
  plain labeled text without ANSI escapes.
- This is not terminal I/O or accessibility evidence. Command-loop behavior,
  keyboard/focus behavior, and screen-reader semantics remain open.

## Evidence

- Three focused renderer tests passed.
- Full pinned Rust suite: 126 tests passed; one compile-fail RustDoc test
  passed.
- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

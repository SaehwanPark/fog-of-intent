# Request Summary

## Requested Outcome

Define typed adapter write requests for the stable `message`, `plan`,
`contingency`, `commit`, and `advance` grammar verbs. Keep payloads borrowed and
host authorization/transition execution outside the CLI module.

## Roadmap Milestone

M3 — CLI Reference Experience, write-request adapter foundation.

## In Scope

- Map grammar values to `CliWriteRequest` variants with explicit payloads.
- Preserve distinctions between message, plan, and contingency text.
- Represent commit and advance as separate typed requests.
- Add typed non-write errors and transcript-style tests; synchronize core docs.

## Non-Goals

- No domain intent mapping, legality validation, host lifecycle, transport,
  execution inputs, terminal rendering, or committed-history mutation.

## Verification

- `cargo +1.96.0 fmt --check`
- `cargo +1.96.0 clippy --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- repository checker and checker unit tests
- `git diff --check`

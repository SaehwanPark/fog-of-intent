# M3 CLI Write Requests Handoff

## Outcome

Added typed adapter write requests for message, plan, contingency, commit, and
advance grammar values. Payloads remain borrowed and distinct; host validation
and transition authority are unchanged.

## Changed Files

`src/cli.rs`, write-request tests, package metadata, core project documents, and
the inspectable design/QA artifacts for this slice.

## Verification and QA

Full locked Rust/repository checks pass. Domain QA status is `pass`; see
`_workspace/03-domain-qa-m3-cli-write-requests.md`.

## Limits and Next Slice

This is an adapter request foundation. Domain mapping, host lifecycle, terminal
rendering, transport, and committed-history mutation remain unimplemented.

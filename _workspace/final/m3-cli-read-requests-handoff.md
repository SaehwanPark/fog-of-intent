# M3 CLI Read Requests Handoff

## Outcome

Mapped `observe`, bounded `inspect`, and contextual `help` to typed read-only
adapter requests with actor-visible target restrictions and static command
metadata.

## Changed Files

`src/cli.rs`, read-request/help tests, package metadata, core project documents,
and the inspectable design/QA artifacts for this slice.

## Verification and QA

Full locked Rust/repository checks pass. Domain QA status is `pass`; see
`_workspace/03-domain-qa-m3-cli-read-requests.md`.

## Limits and Next Slice

This is a read-only adapter foundation. Terminal rendering, host lifecycle,
domain flow execution, persistence, and accessibility remain unimplemented.

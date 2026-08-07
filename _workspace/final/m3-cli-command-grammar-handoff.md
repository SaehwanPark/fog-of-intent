# M3 CLI Command Grammar Handoff

## Outcome

Added a dependency-free typed parser for the stable M3 in-session command
grammar. It returns borrowed adapter payloads and typed parse errors without
touching simulation state or terminal I/O.

## Changed Files

`src/cli.rs`, parser tests, package metadata, core project documents, and the
inspectable design/QA artifacts for this slice.

## Verification and QA

Full locked Rust/repository checks pass. Domain QA status is `pass`; see
`_workspace/03-domain-qa-m3-cli-command-grammar.md`.

## Limits and Next Slice

This defines grammar only. Host lifecycle, rendering, domain mapping, save/load,
replay/debrief flows, and accessibility remain unimplemented.

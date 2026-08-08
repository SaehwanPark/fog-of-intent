# M6 Scenario-Frequency Codec Request Summary

## Target slice

Add a bounded line-oriented codec for the validated
`m6-scripted-agent-fixture-frequency-v1` report. Preserve its fixed schema,
selection total, and two ordered scenario-count rows while requiring decoded
text to match an already verified report before returning trusted evidence.

## Required behavior

- Encode exactly five newline-terminated lines with closed field names and row
  IDs under a 4096-byte bound.
- Decode with bounded parsing, rejecting unknown/duplicate/missing fields,
  wrong schema, malformed counts, wrong row IDs/order, count-sum mismatch,
  extra lines, and oversized input.
- Compare structurally valid decoded values with a constructor-validated
  expected report and return a bounded mismatch error on tampering.
- Round-trip both the four-selection 2/2 report and the singleton 1/0 report.

## Non-goals

This slice does not add durable export, arbitrary report construction,
population/distributional sampling, outcomes, persistence, providers, or
transport authority.

## Verification

Use canonical text, round trips, malformed-field tables, provenance/count
tampering, inclusive count values, and oversized input in one focused agent
test. Run the pinned Rust, repository, Python, formatter, Clippy, and diff
gates.

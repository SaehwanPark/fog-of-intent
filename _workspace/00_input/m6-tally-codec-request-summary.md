# M6 Selected-Intent Tally Codec Request Summary

## Target slice

Add a bounded line-oriented codec for the verified selected-intent tally
report, preserving its actor-safe fields and ordered profile rows.

## Required behavior

- Encode/decode `m6-scripted-agent-matched-scenario-tally-v1` with bounded text
  and closed profile/rule/count fields.
- Preserve observer, pair/observation counts, row order, and all five intent
  counters through round trip.
- Reject oversized, wrong-schema, unknown/duplicate/missing fields, malformed
  row values, wrong profile/rule identities, count mismatches, and extra lines.
- Parse within the shared fixed byte/line bounds before exposing a report.

## Non-goals

This codec does not persist files, run policies, generate scenarios or
populations, calculate outcomes, or provide provider/calibration support.

## Verification

Add focused canonical round-trip and malformed-input cases over a verified
tally, then run the pinned Rust, repository, Python, and diff gates.

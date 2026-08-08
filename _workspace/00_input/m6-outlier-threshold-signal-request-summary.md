# M6 Outlier-Threshold Signal Request Summary

## Requested slice

Add a provisional, fixed-fixture threshold signal over the existing verified
largest signed intent-count delta. This is a bounded metric-side signal, not
calibrated outlier detection or representative replay selection.

## Required contract

- Use `m6-scripted-agent-tally-outlier-threshold-v1`.
- Use `m6-fixed-intent-delta-outlier-threshold-v1`.
- Use an inclusive magnitude threshold of exactly `2`.
- Return only `above_threshold`, `below_threshold`, or `no_candidate`.
- Read the existing verified comparison without rerunning policy or mutating
  any report.

## Explicit limits

Do not claim outlier calibration, causal attribution, population inference,
representative replay selection, persistence, provider behavior, or human
evidence.

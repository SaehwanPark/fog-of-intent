# M6 Profile Intent Distribution Summary Request Summary

## Target slice

Add a deterministic intent-share projection to each row of the existing
verified profile-aware selected-intent tally.

## Required behavior

- Preserve `m6-scripted-agent-matched-scenario-tally-v1`, profile/rule order,
  counts, and codec behavior unchanged.
- Expose five ordered integer basis-point shares in
  `[Stabilize, Contest, Yield, Recall, Withdraw]` order.
- Derive shares only from each constructor-verified row's bounded observation
  count; assign any integer remainder to the final Withdraw row so the sum is
  exactly 10,000.
- Provide a pure Markdown projection containing only profile/rule labels,
  bounded observation counts, and the five ordered shares.

## Non-goals

This is not a policy rerun, a population distribution, an outcome or strategic
metric, a build comparison, durable export, persistence, calibration, or human
evidence. Broader population-level metrics remain open.

## Verification

Extend the existing profile-aware tally regression with exact cautious 7/1,
risk-taking 8/0, and yielding 8/0 intent counts, shares, stable order, and
complete Markdown. Rerun all pinned gates.

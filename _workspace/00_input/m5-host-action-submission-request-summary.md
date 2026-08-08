# M5 Host-Action Submission Request Summary

## Requested Outcome

Extend the validated actor-action boundary with host-owned fixture submission:
append one accepted action through the existing lane/history transition and
close exactly one window, while stale, duplicate, closed, or transition-failed
actions fail without exposing raw domain details.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded host-owned action submission slice.

## In Scope

- `CliScenarioHost::submit_actor_action` over the existing DTO validator and
  deterministic `advance` path.
- Explicit actor-safe transition-rejection category.
- Focused first/second-window success, duplicate/stale, complete-window, and
  malformed-execution regressions.
- Canonical/workspace docs and LESSONS synchronization.

## Non-Goals

- Network/MCP framing, retries, reconnect, simultaneous submissions, or
  privileged experiment-controller tools.
- New transition mechanics, execution generation, or raw host error payloads.
- Plan/message/contingency submission through this actor DTO path.

## Authority and Evidence

The host remains the sole owner of append, transition invocation, and window
closure. The lane validator remains the legality authority. Failed validation
or execution must leave history unchanged; successful submission returns the
existing actor-safe `Advanced` projection.

## Verification

Focused host tests cover two successful fixture submissions, reused-action
rejection, complete-window rejection, and malformed-execution redaction. Full
repository format, Clippy, Rust, repository, Python, and diff checks remain
required.

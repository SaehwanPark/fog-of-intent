# M5 Core Boundary Guard Request Summary

## Requested Outcome

Verify that deterministic core modules remain independent of transport and async
orchestration while adapter-edge work is still deferred.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- Maintain an explicit list of deterministic core Rust modules.
- Reject async syntax/runtime imports, wall-clock imports, and network transport
  types in those modules through the repository checker.
- Add focused checker coverage for forbidden and clean core inputs.
- Synchronize canonical and workspace evidence with the verified boundary.

## Non-Goals

- Adding transport framing, an async runtime, reconnect, networking, provider
  integration, or a complete MCP adapter.
- Restricting synchronous edge I/O in `main`, command-loop, storage, or text
  projection modules.
- Claiming that source ownership checks prove runtime transport behavior.

## Verification

- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- The standard Rust format, Clippy, test, and diff gates.

## Evidence Limits

The guard proves only that the checked core source does not import or use the
listed async, wall-clock, or network transport primitives. Adapter behavior,
transport protocol compatibility, reconnect, and complete MCP execution remain
future work.

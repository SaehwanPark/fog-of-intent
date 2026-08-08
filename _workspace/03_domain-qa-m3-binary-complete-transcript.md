# Domain QA — M3 Binary Complete Transcript

## Scope

Review the clean-checkout executable transcript for command ordering, process
status, actor-safe labeled output, and preservation of host/kernel authority.

## Required checks

- Verify the real binary, not a direct host call, completes both fixture windows
  from the documented public command sequence.
- Verify observation, staged communication/contingency, commits, both advances,
  replay, debrief, and quit are represented by labeled output.
- Verify success status and empty stderr, with no true-state/hash/raw-domain
  leakage or changed persistence behavior.
- Verify docs call this bounded fixture transcript evidence and keep complete
  playable behavior and human accessibility claims open.

## Claim limit

This slice proves one executable transcript over the existing deterministic
fixture. The full suite has 154 Rust unit tests, seven binary integration tests,
and one compile-fail RustDoc test. It does not prove a complete scenario,
balance, user enjoyment, branch graphs, durable storage, or human accessibility.

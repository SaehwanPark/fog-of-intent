# M6 Operational-Log Codec and Store Design

## Goal and roadmap milestone

Give the bounded caller-owned operational event log a stable text identity and
an injected persistence edge without turning it into simulation history or a
runtime diagnostics system.

## Evidence boundary

The evidence question is whether a closed, payload-free sequence of at most
16 labels can be encoded, rejected when malformed, and stored independently
of existing host and batch cursor artifacts. This is codec/store evidence
only; it does not establish runtime event production, crash recovery, or
operational observability.

## Contract

`ScriptedAgentOperationalLog` uses codec schema
`m6-scripted-agent-operational-log-v1`, a 4096-byte inclusive input bound, and
two header lines followed by one `event=` line per record. The five event IDs
are closed and ordered. The decoder rejects unknown, duplicate, missing,
unsupported, invalid, over-count, over-line, and oversized input before
returning a trusted log.

`ScriptedAgentOperationalLogStore` delegates bounded bytes to the injected
file-store edge with `.foi-operational-log` and `.foi-operational-log.tmp`
suffixes. It shares run-ID validation and atomic replacement, but its path is
distinct from `.foi-artifact` and `.foi-batch-run`; same-root/same-run-ID
coexistence is part of the contract.

## Authority and information limits

The log contains event IDs only. It has no actor observation, true state,
decision, input, hash, trace, duration, provider, or raw error payload. The
codec and store own no policy, legality, transition, execution, history,
replay, scheduling, or scenario authority.

## Verification contract

The focused evidence covers exact canonical text, round trips, every closed
malformed branch, the 4096/4097 byte boundary, the inclusive 16-entry bound,
same-root/same-run-ID coexistence with host and checkpoint artifacts, and
storage/decode failures that leave caller-owned logs unchanged. Full Rust,
repository, Python, formatter, Clippy, and diff gates are required.

## Open boundaries

Runtime producers beyond the existing caller-driven batch/checkpoint wrappers,
automatic failure detection, crash recovery, rotation, tracing/transport,
durations, diagnostics, external export, scheduling, durable scenario-wide
replay, providers/models, and human operational evidence remain open.

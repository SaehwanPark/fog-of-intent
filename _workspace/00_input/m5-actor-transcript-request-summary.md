# M5 Provider-Neutral Actor Transcript Request Summary

## Requested Outcome

Define a bounded provider-neutral actor transcript record that identifies the
tool/schema used for one actor-facing operation, its bound observation, and a
closed accepted/rejected outcome without retaining payloads or raw errors.

## Roadmap Milestone

M5 — Model-Agnostic MCP Play, bounded actor-protocol library evidence.

## In Scope

- A versioned `m5-actor-transcript-v1` DTO with closed operation/tool IDs for
  observation, draft, draft receipt, commit, and action.
- Constructor-owned tool/schema IDs and a closed accepted/rejected result.
- Exact bounded encode/decode coverage, including schema/tool mismatch and
  malformed-field rejection.
- Actor-visible redaction assertions for payload, state, hash, execution, and
  transport/provenance fields.

## Non-Goals

- Capturing raw requests/responses, prompts, provider/model metadata, or
  operational logs.
- Persisting transcripts, driving a transport, replaying simulation history,
  or adding authorization, transition, or host lifecycle authority.

## Expected Outputs

- `ActorTranscriptDto` plus closed tool/result enums and exact codec.
- Focused protocol evidence and synchronized core/workspace documents.

## Verification

- `cargo +1.96.0 fmt --all -- --check`
- `cargo +1.96.0 clippy --locked --all-targets --all-features -- -D warnings`
- `cargo +1.96.0 test --locked`
- `python3 scripts/check_repository.py`
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts -p 'test_*.py'`
- `git diff --check`

## Evidence Limits

Evidence is one pure library transcript codec over one fixed closed catalog.
It does not establish transport delivery, persistence, provider compatibility,
complete MCP behavior, or human accessibility.

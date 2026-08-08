# Domain QA — M4 Candidate Breadth

## Status

Pass for the bounded candidate-generation evidence.

## Reviewed inputs

- `ScriptedAgent::generate_candidates` and its safe/RiverSide regression.
- Actor-visible advertised intents and visible threat-response fields.
- Stable selection, host validation, and synchronized M4 documentation.

## Findings

- Safe generation returns four unique advertised intents; RiverSide adds only
  the one visible `Withdraw` response for five total.
- No candidate is sourced from true state, hidden hashes, execution inputs, or
  random sampling.
- Existing selection and actor-bound request behavior remain unchanged.
- The evidence is correctly labeled candidate breadth rather than strategic
  diversity or population behavior.

## Claim limits

This proves a two-observation fixture generation relation only. It does not
prove creativity, action diversity, outcomes, strategic quality, or human
realism.

## Verification evidence

The focused agent suite has ten tests. The full repository target is 164 Rust
unit tests, seven binary integration tests, and one compile-fail RustDoc test.

# ADR-0001: Host-Owned Authoritative Transition Boundary

- **Status:** Accepted for M1 implementation
- **Date:** 2026-08-04
- **Scope:** Deterministic kernel, input resolution, history, replay, and future
  adapters

## Context

Fog of Intent needs reproducible outcomes while allowing human, scripted, and
future model-driven clients to submit decisions through different surfaces.
The initial repository has no implementation yet, so the boundary must be
recorded before the M1 kernel introduces public types or dependencies.

The core distinctions are documented in
[`docs/TERMINOLOGY.md`](../TERMINOLOGY.md): true state is not an observation,
intent is not execution, and runtime diagnostics are not committed history.

## Decision

The application host is the sole owner of simulation truth. It invokes a
deterministic transition kernel as a pure evaluator and commits history under
its own authority:

```text
prior true state + validated commands + resolved inputs + ruleset
  -> ordered events + attributed effects + next true state + state hash
```

- The host derives actor-specific observations and legal-action references.
- The host closes a decision window and validates the submitted command set
  before transition evaluation.
- An edge resolver supplies explicit, versioned resolved inputs. The transition
  boundary does not create an RNG, read the wall clock, perform I/O, await an
  actor, or consult a model provider.
- The kernel evaluates one transition and returns events, effects, next state,
  and hash to the host. The host commits that result append-only and provides
  the inputs needed for replay verification.
- CLI, MCP, research, persistence, and any future GUI are adapters or edge
  services. They may translate, authorize, collect, render, persist, or inspect
  through explicit contracts, but they may not own legality, transition
  semantics, hidden-state inference, committed history, or replay authority.
- Privileged research inspection is explicit and cannot be passed to ordinary
  actor policies as if it were an observation.

## Consequences

This keeps repeated runs and cross-interface replays comparable, makes hidden
state leakage testable, and lets edge concerns evolve without creating a second
simulation engine. It requires explicit input and compatibility types, a host
window-closure step, and append-only history earlier than a convenience-first
CLI would. Those costs are accepted because they are core product invariants.

This ADR establishes a target contract for M1; it is not evidence that any
kernel, adapter, persistence layer, or replay engine is already shipped.

## Rejected alternatives

- Let each adapter resolve actions locally: rejected because results and legality
  would diverge across interfaces.
- Generate randomness inside the transition: rejected because unrelated draws
  could shift replay outcomes and input identity would be hidden.
- Let runtime logs stand in for history: rejected because logs are mutable,
  incomplete, and not sufficient for causal replay.

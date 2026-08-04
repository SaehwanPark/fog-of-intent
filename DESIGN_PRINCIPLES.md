# Design Principles

This is the concise index for implementation decisions. The proposal and
roadmap contain the longer rationale and sequencing; this file states the
constraints that should survive local refactors.

1. **Intent is not execution.** A plan, commitment, or command describes a
   decision boundary; it does not guarantee mechanical success.
2. **Information is actor-specific.** True state, belief, observation, report,
   and research inspection remain distinct. Ordinary actors act only on the
   information authorized for their role.
3. **One host owns truth.** Validation, ordering, transition, resolved inputs,
   committed history, replay, and causal debriefs have one authoritative owner.
   Adapters present and submit; they do not simulate.
4. **Randomness is explicit data.** Stochastic outcomes are resolved at the
   edge with stable stream and draw identities before deterministic transition
   evaluation.
5. **History is inspectable.** Committed transitions are append-only and retain
   enough provenance to distinguish decisions, coordination, execution, and
   luck.
6. **Bounded behavior is explainable.** Candidate generation, evaluation,
   selection, coordination, and execution are separate concerns. Policies are
   imperfect without becoming arbitrary or omniscient.
7. **Multiple strategies must remain viable.** A scenario should preserve
   meaningful tradeoffs and should not hide one preferred route as the only
   successful answer.
8. **Evidence has limits.** Software and agent tests establish only the
   properties their inputs and models support. Human enjoyment, accessibility,
   trust, learning, and behavioral-validity claims require appropriate human
   evidence.
9. **Vertical slices earn infrastructure.** Add frameworks, adapters,
   persistence, and presentation only when a concrete slice demonstrates their
   need.

See [`docs/TERMINOLOGY.md`](docs/TERMINOLOGY.md) for the controlled vocabulary
and [`docs/adr/0001-authoritative-transition-boundary.md`](docs/adr/0001-authoritative-transition-boundary.md)
for the first architecture decision record.

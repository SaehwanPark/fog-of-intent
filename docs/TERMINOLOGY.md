# Authoritative Terminology

These definitions govern the repository's design and implementation language.
Names may change for readability, but future code must preserve the distinctions
below.

## State and information

- **True state:** The host-owned authoritative state of the simulation at a
  committed point in history. It may contain information that no actor can see.
- **Actor belief:** An actor's internal, possibly mistaken representation of
  what it thinks is true. A belief is not a second simulation authority.
- **Observation:** The host-produced, actor-authorized view available at a
  decision boundary. It may contain uncertainty, last-known information, and
  legal-action references, but not unauthorized latent values.
- **Report:** A human- or actor-facing description of an observation or outcome.
  Reports can be incomplete, delayed, ambiguous, or wrong; they are not a
  privileged state dump.
- **Research inspection:** A separately authorized view for experiment control
  or analysis. It may expose true state, but must never be silently supplied to
  an ordinary actor policy or player-facing interface.
- **Observed structure state:** What one team can tell about one defensive
  structure: a coarse health band, `destroyed`, or `not-visible`. Sight of a
  structure that the team cannot see reports nothing, not even whether it still
  stands. Observed structure state is team-visible shared information.
- **Structure band:** The coarse health state an observation reports for a
  standing structure — `pristine`, `chipped`, or `failing` — classified in exact
  integer basis points of maximum health. Fog trades precision for secrecy here:
  a band is a projection, never the latent health value.

## Decisions and actions

- **Intent:** A strategic aim or posture, such as holding, yielding, or
  preparing a response. Intent expresses what the actor is trying to accomplish,
  not what the kernel must force to happen.
- **Proposal:** A non-binding suggestion offered to another actor or the team.
- **Commitment:** An accepted plan or conditional promise that constrains a
  later decision without bypassing actor authority or legality.
- **Message:** A communication with an author, recipients, content or speech
  act, and any declared urgency or condition. A message can influence policy but
  does not directly mutate state.
- **Command:** A host-submittable request to perform an authorized domain
  action. A command is not valid merely because it is well-formed.
- **Validated command:** A command that the host has checked against the prior
  true state, actor authority, ruleset, and decision window. Validation does not
  promise a favorable outcome.
- **Execution:** The modeled realization of an accepted action under explicit
  execution inputs. Execution can fail or produce an unfavorable result while
  remaining a legal action.

## Transition and history

- **Resolved inputs:** Versioned, explicit inputs supplied to deterministic
  transition evaluation. Categories may include environment, observation,
  policy, communication, coordination, and execution outcomes. The transition
  does not generate hidden randomness.
- **Event:** An ordered domain occurrence recorded by a committed transition.
- **Effect:** An attributed change caused by an event, retaining enough
  provenance to distinguish direct or indirect, immediate or delayed, strategic,
  coordination, execution, and stochastic causes.
- **Committed history:** Append-only authoritative transition records containing
  the validated commands, resolved inputs, events, effects, next-state identity,
  and compatibility metadata needed for replay.
- **Replay:** Re-evaluation of committed inputs from an initial state to verify
  events, effects, next state, and hashes; a terminal snapshot alone is not
  sufficient evidence.
- **Debrief:** A derived explanation of decisions and outcomes that evaluates
  intent, coordination, execution, and luck using the information available at
  the relevant decision time.
- **Turn note:** A host-derived explanation attached to a composed match turn that recorded
  nothing, phrased only from facts the actor-visible observation already reports. It is not
  an event, effect, or authoritative cause: event counts, history, replay, and state hashes
  are unaffected, and a turn that did work carries no note.

## Authority rules

- The **host** owns true state, legality, ordering, transition evaluation,
  history, replay, and debrief generation.
- An **adapter** translates between the host contract and a presentation or
  external protocol. It must not reimplement legality, transition rules, hidden
  state inference, or persistence authority.
- Exact structure health is latent host-authoritative data. A player projection
  reports only the observed structure state above, so an adapter or renderer must
  not print, summarize, or re-derive an exact health value.
- An **ordinary actor** can submit only commands and messages permitted by its
  actor authority and actor-visible observation.
- A **privileged controller** may inspect or generate experiment inputs only in
  an explicitly marked research context. Its outputs are not ordinary actor
  evidence.

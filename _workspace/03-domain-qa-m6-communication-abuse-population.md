# M6 Communication-Abuse Population Domain QA Review

## Milestone and scope review

The change implements the communication-abuse policy population slice in M6
as a bounded caller-declared population report over repeated invalid message
payloads: `ActorCommunicationAbusePopulationReport` under schema
`m6-actor-communication-abuse-population-v1`.

## Evidence and boundary checklist

- [x] Schema is explicitly versioned: `m6-actor-communication-abuse-population-v1`.
- [x] Population size is bounded: 1..=4 (`MAX_ACTOR_COMMUNICATION_ABUSE_POPULATION = 4`).
- [x] Empty population is rejected with `EmptyPopulation`.
- [x] Over-capacity population is rejected with `PopulationTooLarge`.
- [x] Target bounds (sender > 0, recipient > 0, observation_id > 0) are enforced.
- [x] Rejection code is verified against `ActorProtocolCodecError::InvalidValue`.
- [x] Message text is not stored, routed, or delivered in the report.
- [x] Debug representation is checked for privacy against raw payload leakage.
- [x] No new dependencies, async runtime, or external network access added.
- [x] Replay, transition, history, and host authorities remain untouched.

## Verdict

PASS. The slice is minimal, complete, verified, and adheres strictly to
architectural and domain boundaries.

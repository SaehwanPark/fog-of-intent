# M5 Actor Draft Readback Handoff

## Outcome

Implementation is complete; independent three-pass review is pending.
The new host readback exposes only the requesting actor's actor-protocol-staged
metadata through existing bounded draft DTOs and does not mutate host state;
legacy CLI draft text remains on its existing path.

## Verification target

The implementation provides one focused host readback test. The expected full
evidence is 226 Rust unit tests, 7 binary tests, and 3 RustDoc tests, with 25
protocol, 12 session, and 33 host focused tests; formatter, Clippy with
warnings denied, repository checker, 15 Python policy tests, and diff checks
must pass at the reviewed head.

## Limits

This is in-process actor-owned metadata readback. Communication delivery,
recipient visibility, simultaneous drafts, transport, persistence, reconnect,
provider integration, and richer plan/contingency semantics remain open.

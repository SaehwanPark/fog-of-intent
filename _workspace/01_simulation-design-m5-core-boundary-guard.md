# M5 Core Boundary Guard Design

## Contract

`scripts/check_repository.py` owns a small static boundary check over the
deterministic core module list. It rejects:

- async runtime imports and `async`/`.await` syntax;
- wall-clock imports from `std::time`; and
- network transport imports and socket types.

Synchronous stdin/stdout, filesystem, and rendering remain edge concerns and
are intentionally outside the scanned core list.

## Authority and Limits

The checker does not execute or replace the Rust core. It adds no transition,
host, session, history, replay, transport, or provider authority. It is
ownership evidence that keeps future adapter work from pulling runtime concerns
into deterministic modules.

## Verification Contract

- A focused checker fixture rejects each forbidden concern in a core file.
- The same fixture passes after the core file is clean.
- The repository checker passes against every current core module.

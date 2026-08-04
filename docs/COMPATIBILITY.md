# Artifact and Replay Compatibility

This document defines the minimum compatibility contract that M1 fixtures must
follow before schemas or replay bundles become external artifacts. It is a
policy for future implementation, not evidence that serializers or migrations
exist today.

## Versioned identity

Every authoritative artifact records:

- an artifact or schema identifier;
- a compatibility version in `major.minor.patch` form;
- the ruleset identifier and version;
- the scenario identifier and version when a scenario is involved; and
- the producer/tool version when it can affect interpretation.

The minimum artifact set is the manifest, initial state, committed history,
snapshot, replay-hash record, and any derived debrief or metrics file. Public
protocol DTOs, prompts, agent profiles, and extractors use their own identifiers
and versions rather than borrowing internal domain versions.

## Compatibility rules

- A major-version change may alter meaning or authoritative representation and
  is incompatible by default. A reader must reject it unless an explicit
  migration is named and tested.
- A minor-version change may add optional, non-authoritative metadata or fields
  with a documented deterministic default. It must not silently change the
  authoritative hash representation or event ordering.
- A patch-version change is limited to corrections that preserve interpretation,
  canonical ordering, and authoritative equality.
- Ruleset and scenario versions are part of replay identity. Replay verification
  must reject an exact-input mismatch rather than silently substitute the newest
  version.
- Each identifier/version pair binds an immutable canonical semantic definition.
  A behavior-, ordering-, or hash-affecting change must increment the relevant
  ruleset, scenario, schema, or hash-representation version; changing only a
  fixture version is insufficient. When a definition digest is implemented, it
  is recorded alongside the version.
- Unknown fields in authoritative state, history, and hash inputs are not
  ignored. Readers must fail closed until the schema version explicitly defines
  their treatment.
- Migrations are explicit, deterministic, versioned transformations with their
  own fixtures and before/after hash evidence. No implicit migration is allowed
  in the first M1 slice.

## Replay and fixture requirements

An M1 replay fixture records the initial state, ordered validated commands,
resolved inputs, ruleset/scenario identity, expected events and effects, terminal
state hash, the expected next-state hash for every committed transition, and
schema versions. Replay verification compares every transition hash as well as
events, effects, and the terminal state; a terminal snapshot alone is
insufficient.

Fixtures are immutable test inputs. If a ruleset, scenario, schema, or canonical
hash representation changes incompatibly, create a new fixture version and keep
the old fixture available for the compatibility policy it claims to support.

## External use

No schema or replay bundle is externally supported until its compatibility
version, migration policy, and clean-checkout verification are recorded. This
policy does not promise backward compatibility for unimplemented future formats.

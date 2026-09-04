# Artifact and Replay Compatibility

This document defines the minimum compatibility contract for the local M1
snapshot/history fixtures and the internal M2 replay identities before any
schema or replay bundle becomes an external artifact. The repository has a
strict dependency-free `1.0.0` M1 text codec; it does not provide migrations or
external backward-compatibility support.

M9 complete match `m9-complete-match-v1` is retired the same way. Its transition changed
meaning when actor presence became a resolution input, so its catalog
(`m9-complete-match-catalog-v1`) and both scenario ids moved to `-v2` and the interactive host
contract moved to `m9-interactive-match-host-v3`, and moved again to
`m9-interactive-match-host-v4` when the commit-strength words `light`, `committed`, and
`all-in` joined the force slot of `contest` and `siege`. `v4` is purely additive: an integer
means exactly what it meant under `v3`, so every `v3` script, MCP call, and recorded plan
remains valid, and a word resolves to a number in the host before the authority sees the
action. The retired identities have no release, tag,
external codec, or stored artifact, so there is no migration path and no v1 plan remains
executable; a reader must reject `v1`-labeled match input rather than resolving it as `v2`.
The map ruleset identifier is deliberately unchanged, because the presence test reuses the map
layer's existing beat distances and alters none of them.

Two identities were added, not moved. The interactive session `m9-match-onboarding-v1` and its
teaching plan `scenario-complete-onboarding-v1` (a teaching plan has no scripted action list) run
the same `v4` host, and a teaching plan is resolved only by `CompleteMatchCatalog::find`:
`CompleteMatchCatalog::all()` remains the two `-v2` benchmark plans, so the print-and-exit
transcript `m9-complete-match-replay-v1` and every hash quoted from it are unchanged. A reader
that does not know the teaching id must reject it rather than substituting a benchmark.
Separately, `rotate` now accepts every canonical sector spelling that `observe` prints
(`lane:mid:far-side`, `base:opposing`, `river:bot`) in addition to the underscore aliases it
already accepted. That is **additive input acceptance**: a wider set of inputs resolves to the
same sectors it always named, no previously accepted input changes meaning, and no projection
changed — so the host identifier stays `v4` and no replay identity moves.

One removal, recorded for completeness. `OpponentSighting::LastKnown` and the host report's
matching certainty case were deleted: declared, never constructed, and never written to a stored
artifact, replay, or protocol payload — an in-process projection type only. No identifier moves
and no reader can ever meet an old value, which is what makes retiring it legitimate under this
contract rather than a migration to write. The lane subsystem keeps its own `LaneBelief::LastKnown`,
which the lane observation genuinely produces.

M2 v1 is retired internal history. It has no release, tag, external codec, or
supported artifact, so there is no migration path. The current M2 contract uses
ruleset `4`, v3 observation/profile/replay/strategy/scenario/debrief/branch
identities, and explicit base-record replay IDs; any v1/v2 M2 input must fail
closed.

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

## Published contract: reject on mismatch (decision D7)

Reject-on-mismatch is not a missing feature; it is the contract this project publishes
until a second artifact version exists to migrate from. A loader compares the recorded
version and fails closed with an error that names the artifact, the expected version, and
the actual version with its line — `SerializationError::UnsupportedVersion`, produced by
`check_version` in `src/serialization/helpers.rs` for the `1.0.0` snapshot and history
schemas. No loader guesses, defaults, or coerces.

This is a deliberate choice, not a deferred one, and it carries a binding rule:

- **A breaking change to an artifact that circulates must ship its migration in the same
  slice.** Same pull request, same tests: the migration plus its fixtures and before/after
  hash evidence. Splitting them means the first reader to meet a new artifact is a user with
  an unreadable run directory and no path forward.
- **Retiring an identity and re-identifying the content is the other legitimate response**, and
  is what the M9 slices did (`m9-complete-match-v1` → `-v2`, host `v3` → `v4`). It is permitted
  only while the retired identity has no release, no tag, no published codec, and no artifact
  stored outside this repository — the condition each retirement above states explicitly. The
  moment a run directory, replay, or transcript is shared outside the project, that escape hatch
  closes for anything that artifact records, and only the rule above applies.
- **Additive changes are named as additive.** Host `v3` → `v4` is additive: an integer in a
  force slot means what it meant, and a new token is accepted beside it. An additive change must
  be checkable as additive, so it is stated as such here rather than left to a reader.

The release posture in `docs/decision_brief_20260830.md` is what makes the second rule honest:
no tag and no release-ready language before the human-evidence gate, so nothing shipped yet
obliges a migration. Revisit this contract when a second artifact version exists, or when run
directories are shared outside the project — whichever comes first.

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

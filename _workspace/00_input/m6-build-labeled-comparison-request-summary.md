# M6 Build-Labeled Comparison Request Summary

## Target slice

Add caller-declared build labels to the existing verified fixed-fixture
frequency comparison so a bounded delta can retain which declared baseline and
candidate labels were compared.

## Required behavior

- Define a bounded `ScriptedAgentBuildId` value with a stable numeric accessor.
- Preserve the existing comparison constructor and behavior for unlabeled
  caller-declared reports.
- Add a constructor that attaches distinct caller-declared baseline and
  candidate build IDs to the existing ordered comparison fields.
- Expose those labels as metadata only; equality-gate behavior and deltas remain
  unchanged.
- Add focused evidence for distinct IDs, stable order, repeated construction,
  and unchanged no-change/changed comparison outcomes.

## Non-goals

This slice does not derive IDs from binaries, verify source or package identity,
persist build artifacts, infer causality, or claim that a label caused a delta.
It does not add a new codec, population sampling, outcome metrics, or provider
integration.

## Verification

Cover the exact schema/rule IDs, labeled and unlabeled constructors, distinct
build labels, stable ordered deltas, repeated determinism, and existing gate
semantics. Run the pinned Rust, repository, Python, and diff gates.

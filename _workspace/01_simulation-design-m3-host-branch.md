# M3 Host Branch Design

## Boundary

The host remains the sole lifecycle and transition authority. It adapts the
existing `LaneScenarioHistory` first-window record into a temporary immutable
`LaneHistory` parent, calls the pure `branch_from_window` matched-parent
contract, and projects only the resulting counterfactual review. The branch
never replaces host history and does not enter the saved artifact.

## Contract

`branch first` (or `branch` with the omitted point ID) is available only when
the host has exactly one committed window and an alternate staged plan. The
point ID is validated before evaluation. The staged plan is parsed into a
legal alternate `LaneIntent`; execution inputs are copied from the parent
window under the lane's matched-parent policy. The result exposes parent and
branch intent/outcome plus the bounded execution relation. State hashes,
execution traces, branch IDs, and raw domain failures remain private.

The current draft remains available after a read-only branch evaluation, so a
player can compare another alternate plan. Parent replay and save/load remain
authoritative and unchanged.

## Evidence and limits

Focused host tests cover successful comparison, parent immutability, replay and
saved-artifact preservation, missing/unsupported point handling, and malformed
or same-intent plans. Terminal tests cover stable labeled branch text. This
does not establish regenerated execution, branch persistence, branch graphs,
multi-window branching, complete scenario selection, or human accessibility.

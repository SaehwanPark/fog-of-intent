# M4 Profile-Sensitivity Design

## Boundary

The regression supplies two copied observations to the same three pure policy
profiles: the initial safe observation and a visible RiverSide-threat
observation. No true-state value or resolved execution input enters policy
evaluation.

## Expected sensitivity

| Profile | Safe observation | Visible RiverSide threat |
| --- | --- | --- |
| `cautious-laner-v1` | `Stabilize` | `Withdraw` |
| `risk-taking-laner-v1` | `Contest` | `Contest` |
| `yielding-laner-v1` | `Yield` | `Yield` |

All profiles still contain the visible `Withdraw` threat-response candidate in
the threat observation, and every resulting request is passed to the existing
validator.

## Evidence and limits

This is a two-observation library sensitivity check. It shows policy response
to a visible input change, not transition outcomes, strategic value, balance,
role realism, or human behavior.

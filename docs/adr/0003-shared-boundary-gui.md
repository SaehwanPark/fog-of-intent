# ADR-0003: Shared-Boundary GUI Architecture and Presentation-Only Client Contracts

- **Status:** Accepted for M11 implementation
- **Date:** 2026-08-18
- **Scope:** Optional graphical presentation client, host/client boundary, browser support, asset governance, and persistence invariants

## Context

Fog of Intent is a turn-based, AI-native team-strategy simulation with strict determinism, actor-specific information, and causal debriefing. While the command-line interface (`src/cli/`, `src/command_loop.rs`) serves as the primary reference experience, evaluation evidence from M10 usability and accessibility assessments indicates that pure linear text streams introduce cognitive friction when visualizing complex spatial topologies (multi-lane rotations, fog-of-war sightlines), temporal multi-beat timelines (delayed effects, transit progression), and multi-factor causal attribution trees (2D coordination versus execution quadrants).

A graphical user interface (GUI) can improve human spatial and causal comprehension. However, introducing a graphical client carries the severe risk of creating an accidental second simulation engine, leaking latent opponent state through visual renderers, or coupling simulation authority to browser/client lifecycles.

## Decision

The project adopts a **Shared-Boundary GUI Architecture** governed by the following core invariants:

1. **Host-Owned Simulation Truth:**
   - The application host (`src/host.rs`) remains the sole owner of world state, legality validation, window closure, transition evaluation, history appending, replay verification, and persistence.
   - The GUI client is strictly a downstream, presentation-only consumer. It possesses zero simulation authority, zero legality evaluation logic, zero hidden-state inference capabilities, and zero direct persistence authority.

2. **Actor-Visible Versioned DTO Boundary:**
   - The host projects versioned, actor-safe Data Transfer Objects (`GuiMapViewDto`, `GuiTimelineViewDto`, `GuiPlanViewDto`, `GuiDebriefViewDto`, `GuiAccessibilityDto`) under schema `m11-gui-dto-v1`.
   - GUI DTOs contain only information visible to the specific requesting actor. Latent opponent coordinates, true-state hashes, private internal receipts, and uncommitted teammate plans are structurally omitted.

3. **Loopback Transport and Local Packaging:**
   - Communication between the host and client operates over local in-process bindings or loopback IPC (`127.0.0.1`).
   - No remote multi-tenant server or external cloud dependency is introduced.

4. **Browser Baseline and Web Standards:**
   - The GUI presentation target uses standard modern web technologies: semantic HTML5, Vanilla CSS tokens, and JavaScript without heavy framework lock-in.
   - Vector rendering uses standard SVG / Canvas primitives with deterministic coordinate mapping.

5. **Accessibility and Universal Usability (WCAG 2.1 AA):**
   - Every visual and motion element has a direct textual or symbolic equivalent (e.g., `[ALLY]`, `[ENEMY-FOG]`, `[WARD-ACTIVE]`).
   - Color is never the sole carrier of semantic meaning; high contrast and non-color modes are supported.
   - Complete keyboard focus order, aria live-region annotations, and reduced-motion media query support are mandatory.

6. **Asset Governance and Provenance:**
   - All visual and audio assets must have verified content hashes, permissive open-source licenses (MIT/CC0), and explicit provenance records.
   - Procedural vector rendering and original-setting fallback assets are prioritized over third-party raster art.

## Consequences

- **Positive:**
  - Dramatically improves spatial, temporal, and causal comprehension for human players and researchers.
  - Preserves exact parity between CLI, MCP, and GUI projections since all consume the same host-projected actor observations.
  - Prevents hidden-state leakage by enforcing strict compile-time and runtime DTO redaction.
  - Maintains a pure, deterministic, and dependency-light Rust core.

- **Negative / Tradeoffs:**
  - Requires maintaining dedicated DTO projection mappers and schema versioning.
  - Increases the testing surface to verify that GUI DTOs never leak latent state or drift from host observations.

## Rejected Alternatives

- **Client-Side Simulation Resolution:** Rejected because diverging transition logic across CLI, MCP, and GUI would destroy replay reproducibility and violate ADR-0001.
- **Direct True-State Rendering with Client-Side Fog:** Rejected because sending full world state to the browser client allows client-side inspection to bypass the fog of war.
- **Heavy Full-Stack Web Framework (e.g., Electron/React with Node backend):** Rejected because unnecessary runtime bloat and third-party dependencies conflict with project governance and lightweight local execution.

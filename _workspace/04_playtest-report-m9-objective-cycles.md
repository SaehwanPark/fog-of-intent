# Playtest Report: M9 Objective Cycles, Vision Control, and Cross-Map Tradeoffs

**Tester:** FOI Virtual Test Player (`foi-test-player`)
**Milestone:** M9 — Bounded Multi-Lane Match Prototype
**Date:** 2026-08-15

## 1. Playtest Objective

Verify the strategic feel, mechanical correctness, and information boundaries of the M9 neutral objective spawning cycles, vision control (wards, fog-of-war, de-warding), and cross-map tradeoff mechanics.

## 2. Scenario Evaluations

### Scenario 1: `scenario-dragon-contest-v1` (Bot River Dragon Vision & Secure Contest)
- **Flow:** Turn 3 setup -> Turn 4 Allied ward placed in Bot River, Drake spawns (3500 HP). Allied engages for 2000 dmg, Opponent for 500 dmg. Turn 5 Allied executes 1500 secure burst.
- **Outcome:** Drake secured by Allied team; `ObjectiveBuffApplied` effect emitted.
- **Verification:** Events and effects correctly ordered, FNV-1a state hash verified deterministically across repeated runs.

### Scenario 2: `scenario-cross-map-trade-v1` (Cross-Map Dragon Concession for Top Herald)
- **Flow:** Turn 4, both Top Herald (5000 HP) and Bot Drake (3500 HP) active. Opponent commits 4 members to Bot Drake. Allied team consciously concedes Drake and executes `ConcedeAndTrade` for Top Herald.
- **Outcome:** Tradeoff evaluator computes $+500$ bp net value delta (`FavorableTrade`), awarding Herald buff and Mid lane tower pressure shift.
- **Verification:** Strategic tradeoff quantified without outcome bias; no hidden state leakage.

### Scenario 3: `scenario-vision-setup-and-catch-v1` (Top River Ward Placement and Flank Detection)
- **Flow:** Allied Mid laner places defensive ward in Top River on Turn 2. On Turn 3, enemy unit rotates through Top River.
- **Outcome:** Allied `MapVisionGrid` marks Top River as `FullVision`, revealing the flank and allowing safe retreat.
- **Verification:** Vision grid correctly differentiates warded zones from `ConcealedInFog`.

### Scenario 4: `scenario-stealth-objective-sneak-v1` (De-Warding and Undetected Dragon Sneak)
- **Flow:** Opponent had placed a ward in Bot River. On Turn 4, Allied clears the ward (`ClearWard`), and starts Drake under fog-of-war cover. Turn 5 burst completes the sneak.
- **Outcome:** Drake secured without opponent awareness during the execution window.
- **Verification:** De-warding state transitions cleanly; expired wards removed deterministically.

## 3. Findings & Disposition

- **Functional Correctness:** All 4 canonical scenarios run deterministically with identical replay state hashes.
- **Strategic Depth:** Cross-map tradeoff evaluation prevents simplistic "always contest" or "always teamfight" degenerate strategies by making concessions and trades mathematically explicit.
- **Information Secrecy:** Opponent fog-of-war secrecy is strictly preserved.

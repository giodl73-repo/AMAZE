# AMAZE Beat Packages

Beat packages sit between techniques and room-specific beats. They describe a
reusable play pattern that can be implemented by several devices and themed many
ways. Rooms compose packages as data in `PUZZLE-GRAPH.md`, `SCENES.md`,
`BUILD.md`, `PLAYTEST.md`, and `OPS.md`; they should not need custom harness code
for each puzzle unless a package graduates into a new reusable pattern.

## Package card fields

| Field | Meaning |
|---|---|
| Package ID | Stable reusable beat-package identifier. |
| Beat package | Human-readable package name. |
| Use when | When a room should choose the package. |
| Techniques | Technique IDs usually involved. |
| Default devices | Device IDs commonly used. |
| Required room data | What the room must declare to use the package safely. |
| Evidence hooks | Bench/admin/chaos evidence the harness should expect. |
| Avoid when | Smells that mean another package is better. |

## Core beat packages

| Package ID | Beat package | Use when | Techniques | Default devices | Required room data | Evidence hooks | Avoid when |
|---|---|---|---|---|---|---|---|
| PKG-ORDER-RAIL | Order rail / physical sequence | Players build a visible sequence from cards, tiles, tags, or props. | `TECH-SORT-002`, `TECH-FIT-002`, `TECH-REVEAL-001` | `DEV-RAIL-001`, `DEV-TRAY-001` | piece count, start homes, correct order, wrong-order reject, staff ticket/proof path | correct stack bench, wrong stack chaos, missing tile/card admin drill | the sequence is only text or color matching |
| PKG-SOCKET-PROOF | Keyed proof token | A solved station must produce or accept a public proof. | `TECH-FIT-001`, `TECH-REVEAL-001`, `TECH-TEAM-001` | `DEV-SOCKET-001`, `DEV-TRAY-001` | token silhouettes, socket layout, wrong-token reject, staff proof pouch, reset photo | correct/wrong fit bench, lost token admin drill, token-swap chaos | tokens are arbitrary keys with no remembered meaning |
| PKG-COUNTED-PROP-SET | Counted prop/card set | Loose objects, cards, transcripts, tags, or tiles are required for play and reset. | `TECH-SORT-001`, `TECH-SORT-004` | `DEV-BIN-001`, counted card/prop set | exact count, homes, duplicate pouch, reset count card, marked/damaged item path | missing item admin drill, dump chaos, reset-time bench | props are decorative or too tiny to count reliably |
| PKG-OVERLAY-WINDOW | Overlay/window inspection | Players compare through a frame, aperture, transparent overlay, or work-light surface. | `TECH-ALIGN-001`, `TECH-ALIGN-003`, `TECH-ALIGN-004` | `DEV-WINDOW-001`, `DEV-OVERLAY-001` | aperture/overlay size, sightline, glare plan, corner keys, printed backup | seated sightline bench, swap/glare chaos, overlay replacement drill | one player must squint privately or darkness is required |
| PKG-BALANCE-OBJECT | Object behavior / balance test | The correct object proves itself by weight, rest, magnet, pointer, or other physical behavior. | `TECH-FIT-003`, `TECH-REVEAL-001` | `DEV-BALANCE-001`, `DEV-SOCKET-001` | true/false tolerance gap, no-force stops, object homes, staff seal fallback | true/false cycle bench, press/force chaos, object replacement drill | tiny calibration differences decide success |
| PKG-LOW-VOLTAGE-STATE | Low-voltage state control | Switches, LEDs, UV, or indicators provide feedback but must remain recoverable. | `TECH-REVEAL-002`, `TECH-ALIGN-002` | `DEV-SWITCH-001`, `DEV-LED-001`, `DEV-CONTROLLER-001` | switch order, start-state photo, fused power, manual state card, sensory equivalent | rapid-toggle chaos, power/manual-card admin drill, accessibility bench | electricity is the only clue path or hides state from staff |
| PKG-TRANSFORM-DIAL | Transform surface / tactile dial | A table, panel, dial, knob, tuner, or compass changes meaning through movement. | `TECH-ALIGN-004`, `TECH-REVEAL-003`, `TECH-TEAM-002` | `DEV-FOLD-001`, `DEV-DIAL-001` | stops, detents, start marks, no-load script, knob/latch spare, staff mark | correct-use bench, force/lean chaos, knob/latch admin drill | players can lean on it or fine-tune endlessly |
| PKG-COVER-REVEAL | Proof-locked cover or latch | A shell, cover, latch, door, or box opens only after a valid state. | `TECH-FIT-004`, `TECH-REVEAL-003` | `DEV-COVER-001`, `DEV-LATCH-001`, `DEV-FLAG-001` | hinge/cover travel, no-force stop, pinch mitigation, staff open path, post-run repair | wrong-state pull bench, force chaos, staff release drill | resistance feels like sticky hardware |
| PKG-FINAL-CONVERGENCE | Final proof convergence | Multiple earned proofs combine into a final ritual, accusation, machine, or reveal. | `TECH-TEAM-001`, `TECH-TEAM-002`, `TECH-REVEAL-003` | `DEV-FINAL-001`, `DEV-FINAL-002`, `DEV-FINAL-003`, `DEV-TRAY-001` | proof recap, role split, final homes, open/reveal state, hard-exit path | wrong proof bench, crowd chaos, final release admin drill | one player can finish silently or proofs are guessable |
| PKG-OPS-RECOVERY | Operator recovery kit | A C4-C5 part can fail, disappear, jam, or become inaccessible during a run. | `TECH-TEAM-001` plus device-specific techniques | `OPS-002`, `components\diagrams\ops-recovery-kit.excalidraw` | pouch labels, contents, stored location, pull path, target replacement minutes, reset proof | admin replacement drill, reset/transport bench, pouch-map chaos | staff must search a mixed tote or improvise recovery |

## Package composition rules

1. A room beat may use multiple packages, but one package should own the primary
   player action.
2. Use room-specific theme names in room files, but keep package IDs stable.
3. Package evidence is inherited: if a beat uses `PKG-SOCKET-PROOF`, the room
   needs a socket/proof bench test or an explicit risk acceptance.
4. If the same custom beat appears in two rooms, add or update a package here
   before adding more custom room logic.
5. Harness code should validate package evidence and reports; room files should
   define the beat data.

## Harness architecture direction

Keep one Rust package for now, but split the code into modules as seams stabilize:

| Module | Owns |
|---|---|
| `main.rs` | binary entrypoint, CLI parse call, and process exit handling |
| `cli.rs` | argument parsing, command structs, defaults, and help text |
| `domain.rs` + `domain\*.rs` | named Markdown section loaders, typed row contracts, and scene/build/playtest/evidence/catalog document bundles |
| `evidence.rs` | bench/admin/chaos evidence extraction, filtering, status normalization, and sprint filters |
| `markdown.rs` | table parsing/scanning, row lookup, minute parsing, section appends, and report-cell escaping |
| `room.rs` | typed room contract issues, required room-file plus exact heading-line checks for every room artifact, active room discovery, and typed room document reads |
| `scenes.rs` | shared `SCENES.md` table semantics, including team-to-behavior lookup |
| `rules\simulation.rs` | team/chaos selection, reliability event chance, component recovery, and build-time variance rules |
| `rules\timing.rs` | timing fit, slack, hint-pressure, actual-time matching, and acceleration rules |
| `rules\unlocks.rs` | transformation-state and unlock-path readiness rules, beat-reference parsing, and coherence field validation |
| `rules\visual.rs` | C4-C5 visual-reference gate, visual readiness rows/filters, matching, and criticality rules |
| `sim.rs` | deterministic RNG, sampling helpers, and simulation math primitives |
| `commands\mod.rs` | command module registry and `Command` dispatch |
| `commands\check.rs` | structured room-contract issue rendering and visual-reference gate wiring |
| `commands\lint.rs` | `mdpath`-backed room Markdown hygiene checks before deeper parsing |
| `commands\catalog.rs` | inventory and beat-package catalog reports |
| `commands\visuals.rs` | C4-C5 visual readiness report and portfolio sprint queue |
| `commands\bench.rs` | bench/admin/chaos evidence report and portfolio sprint queue |
| `commands\score.rs` | draft rubric scorecards for one room or a room portfolio |
| `commands\simulation.rs` | randomized run simulation, reliability/chaos pressure, and build-time pressure report surfaces |
| `commands\timing.rs` | run sheets, session optimization reports, timing analysis reports, playtest recording, and team matrices |
| `domain` | typed row contracts for repeated rules: scenes, beat cards, components, part diagrams, team/behavior probes, NPC/operator voices, multi-room rotation, variability, chaos, evidence, and catalogs |
| `commands` | `check`, `visuals`, `bench`, `packages`, `simulate`, `record`, `matrix`, `inventory` |
| `rules` | visual readiness, timing policy, simulation reliability, and promotion gates |
| `sim` | deterministic RNG, sampling, and percentage math primitives |

Only split into multiple crates once the model needs to be reused outside the
CLI. Until then, module boundaries give most of the benefit without workspace
overhead.

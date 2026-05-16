# AMAZE Part Visualizations

AMAZE part diagrams are editable Excalidraw files that make reusable parts,
failure paths, backups, and operator recovery visible before fabrication.

## Current diagrams

| Diagram | Purpose | Use when |
|---|---|---|
| `components\diagrams\part-system-overview.excalidraw` | Shows how a room beat turns into technique, device, material, BOM, criticality, spare, admin recovery, and simulation signals. | Reviewing whether a puzzle is physically buildable and recoverable. |
| `components\diagrams\dev-socket-proof-token.excalidraw` | Part card for keyed proof tokens and sockets, including player action, hidden build, failure paths, reset, admin recovery, and simulator fields. | Any room using `DEV-SOCKET-001` or C4 proof tokens. |
| `components\diagrams\dev-final-lite-vs-cabinet.excalidraw` | Compares Manor-lite's visible verdict board to full Manor's proof-locked cabinet, with quality/build tradeoffs. | Reviewing final convergence designs and lite-vs-premium variants. |
| `components\diagrams\dev-overlay-inspection-window.excalidraw` | Part card for inspection windows and transparent overlays, including glare, orientation, reset, spare, and accessibility risks. | Any room using `DEV-WINDOW-001` or `DEV-OVERLAY-001`. |
| `components\diagrams\dev-rail-proof-order.excalidraw` | Part card for token/order rails, including player sequence action, channel/stops, release risk, reset, and staff proof recovery. | Any room using `DEV-RAIL-001` or rail-style proof ordering. |
| `components\diagrams\ops-recovery-kit.excalidraw` | Part card for the operator reset/spare organizer, including pouch labels, live-spare count state, bypass pull path, and replacement target. | Any C4-C5 recovery kit, proof pouch, or reset/spare organizer. |
| `components\diagrams\dev-low-voltage-control.excalidraw` | Part card for switch/LED/UV/power panels, including player-safe controls, fused low-voltage build, printed manual state, and electrical recovery. | Any room using `DEV-SWITCH-001`, `DEV-LED-001`, low-voltage power, or fused distribution. |
| `components\diagrams\dev-counted-prop-card-set.excalidraw` | Part card for counted printed cards, tags, tiles, transcripts, and loose prop sets with duplicate pouches and reset counts. | Any C4 evidence/card/object/tile set that can be bent, marked, dropped, or pocketed. |
| `components\diagrams\dev-transform-dial-surface.excalidraw` | Part card for fold/transform surfaces and tactile dials, including stops, detents, start marks, no-load scripts, and knob recovery. | Any room using `DEV-FOLD-001`, `DEV-DIAL-001`, or a transform/tuning surface. |
| `components\diagrams\dev-latch-final-box.excalidraw` | Part card for central latch-box finales, including visible not-yet state, latch alignment, hinge stops, manual release, and pinch/force risks. | Any room using `DEV-LATCH-001` or a central sealed object reveal. |
| `components\diagrams\dev-pipe-cover-final-machine.excalidraw` | Part card for chunky route boards, proof covers, and pressure/gauge finales, including safe removable pieces, no-force covers, and manual open tokens. | Any room using `DEV-PIPE-001`, `DEV-COVER-001`, or `DEV-FINAL-003`. |

## Required visual layers for a reusable part

Every build-serious `DEV-*`, high-criticality BOM item, or C4-C5 room component
should eventually have a visual card or diagram that shows:

| Layer | What to draw |
|---|---|
| Player face | What players see, touch, move, align, or place. |
| Hidden build | Stops, hinges, magnets, rails, sockets, sensors, wiring, or printed layers. |
| Reset state | Start-state photo target, countable loose parts, and staff-visible proof state. |
| Failure path | What breaks, jams, gets pocketed, gets swapped, or becomes ambiguous. |
| Admin recovery | Spare location, bypass action, replacement target, and operator signal. |
| Upgrade path | Prototype material and public-use material. |

## Visual notation

| Color | Meaning |
|---|---|
| Blue | Player-facing input or physical action. |
| Orange | Build mechanism, hidden support, or state transition. |
| Green | Staff/admin recovery and reset control. |
| Purple | Simulation/scoring signal. |
| Gray | Notes, constraints, and metrics. |

## Diagram rule

A part is not build-ready just because it has a BOM row. It becomes reviewable
when the diagram shows how a player can use it, how staff can reset it, how it
fails, and how the run continues when it fails.

## Role review: current visual layer

| Role | Verdict | What works | What is missing before build readiness |
|---|---|---|---|
| Template Contract Steward | Pass as a reusable system layer. | The diagram standard is general, not room-specific bureaucracy. It catches repeated failure types: physicality, reset, admin recovery, build reliability. | Add a small required diagram hook to build templates only after two or more rooms need it. |
| Build Readiness Auditor | Partial. | The overview makes mounting, parts, and recovery visible as categories. | Individual part diagrams still need dimensions, access clearances, mounting points, transport locks, and service reach. |
| BOM Durability Auditor | Pass for review framing, not for fabrication. | It forces BOM, criticality, replacement time, failure modes, and spares into the same picture. | Each C4-C5 component needs an exploded part card with wear points, spare quantity, and material upgrade path. |
| Reset Ops Reviewer | Partial. | Reset state and admin recovery are first-class visual layers. | Diagrams need reset order, count points, pouch location, and "what staff sees on camera" markers. |
| Simulation Evidence Analyst | Pass as a simulation bridge. | It connects visual design to break/loss/build-pressure simulation signals. | Diagrams should record which simulator fields they feed: `Build time`, `Replacement min`, `Spare qty`, `Criticality`, and `Failure mode`. |

## Review decision

The current diagram is a **system overview**, not a part drawing. It is good for
explaining how AMAZE thinks about parts, but it does not yet show whether a rail,
socket, overlay, latch, plinth, or final board can be fabricated.

Next diagrams should be concrete reusable part cards:

| Priority | Diagram | Why next |
|---:|---|---|
| 1 | `DEV-SOCKET` proof token and socket | Done: `components\diagrams\dev-socket-proof-token.excalidraw`. |
| 2 | `DEV-FINAL` lite verdict board vs proof-locked cabinet | Done: `components\diagrams\dev-final-lite-vs-cabinet.excalidraw`. |
| 3 | `DEV-OVERLAY` inspection window/overlay | Done: `components\diagrams\dev-overlay-inspection-window.excalidraw`. |
| 4 | `DEV-RAIL` proof/order rail | Done: `components\diagrams\dev-rail-proof-order.excalidraw`. |
| 5 | `OPS-RECOVERY` reset/spare organizer | Done: `components\diagrams\ops-recovery-kit.excalidraw`. |
| 6 | `DEV-SWITCH` / `DEV-LED` low-voltage controls | Done: `components\diagrams\dev-low-voltage-control.excalidraw`. |
| 7 | `DEV-BIN` counted prop/card sets | Done: `components\diagrams\dev-counted-prop-card-set.excalidraw`. |
| 8 | `DEV-FOLD` / `DEV-DIAL` transform controls | Done: `components\diagrams\dev-transform-dial-surface.excalidraw`. |
| 9 | `DEV-LATCH` final box | Done: `components\diagrams\dev-latch-final-box.excalidraw`. |
| 10 | `DEV-PIPE` / `DEV-COVER` / `DEV-FINAL-003` machine parts | Done: `components\diagrams\dev-pipe-cover-final-machine.excalidraw`. |

## Part card acceptance checklist

| Check | Required before marking diagram build-ready |
|---|---|
| Scale | Shows approximate dimensions or relative hand/body scale. |
| Player operation | Shows the exact player action and correct/incorrect affordance. |
| Staff view | Shows the proof/reset state staff can verify without entering the room. |
| Mounting/access | Shows fasteners, backing, service access, and what cannot block egress. |
| Failure/recovery | Shows break/loss/jam/swap path and replacement/bypass target. |
| Materials | Shows prototype material and production material upgrade. |
| Simulation fields | Names the BOM fields the simulator uses. |

## Harness readiness scoring

Room `BUILD.md` files can reference reusable diagrams before a part is fully
build-ready, but the `## Part diagrams` row must stay honest:

| Status | Required row state |
|---|---|
| Build-ready | `What it proves` is complete and `Missing before build readiness` says none/build-ready/complete. |
| Draft | `What it proves` is complete and `Missing before build readiness` names the exact remaining room-specific evidence. |
| Blocked | The row lacks `What it proves`, lacks `Missing before build readiness`, or points to a missing `.excalidraw` file. |

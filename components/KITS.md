# AMAZE Room Kits

Kits are reusable bundles for prototyping, operating, transporting, and
maintaining trailer-scale rooms. They sit above individual inventory items and
below room-specific BOMs.

Use kits to avoid rebuilding the same support system for every room.

## Kit card fields

| Field | Meaning |
|---|---|
| Kit ID | Stable bundle identifier. |
| Purpose | What recurring room need the kit satisfies. |
| Includes | Inventory IDs or device IDs normally included. |
| Used when | When a room should include the kit. |
| Exclude when | When the kit is wrong or excessive. |
| Reset/storage | How staff store and verify it. |

## Prototype kits

| Kit ID | Purpose | Includes | Used when | Exclude when | Reset/storage |
|---|---|---|---|---|---|
| KIT-PROTO-001 | Cardboard mechanism sprint | `SCENIC-006`, `CORE-003`, `PRINT-001`, `PRINT-002`, `OPS-004` | proving aha, fit, route, or sequence before fabrication | public playtest or load-bearing build | labeled prototype bin; discard unsafe edges |
| KIT-PROTO-002 | Token and socket sprint | `SCENIC-007`, `PRINT-007`, `SCENIC-004`, `MECH-010` | proof tokens, keyed fit, final sockets, no-force closures | tiny precision or real lock security is required | token count card and duplicate set |
| KIT-PROTO-003 | Route board sprint | `SCENIC-006`, `PRINT-007`, `MECH-008`, `PRINT-002` | pipes, circuits, maps, currents, tracks | route requires structural load or climbing | start-state photo with pieces counted |
| KIT-PROTO-004 | Overlay/window sprint | `PRINT-006`, `SCENIC-004`, `PRINT-001`, `OPS-003` | map, mirror, witness, inspection, alignment beats | visibility is too low without lighting redesign | sleeve overlays; check scratches/glare |

## Room operations kits

| Kit ID | Purpose | Includes | Used when | Exclude when | Reset/storage |
|---|---|---|---|---|---|
| KIT-OPS-001 | Standard operator kit | `OPS-001`, `OPS-002`, `OPS-003`, `OPS-004`, `OPS-005` | every staffed trailer room | never, unless venue supplies equivalent kit | sealed reset tote checked before first run |
| KIT-OPS-002 | Proof-token spare kit | duplicate proof tokens, `PRINT-005`, `OPS-002`, `PRINT-002` | any room with portable proofs or loose puzzle tokens | all proofs are fixed/attached and bypass is enough | locked staff pouch; count each reset |
| KIT-OPS-003 | Printed backup kit | `PRINT-001`, `PRINT-002`, `PRINT-003`, transcripts, visual equivalents | audio, lighting, sensor, or fragile reveal supports required understanding | no powered/sensed/printed clue dependency | operator clipboard plus duplicate envelope |
| KIT-OPS-004 | Reset-photo kit | start-state photos, `OPS-001`, `PRINT-002`, `OPS-003` | any room with more than three movable stations | fixed installations with no movable state | laminated checklist with station photos |
| KIT-OPS-005 | Admin replacement kit | labeled C3-C5 pouches, duplicate critical tokens/cards, bypass tokens, mini timer, recovery log | any room with loose required props, fragile mechanisms, or C4-C5 components | every required item is fixed and has a tested manual release | operator-side go-bag; timed replacement drill before public runs |

## Electronics kits

| Kit ID | Purpose | Includes | Used when | Exclude when | Reset/storage |
|---|---|---|---|---|---|
| KIT-ELEC-001 | Low-voltage light feedback | `ELEC-001`, `ELEC-003`, `ELEC-004`, `ELEC-002` | state lights, dry effects, progress indicators | required clue cannot be understood without light | fused prop power box; pre-run light test |
| KIT-ELEC-002 | Simple sensor proof | `ELEC-008`, `ELEC-010`, `ELEC-005`, `ELEC-004` | detecting drawers, panels, covers, token seats | sensor would be structural or hard to bypass | staff test card; spare flashed board |
| KIT-ELEC-003 | Audio ambience with visual backup | `ELEC-006`, `ELEC-007`, `ELEC-003`, `PRINT-001` | atmosphere or non-required audio prompts | audio would be the only required clue | transcript stored with operator kit |
| KIT-ELEC-004 | Dry motion/effect gag | `ELEC-011`, `ELEC-002`, `ELEC-003`, `SCENIC-008` | fake steam, bubbles, flags, celebratory motion | wet, mist, heat, smoke, or loose-fabric hazards are involved | fan clear; effect can be skipped |

## Mechanical kits

| Kit ID | Purpose | Includes | Used when | Exclude when | Reset/storage |
|---|---|---|---|---|---|
| KIT-MECH-001 | No-force closure kit | `MECH-005`, `MECH-010`, `MECH-007`, `CORE-002` | shells, covers, cabinets, hinged reveals | players could pinch or trap fingers without redesign | inspect stop and hinge before run |
| KIT-MECH-002 | Sliding reveal kit | `MECH-004`, `MECH-007`, `MECH-010`, `PRINT-002` | flags, drawers, windows, proof releases | slide would carry heavy or structural load | clear track; verify reveal hidden |
| KIT-MECH-003 | Selector/index kit | `MECH-006`, `MECH-009`, `PRINT-002`, `CORE-001` | dials, gauges, valves, final selectors | imprecise selection would create false states | reset index marks to photo |
| KIT-MECH-004 | Magnetic removable panel kit | `MECH-008`, `SCENIC-005`, `PRINT-002`, `CORE-001` | route boards, overlays, service panels | magnet strength creates pinch or ingestion risk | polarity marked; spare panel stored |
| KIT-MECH-005 | Dial/tuner kit | `MECH-006`, `MECH-009`, `PRINT-002`, `ELEC-006` optional | radio tuners, gauge faces, selector dials, final tuning | tiny precision or audio-only confirmation would be required | reset to start mark; inspect set screw |
| KIT-MECH-006 | Latch-stack kit | `MECH-001`, `MECH-005`, `MECH-010`, `CORE-002` | final boxes, central-object reveals, proof-gated doors | players can reach pinch points or apply unsafe force | relock latches; check stops and staff release |
| KIT-MECH-007 | Fold/transform surface kit | `MECH-005`, `MECH-001`, `MECH-010`, `CORE-002` | fold-down tables, hinged maps, rotating panels, transform surfaces | surface carries body weight or blocks egress | reset latch/orientation; inspect hinge/stop |

## Transport and maintenance kits

| Kit ID | Purpose | Includes | Used when | Exclude when | Reset/storage |
|---|---|---|---|---|---|
| KIT-TRANS-001 | Loose-prop transport kit | `SCENIC-001`, `OPS-002`, `PRINT-005`, `CORE-005` | any room with removable tokens/props | props are permanently mounted | packed by station, not by material |
| KIT-TRANS-002 | Panel transport kit | `CORE-005`, `OPS-004`, `PRINT-002`, corner guards | removable wall/counter modules | installed trailer with no transport teardown | photo before/after transport |
| KIT-MAINT-001 | Daily repair consumables | `CORE-003`, `OPS-004`, spare labels, `MECH-010` | every prototype or public room | never, unless venue maintenance owns equivalent | restock weekly; no permanent gaffer-tape fixes |

## Kit selection checklist

Every room build pack should answer:

1. Which prototype kits prove the riskiest aha beats?
2. Which operations kits are required before staff can run the room alone?
3. Which electronics kits have visual/manual equivalents?
4. Which mechanical kits create pinch, force, or reset risk?
5. Which transport kits protect the room between venues?
6. Which kit can be removed if reset exceeds target?
7. Which C4-C5 items need `KIT-OPS-005` staged at the operator position?

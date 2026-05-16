# Astral Archive Overnight Build Plan

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$400-1,000` | act board, drawer mock, map overlay, astrolabe, filter rail, daybook, orrery mock |
| Production | `$1,800-5,000+` | premium modular panels, durable vault, lighting, act bins, spares |
| Spares | `$250-650` | duplicate tiles, cards, relics, filters, knobs, manual vault cards |
| Maintenance | `$75-250/run season` | wheel/socket service, card refresh, drawer/rail repair |
| Transport | `$250-700` | modular panel cases, three act bins, vault cover, reset cart |
| Build hours | `90-160` | larger than standard rooms because of act modules and final vault |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|
| P1 | TECH-SORT-001 | act board and role cards | KIT-OPS-003 | public act board, pause clock, role cards | passive/ops | operator names acts/state cards | board home | cards ignored/pocketed |
| P2 | TECH-SORT-003 | star catalog drawers | KIT-PROTO-002 | shallow drawer/card rail | mechanical/passive | staff first coordinate | drawers closed/cards home | drawers yanked |
| P3 | TECH-ALIGN-003 | broken sky map | KIT-PROTO-004 | tile overlay frame and map relic | passive/mechanical | staff map relic after explanation | tiles home | tile force/loss |
| P4 | TECH-TEAM-001 | watch change board | KIT-OPS-003 | state card dock and rest cue | passive/ops | operator recap | card reset | pace break skipped |
| P5 | TECH-ALIGN-001 | astrolabe wheel | KIT-MECH-002 | rotating wheel with detents | mechanical | staff angle token | wheel home | wheel spun/forced |
| P6 | TECH-FIT-002 | spectrum filter rail | KIT-PROTO-004 | sliding filters and prism relic | passive/optical | printed house marks and staff relic | filters stacked | glare/card loss |
| P7 | TECH-SORT-001 | chronicle daybook | KIT-OPS-004 | daybook panels and index cards | passive | staff first date | panels home | text overload |
| P8 | TECH-ALIGN-001 | comet clock | KIT-MECH-002 | clock dial and dawn token | mechanical | staff dawn token | dial home | dial drift |
| P9 | TECH-TEAM-001 | relic sockets | KIT-PROTO-002 | three relic sockets and dawn slot | mechanical | staff socket verification | sockets empty | wrong relic force |
| P10 | TECH-REVEAL-003 | orrery vault | KIT-PROTO-002 | large wheel, star field, exit token | mechanical/low-voltage optional | manual vault card | vault closed | wheel jam/pinch |

## Bill of materials

| Inventory ID | Component | Beat IDs | Visual reference | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|
| PRINT-005 | act board, role cards, state cards, pause clock card | P1/P4/P7 | diagrams\astral-orrery-vault.excalidraw | 1 | 1 | C4 required | 6 | 2 | `$20-75` | print shop, office supply | Planning | 0-1 week | C/B | state card lost or too wordy | operator recap | duplicate act board pack | count/sleeve |
| CUSTOM-SCENIC | star catalog drawer board and route tiles | P2 | diagrams\astral-orrery-vault.excalidraw | 1 | 1 | C4 required | 12 | 4 | `$75-250` | maker shop, hardware | Planning | 1-3 weeks | B | drawer jam, tile loss | staff coordinate rail | spare tile set | drawer/rail check |
| CUSTOM-SCENIC | broken sky map overlay and map relic | P3 | diagrams\astral-orrery-vault.excalidraw | 1 | 1 | C4 required | 10 | 3 | `$50-180` | plastics, print shop | Planning | 1-2 weeks | B/C | overlay scratch, tile ambiguity | staff map relic | duplicate overlay/relic | clean/check |
| CUSTOM-SCENIC | astrolabe wheel and angle token | P5 | diagrams\astral-orrery-vault.excalidraw | 1 | 1 | C4 required | 12 | 3 | `$80-250` | hardware, maker shop | Planning | 1-3 weeks | B | wheel drift or pointer bend | staff angle token | spare pointer/token | detent test |
| SCENIC-004 plus PRINT-005 | spectrum filter rail and prism relic | P6 | diagrams\astral-orrery-vault.excalidraw | 1 | 1 | C4 required | 10 | 3 | `$45-180` | plastics, print shop | Planning | 1-2 weeks | B/C | glare, color mismatch, filter loss | printed house marks | spare filters/relic | glare test |
| PRINT-005 plus CUSTOM-SCENIC | chronicle daybook panels and comet clock | P7/P8 | diagrams\astral-orrery-vault.excalidraw | 1 | 1 | C4 required | 12 | 3 | `$40-160` | print shop, maker shop | Planning | 1-2 weeks | B/C | text overload, dial drift | staff first date/dawn token | duplicate panels/dial pointer | count/test |
| CUSTOM-SCENIC plus ELEC-LOWV | orrery vault, relic sockets, star field, exit token | P9/P10 | diagrams\astral-orrery-vault.excalidraw | 1 | 0 | C5 showstopper | 24 | 5 | `$200-800+` | maker shop, hardware, LED kit | Planning | 2-5 weeks | B | socket jam, wheel jam, final reveal fail | manual vault card | rear hatch/manual release | socket/wheel test |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| fatigue drift | full run | players slow, sit on floor, disengage | watch change and role rotation | add mandatory rest cues |
| memory overload | P7-P10 | asks earlier-act meaning | use state cards | reduce text/card count |
| drawer/tile loss | P2-P3 | count mismatch | duplicate tile pack | larger/tethered tiles |
| color/glare block | P6 | avoids filters | work-light and printed marks | high-contrast backup |
| C5 vault jam | P9-P10 | relic/wheel stuck | manual vault card | tolerance and service hatch |

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|
| P2 | catalog drawers | L1 cardboard | medium | chaotic/confused | search soup | random drawer gives route | correct drawer hidden | route rail filled by coordinate logic | reset over 8 minutes |
| P5 | astrolabe wheel | L1 cardboard | medium-high | chaotic | random spinning | accidental angle | earned angle fails | team cites Act I state | no detents |
| P6 | spectrum filters | L1 cardboard | high | accessibility-varied | color/glare | wrong filter readable | true filter unreadable | high-contrast route works | color-only clue |
| P9/P10 | orrery vault | L1 cardboard | high | operator stress/chaotic | socket or wheel jam | final opens without relics | relics fail to unlock | manual vault works under 5 min | no recovery path |

## Part diagrams

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| Act board and state cards | diagrams\astral-orrery-vault.excalidraw | Shows long-session state externalization, pause clock, and operator-visible progress. | draft: final board dimensions and copy length |
| Star catalog and sky map | diagrams\astral-orrery-vault.excalidraw | Shows drawer/tile rail, map overlay, relic output, and reset bin. | draft: drawer count and tile size |
| Astrolabe and spectrum rail | diagrams\astral-orrery-vault.excalidraw | Shows wheel/filter flow, high-contrast backup, and act relic output. | draft: glare and detent specs |
| Chronicle and comet clock | diagrams\astral-orrery-vault.excalidraw | Shows daybook panels, state-card dependency, clock dial, and dawn token. | draft: text-load test |
| Orrery vault | diagrams\astral-orrery-vault.excalidraw | Shows relic sockets, star field, wheel, service hatch, final exit token, and manual bypass. | draft: socket tolerance, wheel guard, and transport cover |

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|
| act/state card pack | 1 | C4 | staff binder | 2 | lost/bent |
| route tile set | 1 | C4 | Act I bin | 4 | tile lost |
| map overlay/relic | 1 | C4 | Act I bin | 3 | scratch/loss |
| astrolabe pointer/token | 1 | C4 | Act II bin | 3 | drift/bend |
| filter set/prism relic | 1 | C4 | Act II bin | 3 | glare/loss |
| daybook/dawn token pack | 1 | C4 | Act III bin | 3 | text/card damage |
| manual vault card | 1 | C5 | operator clipboard | 1 | any vault failure |

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|
| ops/state layer | P1/P4/P7 | 6 | 10 | 3 | duplicate card pack | state cards too verbose |
| Act I map archive | P2/P3 | 18 | 35 | 8 | tile/overlay reset | reset too slow |
| Act II instruments | P5/P6 | 18 | 35 | 6 | wheel/filter recovery | color inaccessible |
| Act III chronicle | P7/P8 | 12 | 24 | 5 | card/dial recovery | memory overload |
| Orrery vault | P9/P10 | 24 | 50 | 6 | manual vault release | C5 recovery over 5 min |

## Criticality map

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | orrery vault | premium final reveal and exit token depend on it | manual vault card and rear hatch | timed 5-minute release |
| C4 required | act board, drawers, map, astrolabe, filters, daybook/clock | required act relics depend on them | duplicate cards/tokens/relics | timed replacement drills |
| C3 beat-support | optional lore, scenic drawers, lighting | atmosphere and delight | remove/repair | post-run repair |
| C2 helper | chimes/ticks | flavor only | mute | operator control |
| C1 cosmetic | star labels, scenic wear | atmosphere | repair consumables | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft | mechanism map | cardboard prototype |
| BOM line for every required component | Draft | BOM above | source cleanup |
| Inventory ID or custom fabrication plan for every critical component | Draft | custom scenic rows | catalog mapping |
| Material and construction plan for every critical component | Draft | schedule and BOM | material tests |
| Technique/device/kit IDs for every required beat | Draft | mechanism map | verify IDs |
| Visual reference for every C4-C5 component | Draft | part diagrams rows | dimensions pending |
| Criticality, spare count, build time, and replacement time assigned | Draft | BOM/spare kit | drill |
| C4-C5 admin replacement drill passes in time | No | PLAYTEST queue | run drills |
| Device review level/risk band assigned | Draft | review matrix | bench evidence |
| Known cost band for critical components | Draft | budget/BOM | sourcing pass |
| Manual bypass for powered/sensed/fragile beats | Draft | bypass column | operator script |
| Durability class not D for required components | Draft | BOM | material upgrade |
| Reset action verifiable under time pressure | Draft | OPS reset | 25-minute reset drill |

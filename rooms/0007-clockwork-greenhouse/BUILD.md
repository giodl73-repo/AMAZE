# Clockwork Greenhouse Build Plan

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$160-450` | printed log, seed trays, valve board, prism mock, cardboard bloom wheel |
| Production | `$600-1,500+` | durable trays, acrylic prism, mounted valves, mechanical bloom wheel |
| Spares | `$100-250` | duplicate tokens, seed tiles, valves, shade cards, manual bloom card |
| Maintenance | `$25-120/run season` | label/card refresh, token replacement, slider/wheel checks |
| Transport | `$75-200` | flat panel sleeves, lidded trays, token pouches |
| Build hours | `28-45` | depends on bloom wheel durability |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|
| P1 | TECH-SORT-001 | gardener log and role tags | KIT-OPS-003 | log, role tags, proof rail silhouettes | passive print | operator names four systems | log closed, tags stacked | tags pocketed |
| P2 | TECH-SORT-003 | seed bed trays | KIT-PROTO-002 | oversized seed tokens and soil texture tiles | passive tactile | staff root token after explanation | trays sorted, root hidden | pieces dropped or hidden |
| P3 | TECH-ALIGN-001 | irrigation valve board | KIT-MECH-002 | rotating valve pointers with detents | mechanical | staff first valve/water token | valves home | spinning/forcing valves |
| P4 | TECH-ALIGN-003 | prism shade slider | KIT-PROTO-004 | acrylic or printed slider and shade cards | passive/optical | printed light token card | slider home, cards stacked | glare or slider force |
| P5 | TECH-TEAM-001 | bloom wheel | KIT-PROTO-002 | four token sockets and rotating flower wheel | mechanical | manual bloom card and exit token | tokens removed, wheel closed | wheel forced or fingers pinched |

## Bill of materials

| Inventory ID | Component | Beat IDs | Visual reference | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|
| PRINT-005 | gardener log, role tags, proof rail card | P1 | diagrams\greenhouse-bloom-wheel.excalidraw | 1 | 1 | C3 beat-support | 2 | 1 | `$5-30` | print shop, office supply | Planning | 0-1 week | C | tags lost or too wordy | operator names systems | duplicate tag set | count/sleeve |
| CUSTOM-SCENIC | seed tray set, soil tiles, root token | P2 | diagrams\greenhouse-bloom-wheel.excalidraw | 1 | 1 | C4 required | 4 | 2 | `$15-60` | craft store, maker shop | Planning | 0-1 week | B/C | seed token lost or ambiguous texture | staff root token | duplicate seed/tile pouch | count tray |
| CUSTOM-SCENIC | irrigation valve board and water token | P3 | diagrams\greenhouse-bloom-wheel.excalidraw | 1 | 1 | C4 required | 5 | 2 | `$20-80` | hardware, craft store | Planning | 0-2 weeks | B | pointer drift or valve force | staff water token | spare pointer/knob | detent test |
| SCENIC-004 plus PRINT-005 | prism shade slider, shade cards, light token | P4 | diagrams\greenhouse-bloom-wheel.excalidraw | 1 | 1 | C4 required | 5 | 3 | `$20-90` | plastics, print shop | Planning | 0-2 weeks | B/C | glare, scratch, sticky slider | printed light card | spare shade card/slider overlay | clean/test |
| CUSTOM-SCENIC | bloom wheel, sockets, petals, exit token | P5 | diagrams\greenhouse-bloom-wheel.excalidraw | 1 | 0 | C5 showstopper | 8 | 2 | `$35-140` | maker shop, hardware | Planning | 0-2 weeks | B | wheel jam, socket jam, pinch | manual bloom card | rear hatch/manual release | socket/wheel test |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| seed token loss | P2 | tray count mismatch | duplicate token pouch | oversize/tether tokens |
| valve drift | P3 | pointer not on home mark | staff water token after solve | stronger detents |
| prism glare | P4 | players avoid/squint | work light and printed card | matte high contrast |
| wheel jam | P5 | token/wheel will not move | manual bloom card | bevels, larger tolerances |
| pinch risk | P5 | fingers near petals/wheel | no-force script, finger gaps | guard ring |

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|
| P2 | seed tray | L1 cardboard | medium | family/confused | texture ambiguity | random tray earns token | correct tray unclear | team explains texture match | pieces too small |
| P3 | valve board | L1 cardboard | medium | chaotic | spinning valves | random order passes | correct order fails | team follows leaf veins | no tactile stops |
| P4 | prism slider | L1 cardboard | medium-high | accessibility-varied | glare | wrong angle readable | true angle hidden | readable under work light | not accessible |
| P5 | bloom wheel | L1 cardboard | high | chaotic/operator stress | wheel jam | opens without proofs | proofs fail to seat | roles verify and wheel opens | manual release unclear |

## Part diagrams

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| P2 seed tray/root token | diagrams\greenhouse-bloom-wheel.excalidraw | Shows tray count, token size, root token home, and reset photo layout. | draft: final token dimensions |
| P3 valve board/water token | diagrams\greenhouse-bloom-wheel.excalidraw | Shows valve board, detent idea, water token output, and staff bypass. | draft: detent hardware choice |
| P4 prism shade/light token | diagrams\greenhouse-bloom-wheel.excalidraw | Shows slider, work-light sightline, light token, and printed backup. | draft: glare test dimensions |
| P5 bloom wheel | diagrams\greenhouse-bloom-wheel.excalidraw | Shows four sockets, flower wheel, finger gap, service hatch, and manual release. | draft: wheel tolerance and guard ring |

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|
| role tag set | 1 | C3 | staff pouch A | 1 | bent/lost |
| seed/token set | 1 | C4 | staff pouch B | 2 | missing/marked |
| valve knob/pointer and water token | 1 | C4 | repair tin | 2 | pointer drift |
| shade card and light token | 1 | C4 | staff pouch C | 3 | glare/scratch |
| manual bloom card | 1 | C5 | operator clipboard | 1 | wheel jam |

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|
| printed greenhouse kit | P1/P4 | 3 | 5 | 1 | duplicate card swap | reading load too high |
| seed tray kit | P2 | 4 | 6 | 2 | full token replacement | pieces too small |
| utility wall | P3/P4 | 8 | 14 | 3 | valve/slider recovery | glare or valve drift |
| bloom wheel | P5 | 8 | 16 | 3 | manual bloom release | pinch/jam unresolved |

## Criticality map

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | bloom wheel | final reveal and exit payoff depend on it | manual bloom card and staff release | timed 2-minute release |
| C4 required | seed tray, valve board, prism slider | each required proof depends on it | duplicate tokens/cards/knobs | timed replacement |
| C3 beat-support | gardener log, role tags | prevents search soup | duplicate print set | reset count |
| C2 helper | optional plant lore | delight only | remove if confusing | post-run repair |
| C1 cosmetic | vines/labels | atmosphere | repair consumables | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft | mechanism map | cardboard prototype |
| BOM line for every required component | Draft | BOM above | source cleanup |
| Inventory ID or custom fabrication plan for every critical component | Draft | custom scenic rows | catalog mapping |
| Material and construction plan for every critical component | Draft | BOM/schedule | material test |
| Technique/device/kit IDs for every required beat | Draft | mechanism map | verify IDs |
| Visual reference for every C4-C5 component | Draft | part diagrams rows | dimensions pending |
| Criticality, spare count, build time, and replacement time assigned | Draft | BOM/spare kit | drill |
| C4-C5 admin replacement drill passes in time | No | PLAYTEST queue | run drills |
| Device review level/risk band assigned | Draft | review matrix | bench evidence |
| Known cost band for critical components | Draft | budget/BOM | sourcing pass |
| Manual bypass for powered/sensed/fragile beats | Draft | bypass column | operator script |
| Durability class not D for required components | Draft | BOM | material upgrade |
| Reset action verifiable under time pressure | Draft | OPS reset | timed reset |

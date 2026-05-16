# Stormglass Station Build Plan

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$175-500` | printed log, dial board, vane rack, tide slider, proof cartridges, simple console |
| Production | `$650-1,600+` | durable panels, acrylic console face, LED route, mounted signal buttons |
| Spares | `$100-250` | duplicate cards, cartridges, overlays, button caps, printed route cards |
| Maintenance | `$30-125/run season` | card refresh, acrylic polish, latch/socket checks |
| Transport | `$75-200` | flat wall-panel sleeves, cartridge pouch, console cover |
| Build hours | `30-48` | includes cardboard console and one mounted wall prototype |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|
| P1 | TECH-SORT-001 | station log and role cards | KIT-OPS-003 | bound log, checklist, role card stack | passive print | operator names three readings | log closed, cards stacked | cards bent or pocketed |
| P2 | TECH-ALIGN-001 | barometer dial | KIT-PROTO-004 | rotating pointer with detents and pressure cartridge | mechanical | operator sets dial and gives staff cartridge | dial home, cartridge hidden | pointer spun hard |
| P3 | TECH-SORT-003 | wind vane card rack | KIT-OPS-004 | vane arrow and ordered word-card rack | passive/mechanical | operator places first card and gives staff cartridge | vane home, cards sorted | cards shuffled/lost |
| P4 | TECH-FIT-003 | tide gauge slider | KIT-MECH-002 | float slider, overlay marks, tide cartridge | mechanical | printed aligned tide card and staff cartridge | slider home, overlay pouch | slider forced or overlay swapped |
| P5 | TECH-TEAM-001 | stormglass proof console | KIT-PROTO-002 | proof sockets, route shutter, beacon buttons | mechanical/low-voltage optional | staff route card and manual beacon script | cartridges removed, shutter closed | socket jam, button mashing |

## Bill of materials

| Inventory ID | Component | Beat IDs | Visual reference | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|
| PRINT-005 | station log and role cards | P1 | diagrams\stormglass-console.excalidraw | 1 | 1 | C3 beat-support | 2 | 1 | `$5-30` | print shop, office supply | Planning | 0-1 week | C | cards bent, lost, or too wordy | operator names three readings | duplicate role card set | count and sleeve |
| CUSTOM-SCENIC | barometer dial board and pressure cartridge | P2 | diagrams\stormglass-console.excalidraw | 1 | 1 | C4 required | 5 | 2 | `$20-75` | craft store, maker shop | Planning | 0-2 weeks | B/C | pointer drift, cartridge latch wear | staff pressure cartridge | spare pointer and staff cartridge | detent check |
| PRINT-005 plus CUSTOM-SCENIC | wind vane rack and wind cartridge | P3 | diagrams\stormglass-console.excalidraw | 1 | 1 | C4 required | 4 | 2 | `$15-60` | print shop, craft store | Planning | 0-1 week | C/B | card loss or ambiguous order | staff first-card hint and cartridge | duplicate wind card set | card count |
| CUSTOM-SCENIC | tide gauge slider, overlay, tide cartridge | P4 | diagrams\stormglass-console.excalidraw | 1 | 1 | C4 required | 5 | 3 | `$20-90` | plastics, craft store | Planning | 0-2 weeks | B/C | glare, sticky slider, overlay swap | printed tide answer card | spare overlay and staff cartridge | slider clean/test |
| CUSTOM-SCENIC plus ELEC-LOWV | stormglass proof console and route shutter | P5 | diagrams\stormglass-console.excalidraw | 1 | 0 | C5 showstopper | 8 | 2 | `$40-150` | maker shop, hardware, LED kit | Planning | 0-2 weeks | B | socket jam, shutter stick, route hidden | staff route card/manual reveal | service hatch and manual route card | socket fit and shutter test |
| ELEC-LOWV | harbor signal button panel | P5 | diagrams\stormglass-console.excalidraw | 1 | 1 | C4 required | 3 | 2 | `$15-65` | electronics kit, hardware | Planning | 0-2 weeks | B/C | button cap loss or false press | operator calls signal verbally | spare button cap and printed route | button test |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| Pointer drift | P2 | dial no longer lines up with cartridge mark | staff gives pressure cartridge after explanation | add keyed detents |
| Card loss | P3 | missing wind word card | use duplicate pouch | tether or magnetic board |
| Overlay glare | P4 | players avoid gauge or squint | work light and printed tide card | matte laminate and larger marks |
| Socket jam | P5 | cartridge will not seat | use staff proof cartridge and manual route | widen tolerances and bevel edges |
| Button mashing | P5 | beacon pressed before route | proof shutter script | covered buttons or inactive props |

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|
| P2 | barometer dial | L1 cardboard | medium | chaotic and speed teams | random spinning | pointer lands on code accidentally | detent misses code | team says why pressure is falling | no tactile stop |
| P3 | wind vane rack | L1 cardboard | low-medium | confused and fighting teams | text order ambiguity | random order spells answer | correct order not obvious | team uses arrow first | card labels require long reading |
| P4 | tide gauge slider | L1 cardboard | medium | accessibility-varied teams | glare or tiny marks | adjacent height appears valid | true height hard to see | team uses slider and overlay | not readable under work light |
| P5 | stormglass console | L1 cardboard | high | chaotic and operator stress teams | socket jam or one-player control | route visible without proof | cartridge fails to seat | three cartridges seated and route called | manual release unclear |

## Part diagrams

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| P2 barometer dial | diagrams\stormglass-console.excalidraw | Shows dial, pressure cartridge home, player-facing pointer, and reset state. | draft: exact detent dimensions and mounting holes |
| P3 wind vane rack | diagrams\stormglass-console.excalidraw | Shows wind card rack order, cartridge output, and staff count pouch. | draft: card size and tether decision |
| P4 tide gauge slider | diagrams\stormglass-console.excalidraw | Shows slider, overlay reading zone, cartridge output, and work-light sightline. | draft: high-contrast mark dimensions |
| P5 stormglass proof console | diagrams\stormglass-console.excalidraw | Shows three proof sockets, route shutter, signal panel, service hatch, and manual bypass. | draft: socket tolerance and shutter material |

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|
| role card set | 1 | C3 | staff pouch A | 1 | bent/lost |
| pressure cartridge and pointer | 1 | C4 | staff pouch B | 2 | latch or pointer failure |
| wind card set and cartridge | 1 | C4 | staff pouch B | 2 | missing/marked card |
| tide overlay and cartridge | 1 | C4 | staff pouch C | 3 | glare/scratch/sticky slider |
| manual route card | 1 | C5 | operator clipboard | 1 | console jam |
| signal button cap | 2 | C4 | repair tin | 2 | cap loosened |

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|
| printed station kit | P1/P3 | 3 | 4 | 1 | duplicate card swap | cards not countable |
| instrument wall | P2-P4 | 10 | 16 | 3 | cartridge and slider swap | marks unreadable |
| stormglass console | P5 | 8 | 16 | 3 | manual route release | socket jam not recoverable |
| signal panel | P5 | 3 | 6 | 1 | button cap replacement | false press can solve |

## Criticality map

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | stormglass proof console | final route and exit payoff depend on it | manual route card plus staff reveal | timed 2-minute manual release |
| C4 required | barometer, wind rack, tide gauge, signal panel | each required proof or final action depends on it | duplicate cartridges/cards/overlay/buttons | timed replacement drill |
| C3 beat-support | station log and role cards | helps avoid search soup | duplicate printed set | reset count |
| C2 helper | optional storm lore | delight only | remove if confusing | post-run repair |
| C1 cosmetic | scenic labels and weather wear | atmosphere only | repair consumables | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft | mechanism map | cardboard prototype |
| BOM line for every required component | Draft | BOM above | source ID cleanup |
| Inventory ID or custom fabrication plan for every critical component | Draft | custom scenic rows | map to catalog IDs |
| Material and construction plan for every critical component | Draft | BOM and schedule | material test |
| Technique/device/kit IDs for every required beat | Draft | mechanism map | verify catalog IDs |
| Visual reference for every C4-C5 component | Draft | part diagrams rows | dimensions pending |
| Criticality, spare count, build time, and replacement time assigned | Draft | BOM and spare kit | replacement drill |
| C4-C5 admin replacement drill passes in time | No | PLAYTEST drill queue | run bench/admin tests |
| Device review level/risk band assigned | Draft | review matrix | bench evidence |
| Known cost band for critical components | Draft | budget/BOM | sourcing pass |
| Manual bypass for powered/sensed/fragile beats | Draft | bypass column | operator script test |
| Durability class not D for required components | Draft | BOM | material upgrade if needed |
| Reset action verifiable under time pressure | Draft | reset paths and OPS | reset photo cards |

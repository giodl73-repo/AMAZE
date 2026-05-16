# Signal in the Silverstream - Build Economics

This is an initial build pass, not a vendor quote.

Use `docs/build-economics.md`, `components/TECHNIQUES.md`,
`components/DEVICES.md`, `components/KITS.md`,
`components/DEVICE-REVIEW.md`, `components/INVENTORY.md`, and
`components/SOURCING.md` before build readiness review.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$200-600` | paper route set, mock switch panel, temporary UV cabinet, radio audio test |
| Production | `$600-2,000+` | mounted panels, durable table mechanism, finished scenic props |
| Spares | `$75-250` | duplicate postcards, labels, cabinet objects, switch caps, radio knobs |
| Maintenance | `$25-100/run season` | batteries, cleaning, replacement labels, calibration checklist |
| Transport | `$50-200` | padded cases for removable props and shock protection for electronics |
| Build hours | `20-35` | route/map print pack, switch/UV mockup, fold table prototype, radio final |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|
| P1 | `TECH-SORT-002`, `TECH-ALIGN-001` | `DEV-TRAY-001` route rail/map | `KIT-PROTO-004`, `KIT-OPS-004` | postcards, wall map, mileage plaques | analog | operator confirms next stop | shuffle postcards | cards bent, pocketed, or marked |
| P2 | `TECH-SORT-002`, `TECH-REVEAL-002` | `DEV-SWITCH-001`, `DEV-LED-001` | `KIT-ELEC-001`, `KIT-OPS-003` | low-voltage prop breaker switches | electrical | operator triggers UV circuit manually | reset switches | random force on switches |
| P3 | `TECH-SORT-004`, `TECH-REVEAL-002` | `DEV-BIN-001`, `DEV-LED-001` | `KIT-ELEC-001`, `KIT-OPS-002`, `KIT-OPS-004` | UV-lit cabinet and removable galley objects | lighting | reveal one frequency digit | replace objects | objects dropped, hidden, or pocketed |
| P4 | `TECH-ALIGN-004`, `TECH-REVEAL-003` | `DEV-FOLD-001` fold-down table | `KIT-MECH-007`, `KIT-MECH-003` | rotating/latching table with etched compass rose | analog/mechanical | operator points to compass mark | reset latch and orientation | players lean on or force table |
| P5 | `TECH-TEAM-002`, `TECH-REVEAL-004` | `DEV-DIAL-001`, `DEV-AUDIO-001` | `KIT-MECH-005`, `KIT-ELEC-003`, `KIT-OPS-003` | tactile radio dial and audio playback | audio/electrical | operator plays final broadcast | reset station and audio state | knob loosened or audio misfires |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|
| PRINT-001 | laminated postcards | P1 | 1 | 1 | C4 required | 1 | 1 | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | bending, loss, markings | spare set | duplicate route pouch at operator station | wipe and inspect |
| PRINT-001 or CUSTOM-PRINT | mounted route map | P1 | 1 | 1 | C3 beat-support | 2 | 2 | `$5-60` | print shop or local fabrication | Planning | 0-1 week | B | scratches or peeling | operator verbal clue | reprint file and verbal route cue | clean, reattach |
| ELEC-002 | prop breaker switches | P2 | 6 | 2 | C4 required | 3 | 2 | `$2-12` each | Digi-Key, Mouser, Adafruit, Amazon | Planning | 0-1 week | B | switch false negative | manual UV trigger | manual trigger staged in operator controls | test before session |
| ELEC-001 | UV cabinet strip | P3 | 1 | 1 | C4 required | 2 | 2 | `$10-35` | Amazon, Adafruit, big-box lighting | Planning | 0-1 week | B | light failure | reveal backup labels | printed frequency labels in backup envelope | test, replace strip |
| SCENIC-005 or CUSTOM-SCENIC | galley object set | P3 | 1 | 1 | C4 required | 3 | 2 | `$15-75` | general retail, thrift/scenic, local fabrication | Planning | 0-2 weeks | C | missing object | spare set | duplicate object pouch and count card | count after reset |
| MECH-001 or MECH-004 | rotating table latch/track | P4 | 1 | 1 | C5 showstopper | 6 | 1 | `$12-70` | big-box hardware, McMaster-Carr, local fabrication | Planning | 0-2 weeks | Unknown | jams or loosens | operator compass clue | staff compass mark bypass; post-run latch swap | tighten and test |
| MECH-006 | radio dial/knob | P5 | 1 | 1 | C4 required | 2 | 1 | `$3-25` | Amazon, McMaster-Carr, electronics suppliers | Planning | 0-1 week | B | knob loosened or slips | operator playback | spare knob and visible tuned mark | inspect set screw |
| ELEC-006 | audio module/player | P5 | 1 | 1 | C3 beat-support | 2 | 2 | `$15-50` | Amazon, Adafruit audio board, thrift retail | Planning | 0-1 week | C | audio failure | operator playback | operator device and transcript backup | pre-run audio test |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| switch panel fails to trigger UV | P2, P3 | operator sees correct sequence but no light | manual UV trigger | separate puzzle logic from fragile circuit |
| table latch jams | P4 | players find compass but cannot rotate/latch | point to backup bearing clue | prototype latch under force |
| radio audio misfires | P5 | correct tuning with no playback | operator plays final broadcast | add visible success indicator |

## Device play profiles

| Beat ID | Device/module | Crowd profile | Advantages in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P1 | `DEV-TRAY-001` route rail/map | helper | makes route order public and physical | postcards become search soup or private reading | low card count, mileage plaques, map silhouettes |
| P2 | `DEV-SWITCH-001` breaker panel | tactile helper | trailer systems feel real while staying low-voltage | random switching or unclear accepted state | fused low voltage, labels, state light/manual bypass |
| P3 | `DEV-LED-001` UV cabinet / `DEV-BIN-001` inventory | discovery helper | missing objects and light reveal make galley physical | UV dependence, object loss, or color-only reading | visible backup labels, object count, duplicate set |
| P4 | `DEV-FOLD-001` transform table | transformation pleaser | trailer furniture changing meaning is memorable | players lean, force latch, or cannot reach orientation | no-load rule, hinge/stop inspection, staff compass fallback |
| P5 | `DEV-DIAL-001` radio / `DEV-AUDIO-001` playback | classic finale pleaser with audio risk | tactile tuning and broadcast make the route feel alive | fine-tuning drift or audio-only success | detents/big marks, visual tuned state, transcript/playback backup |

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|
| P1 | `DEV-TRAY-001` route rail/map | L0 idea | Green | helper | route feels like worksheet | wrong route accepted | correct order not recognized | route order visible | team searches cards after Hint 2 |
| P2 | `DEV-SWITCH-001` breaker panel | L0 idea | Orange | tactile helper | random toggling | wrong sequence powers UV | correct sequence fails | UV/state light visible | powered fault blocks room without bypass |
| P3 | `DEV-LED-001` UV cabinet | L0 idea | Orange | discovery helper | light-only clue or missing object | wrong/missing object gives digit | correct object state unreadable | frequency digits visible | low-light/UV path has no print equivalent |
| P4 | `DEV-FOLD-001` transform table | L0 idea | Yellow | transformation pleaser | force, reach, or latch jam | wrong orientation yields bearing | correct orientation sticks | table/bearing visible | players can lean/force unsafe state |
| P5 | `DEV-DIAL-001` / `DEV-AUDIO-001` radio | L0 idea | Orange | finale pleaser | fine-tune/audio frustration | wrong tune plays final | correct tune silent | tuned mark/playback visible | audio failure blocks finale |

## Part diagrams

Use `components\VISUALS.md`.

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| P1 route cards and public rail | `components\diagrams\dev-rail-proof-order.excalidraw` | public order state, card/token homes, wrong-order reject/recovery | build-ready: postcard dimensions and map sightline declared below; replace with measured prototype photo after route test |
| P1/P3 counted postcards and galley object set | `components\diagrams\dev-counted-prop-card-set.excalidraw` | count card, duplicate pouch, missing/bent/marked item recovery | build-ready: final object list and reset photo target declared below; replace with prototype reset photo |
| P2/P3 breaker, UV, and manual light backup | `components\diagrams\dev-low-voltage-control.excalidraw` | low-voltage player controls, fused build, printed manual state recovery | build-ready: switch layout and UV-safe printed equivalent declared below; replace with wired prototype photo after low-voltage test |
| P4 fold table/latch and P5 radio dial | `components\diagrams\dev-transform-dial-surface.excalidraw` | stops, detents, start marks, no-load script, knob/latch recovery | build-ready: table hinge/track dimensions and radio scale marks declared below; replace with measured prototype photo after bench test |

## Kit selection

| Kit ID | Applies to | Why included | Remove/simplify if |
|---|---|---|---|
| `KIT-PROTO-004` | P1 map/route comparison | tests map/card readability and private-solve risk | map becomes a fixed printed order clue |
| `KIT-ELEC-001` | P2/P3 lights | keeps low-voltage lighting fused and manually recoverable | passive labels replace required lighting |
| `KIT-ELEC-003` | P5 audio | ensures final broadcast has transcript/visual equivalent | audio becomes reward only |
| `KIT-MECH-003` | P4 bearing mark | supports readable index/detent for transformed table | table mark becomes a static printed card |
| `KIT-MECH-005` | P5 radio dial | keeps tuning tactile but resettable | final tuning becomes a simple selector card |
| `KIT-MECH-007` | P4 fold-down table | forces hinge/stop/no-load review | table is replaced with wall panel |
| `KIT-OPS-002` | P3 objects | duplicate object set prevents dead sessions | objects are fixed to cabinet |
| `KIT-OPS-003` | powered/audio backups | printed equivalents for UV/audio states | no powered/audio required states remain |
| `KIT-OPS-004` | route, cabinet, table, radio | reset photos for all movable states | movable states reduce below three |
| `KIT-OPS-005` | C4-C5 recovery | stages route/object/radio spares and table/radio bypass scripts | all required items are fixed and bypasses are proven |

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|
| postcard set | 1 | C4 | operator kit | 1 | any card is damaged or missing |
| galley object set | 1 | C4 | P3 pouch/reset bin | 2 | object missing or visibly damaged |
| switch caps | 3 | C3-C4 | parts pouch | 3 | cap cracks or label wears |
| UV strip | 1 | C4 post-run | parts pouch | 5 | strip flickers or fails pre-run |
| radio knob | 1 | C4 | parts pouch | 1 | knob loosens or slips |

## C5 transform table and radio surface spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Fold table envelope | fold/rotate surface target: 24-30 in wide by 16-20 in deep, mounted at 30-36 in height; no load-bearing player use | keeps trailer aisle clear and prevents leaning/force from becoming a solve step | cardboard/table mockup placed in floorplan with seated and standing reach check |
| Hinge/track | two supported hinges or a guided track with hard stops at start and solve orientations; no exposed pinch gap under 1/4 in in player reach | prevents pinch and ambiguous "sticky hardware" feel | 10 open/close cycles with no finger pinch and no false bearing |
| Latch/stop | visible latch or detent holds start and solve positions; wrong orientation has a clear not-yet stop | players get feedback without forcing the furniture | wrong-state pull/twist test with no false reveal |
| Compass/bearing mark | high-contrast compass rose at least 6 in diameter; solved bearing mark visible from group distance and duplicated on staff card | success is visual, not fine motor or private reading | photo from seated checker and operator sightline |
| Radio dial scale | dial face target: 4-6 in diameter, bold detents at route frequencies, reset mark at start, spare knob with set screw | final tuning is tactile but resettable and not audio-only | two reset cycles with tuned mark matching operator key |
| Staff bypass | staff compass mark and operator playback card staged with radio knob spare | C5 recovery stays under 1 minute if table jams | timed staff compass/playback drill under 60 seconds |

## C4 route, galley, and UV cabinet spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Route postcards | 6 oversized laminated postcards, 4 x 6 in minimum, with staff-only reset index on back and one public rail/map home per card | keeps P1 physical and countable without private paper search | route rail reset photo with all six cards |
| Map sightline | route wall mounted at shared viewing height; key route marks readable from seated checker at 3-5 ft | keeps navigation clue accessible and off the floor | seated/standing sightline photo |
| Galley object list | 6 chunky removable objects: mug, tin plate, spice jar, towel tag, ticket stub, and key fob; only 3 are active UV/route evidence | enough tactile inventory without search soup | object count card and active/inactive reset photo |
| Object homes | shallow galley tray or cabinet silhouettes with no loose floor props; duplicate object pouch staged at operator station | missing objects become visible during reset and recoverable in run | two-minute missing-object replacement drill |
| Breaker switch layout | six low-voltage prop switches in left-to-right route order, with start/off marks, high-contrast labels, and operator-visible solved indicator | random toggling cannot create unsafe or hidden state | chaotic-toggle test plus reset photo |
| UV-safe equivalent | UV reveal is duplicated by sealed printed frequency-label envelope or visible-light card in operator kit | required clue does not rely on UV exposure, darkness, or color-only reading | UV-off solve path gives same frequency digit |
| Manual light path | operator manual UV/label trigger is separate from player switches and staged in the control kit | electrical fault does not block P3 | timed manual reveal under 2 minutes |

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|
| Route rail and postcard set | P1 | 2 | 4 | 1 | missing postcard replaced in 1 min | team cannot read route without private paper search |
| Breaker/UV cabinet | P2-P3 | 5 | 10 | 2 | manual UV/printed label recovery in 2 min | correct input can leave players stuck |
| Galley object set | P3 | 2 | 5 | 2 | missing object replaced in 2 min | object count cannot be verified |
| Fold table/latch | P4 | 6 | 12 | 2 | staff compass fallback in 1 min | table can be leaned on, pinched, or jammed |
| Radio final | P5 | 3 | 6 | 1 | knob/playback recovery in 1-2 min | final success depends on audio only |

## Criticality map

Use `components\RELIABILITY.md`.

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | rotating table latch/track | unsafe or jammed transform can block bearing and create force risk | staff compass mark plus post-run latch spare | timed staff fallback |
| C4 required | postcards, switches, UV strip, galley objects, radio knob | required path can stall if missing, false-negative, or unreadable | duplicate cards/objects, manual UV, spare knob | 1-2 minute drill |
| C3 beat-support | mounted map, audio player | supports clarity/finale but has verbal/transcript backup | reprint/audio backup | operator stress pass |
| C2 helper | scenic labels and non-required radio flavor | helpful but not required | repair consumables | post-run repair |
| C1 cosmetic | non-clue trailer dressing | no solve dependency | repair consumables | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Technique/device/kit IDs for every required beat | Draft pass | P1-P5 mapped to `TECH-*`, `DEV-*`, and `KIT-*` IDs | Refine after first bench tests. |
| Criticality, spare count, build time, and replacement time assigned | Draft | C3-C5 items have replacement targets and staged spares | Time admin drills. |
| C4-C5 admin replacement drill passes in time | TBD | drill plan only | Run route/object/table/radio replacement drills. |
| Device review level/risk band assigned | Draft pass | P1-P5 have L0/risk/crowd/frustration rows | Promote each to L1 after cardboard/bench tests. |
| Manual bypass for powered/audio beats | Draft | P2, P3, and P5 have manual or printed fallback | Test under operator stress. |
| Reset action verifiable under time pressure | TBD | reset plan only | Run reset drill against target. |

## Bench test plan

| Test ID | Device | Pass | What staff observes | Pass condition |
|---|---|---|---|---|
| BT-0001-P1-C | `DEV-TRAY-001` route rail/map | correct-use | team orders postcards against map/mileage plaques | route order becomes public, not private reading |
| BT-0001-P2-H | `DEV-SWITCH-001` breaker panel | chaotic-use | switches toggled randomly and quickly | no unsafe state; bypass/manual UV works |
| BT-0001-P3-X | `DEV-LED-001` UV cabinet | accessibility | team uses visible/printed equivalent without UV dependence | frequency digits remain fair |
| BT-0001-P4-H | `DEV-FOLD-001` transform table | chaotic-use | players lean/force/rotate table incorrectly | no pinch/false bearing; staff fallback works |
| BT-0001-P5-C | `DEV-DIAL-001` radio | correct-use | team tunes final route state | visual tuned state and audio payoff align |
| BT-0001-P5-X | `DEV-AUDIO-001` final broadcast | accessibility | team receives final success without relying on hearing | transcript/visual confirmation works |
| BT-0001-R | full room | reset/transport | route, switches, cabinet objects, table, radio reset from photos | operator reset stays within target |

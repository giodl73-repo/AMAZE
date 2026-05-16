# The Black Box at Mile Zero - Build

Use `docs/build-economics.md`, `components/TECHNIQUES.md`,
`components/DEVICES.md`, `components/KITS.md`,
`components/DEVICE-REVIEW.md`, `components/INVENTORY.md`, and
`components/SOURCING.md` before build readiness review.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$250-750` | Prioritize central box mockup, rail, transcript/audio loop, and low-voltage cabinet. |
| Production | `$750-2,500+` | Depends on final object size, latch quality, scenic finish, lighting, and enclosure fabrication. |
| Spares | `$75-250` | Evidence tags, transcript cards, latch parts, switches, bulbs/LEDs, printed backups. |
| Maintenance | `$25-100/run season` | Box latch inspection, cards/labels, batteries, small hardware replacement. |
| Transport | `$50-250` | Central object transport lock, foam, bins, tie-downs, and service kit. |
| Build hours | `25-45` | central box shell, witness windows, utility cabinet, latch stack, evidence packs |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|
| P1 | `TECH-SORT-001`, `TECH-TEAM-001` | `DEV-TRAY-001` evidence rail | `KIT-OPS-002`, `KIT-OPS-004` | evidence rail and removable tags | passive prop | operator names first match | tags rehung in seed order | tags pocketed/dropped |
| P2 | `TECH-ALIGN-003`, `TECH-REVEAL-003` | `DEV-WINDOW-001`, `DEV-FLAG-001` | `KIT-PROTO-004`, `KIT-MECH-002` | sliding witness windows on box | mechanical prop | operator sets first window | sliders reset closed | slider forced or jammed |
| P3 | `TECH-REVEAL-004`, `TECH-SORT-003` | `DEV-AUDIO-001` plus transcript cards | `KIT-ELEC-003`, `KIT-OPS-003` | audio recorder plus transcript cards | powered/passive hybrid | printed transcript only | audio rewound, cards replaced | audio ignored or device fails |
| P4 | `TECH-REVEAL-002`, `TECH-SORT-002` | `DEV-SWITCH-001`, `DEV-LED-001` | `KIT-ELEC-001`, `KIT-OPS-003` | low-voltage utility cabinet | powered prop | manual indicator card | switches reset off | rapid toggling or switch damage |
| P5 | `TECH-FIT-004`, `TECH-TEAM-002`, `TECH-REVEAL-003` | `DEV-LATCH-001` central latch stack | `KIT-MECH-006`, `KIT-TRANS-001`, `KIT-OPS-002` | central latch stack and one-handle reveal | mechanical/powered optional | operator manual release | latches relocked, reveal replaced | pulling/twisting latch |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|
| SCENIC-001 or SCENIC-002 | central black box enclosure | P2, P5 | 1 | 0 | C5 showstopper | 8 | 1 | `$25-100` prototype shell; `$100-250+` custom | Harbor Freight/Amazon/ULINE or local fabrication | Planning | 0-3 weeks | B/Unknown | latch/hinge damage, enclosure shift | manual release | staff release/open-state card at operator station | inspect every run day |
| MECH-004 | sliding witness window track | P2 | 4 | 1 | C4 required | 4 | 2 | `$12-45` | big-box hardware or McMaster-Carr | Planning | 0-1 week | B | jammed slider | operator sets state | first-window bypass and spare track section | clean/inspect tracks |
| SCENIC-004 | acrylic witness windows | P2 | 4 | 1 | C4 required | 2 | 2 | `$10-60` | big-box hardware or local plastics | Planning | 0-1 week | B/C | scratched/cracked panel | operator announces state | spare rounded panel in box kit | replace rounded panels |
| CORE-002 | evidence rail/bracket stock | P1 | 1 | 1 | C3 beat-support | 2 | 3 | `$8-35` | big-box hardware or McMaster-Carr | Planning | 0-1 week | A | tags detach or rail bends | spare tags | rail offcut and temporary clip path | inspect mounting |
| PRINT-003 | evidence tags/cards | P1, P2 | 16 | 1 | C4 required | 1 | 1 | `$5-20` | office supply, Amazon, print shop | Planning | 0-1 week | C | lost, bent, marked | duplicate pouch | duplicate tag pouch at operator station | count after each run |
| ELEC-006 | rugged/simple audio player | P3 | 1 | 1 | C3 beat-support | 2 | 2 | `$15-50` | Amazon, Adafruit, thrift retail | Planning | 0-1 week | C | battery/audio failure | transcript | printed transcript is equivalent path | charge/test daily |
| PRINT-001 | transcript cards | P3 | 1 | 1 | C4 required | 1 | 1 | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | missing card | spare set | spare transcript envelope | count/reset |
| ELEC-002 | low-voltage switches | P4 | 6 | 2 | C4 required | 3 | 2 | `$2-12` each | Digi-Key, Mouser, Adafruit, Amazon | Planning | 0-1 week | B | switch fault | manual cabinet card | manual card path staged at cabinet | pre-run switch test |
| ELEC-001 | cabinet LEDs/state lights | P4 | 6 | 1 | C3 beat-support | 2 | 2 | `$10-35` | Amazon, Adafruit, big-box lighting | Planning | 0-1 week | B | light fault | manual cabinet card | manual card path preserves clue | pre-run light test |
| ELEC-003 | low-voltage power supply | P4 | 1 | 1 | C4 required | 2 | 2 | `$10-30` | Amazon, Digi-Key, Adafruit | Planning | 0-1 week | B | power failure | passive/manual card path | spare supply or passive cabinet card | inspect/test daily |
| ELEC-004 | fused low-voltage distribution | P4 | 1 | 1 | C4 required | 2 | 3 | `$8-25` | automotive aisle, Amazon, electronics suppliers | Planning | 0-1 week | B | blown fuse, loose wire | passive/manual card path | fuse kit and manual cabinet card | inspect fuses |
| MECH-001 | final latch stack | P5 | 4 | 2 | C5 showstopper | 5 | 1 | `$5-25` each | big-box hardware, McMaster-Carr | Planning | 0-1 week | B | jam, pinch, misalignment | staff release | operator manual release; post-run latch swap | inspect and lubricate |
| MECH-005 | hinges for final reveal | P5 | 3 | 1 | C4 required | 2 | 3 | `$5-30` | big-box hardware, McMaster-Carr | Planning | 0-1 week | A/B | hinge sag or pinch | staff opens panel | hinge/screw pouch for post-run swap | inspect screws/clearance |
| OPS-002 | reset/spare organizer | all | 1 | 0 | C5 showstopper | 1 | 1 | `$5-25` | Harbor Freight, Walmart, Amazon | Planning | 0-1 week | B | spares disorganized | operator checklist | block next run if count fails | restock weekly |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| Central box jam | P2, P5 | player force or no state change | stop play, bypass, inspect | use robust hardware and visible feedback |
| Evidence loss | P1-P3 | reset count mismatch | use duplicate tag/card | tether or larger components |
| Audio unfairness | P3 | repeated replay with no progress | direct team to transcript | make transcript equivalent |
| Cabinet state drift | P4 | lights do not match switch state | manual indicator card | simpler passive alternative |

## Device play profiles

| Beat ID | Device/module | Crowd profile | Advantages in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P1 | `DEV-TRAY-001` evidence rail | helper | makes evidence public and prevents loose-tag chaos | rail becomes a dumping strip or tags feel like paperwork | silhouettes, low tag count, operator-visible reset photo |
| P2 | `DEV-WINDOW-001` witness windows | central-object pleaser | players see the black box change state physically | windows are treated as labels or sliders jam | large windows, visible before/after states, staff-set first window |
| P3 | `DEV-AUDIO-001` recorder with transcript | atmosphere pleaser with access risk | recorder feels like a witness while transcript keeps it fair | repeated replay becomes audio-only frustration | printed transcript is equivalent, not backup-only |
| P4 | `DEV-SWITCH-001` / `DEV-LED-001` utility cabinet | tactile helper | safe switching makes investigation feel like operating the site | random toggling or light-state drift | fused low voltage, manual indicator card, reset photo |
| P5 | `DEV-LATCH-001` central box | finale pleaser with force risk | opening one sealed object is memorable and thematic | latch feels stuck or invites pulling | no-force stops, clear "not yet" state, staff release |

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|
| P1 | `DEV-TRAY-001` evidence rail | L0 idea | Green | helper | evidence feels like filing | wrong tag treated as active evidence | correct tag missed | active tag set visible | team searches tags after Hint 2 |
| P2 | `DEV-WINDOW-001` witness windows | L0 idea | Yellow | central-object pleaser | slider jam or unclear state | wrong state activates evidence | correct state not visible | window state visible on box | players solve around box entirely |
| P3 | `DEV-AUDIO-001` recorder | L0 idea | Orange | atmosphere pleaser | audio replay loop with no progress | wrong transcript word accepted | correct skip/count not heard | transcript index path visible | transcript-only team cannot solve |
| P4 | `DEV-SWITCH-001` utility cabinet | L0 idea | Orange | tactile helper | random switching or light mismatch | wrong sequence lights latch path | correct sequence fails | cabinet state visible | switch state blocks final without bypass |
| P5 | `DEV-LATCH-001` box reveal | L0 idea | Yellow, Orange if powered | finale pleaser | players pull harder when latch resists | box opens before reconstruction | solved state fails to open | latch/reveal state visible | any false open or unsafe force |

## Part diagrams

Use `components\VISUALS.md`.

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| P1/P2 evidence tags and P3 transcript cards | `components\diagrams\dev-counted-prop-card-set.excalidraw` | count card, duplicate pouch, missing/bent/marked item recovery | build-ready: evidence count, tag homes, and transcript reset photo declared below; replace with run-ready photo after prototype pack |
| P2 witness windows and slider states | `components\diagrams\dev-overlay-inspection-window.excalidraw` | visibility, glare/parallax, serviceable window state, printed backup | build-ready: black-box window dimensions and slider stops declared below; replace with measured prototype photo after bench test |
| P4 low-voltage utility cabinet | `components\diagrams\dev-low-voltage-control.excalidraw` | safe switches, fused build, manual state card, power/light recovery | build-ready: switch order and cabinet state photo declared below; replace with wired prototype photo after low-voltage test |
| P5 central black box, latch stack, hinges, and enclosure | `components\diagrams\dev-latch-final-box.excalidraw` | latch alignment, hinge stops, manual release, force/pinch risks | build-ready: enclosure dimensions, handle travel, and transport lock declared below; replace with measured prototype photo after bench test |
| all reset/spare organizer | `components\diagrams\ops-recovery-kit.excalidraw` | pouch labels, count state, operator pull path, replacement target | build-ready: C5 recovery pouch map declared below; replace with photo after prototype pack is assembled |

## Kit selection

| Kit ID | Applies to | Why included | Remove/simplify if |
|---|---|---|---|
| `KIT-PROTO-004` | P2 witness windows | tests readability of window states before fabricating box | window state can be printed passively |
| `KIT-MECH-002` | P2 sliders/windows | supports sliding reveal without load-bearing hardware | sliders jam twice in bench test |
| `KIT-ELEC-003` | P3 recorder | ensures audio has transcript/visual equivalent | audio becomes pure ambience |
| `KIT-ELEC-001` | P4 cabinet | keeps low-voltage feedback fused and testable | passive cabinet cards prove enough |
| `KIT-MECH-006` | P5 latch stack | forces no-force latch review and staff release | final reveal becomes a simple flag/card |
| `KIT-OPS-002` | evidence/latch proofs | spare tags/proofs prevent dead sessions | all critical states become mounted/fixed |
| `KIT-OPS-004` | full room | reset photos for tags, windows, cabinet, latches | movable states reduce below three |
| `KIT-OPS-005` | C4-C5 recovery | stages duplicate evidence, transcript, manual cabinet cards, and latch release | all required states are fixed and manual release is tested |
| `KIT-TRANS-001` | central box and loose evidence | protects box, tags, and transcript cards between venues | room is permanently installed |

## Abuse cases

| Component/beat | Pull/twist/drop/spill/force scenario | Expected result | Design protection |
|---|---|---|---|
| central box | player pulls handle before solved | handle does not move dangerously | mechanical stop and no-force label in fiction |
| windows | player shoves slider | slider stops without pinch | rounded edges, serviceable tracks |
| evidence tags | player drops or pockets tag | operator can identify missing tag | countable tags and spare pouch |
| cabinet | player rapid-toggles switches | no unsafe electrical state | low-voltage only and fused circuit |

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|
| evidence tag set | 1 full set | C4 | operator kit | 1 | missing, bent, or marked |
| transcript card set | 1 full set | C4 | operator kit | 1 | missing or unreadable |
| latch parts | 2 latches plus matching screws | C5 post-run | maintenance kit | 5 | any binding or visible wear |
| LEDs/switches | 2 switches plus LED strip/indicator spares | C3-C4 | maintenance kit | 3 | pre-run test fails |
| low-voltage power supply | 1 | C4 | operator/maintenance kit | 2 | power fault or intermittent state |
| window panel/track parts | 1 panel/track section | C4 post-run | maintenance kit | 5 | scratching, cracking, or sticking |

## C5 enclosure and latch release spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Central box envelope | tabletop/service-height box target: 24-30 in wide, 16-20 in deep, 14-18 in high; rounded exterior corners; no hidden weight-bearing surface | large enough for public focus but small enough for trailer aisle clearance and staff reach | cardboard shell placed in floorplan path with 2-person pass-by check |
| Witness window face | four sliding windows on player face, each target 4 x 6 in minimum visible state; sliders stop before finger pinch zone | state is visible from group distance and cannot trap fingers | photo of closed/open window states from operator sightline |
| Final reveal lid/door | one reveal face with 1-2 in handle travel before hard stop; reveal opens only after latch stack or staff release | players feel "not yet" without needing force | pull test with no false open after 10 moderate attempts |
| Latch stack | four visible latches/hasps, each with clear open/closed state and 1/8-1/4 in alignment tolerance; no latch bears trailer structure load | latches are puzzle state, not structural security | two reset cycles with no binding and no pinch points |
| Staff release | red-tab BOX-RELEASE pouch opens/reveals from operator side without reaching through player crowd | C5 recovery target stays 1 minute | timed staff release drill under 60 seconds |
| Transport lock | separate non-puzzle strap/clip used only during transport; removed before play and stored in operator kit | prevents confusing transport hardware with puzzle hardware | pre-run checklist line and reset photo |

## C5 recovery pouch map

| Pouch/slot | Contents | Covers | Stored where | Operator pull path | Live replacement target | Reset proof |
|---|---|---|---|---|---:|---|
| BOX-RELEASE | manual release key/card plus open-state card | central box enclosure, final latch stack | operator hip pouch, red tab | announce manual release, open box, place open-state card, log latch service | 1 | relocked/start-state photo |
| BOX-LATCH-POST | 2 latches, screws, hinge driver | final latch stack post-run swap | maintenance kit behind operator station | hold next run if latch binds; swap after players exit | 5 | latch moves freely twice |
| EVIDENCE-DUPES | evidence tag set plus transcript card set | evidence tags/cards, transcript cards | operator kit, blue tab | replace missing tag/card, mark original for repair | 1 | count card and tag order photo |
| CABINET-MANUAL | manual cabinet state card, spare switch caps, spare power supply, fuse kit | utility cabinet switches, power, fuse path | operator/maintenance kit, yellow tab | switch to passive/manual state card and continue clue path | 2 | switch-off photo plus power test |
| WINDOW-SERVICE | spare rounded panel/track section | witness windows/sliding tracks | maintenance kit, clear sleeve | staff-set first window during run; swap track post-run | 2 during run, 5 post-run | window start-state photo |

## C4 evidence, window, and utility-cabinet spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Evidence tag count | 16 evidence tags/cards total, numbered on staff-facing backs only, with four rail homes matching P1/P2 active sets and a reset count card | prevents missing evidence from masquerading as puzzle ambiguity | reset photo shows all 16 tags plus active rail order |
| Tag homes | rail homes are waist/table height, outside egress, with silhouettes or hook labels for each active tag group | keeps evidence physical and operator-visible without floor props | operator camera can verify active/missing tag state |
| Transcript pack | one complete transcript set plus one duplicate in `EVIDENCE-DUPES`; transcript is equivalent to audio, not a hint-only backup | required information does not depend on hearing or batteries | transcript-only solve path tested once |
| Witness windows | four acrylic windows, each 4 x 6 in minimum visible state, rounded edges, with closed/open marks readable from 3-5 ft | window state is public and not a tiny slider clue | seated checker and operator sightline photos |
| Slider stops | each slider has hard stops before pinch zone, a visible start mark, and a staff-set first-window bypass path | players get "not yet" without forcing the track | 10 shove/pull attempts with no false state or pinch |
| Utility switch order | six switches arranged left-to-right in route order with high-contrast labels, protective low-voltage spacing, and staff-only reset marks | discourages random toggling and makes reset visible | cabinet start-state photo matches operator key |
| Cabinet power path | fused low-voltage supply only; player side exposes switches/indicators, not wiring; manual state card sits in `CABINET-MANUAL` pouch | powered failure cannot block the clue or create unsafe access | power-off/manual-card drill under 2 minutes |
| Cabinet state photo | reset photo captures switch-off state, indicator test, manual card location, and fuse/power spare location | gives staff a concrete reset target | two reset cycles with photo comparison |

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|
| Central box shell and latch stack | P2, P5 | 8 | 16 | 2 | staff release in 1 min | any false open, pinch, or unobservable state |
| Witness windows/sliders | P2 | 4 | 8 | 2 | jammed window staff-set in 2 min | slider force causes false evidence |
| Evidence rail and tag set | P1-P2 | 2 | 4 | 2 | missing tag replaced in 1 min | tag count cannot be verified |
| Recorder/transcript path | P3 | 2 | 4 | 1 | transcript-only path in 1 min | hearing is required |
| Utility cabinet | P4 | 5 | 10 | 2 | manual cabinet card in 2 min | switch/light failure blocks final |

## Criticality map

Use `components\RELIABILITY.md`.

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | central box enclosure, final latch stack, reset organizer | final reveal and safe recovery depend on them | manual release plus latch parts and count checklist | timed release drill |
| C4 required | windows/tracks, evidence tags, transcripts, switches, power/fuse path, hinges | required evidence or final path can stall | duplicate tags/transcripts, manual cabinet card, spare power/switches | 1-3 minute replacement drill |
| C3 beat-support | evidence rail, audio player, cabinet LEDs | clarity/atmosphere but bypassable | printed/passive equivalent | operator stress pass |
| C2 helper | ambience and non-required dressing | no required dependency | repair consumables | post-run repair |
| C1 cosmetic | black-box scratches/scenic dressing | no solve dependency | touch-up kit | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft | P1-P5 have proposed mechanisms | Prototype central box before graph promotion. |
| BOM line for every required component | Draft | inventory-backed BOM above | Verify quantities during prototype. |
| Inventory ID or custom fabrication plan for every critical component | Draft | common inventory IDs assigned; central box may become custom | Decide whether black box is stock case, scenic build, or fabricated module. |
| Technique/device/kit IDs for every required beat | Draft pass | P1-P5 mapped to `TECH-*`, `DEV-*`, and `KIT-*` IDs | Refine after first prototype. |
| Criticality, spare count, build time, and replacement time assigned | Draft | C3-C5 items have replacement targets and staged spares | Time admin drills. |
| C4-C5 admin replacement drill passes in time | TBD | drill plan only | Run evidence/window/cabinet/latch replacement drills. |
| Device review level/risk band assigned | Draft pass | P1-P5 have L0/risk/crowd/frustration rows | Promote each to L1 after cardboard/bench tests. |
| Known cost band for critical components | Draft | planning bands assigned | Check current prices before prototype budget lock. |
| Manual bypass for powered/sensed/fragile beats | Draft | P3-P5 bypasses named | Test staff use under timing pressure. |
| Durability class not D for required components | TBD | no prototype | Assign durability class after mockup. |
| Reset action verifiable under time pressure | TBD | reset plan only | Run reset drill against 10-minute target. |

## Bench test plan

| Test ID | Device | Pass | What staff observes | Pass condition |
|---|---|---|---|---|
| BT-0002-P1-C | `DEV-TRAY-001` evidence rail | correct-use | team hangs active evidence by matching marks | evidence state is public and not search soup |
| BT-0002-P2-C | `DEV-WINDOW-001` witness windows | correct-use | window slides change visible box state | players explain the state change |
| BT-0002-P2-H | `DEV-WINDOW-001` witness windows | chaotic-use | sliders are shoved or wrong states tried | no jam, pinch, or false active evidence |
| BT-0002-P3-X | `DEV-AUDIO-001` recorder | accessibility | team solves from transcript without hearing audio | route phrase/index remains fair |
| BT-0002-P4-H | `DEV-SWITCH-001` cabinet | chaotic-use | switches are toggled rapidly/wrongly | no unsafe state and manual card recovers |
| BT-0002-P5-H | `DEV-LATCH-001` box reveal | chaotic-use | handle/latches are pulled before solved | no false open and no unsafe force path |
| BT-0002-R | full room | reset/transport | tags, windows, recorder, cabinet, and box reset from photos | one operator resets under 10 minutes |


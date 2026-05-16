# The Brineworks at Low Tide - Build

Use `docs/build-economics.md`, `components/TECHNIQUES.md`,
`components/DEVICES.md`, `components/KITS.md`,
`components/DEVICE-REVIEW.md`, `components/INVENTORY.md`, and
`components/SOURCING.md` before build readiness review.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$200-650` | foamcore griddle, printed tiles, pipe board, clam register mockup, dry bubble cards. |
| Production | `$700-2,200+` | durable counter props, wall-mounted pipe board, finished griddle, lighting, scenic finish. |
| Spares | `$75-250` | tile set, pipe elbows, token set, labels, hinges, LEDs. |
| Maintenance | `$25-100/run season` | print refresh, hinge tightening, token replacement, label cleaning. |
| Transport | `$50-200` | bin lids, panel locks, griddle transport lock, token organizer. |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|
| P1 | `TECH-SORT-001` | `DEV-BIN-001` plus printed shift frame | `KIT-OPS-003`, `KIT-OPS-004` | shift card, category plaques | passive | operator names categories | card/plaques reset | pages bent |
| P2 | `TECH-SORT-002`, `TECH-FIT-002` | `DEV-RAIL-001` | `KIT-PROTO-001`, `KIT-PROTO-002`, `KIT-OPS-002` | ingredient tiles, order rail, service ticket | analog/mechanical | operator demonstrates one rejected tile and gives staff ticket | tiles/ticket reset | tiles dumped/pocketed |
| P3 | `TECH-ALIGN-002` | `DEV-PIPE-001` | `KIT-PROTO-003`, `KIT-MECH-004`, `KIT-OPS-004` | pipe board, elbows, current gear | analog/mechanical | operator sets one elbow and gives staff gear | elbows/gear reset | elbows forced or removed |
| P4 | `TECH-FIT-001`, `TECH-FIT-004` | `DEV-COVER-001`, `DEV-SOCKET-001` | `KIT-PROTO-002`, `KIT-MECH-001`, `KIT-OPS-002` | clam register, chunky bell tokens, pearl hinge cup, visual ring mark, shell tag | analog/mechanical | operator demonstrates wrong bell fit and gives staff tag | bells/cup/tag reset | clam forced, cup worn, or bell pocketed |
| P5 | `TECH-TEAM-002`, `TECH-REVEAL-003` | `DEV-FINAL-003`, `DEV-SOCKET-001` | `KIT-PROTO-002`, `KIT-MECH-003`, `KIT-ELEC-001`, `KIT-OPS-002` | pressure griddle, proof sockets, gauge, open token | mechanical/lighting optional | staff release/open token | sockets/gauge/tokens reset | token jammed or griddle forced |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|
| CUSTOM-SCENIC plus CORE-001 | pressure griddle cabinet/counter | P5 | 1 | 0 | C5 showstopper | 8 | 1 | `$75-250+` | local fabrication, big-box panels | Planning | 1-3 weeks | Unknown | socket/gauge shift | staff open token | operator uses manual open token and logs griddle service | inspect run day |
| CUSTOM-SCENIC or PRINT-005 | order/current/bell proof tokens and sockets | P2-P5 | 3 | 1 | C4 required | 2 | 1 | `$10-60` | craft stores, local fabrication, Amazon | Planning | 0-2 weeks | B/C | token loss/jam | staff proof tokens | locked proof pouch at operator station | count/fit-test |
| PRINT-001 | shift card/category plaques/P0 labels | P1-P5 | 1 | 1 | C3 beat-support | 1 | 1 | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | bent/marked/missing | duplicate set | duplicate envelope on clipboard | count/replace |
| PRINT-005 or CUSTOM-SCENIC | oversized ingredient tiles | P2 | 6 | 1 | C4 required | 1 | 1 | `$5-40` | print shop, craft stores | Planning | 0-1 week | C/B | tile loss, edge wear | staff ticket | spare tile set in P2 pouch | count/reset |
| PRINT-005 | ingredient bins/tray | P2 | 1 | 1 | C2 helper | 1 | 2 | `$5-25` | Amazon, ULINE, craft stores | Planning | 0-1 week | C/B | prop clutter | operator tray map | spare tray in reset tote | restock/reset |
| CUSTOM-MECH or MECH-004 | `DEV-RAIL-001` order rail release | P2 | 1 | 1 | C4 required | 3 | 2 | `$10-50` | local fabrication, big-box hardware | Planning | 0-2 weeks | Unknown | false release/stick | staff ticket | staff ticket substitutes for release if rail sticks | test daily |
| CORE-001 plus CUSTOM-MECH | `DEV-PIPE-001` pipe board and chunky elbows | P3 | 1 | 2 | C4 required | 4 | 2 | `$25-120` | big-box hardware, local fabrication | Planning | 0-2 weeks | B/Unknown | elbow drift/force | staff gear | two spare elbows in P3 pouch; staff can set route leg | state photo |
| CUSTOM-SCENIC or MECH-005 | `DEV-COVER-001` clam register hinge/prop with pearl cup | P4 | 1 | 1 | C4 required | 4 | 2 | `$15-75` | craft stores, local fabrication | Planning | 0-2 weeks | Unknown | hinge stick, pinch, cup tolerance drift | staff tag | spare cup insert plus staff tag bypass | inspect hinge/cup |
| PRINT-005 or CUSTOM-SCENIC | chunky bell tokens with one pearl-fit token | P4 | 4 | 1 | C4 required | 1 | 1 | `$5-25` | craft stores, print shop | Planning | 0-1 week | C/B | token loss or wrong token fits | staff tag | spare bell set in P4 pouch | count/fit-test |
| ELEC-001 | dry bubble/open lighting | P5/reward | 1 | 1 | C2 helper | 1 | 2 | `$10-35` | Amazon, Adafruit, big-box lighting | Planning | 0-1 week | B | light failure | printed open token | skip lighting and use printed payoff | pre-run test |
| OPS-002 | reset/spare organizer | all | 1 | 0 | C5 showstopper | 1 | 1 | `$5-25` | Harbor Freight, Walmart, Amazon | Planning | 0-1 week | B | spares disorganized | checklist | operator blocks next run if organizer count fails | restock weekly |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| Tile search soup | P2 | players rummage/dump bins | reframe rail and count tiles | reduce tile count, bigger edge shapes |
| Pipe board fiddly or high | P3 | repeated tiny adjustments/reach | set one elbow | enlarge/lower elbows |
| Bell solve becomes audio-only | P4 | players listen instead of testing pearl fit | point to visual ring mark and hinge cup | make clam closure obvious |
| Clam fit tolerance unfair | P4 | wrong bell closes, or pearl bell fails | demonstrate intended fit and use staff tag | enlarge cup, exaggerate nub differences, add no-force stop |
| Clam or griddle force risk | P4-P5 | pulling/pressing props | no-force script and staff bypass | stronger stops/rounded edges |
| Proof tokens too similar | P2-P5 | wrong socket attempts | name category proofs | distinct silhouettes |
| Reset exceeds 10 min | all | timed reset over target | simplify loose parts | reduce tiles/elbows/bell tokens |

## Device play profiles

| Beat ID | Device/module | Crowd profile | Advantages in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P2 | `DEV-RAIL-001` order rail | tactile pleaser | physical lunch-building is funny, visible, and easy for mixed ages | tile pile becomes search soup or color-only matching | 5-7 oversized tiles, edge-shape rule, wrong-tile rejection |
| P3 | `DEV-PIPE-001` current board | family/team pleaser | gives the wall a physical group task and lets a seated checker guide | small elbows, high reach, or ambiguous route | waist-height board, chunky arrows, start/end marks, staff-set elbow fallback |
| P4 | `DEV-COVER-001` plus `DEV-SOCKET-001` clam register | comic pleaser with force risk | clam approval is memorable and physical without audio dependence | wrong bells almost fit or players force the shell | exaggerated pearl cup, soft stop, visible ring mark, no-force hint |
| P5 | `DEV-FINAL-003` pressure griddle | signature machine pleaser | all proofs converge into one central restart/open moment | players dump tokens or cannot tell accepted state | keyed proof sockets, big gauge/open token, caller/checker/handler roles |

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|
| P1 | printed shift frame / `DEV-BIN-001` | L0 idea | Green | helper | players treat bins as search before categories | category plaques imply solved state | players miss category frame | team names order/current/bell | category confusion persists after Hint 2 |
| P2 | `DEV-RAIL-001` order rail | L0 idea | Yellow | tactile pleaser | tile pile, color-only matching, sticky release | wrong stack releases ticket | correct stack fails to release | ticket in rail/proof home | confused team cannot explain edge rule |
| P3 | `DEV-PIPE-001` current board | L0 idea | Yellow | family/team pleaser | high reach, fiddly elbows, ambiguous path | wrong route releases gear | correct route not recognized | blue gear released or route photo matches | seated checker cannot participate |
| P4 | `DEV-COVER-001` / `DEV-SOCKET-001` clam register | L0 idea | Yellow | comic pleaser with force risk | wrong bells nearly fit or shell invites pushing | wrong bell reveals shell tag | pearl bell needs force or tag sticks | shell tag visible; clam closed to ring mark | any team can force shell/tag |
| P5 | `DEV-FINAL-003` / `DEV-SOCKET-001` pressure griddle | L0 idea | Yellow, Orange if lit | signature machine pleaser | random token dumping or unclear accepted state | wrong token opens griddle | earned proof fails to seat | three sockets/gauge/open token visible | false open or staff cannot bypass |

## Part diagrams

Use `components\VISUALS.md`.

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| P2 order rail and ingredient tiles | `components\diagrams\dev-rail-proof-order.excalidraw` | public sequence action, channel/stops, tile loss, jam, false release, staff ticket recovery | build-ready: tile dimensions and wrong-tile reject behavior declared below; replace with measured prototype photo after rail test |
| P2/P4/P5 proof tokens and sockets | `components\diagrams\dev-socket-proof-token.excalidraw` | token fit, wrong-token reject path, camera-visible state, staff proof pouch | build-ready: proof silhouettes and griddle socket layout declared below; replace with measured prototype photo after fit test |
| P2/P4 counted tiles and bell token set | `components\diagrams\dev-counted-prop-card-set.excalidraw` | count card, duplicate pouch, missing/bent/marked item recovery | build-ready: tile/bell count cards and pouch labels declared below; replace with prototype reset photo |
| P3 pipe board, P4 clam cover, and P5 pressure griddle | `components\diagrams\dev-pipe-cover-final-machine.excalidraw` | chunky route board, no-force cover/cup, pressure/gauge open state, manual staff gear/tag/open token | build-ready: pipe board height, clam hinge stop, and griddle gauge dimensions declared below; replace with measured prototype photo after bench test |
| all reset/spare organizer | `components\diagrams\ops-recovery-kit.excalidraw` | pouch labels, count state, operator pull path, replacement target | build-ready: C5 recovery pouch map declared below; replace with photo after prototype pack is assembled |

## Kit selection

| Kit ID | Applies to | Why included | Remove/simplify if |
|---|---|---|---|
| `KIT-PROTO-001` | P2, P4, P5 rough faces | proves fit/reveal aha before fabrication | cardboard edges create misleading failure |
| `KIT-PROTO-002` | P2, P4, P5 tokens/sockets | proves keyed proofs, no-force closure, and final sockets | tokens become too many to reset |
| `KIT-PROTO-003` | P3 current board | proves route readability and seated checker role | board height/route remains ambiguous |
| `KIT-OPS-001` | full room | baseline operator tools for staffed trailer runs | venue supplies equivalent kit |
| `KIT-OPS-002` | P2-P5 proofs | duplicate proof set prevents dead sessions | proofs become fully attached/fixed |
| `KIT-OPS-003` | P1, optional audio/light backups | keeps any sensory flavor non-required | no powered/audio prompts remain |
| `KIT-OPS-004` | all movable stations | reset photos for rail, pipe, clam, griddle | movable state count is reduced below three |
| `KIT-OPS-005` | C4-C5 recovery | stages pouches, bypass tokens, and timed admin replacement drills | all required props become fixed and manual releases are tested |
| `KIT-MECH-001` | P4 clam | no-force stop and hinge safety | clam becomes a flat socket with no closure |
| `KIT-MECH-003` | P5 griddle gauge | creates big readable accepted/open state | gauge is replaced by a static open token |
| `KIT-MECH-004` | P3 pipe board | removable route pieces with marked polarity | route becomes fixed hinged elbows |
| `KIT-ELEC-001` | P5 dry bubble/open light | optional feedback and delight only | lighting adds reset/reliability debt |
| `KIT-TRANS-001` | loose tokens/props | protects proofs, bells, tiles between venues | tokens are permanently tethered |
| `KIT-MAINT-001` | all prototype devices | daily tightening, labels, bumpers, small repairs | public build has dedicated maintenance pack |

## Abuse cases

| Component/beat | Pull/twist/drop/spill/force scenario | Expected result | Design protection |
|---|---|---|---|
| ingredient tiles | player dumps/drops/pockets tile | tile survives; count reveals missing | oversized tiles, count card, spare set |
| pipe elbows | player twists/pulls hard | elbow stops or detaches safely without sharp edge | rounded parts, sacrificial magnets/clips |
| clam register | player forces clam closed or jams wrong bell into cup | hinge/stops survive and no false solve | hinge stop, oversized cup, no-force script |
| griddle sockets | player jams wrong token | socket rejects safely | keyed shapes, rounded tokens |
| bubble/open effect | player expects steam/water | no heat/water present | dry lights/cards only |

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|
| printed card/label set | 1 full set | C3 | operator clipboard | 1 | label missing/bent/marked |
| ingredient tile set | 1 full set | C4 | P2 staff pouch | 1 | tile missing/worn |
| pipe elbow | 2 | C4 | P3 staff pouch | 2 | elbow cracked/loose |
| bell token set | 1 full set | C4 | P4 staff pouch | 1 | token missing/worn |
| pearl cup insert | 1 | C4 | maintenance kit at operator station | 2 | cup worn, loose, or wrong bell fits |
| proof token set | 1 full set | C4 | locked operator proof pouch | 1 | token missing/jammed |
| hinge/screw/magnet set | 1 set | C3-C4 post-run | maintenance kit | 5 | prop loosens |
| LED strip/light | 1 set | C2 | maintenance kit | 2 | pre-run light test fails |

## C5 recovery pouch map

| Pouch/slot | Contents | Covers | Stored where | Operator pull path | Live replacement target | Reset proof |
|---|---|---|---|---|---:|---|
| GRIDDLE-OPEN | manual open token, staff ticket, proof-state card | pressure griddle cabinet/counter | operator hip pouch, red tab | verify proofs, hand open token, log griddle service | 1 | griddle start-state photo |
| PROOF-STAFF | order/current/bell proof token set | proof tokens and sockets | locked operator proof pouch | hand matching proof token, call proof state | 1 | proof token count/fit-test |
| P2-TILES | ingredient tile set and staff ticket | oversized ingredient tiles, order rail release | P2 staff pouch, blue tab | replace tile or hand staff ticket if rail sticks | 1 | tile count and rail clear photo |
| P3-PIPE | 2 pipe elbows and staff gear | pipe board and chunky elbows | P3 staff pouch, green tab | swap elbow or set one route leg with staff gear | 2 | route start-state photo |
| P4-CLAM | bell token set, pearl cup insert, staff tag | clam register and bell tokens | P4 staff pouch plus maintenance cup sleeve | replace bell/cup or hand staff tag after visible fit attempt | 1-2 | clam closes to ring mark |
| ORGANIZER-MAP | laminated pouch map and pouch labels | reset/spare organizer failure | operator clipboard front page | use map to locate station pouch; relabel before next run | 1 | pouch map checked against kit |

## C5 pipe, clam, and griddle machine spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Pipe board height | board centerline target 42-48 in from floor, bottom edge no lower than 30 in; route readable by seated checker from 3-5 ft | keeps team role accessible and avoids high-reach fiddling | floorplan photo with seated checker pointing route |
| Pipe elbows | chunky elbows at least 2 in wide with rounded edges, magnetic/clip retention, and hard orientation marks | pieces can detach safely under force without sharp edges | 10 twist/pull attempts with safe detachment and no route false positive |
| Clam hinge stop | clam cover opens/closes through 45-70 degrees with soft stop; no pinch gap in player reach; visible ring mark confirms correct closure | wrong bell should not invite force or pinch fingers | wrong-bell closure test: 10 attempts, no false shell tag |
| Pearl cup | cup accepts only pearl-fit token with at least 1/8 in visible tolerance gap against wrong bells | fit aha is physical and fair, not audio-only | fit-test photo of correct bell and worst wrong bell |
| Griddle gauge | gauge face target 6-8 in wide with three proof zones plus open zone; accepted/open state visible across trailer | finale state is public and operator-visible | proof token seating photo from operator sightline |
| Manual open token | GRIDDLE-OPEN pouch lets operator hand open token after verifying proofs, without reaching into player crowd | C5 recovery target stays 1 minute | timed open-token drill under 60 seconds |

## C4 proof-token and griddle-socket spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Order proof | 3-4 in ticket-tab silhouette with one clipped corner, labeled by category icon only | keeps the lunch-order proof distinct from final answer text | wrong proof rejects in current/bell sockets |
| Current proof | 3-4 in blue gear/wave silhouette with asymmetric tooth or arrow key | makes P3 routing proof tactile and not color-only | seated checker can identify token from 3-5 ft |
| Bell proof | 3-4 in shell/bell silhouette with a raised pearl nub marker | ties clam solve to final machine without relying on sound | correct token seats; order/current proofs visibly reject |
| Griddle socket layout | three sockets arranged left-to-right as order, current, bell; each has matching silhouette label, 1/8-1/4 in clearance, and rounded reject ramp | prevents token dumping and gives operator-visible proof state | 10 wrong-token attempts per socket with no false gauge movement |
| Gauge coupling | each accepted proof advances a visible gauge/check marker; all three seated states are visible before open token appears | makes progress public and avoids hidden sensor trust issues | photo from operator sightline with zero, partial, and all-proof states |
| Staff proof set | duplicate order/current/bell proof set stored in locked `PROOF-STAFF` pouch | lost or jammed proof cannot dead-end the room | timed staff proof handoff under 60 seconds |

## C4 tile, rail, and bell count spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Ingredient tiles | 6 oversized tiles, 3 x 4 in minimum and at least 1/4 in thick, each with an asymmetric edge/key plus high-contrast ingredient icon | tile solve is tactile and not color-only | seated checker identifies all tiles from 3-5 ft |
| Order rail | waist-height rail with 6 tile homes, channel stops, and no pinch gap; only the correct order aligns the release marker | wrong order feels like a reject, not a jam | 10 wrong-order attempts with no staff ticket release |
| Wrong-tile reject | each nonmatching tile visibly overhangs or stops short rather than nearly fitting | discourages force and guessing | worst wrong-tile photo beside correct tile |
| Tile count card | `P2-TILES` pouch has a six-tile count card, spare staff ticket, and tile order reset photo | missing tile is found before next run or recovered during run | one-minute missing-tile replacement drill |
| Bell token count | 4 bell tokens total, one pearl-fit token plus three wrong bells, each 3-4 in with tactile differences | fit puzzle stays physical and resettable | count and fit-test photo for all four bells |
| Pouch labels | `P2-TILES` and `P4-CLAM` pouches use color tabs plus contents silhouettes; labels match the recovery pouch map | staff should not search a mixed tote during live recovery | operator finds correct pouch from map in under 30 seconds |

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|
| Cardboard rail and tiles | P2 | 3 | 6 | 1 | missing tile replaced in 1 min | players cannot tell edge rule |
| Pipe board and elbows | P3 | 4 | 8 | 2 | elbow swap in 2 min | seated checker cannot verify route |
| Clam register and bell set | P4 | 5 | 10 | 2 | bell/cup swap in 2 min | wrong bell fits or shell can be forced |
| Pressure griddle final | P5 | 6 | 12 | 2 | staff open token in 1 min | wrong proof opens or staff cannot bypass |
| Operator spares/reset pack | all | 2 | 3 | 3 | count fail recovered before next run | staff searches mixed tote |

## Criticality map

Use `components\RELIABILITY.md`.

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | pressure griddle, reset/spare organizer | final cannot complete, or staff cannot recover/reset safely | manual open token plus organizer count | timed operator drill |
| C4 required | proof tokens, ingredient tiles, rail, pipe board, clam, bell tokens | blocks required path if missing or jammed | duplicate token/tile/bell sets and station pouches | 1-2 minute replacement drill |
| C3 beat-support | printed cards/labels, hinge/screw/magnet set | slows or confuses path but can be explained | duplicate print set and repair kit | reset check and post-run swap |
| C2 helper | ingredient tray, dry bubble light | helpful or delightful but not required | spare tray/light or skip effect | post-run repair acceptable |
| C1 cosmetic | scenic dressing not listed in BOM | no solve dependency | repair consumables only | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft | P1-P5 have proposed mechanisms and proof convergence | Build cardboard prototype. |
| BOM line for every required component | Draft | inventory-backed planning BOM above | Verify quantities after P0. |
| Inventory ID or custom fabrication plan for every critical component | Draft | common IDs/custom plans named | Price griddle, pipe board, clam prop. |
| Technique/device IDs for every required beat | Draft pass | P1-P5 mapped to `TECH-*` and `DEV-*` IDs | Refine device choices after P0. |
| Kit IDs for every required beat | Draft pass | P1-P5 mapped to prototype/ops/mechanical/electronics kits | Remove excess kits after P0 reset timing. |
| Device review level/risk band assigned | Draft pass | P1-P5 have L0/risk/crowd/frustration rows | Promote each to L1 after cardboard sprint. |
| Known cost band for critical components | Draft | planning bands assigned | Check current prices before prototype budget lock. |
| Manual bypass for powered/sensed/fragile beats | Draft | P2-P5 bypasses named | Test staff use under timing pressure. |
| Durability class not D for required components | TBD | no prototype | Assign after handling mockup. |
| Reset action verifiable under time pressure | TBD | reset plan only | Run 10-minute reset drill. |

## Cardboard prototype plan

| Prototype item | Low-cost build | Test | Pass condition |
|---|---|---|---|
| Order rail | foamcore rail and 6 printed tiles | does team build stack without reading menu? | confused team earns ticket by physical fit |
| Pipe board | foamcore board, arrows, magnetic/card elbows | can seated checker route blue tide? | route legible under work light |
| Clam register | hinged foamcore shell, oversized pearl cup, and four bell tokens | does pearl-fit closure carry solve? | no audio required; wrong bells fail visibly; no force |
| Pressure griddle | foamcore face with three sockets and open token | do wrong tokens fail safely? | no false open, no damage |
| Proof tokens | oversized ticket/gear/shell tag | are categories distinct? | players name each proof |
| Dry bubble gags | printed reward cards/LED | do they delight without clue noise? | optional and ignored when stuck |

## Bench test plan

| Test ID | Device | Pass | What staff observes | Pass condition |
|---|---|---|---|---|
| BT-0004-P2-C | `DEV-RAIL-001` order rail | correct-use | team stacks true ingredients by edge shape | ticket releases and team explains the edge rule |
| BT-0004-P2-W | `DEV-RAIL-001` order rail | wrong-use | plausible wrong tiles/orientations are tried | wrong stack visibly rejects without jam |
| BT-0004-P3-X | `DEV-PIPE-001` current board | accessibility | seated checker calls route while handler turns pieces | checker can verify blue path without high reach |
| BT-0004-P4-W | `DEV-COVER-001` clam | wrong-use | three wrong bells tested, shell pushed lightly | wrong bells fail visibly; shell does not false-close |
| BT-0004-P4-C | `DEV-COVER-001` clam | correct-use | pearl bell seats in cup | clam closes without force and ring/tag is visible |
| BT-0004-P5-H | `DEV-FINAL-003` griddle | chaotic-use | wrong proof tokens and random seating attempts | wrong tokens reject; no false open |
| BT-0004-R | full proof set | reset/transport | tokens packed, moved, reset from photo | reset under 10 minutes with no missing proof |

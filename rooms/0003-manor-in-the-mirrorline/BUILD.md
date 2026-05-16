# The Manor in the Mirrorline - Build

Use `docs/build-economics.md`, `components/TECHNIQUES.md`,
`components/DEVICES.md`, `components/KITS.md`,
`components/DEVICE-REVIEW.md`, `components/INVENTORY.md`, and
`components/SOURCING.md` before build readiness review.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$250-800` | cabinet mockup, portrait frames, prop tray, room panels, work-light reveal. |
| Production | `$800-2,500+` | finished cabinet, modular stations, durable props, mounted lighting, scenic finish. |
| Spares | `$100-300` | printed evidence, object duplicates, selector knobs, LEDs, hinges, labels. |
| Maintenance | `$25-125/run season` | label/card refresh, frame/hinge tightening, prop replacement, light checks. |
| Transport | `$75-250` | cabinet locks, prop bins, frame padding, modular panel protection. |
| Build hours | `45-75` | proof-locked cabinet, portrait frames, plinth, overlays, object/proof sets, reset pack. |

## Puzzle mechanism map

| Beat ID | Technique | Device/module | Kit(s) | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|---|---|---|
| P1 | `TECH-SORT-001` | printed frame plus `DEV-BIN-001` | `KIT-OPS-003`, `KIT-OPS-004` | house ledger, bell, category plaques | passive/audio optional | operator names categories | ledger and bell reset | pages bent or bell overused |
| P2 | `TECH-SORT-003`, `TECH-FIT-002` | `DEV-WINDOW-001`, `DEV-SOCKET-001` | `KIT-PROTO-002`, `KIT-PROTO-004`, `KIT-OPS-002` | portrait frames, alibi tokens, portrait-back truth windows, suspect proof token | analog/mechanical | operator demonstrates one true alibi line and provides staff token | frames closed, tokens reset, crest hidden | frames pulled, token swapped, or token pocketed |
| P3 | `TECH-FIT-003` | `DEV-BALANCE-001`, `DEV-SOCKET-001` | `KIT-PROTO-002`, `KIT-MECH-002`, `KIT-OPS-002` | object sideboard, mirrorline balance plinth, object proof token | passive/mechanical | operator demonstrates one failed test and provides staff token | objects, plinth, and token reset | objects/token dropped or plinth forced |
| P4 | `TECH-ALIGN-001`, `TECH-ALIGN-003` | `DEV-WINDOW-001`, `DEV-OVERLAY-001` | `KIT-PROTO-004`, `KIT-OPS-003`, `KIT-OPS-004` | miniature room panels, work-light inspection windows, transparent overlays, location proof token | analog/lighting | printed false-room alignment card and staff token | panels, overlays, and token reset | panel forced, overlay swapped, or glare issue |
| P5 | `TECH-TEAM-001`, `TECH-TEAM-002`, `TECH-REVEAL-003` | `DEV-TRAY-001`, `DEV-FINAL-002` | `KIT-PROTO-002`, `KIT-MECH-003`, `KIT-ELEC-001`, `KIT-OPS-002` | proof rail, proof-token sockets, selector covers, accusation sliders, mirror reveal | mechanical/lighting optional | staff release/reveal or staff proof token | proof rail, sockets, covers, sliders, reveal reset | proof rail cluttered, cover forced, token jammed, slider slipped, or guessed |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Criticality | Build time | Replacement min | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Admin recovery | Maintenance |
|---|---|---|---:|---:|---|---:|---:|---:|---|---|---|---|---|---|---|---|
| CUSTOM-SCENIC plus CORE-001 | proof-locked mantel mirror accusation cabinet | P5 | 1 | 0 | C5 showstopper | 10 | 1 | `$100-250+` | local fabrication, big-box panels | Planning | 1-3 weeks | Unknown | cover/slider jam, cabinet shift | staff release/reveal | manual reveal token and staff release at operator station | inspect every run day |
| MECH-006 | suspect/object/location selector sliders or knobs | P5 | 3 | 3 | C4 required | 3 | 2 | `$3-25` each | Amazon, McMaster-Carr, electronics suppliers | Planning | 0-1 week | B | slider slip or set screw loosens | operator sets category | spare knobs/sliders in P5 pouch; operator can set category | inspect set screws/stops |
| CUSTOM-MECH or MECH-004 | selector cover proof gates | P5 | 3 | 1 | C5 showstopper | 6 | 1 | `$15-75` | local fabrication, big-box hardware, McMaster-Carr | Planning | 0-2 weeks | Unknown | cover sticks, opens without proof, or can be forced | staff opens cover and notes proof state | staff cover bypass plus post-run spare gate | fit-test with every reset |
| CUSTOM-SCENIC or PRINT-005 | category proof tokens and cabinet sockets: suspect crest, object seal, room-key tile | P2-P5 | 3 | 1 | C4 required | 3 | 1 | `$10-60` | local fabrication, craft stores, Amazon, big-box hardware | Planning | 0-2 weeks | B/C | token lost, wrong socket fit, socket jam | staff proof token set | locked proof pouch at operator station | count and fit-test each reset |
| PRINT-005 or CUSTOM-SCENIC | public proof tray/evidence rail with token silhouettes | P2-P5 | 1 | 1 | C3 beat-support | 2 | 2 | `$5-30` | office supply, craft stores, big-box hardware | Planning | 0-1 week | C/B | tray becomes prop dump, token home unclear | operator calls proof state | temporary silhouette card and operator proof call | count and reset photo |
| SCENIC-004 | acrylic mirror/window | P5 | 2 | 1 | C3 beat-support | 2 | 3 | `$10-60` | big-box hardware, local plastics | Planning | 0-1 week | B/C | scratch, glare, crack | printed reveal card | printed reveal card covers failure mid-run | clean/replace |
| PRINT-001 | optional house echo cards / final mirror echo strip | P2-P5 | 4 | 1 | C2 helper | 1 | 2 | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | echo mistaken for required clue, card wear | operator says echo is memory only | remove echoes during run if confusing | count/replace |
| SCENIC-003 | portrait/case board frames with truth-window backs | P2 | 5 | 1 | C4 required | 5 | 2 | `$5-25` each | office supply, Amazon, thrift | Planning | 0-1 week | B/C | frame damage, sticky hinge, or swapped label | operator exclusion | spare frame and true-line demo | reset photo check |
| PRINT-001 | portrait/evidence card set | P1-P2 | 1 | 1 | C4 required | 2 | 1 | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | bent, marked, missing | duplicate set | duplicate portrait envelope | count/replace |
| PRINT-003 | alibi/evidence tags/tokens with bold mirrorline marks | P2 | 10 | 1 | C4 required | 2 | 1 | `$5-20` | office supply, Amazon, print shop | Planning | 0-1 week | C | lost, bent, or swapped token | duplicate pouch | alibi token pouch at operator station | count after reset |
| PRINT-001 | P0 answer-set labels: five suspects, six objects, five rooms, operator key | P1-P5 | 1 | 1 | C3 beat-support | 1 | 2 | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | label confusion or accidental answer exposure | operator key only | duplicate label set hidden from players | count and hide key |
| PRINT-005 | prop object tray/pouches | P3 | 1 | 1 | C3 beat-support | 1 | 2 | `$5-25` | Amazon, ULINE, craft stores | Planning | 0-1 week | C | disorganized props | operator tray map | spare pouch and tray map | restock/reset |
| CUSTOM-SCENIC or SCENIC-005 | safe prop objects | P3 | 6 | 1 | C4 required | 5 | 2 | `$15-100` | general retail, thrift/scenic, local fabrication | Planning | 0-2 weeks | C | object loss/breakage | duplicate object/mark | duplicate object set and staff object seal | count each reset |
| CUSTOM-MECH or MECH-004 | mirrorline balance plinth | P3 | 1 | 1 | C4 required | 5 | 2 | `$15-75` | local fabrication, big-box hardware, McMaster-Carr | Planning | 0-2 weeks | Unknown | tolerance drift, sticky pointer, false positive | operator demonstration/staff token | staff object seal and post-run plinth swap | level and pointer test each run day |
| SCENIC-005 or MECH-003 | hidden weight/magnet behavior for true object | P3 | 1 | 1 | C4 required | 2 | 2 | `$5-30` | craft stores, Amazon, McMaster-Carr | Planning | 0-1 week | B/C | magnet/weight loosens | duplicate object or staff token | duplicate true object and staff seal | inspect true object |
| ELEC-001 | reveal/work-light LEDs | P3-P4 | 1 | 1 | C3 beat-support | 2 | 2 | `$10-35` | Amazon, Adafruit, big-box lighting | Planning | 0-1 week | B | light failure | printed/high-contrast marks | work-light bypass and printed marks | pre-run light test |
| CORE-001 | miniature room panel backs | P4 | 5 | 1 | C4 required | 5 | 3 | `$15-80` | Home Depot, Lowe's, local lumber | Planning | 0-1 week | B | panel warp/damage | printed alignment card | printed alignment card during run; panel spare post-run | inspect mounting |
| SCENIC-004 | inspection windows and transparent overlays | P4 | 5 | 1 | C4 required | 4 | 2 | `$10-60` | big-box hardware, local plastics, print shop | Planning | 0-1 week | B/C | glare, scratches, swapped overlay | printed false-room alignment card | spare overlay set and printed card | clean/count/reset |
| MECH-005 | panel/frame hinges | P2, P4 | 10 | 1 | C3 beat-support | 2 | 4 | `$5-30` | big-box hardware, McMaster-Carr | Planning | 0-1 week | A/B | hinge sag/pinch | operator opens panel | hinge/screw set for post-run swap | inspect screws |
| OPS-002 | reset/spare organizer | all | 1 | 1 | C5 showstopper | 1 | 1 | `$5-25` | Harbor Freight, Walmart, Amazon | Planning | 0-1 week | B | spares disorganized | checklist | operator uses duplicate pouch map and backup organizer before next beat/reset | restock weekly |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| Cabinet selector covers allow guessing | P5 | covers open without proof or players spin sliders before evidence | pause and require category proof | proof-token sockets physically gate selector covers |
| Proof tokens reveal too much too early | P2-P5 | players infer final answer from token design before solving station | keep tokens category-shaped, not answer-labeled | hide exact names until final selector stage |
| Proof tokens are too similar | P2-P5 | players try tokens in wrong sockets repeatedly | use distinct silhouette/tactile keys | crest/seal/tile shapes and matching socket icons |
| Proof tray becomes clutter | P2-P5 | players dump unrelated props or hide tokens under papers | clear tray and name proof state | shallow rail with only three silhouettes, no flat dumping surface |
| Optional echoes become clue noise | P2-P5 | teams reread echo cards while stuck on required solve | operator labels them as house memories | reveal echoes only after proof token is earned; one sentence max |
| Portrait/gallery too text-heavy | P2 | one player reads while others idle | assign token tester/checker roles | reduce text, make truth-window line the proof |
| Portrait truth-window subtle or swapped | P2 | players cannot tell true line from broken line or tokens are in wrong homes | demonstrate one true alibi line | use bolder marks, keyed token homes, and reset photo |
| Prop loss | P3 | tray count mismatch | use duplicate/spare | tether or oversize objects |
| Balance plinth false positive | P3 | wrong object levels pointer or releases seal | operator resets and uses staff token if needed | widen tolerance gap between true and false objects |
| Balance plinth false negative | P3 | true object fails to level pointer | operator demonstrates intended behavior and uses staff token | simplify mechanism or make pointer purely visual |
| Overlay/glare unfairness | P4 | players cannot align overlays or read window marks | use printed false-room alignment card | larger overlays, matte finish, high-contrast marks |
| Reset complexity exceeds 10 min | all | reset drill over target | simplify stations | reduce independent loose parts |

## Device play profiles

| Beat ID | Device/module | Crowd profile | Advantages in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P2 | `DEV-WINDOW-001` truth windows / `DEV-SOCKET-001` tokens | deduction pleaser with fiddly risk | makes alibis handled and seen instead of privately read | tiny line marks, swapped tokens, or force-fit alibi homes | bold mirrorline marks, keyed token pockets, one demonstrated true line |
| P3 | `DEV-BALANCE-001` object plinth | discovery pleaser | turns "which object" into a physical behavior test | false balance, drift, hidden-weight suspicion, or prop loss | wide tolerance gap, visible pointer, count tray, staff seal fallback |
| P4 | `DEV-OVERLAY-001` / `DEV-WINDOW-001` room inspection | aha pleaser with accessibility risk | miniature rooms feel like a compressed manor and reuse earned proofs | glare, parallax, swapped overlays, or one-player squinting | work-light route, matte/high-contrast marks, corner keys, duplicate printed card |
| P5 | `DEV-FINAL-002` proof-locked cabinet / `DEV-TRAY-001` proof rail | premium finale pleaser | proof seating unlocks accusation roles and prevents unsupported guesses | cover force, token dumping, slider fiddling, or one-player domination | keyed sockets, shallow proof rail, caller/checker/handler roles, staff-open bypass |

## Device review matrix

| Beat ID | Device/module | Review level | Risk band | Crowd profile | Frustration trigger | False positive | False negative | Operator success signal | Kill criteria |
|---|---|---|---|---|---|---|---|---|---|
| P1 | ledger/category frame | L0 idea | Green | onboarding helper | players read ledger like homework | categories imply solved proof | team misses suspect/object/location frame | team names three categories | category confusion persists after Hint 2 |
| P2 | `DEV-WINDOW-001` / `DEV-SOCKET-001` truth windows | L0 idea | Yellow | deduction pleaser | token swaps or subtle broken line | wrong portrait releases crest | true broken line not noticed | suspect crest visible in proof rail | confused team solves by reading only |
| P3 | `DEV-BALANCE-001` object plinth | L0 idea | Yellow | discovery pleaser | players press pointer or distrust weight behavior | wrong object levels/releases seal | hand mirror fails to level/release | object seal visible in proof rail | any false object earns seal twice |
| P4 | `DEV-OVERLAY-001` / `DEV-WINDOW-001` room inspection | L0 idea | Yellow, Orange if lit | aha pleaser | glare, parallax, or private squinting | false room releases tile | observatory refusal unreadable | room-key tile visible in proof rail | accessibility pass cannot read work-light path |
| P5 | `DEV-FINAL-002` / `DEV-TRAY-001` cabinet and proof rail | L0 idea | Yellow, Orange if lit | premium finale pleaser | cover force or one-player slider control | wrong token opens cover/reveal | earned token fails to open cover | three covers/sliders/reveal visible | false reveal or staff cannot bypass |

## Part diagrams

Use `components\VISUALS.md`.

| Beat/device | Diagram | What it proves | Missing before build readiness |
|---|---|---|---|
| P2/P3/P4 proof tokens and sockets | `components\diagrams\dev-socket-proof-token.excalidraw` | token fit, wrong-token reject path, camera-visible state, staff proof pouch | build-ready: room-specific token dimensions and silhouettes declared below; replace with measured prototype photo after fit test |
| P2/P4 truth windows, room panels, and overlays | `components\diagrams\dev-overlay-inspection-window.excalidraw` | glare/orientation/reset risks and printed backup path | build-ready: seated sightline, window sizes, and work-light placement declared below; replace with measured prototype photo after glare test |
| P3 object plinth and true-object proof path | `components\diagrams\dev-socket-proof-token.excalidraw` | proof-token handoff, staff token fallback, and fit-test/reset state | build-ready: balance-plinth tolerance, no-force stops, and proof handoff declared below; replace with measured prototype photo after bench test |
| P5 proof-locked cabinet, selectors, and proof rail | `components\diagrams\dev-final-lite-vs-cabinet.excalidraw` | premium cabinet build pressure, lite fallback, proof-state visibility, final-board failure paths | build-ready: cabinet dimensions, hinges/stops, selector detents, and manual release path declared below; replace with measured prototype photo after bench test |
| all reset/spare organizer | `components\diagrams\ops-recovery-kit.excalidraw` | pouch labels, count state, operator pull path, replacement target | build-ready: C5 recovery pouch map declared below; replace with photo after prototype pack is assembled |

## Kit selection

| Kit ID | Applies to | Why included | Remove/simplify if |
|---|---|---|---|
| `KIT-PROTO-002` | P2, P3, P5 proof tokens/sockets | proves distinct crest/seal/tile sockets and false-fit resistance | tokens become too many or too specific too early |
| `KIT-PROTO-004` | P2 truth windows, P4 overlays | tests glare, line contrast, and public readability before scenic finish | visual path remains too subtle after one revision |
| `KIT-OPS-002` | all proof tokens | duplicate staff proof set prevents dead sessions | proofs become fully attached or non-portable |
| `KIT-OPS-003` | ledger, bell, lights, printed backups | keeps audio/light/theatrical effects non-required | no sensory flavor remains |
| `KIT-OPS-004` | all movable stations | reset photos for frames, objects, overlays, proof rail, cabinet | movable states reduce below three |
| `KIT-OPS-005` | C4-C5 recovery | stages proof, object, overlay, printed, and cabinet bypass pouches | required items become fixed and all manual releases are tested |
| `KIT-MECH-002` | P3 plinth/release | supports visible pointer or seal release without heavy fabrication | plinth becomes purely printed comparison |
| `KIT-MECH-003` | P5 selectors | gives accusation sliders readable detents and reset marks | final answer becomes a single open token |
| `KIT-ELEC-001` | P4/P5 work-light/reveal | optional work-light and mirror reveal support | lighting adds ambiguity or reset debt |
| `KIT-TRANS-001` | loose proof/object/token sets | protects tokens, objects, overlays, and labels between venues | all high-touch props are mounted/tethered |
| `KIT-MAINT-001` | full prototype | covers daily tightening, labels, soft stops, and small repairs | production build gets dedicated maintenance pack |

## Abuse cases

| Component/beat | Pull/twist/drop/spill/force scenario | Expected result | Design protection |
|---|---|---|---|
| accusation selector covers/sliders | player forces cover or spins slider | covers/sliders survive and do not falsely solve | robust stops, sacrificial cover tabs, operator-visible state |
| proof rail/tokens/sockets | player dumps props, jams wrong token, or pockets token | rail shows missing state; socket resists without damage; staff has spare token | three silhouette homes, keyed shapes, rounded edges, spare staff token set |
| portrait frames/truth windows | player yanks frame open or forces token into window | frame stops without breaking; token cannot jam | hinges/stops, rounded edges, oversized token slot |
| prop objects | player drops or pockets object | object survives or duplicate exists | prop tray count and spare set |
| balance plinth | player presses, leans, or forces pointer | plinth stops without pinch or false release | no-force stops, rounded pointer, operator-visible test |
| room panels/overlays | player leans, scratches, swaps, or bends overlay | panel remains mounted; overlay can be replaced | reinforced hinges, rounded acrylic, duplicate overlay set |

## Spare kit

| Spare | Quantity | Criticality covered | Stored where | Admin replace target | Replace when |
|---|---:|---|---|---:|---|
| evidence/portrait card set | 1 full set | C4 | operator kit | 1 | card missing, bent, or marked |
| alibi token set | 1 full set | C4 | P2 pouch/operator kit | 1 | token missing, bent, or unreadable |
| object set | 1 partial/full set | C4 | P3 pouch/reset bin | 2 | object missing or damaged |
| balance-plinth pointer/weight parts | 1 set | C4 post-run | maintenance kit | 5 | pointer sticks or true object no longer balances |
| selector knob | 3 | C4 | P5 pouch | 2 | loosened or cracked |
| selector cover tab/hinge | 1 set | C5 post-run | maintenance kit | 5 | cover sticks, bends, or opens without proof |
| proof token set | 1 full set | C4 | locked operator proof pouch | 1 | token missing, damaged, or loose in socket |
| proof rail label/silhouette set | 1 set | C3 | operator kit | 2 | label peels, rail unclear, or prop dumping starts |
| house echo card set | 1 full set | C2 | operator kit | 2 | card missing, bent, marked, or too worn to read |
| LEDs/light strip | 1 strip/indicator set | C3 | maintenance kit | 2 | pre-run light test fails |
| room overlay set | 1 full set | C4 | P4 pouch/operator kit | 2 | overlay scratched, bent, or missing |
| hinge/screw set | 1 set | C3 post-run | maintenance kit | 5 | frame or panel loosens |
| backup organizer map/pouch labels | 1 set | C5 | operator clipboard | 1 | primary organizer count or labels fail |

## C5 cabinet and selector release spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Mantel cabinet envelope | wall or tabletop cabinet target: 28-34 in wide, 14-18 in deep, 18-24 in high; mounted below standing shoulder height and clear of egress | finale is public and reachable without blocking trailer flow | cardboard volume placed in room path with operator sightline photo |
| Proof-token sockets | three distinct sockets on front edge or proof rail, each keyed by silhouette and separated by at least 3 in | wrong proof rejects visibly and staff can see proof state | fit-test photo: correct token seated, wrong token rejected |
| Selector covers | three lightweight covers over suspect/object/location selectors; each cover has soft stop and sacrificial tab; no cover supports player weight | wrong state feels intentionally locked, not sticky or forceable | 10 wrong-state pulls with no false open |
| Selector detents | selector knob/slider travel has 5-6 large detents, high-contrast labels, and reset mark; set screw accessible from staff side | final choice is deliberate and resettable | two reset cycles with labels matching operator key |
| Manual release | CABINET-RELEASE pouch contains staff release card and manual reveal token; staff can open covers/reveal from operator side after verifying proofs | C5 bypass target stays 1 minute without exposing answer early | timed release drill under 60 seconds |
| Post-run service | CABINET-POST pouch holds cover tab/hinge set, spare selector knobs, and driver; false-open or sticky cover blocks next run | fragile premium hardware has a breakage path | post-run replacement drill under 5 minutes |

## C5 recovery pouch map

| Pouch/slot | Contents | Covers | Stored where | Operator pull path | Live replacement target | Reset proof |
|---|---|---|---|---|---:|---|
| PROOF-STAFF | full staff proof token set, proof rail label/silhouette set | proof tokens, proof rail, proof-state ambiguity | locked operator proof pouch, red tab | hand in matching staff proof, call proof state, log missing/damaged token | 1 | token count and proof rail photo |
| CABINET-RELEASE | manual reveal token, staff release card, selector-cover bypass note | mantel cabinet, selector cover proof gates | operator station release sleeve | pause finale, verify earned proofs, staff-release cover/reveal | 1 | covers closed, sliders reset, release card returned |
| CABINET-POST | selector cover tab/hinge set, selector knobs, driver | selector cover/knob post-run failure | maintenance kit, orange tab | block next run if cover sticks or false-opens; swap after exit | 5 | cover/slider fit-test |
| P2-P4-DUPES | evidence cards, alibi tokens, object set, overlay set | required deduction components | station pouches P2/P3/P4, blue tabs | replace missing station prop or switch to printed alignment/demo path | 1-2 | station count cards and reset photos |
| ORGANIZER-MAP | laminated pouch map and spare labels | reset/spare organizer failure | operator clipboard front page | use map to locate pouch; relabel any missing pouch before next run | 1 | pouch map checked against kit |

## C4 proof-token and balance-plinth spec

| Element | Draft build-ready spec | Safety/reset reason | Prototype proof required |
|---|---|---|---|
| Suspect crest token | 3-4 in wide, at least 1/4 in thick, asymmetric crest outline with one flat keyed edge; no suspect name printed on the token | prevents early answer leakage and wrong-socket guessing | correct crest seats; object seal and room-key tile visibly reject |
| Object seal token | 3-4 in wide circular seal with one raised tab/notch; token color/texture differs from crest and key tile | gives low-vision/tactile separation and fast reset count | blind/tactile sort test distinguishes all three proof tokens |
| Room-key tile | 3-4 in long key/tile silhouette with a rectangular tongue; orientation mark on back only for staff reset | keeps final category distinct without exposing the room answer | wrong orientation rejects without jamming |
| Cabinet/proof sockets | sockets are 1/8-1/4 in larger than correct token edge, with rounded reject ramps and camera-visible seated/not-seated state | avoids force-fit, pinch, and hidden success ambiguity | 10 wrong-token attempts per socket with no false seat |
| Staff proof set | duplicate crest/seal/tile stored in locked `PROOF-STAFF` pouch with matching silhouette labels | any lost/jammed C4 proof is recoverable inside 1 minute | timed staff handoff and proof-call drill under 60 seconds |
| Balance plinth tolerance | true object moves pointer at least 20 degrees or to a marked green zone; nearest false object remains at least 10 degrees away from pass zone | makes the object proof physical but not precision-weight dependent | five-cycle true/false object test on level and slightly uneven floor |
| Balance no-force stops | pointer and tray have soft stops, rounded contact points, and no load-bearing player surface | prevents players from pressing the mechanism into a false solve | press/lean abuse test produces no seal and no pinch |
| Object proof handoff | object seal appears only after operator-visible true-object pass or staff token bypass | keeps P3 recovery consistent with token system | reset photo shows object tray, plinth zero mark, and seal hidden |

## C4 truth-window and overlay sightline spec

| Element | Draft build-ready spec | Accessibility/reset reason | Prototype proof required |
|---|---|---|---|
| Suspect truth windows | portrait backs use 4 x 6 in minimum readable cards with a 2 x 3 in truth-window aperture; hinge opens to a hard stop and cannot swing into aisle | lets two players inspect without yanking frames or blocking egress | seated checker photo from 3-5 ft and hinge-stop pull test |
| Portrait marks | true/broken mirrorline marks use high-contrast strokes at least 1/4 in wide, duplicated by shape not color only | keeps solve visible in low light and color-safe | low-light read test with work light off/on |
| Room panel backs | miniature room panels target 8 x 10 to 12 x 12 in faces at table/waist height; every active clue sits between 30 and 48 in from floor | avoids crawling, high reach, and one-player squinting | tape-out with seated checker and standing handler |
| Overlay set | transparent overlays have keyed corner icons, matte sleeve storage, and at least one bold landmark visible before final alignment | prevents endless flipping and scratch/glare failures | correct orientation found in under 30 seconds after one hint |
| Work-light placement | low-voltage work light sits above or beside room panels, angled 30-45 degrees across the surface, not into player eyes or acrylic glare | creates accessible low-light path without requiring darkness | glare photo from seated checker and operator camera |
| Printed backup | `P2-P4-DUPES` pouch includes false-room alignment card and one duplicate overlay/card set | required clues survive scratched acrylic or lighting failure | staff swaps/announces printed backup in under 2 minutes |

## Build and replacement schedule

| Build package | Applies to | Prototype build time | Production build time | Reset check time | Replacement drill | Blocker if |
|---|---|---:|---:|---:|---|---|
| Proof-locked cabinet | P5 | 10 | 20 | 3 | staff release in 1 min | wrong proof opens cover or staff cannot bypass |
| Portrait truth windows | P2 | 6 | 12 | 2 | missing/swapped token recovered in 1 min | marks are subtle or homes are ambiguous |
| Object sideboard/plinth | P3 | 7 | 14 | 2 | missing object or staff seal in 2 min | false object succeeds or true object fails twice |
| Room panels/overlays | P4 | 7 | 14 | 2 | overlay/printed card recovery in 2 min | seated/low-light checker cannot solve |
| Proof rail and reset pack | P2-P5 | 3 | 6 | 3 | proof token hand-in in 1 min | operator cannot read proof state |

## Criticality map

Use `components\RELIABILITY.md`.

| Criticality | Items | Why critical | Required backups | Admin recovery proof |
|---|---|---|---|---|
| C5 showstopper | mantel cabinet, selector cover gates, reset organizer | final can false-open, jam, or become unrecoverable | manual reveal/release plus post-run cover spares | timed staff release drill |
| C4 required | proof tokens, portrait frames/cards, alibi tokens, objects, plinth, true object behavior, room panels, overlays, selectors | required deduction path depends on them | duplicate token/card/object/overlay sets and staff proof tokens | 1-3 minute admin drills |
| C3 beat-support | proof rail, mirror/window, labels, tray, work lights, hinges | supports clarity, state, or access but can be bypassed | printed cards, repair kit, reset photos | operator stress pass |
| C2 helper | house echoes and ambience | delight only | optional reprint | post-run repair |
| C1 cosmetic | manor dressing not tied to proof | no solve dependency | touch-up kit | visual inspection |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft pass | P1-P5 have proposed mechanisms plus proof-token convergence and proof-locked selector covers | Build cardboard sprint before scenic detail. |
| BOM line for every required component | Draft | inventory-backed BOM above | Verify quantities once suspect/object/location count is set. |
| Inventory ID or custom fabrication plan for every critical component | Draft | custom cabinet named; common IDs assigned | Price and sketch cabinet fabrication path. |
| Technique/device/kit IDs for every required beat | Draft pass | P1-P5 mapped to `TECH-*`, `DEV-*`, and `KIT-*` IDs | Refine after P0 sprint. |
| Criticality, spare count, build time, and replacement time assigned | Draft | C3-C5 items have replacement targets and staged spares | Time admin drills. |
| C4-C5 admin replacement drill passes in time | TBD | drill plan only | Run token/object/overlay/cabinet replacement drills. |
| Device review level/risk band assigned | Draft pass | P1-P5 have L0/risk/crowd/frustration rows | Promote each to L1 after cardboard sprint. |
| Known cost band for critical components | Draft | planning bands assigned | Check current prices before prototype budget lock. |
| Manual bypass for powered/sensed/fragile beats | Draft | P3-P5 bypasses named | Test staff use under timing pressure. |
| Durability class not D for required components | TBD | no prototype | Assign durability after handling mockup. |
| Reset action verifiable under time pressure | TBD | reset plan only | Run reset drill against 10-minute target. |

## Prototype sprint 0003-P0

Goal: prove the room's physical deduction loop before scenic production. This
sprint should use cardboard, foamcore, printed cards, tape, binder clips, and
cheap LEDs/work lights. Do not solve finish quality yet.

| Sprint item | Build first | Minimum materials | Test team | Pass/fail gate |
|---|---|---|---|---|
| Tape-out route | station footprints, proof rail, cabinet role lane, visible exit | painter's tape, stools/tables, paper station signs | operator stress team | 4 players can split, converge, and exit without crossing egress or blocking the cabinet camera |
| Portrait truth window | two true alibi windows plus one broken one | 3 frames/foamcore panels, printed alibi tokens, bold line marks | confused team | team tests tokens before hint time and can name why one portrait is false |
| Balance plinth | one true object and two false objects | foamcore/wood seesaw, safe props, visible pointer | confused/chaotic team | true object visibly succeeds; false objects visibly fail; no force gives false release |
| Room inspection window | two false rooms and one true room | foamcore panels, acetate overlays, work light | accessibility-varied team | seated/low-light-sensitive player can identify one false alignment and the true refusal |
| Proof rail and token homes | three token silhouettes and no dumping surface | shallow rail, printed silhouettes, oversized tokens | operator stress team | operator can read proof state on camera; players do not place unrelated props there |
| Proof-locked cabinet face | three sockets, covers, sliders, and mirror card | foamcore cabinet face, paper covers, printed sliders, staff bypass marks | fighting/chaotic team | wrong token fails safely; right token opens only its cover; caller/checker/handler roles are usable |
| House echoes | three station-back cards and final mirror strip | printed one-sentence cards | amazing/confused teams | amazing team enjoys them; confused team can ignore them while stuck |

### P0 print packet

Print this as plain prototype material before any scenic art.

| Packet item | Content | Quantity | Hidden from players? |
|---|---|---:|---|
| Suspect labels | Lady Maribel Vane; Orris Pike; Cora Fen; Ivo Quill; Nella Ash | 5 | no |
| Object labels | candlestick; letter opener; snuff box; iron key; hand mirror; bell-pull weight | 6 | no |
| Room labels | library; greenhouse; observatory; kitchen; parlor | 5 | no |
| Operator answer key | Nella Ash / hand mirror / observatory | 1 | yes |
| Reset map | Nella crest home, hand mirror true-object setup, observatory room-key home | 1 | yes |
| Category icons | portrait, seal, doorway | 1 set | no |
| Echo cards | three station echoes plus final mirror echo | 1 set | no, but reveal post-proof only |

### Sprint stop rules

Stop adding theme detail if any of these fail:

1. A required proof token can be earned by reading only.
2. A wrong token opens a cabinet cover or creates a false solve.
3. A seated/low-light-sensitive player cannot participate in P2, P4, or P5.
4. Reset of the cardboard room exceeds 10 minutes after two practice resets.
5. The proof rail blocks egress or attracts unrelated props after one operator prompt.

## Cardboard prototype plan

| Prototype item | Low-cost build | Test | Pass condition |
|---|---|---|---|
| Portrait truth window | hinged frame back with a bold printed line and removable alibi token | can players see five true alibis and one broken mirrorline? | confused team identifies suspect by token test, not paragraph reading |
| Suspect crest | foamcore/card portrait crest with diagonal keyed edge | can players connect it to the contradicted portrait? | token is earned from the broken truth-window, not reading a paragraph |
| Object seal | weighted cardboard/wood disk or medallion with tactile rim | does balance behavior reveal the seal fairly? | true object levels/releases seal; wrong objects visibly fail |
| Mirrorline balance plinth | foamcore/wood seesaw or pointer plate with object rest | can players infer "test objects here" without a hint? | at least one confused-team pass uses plinth before hint time |
| Room inspection windows | foamcore room panels with acetate windows and two transparent overlays | can players compare rooms under work light without subtle shadows? | false rooms align clearly; true room visibly refuses alignment |
| Room-key tile | rectangular tile with room silhouette and high-contrast line | does the solved station release/identify the tile fairly? | low-light-sensitive player can solve and claim tile |
| Cabinet sockets and selector covers | three labeled/keyed foamcore slots facing camera with paper/foamcore covers over selectors | do wrong tokens fail safely and do right tokens visibly open access? | no false fit, no force, no ambiguity; cover opens only after proof |
| Proof tray/evidence rail | shallow rail with three silhouettes and no open dumping surface | does it make progress public without attracting unrelated props? | team places only crest/seal/tile there; operator can read state on camera |
| Accusation sliders | three low-friction sliders hidden behind covers | can caller/checker/handler split final action? | one player calls, one checks proof, one sets slider without confusion |
| House echo cards | three station-back cards plus one final mirror echo strip | do fast teams enjoy the room more without treating echoes as clues? | amazing team reads them as reward; confused team can ignore them |
| Staff proof tokens | duplicate plain tokens marked operator-only | can operator accelerate without breaking fiction? | staff can seat a temporary proof cleanly |

## Bench test plan

| Test ID | Device | Pass | What staff observes | Pass condition |
|---|---|---|---|---|
| BT-0003-P2-C | `DEV-WINDOW-001` truth windows | correct-use | team tests alibi tokens in their own portrait backs | Nella's broken mirrorline is identified by physical contradiction |
| BT-0003-P2-W | `DEV-SOCKET-001` alibi/token homes | wrong-use | tokens are swapped, rotated, and tried in wrong homes | wrong placements reject or remain visibly wrong without jam |
| BT-0003-P3-C | `DEV-BALANCE-001` object plinth | correct-use | hand mirror is placed on plinth | pointer/seal success is obvious and repeatable |
| BT-0003-P3-H | `DEV-BALANCE-001` object plinth | chaotic-use | wrong objects are pressed, dropped, or retried | no false seal release and prop count remains recoverable |
| BT-0003-P4-X | `DEV-OVERLAY-001` inspection windows | accessibility | seated/low-light-sensitive checker compares overlays under work light | false alignments and true refusal are readable without spooky darkness |
| BT-0003-P5-H | `DEV-FINAL-002` cabinet | chaotic-use | wrong tokens, forced covers, and slider spinning are attempted | no cover opens without proof and no false reveal occurs |
| BT-0003-R | full proof/reset kit | reset/transport | frames, props, overlays, proof rail, and cabinet are reset from photos | one operator resets under 10 minutes with all proofs accounted for |



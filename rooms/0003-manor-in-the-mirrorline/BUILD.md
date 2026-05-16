# The Manor in the Mirrorline - Build

Use `docs/build-economics.md`, `components/INVENTORY.md`, and
`components/SOURCING.md` before build readiness review.

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$250-800` | cabinet mockup, portrait frames, prop tray, room panels, work-light reveal. |
| Production | `$800-2,500+` | finished cabinet, modular stations, durable props, mounted lighting, scenic finish. |
| Spares | `$100-300` | printed evidence, object duplicates, selector knobs, LEDs, hinges, labels. |
| Maintenance | `$25-125/run season` | label/card refresh, frame/hinge tightening, prop replacement, light checks. |
| Transport | `$75-250` | cabinet locks, prop bins, frame padding, modular panel protection. |

## Puzzle mechanism map

| Beat ID | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|
| P1 | house ledger, bell, category plaques | passive/audio optional | operator names categories | ledger and bell reset | pages bent or bell overused |
| P2 | portrait frames, alibi tokens, suspect proof token | analog/mechanical | operator gives one exclusion and staff token | frames closed, tokens reset | frames pulled or token pocketed |
| P3 | object sideboard, mirrorline balance plinth, object proof token | passive/mechanical | operator demonstrates one failed test and provides staff token | objects, plinth, and token reset | objects/token dropped or plinth forced |
| P4 | miniature room panels, work-light inspection windows, transparent overlays, location proof token | analog/lighting | printed false-room alignment card and staff token | panels, overlays, and token reset | panel forced, overlay swapped, or glare issue |
| P5 | proof-token sockets, accusation selectors, mirror reveal | mechanical/lighting optional | staff release/reveal | sockets, selectors, reveal reset | selector forced, token jammed, or guessed |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Maintenance |
|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| CUSTOM-SCENIC plus CORE-001 | mirrorline accusation cabinet | P5 | 1 | 0 | `$100-250+` | local fabrication, big-box panels | Planning | 1-3 weeks | Unknown | selector jam, cabinet shift | staff release/reveal | inspect every run day |
| MECH-006 | suspect/object/location selector knobs | P5 | 3 | 1-3 | `$3-25` each | Amazon, McMaster-Carr, electronics suppliers | Planning | 0-1 week | B | knob slip or set screw loosens | operator sets category | inspect set screws |
| CUSTOM-SCENIC or PRINT-005 | category proof tokens and cabinet sockets: suspect crest, object seal, room-key tile | P2-P5 | 3 tokens plus 3 sockets | 1 token set | `$10-60` | local fabrication, craft stores, Amazon, big-box hardware | Planning | 0-2 weeks | B/C | token lost, wrong socket fit, socket jam | staff proof token set | count and fit-test each reset |
| SCENIC-004 | acrylic mirror/window | P5 | 1-3 | 1 | `$10-60` | big-box hardware, local plastics | Planning | 0-1 week | B/C | scratch, glare, crack | printed reveal card | clean/replace |
| SCENIC-003 | portrait/case board frames | P2 | 4-6 | 1 | `$5-25` each | office supply, Amazon, thrift | Planning | 0-1 week | B/C | frame damage or swapped label | operator exclusion | reset photo check |
| PRINT-001 | portrait/evidence card set | P1-P2 | 1 set | 1 set | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | bent, marked, missing | duplicate set | count/replace |
| PRINT-003 | alibi/evidence tags | P2 | 8-12 | 1 set | `$5-20` | office supply, Amazon, print shop | Planning | 0-1 week | C | lost or swapped tag | duplicate pouch | count after reset |
| PRINT-005 | prop object tray/pouches | P3 | 1-2 | 0-1 | `$5-25` | Amazon, ULINE, craft stores | Planning | 0-1 week | C | disorganized props | operator tray map | restock/reset |
| CUSTOM-SCENIC or SCENIC-005 | safe prop objects | P3 | 5-7 | 1 set | `$15-100` | general retail, thrift/scenic, local fabrication | Planning | 0-2 weeks | C | object loss/breakage | duplicate object/mark | count each reset |
| CUSTOM-MECH or MECH-004 | mirrorline balance plinth | P3 | 1 | 0-1 | `$15-75` | local fabrication, big-box hardware, McMaster-Carr | Planning | 0-2 weeks | Unknown | tolerance drift, sticky pointer, false positive | operator demonstration/staff token | level and pointer test each run day |
| SCENIC-005 or MECH-003 | hidden weight/magnet behavior for true object | P3 | 1 set | 1 set | `$5-30` | craft stores, Amazon, McMaster-Carr | Planning | 0-1 week | B/C | magnet/weight loosens | duplicate object or staff token | inspect true object |
| ELEC-001 | reveal/work-light LEDs | P3-P4 | 1-2 | strip/LED spares | `$10-35` | Amazon, Adafruit, big-box lighting | Planning | 0-1 week | B | light failure | printed/high-contrast marks | pre-run light test |
| CORE-001 | miniature room panel backs | P4 | 4-5 | 1 panel | `$15-80` | Home Depot, Lowe's, local lumber | Planning | 0-1 week | B | panel warp/damage | printed alignment card | inspect mounting |
| SCENIC-004 | inspection windows and transparent overlays | P4 | 4-5 windows plus 2 overlays | 1 overlay set | `$10-60` | big-box hardware, local plastics, print shop | Planning | 0-1 week | B/C | glare, scratches, swapped overlay | printed false-room alignment card | clean/count/reset |
| MECH-005 | panel/frame hinges | P2, P4 | 6-12 | 1 set | `$5-30` | big-box hardware, McMaster-Carr | Planning | 0-1 week | A/B | hinge sag/pinch | operator opens panel | inspect screws |
| OPS-002 | reset/spare organizer | all | 1 | 0 | `$5-25` | Harbor Freight, Walmart, Amazon | Planning | 0-1 week | B | spares disorganized | checklist | restock weekly |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| Cabinet selectors allow guessing | P5 | repeated random changes or missing proof tokens | pause and require category proof | proof-token sockets gate or validate selectors |
| Proof tokens reveal too much too early | P2-P5 | players infer final answer from token design before solving station | keep tokens category-shaped, not answer-labeled | hide exact names until final selector stage |
| Proof tokens are too similar | P2-P5 | players try tokens in wrong sockets repeatedly | use distinct silhouette/tactile keys | crest/seal/tile shapes and matching socket icons |
| Portrait/gallery too text-heavy | P2 | one player reads while others idle | assign physical comparison roles | reduce text, increase tokens |
| Prop loss | P3 | tray count mismatch | use duplicate/spare | tether or oversize objects |
| Balance plinth false positive | P3 | wrong object levels pointer or releases seal | operator resets and uses staff token if needed | widen tolerance gap between true and false objects |
| Balance plinth false negative | P3 | true object fails to level pointer | operator demonstrates intended behavior and uses staff token | simplify mechanism or make pointer purely visual |
| Overlay/glare unfairness | P4 | players cannot align overlays or read window marks | use printed false-room alignment card | larger overlays, matte finish, high-contrast marks |
| Reset complexity exceeds 10 min | all | reset drill over target | simplify stations | reduce independent loose parts |

## Abuse cases

| Component/beat | Pull/twist/drop/spill/force scenario | Expected result | Design protection |
|---|---|---|---|
| accusation selectors | player spins/forces knobs | selectors survive and do not falsely solve | robust knobs, stops, operator-visible state |
| proof tokens/sockets | player jams wrong token or pockets token | socket resists without damage; staff has spare token | keyed shapes, rounded edges, spare staff token set |
| portrait frames | player yanks frame open | frame stops without breaking | hinges/stops, rounded edges |
| prop objects | player drops or pockets object | object survives or duplicate exists | prop tray count and spare set |
| balance plinth | player presses, leans, or forces pointer | plinth stops without pinch or false release | no-force stops, rounded pointer, operator-visible test |
| room panels/overlays | player leans, scratches, swaps, or bends overlay | panel remains mounted; overlay can be replaced | reinforced hinges, rounded acrylic, duplicate overlay set |

## Spare kit

| Spare | Quantity | Stored where | Replace when |
|---|---:|---|---|
| evidence/portrait card set | 1 full set | operator kit | card missing, bent, or marked |
| object set | 1 partial/full set | reset bin | object missing or damaged |
| balance-plinth pointer/weight parts | 1 set | maintenance kit | pointer sticks or true object no longer balances |
| selector knob | 1-3 | maintenance kit | loosened or cracked |
| proof token set | 1 full set | operator kit | token missing, damaged, or loose in socket |
| LEDs/light strip | 1 strip/indicator set | maintenance kit | pre-run light test fails |
| room overlay set | 1 full set | operator kit | overlay scratched, bent, or missing |
| hinge/screw set | 1 set | maintenance kit | frame or panel loosens |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft | P1-P5 have proposed mechanisms plus proof-token convergence | Prototype final cabinet proof sockets and P4 light panel. |
| BOM line for every required component | Draft | inventory-backed BOM above | Verify quantities once suspect/object/location count is set. |
| Inventory ID or custom fabrication plan for every critical component | Draft | custom cabinet named; common IDs assigned | Price and sketch cabinet fabrication path. |
| Known cost band for critical components | Draft | planning bands assigned | Check current prices before prototype budget lock. |
| Manual bypass for powered/sensed/fragile beats | Draft | P3-P5 bypasses named | Test staff use under timing pressure. |
| Durability class not D for required components | TBD | no prototype | Assign durability after handling mockup. |
| Reset action verifiable under time pressure | TBD | reset plan only | Run reset drill against 10-minute target. |

## Cardboard prototype plan

| Prototype item | Low-cost build | Test | Pass condition |
|---|---|---|---|
| Suspect crest | foamcore/card portrait crest with diagonal keyed edge | can players connect it to the contradicted portrait? | token is earned without reading a paragraph |
| Object seal | weighted cardboard/wood disk or medallion with tactile rim | does balance behavior reveal the seal fairly? | true object levels/releases seal; wrong objects visibly fail |
| Mirrorline balance plinth | foamcore/wood seesaw or pointer plate with object rest | can players infer "test objects here" without a hint? | at least one confused-team pass uses plinth before hint time |
| Room inspection windows | foamcore room panels with acetate windows and two transparent overlays | can players compare rooms under work light without subtle shadows? | false rooms align clearly; true room visibly refuses alignment |
| Room-key tile | rectangular tile with room silhouette and high-contrast line | does the solved station release/identify the tile fairly? | low-light-sensitive player can solve and claim tile |
| Cabinet sockets | three labeled/keyed foamcore slots facing camera | do wrong tokens fail safely and clearly? | no false fit, no force, no ambiguity |
| Staff proof tokens | duplicate plain tokens marked operator-only | can operator accelerate without breaking fiction? | staff can seat a temporary proof cleanly |



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
| P2 | portrait frames, alibi tokens, portrait-back truth windows, suspect proof token | analog/mechanical | operator demonstrates one true alibi line and provides staff token | frames closed, tokens reset, crest hidden | frames pulled, token swapped, or token pocketed |
| P3 | object sideboard, mirrorline balance plinth, object proof token | passive/mechanical | operator demonstrates one failed test and provides staff token | objects, plinth, and token reset | objects/token dropped or plinth forced |
| P4 | miniature room panels, work-light inspection windows, transparent overlays, location proof token | analog/lighting | printed false-room alignment card and staff token | panels, overlays, and token reset | panel forced, overlay swapped, or glare issue |
| P5 | proof rail, proof-token sockets, selector covers, accusation sliders, mirror reveal | mechanical/lighting optional | staff release/reveal or staff proof token | proof rail, sockets, covers, sliders, reveal reset | proof rail cluttered, cover forced, token jammed, slider slipped, or guessed |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Maintenance |
|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| CUSTOM-SCENIC plus CORE-001 | proof-locked mantel mirror accusation cabinet | P5 | 1 | 0 | `$100-250+` | local fabrication, big-box panels | Planning | 1-3 weeks | Unknown | cover/slider jam, cabinet shift | staff release/reveal | inspect every run day |
| MECH-006 | suspect/object/location selector sliders or knobs | P5 | 3 | 1-3 | `$3-25` each | Amazon, McMaster-Carr, electronics suppliers | Planning | 0-1 week | B | slider slip or set screw loosens | operator sets category | inspect set screws/stops |
| CUSTOM-MECH or MECH-004 | selector cover proof gates | P5 | 3 | 1 set | `$15-75` | local fabrication, big-box hardware, McMaster-Carr | Planning | 0-2 weeks | Unknown | cover sticks, opens without proof, or can be forced | staff opens cover and notes proof state | fit-test with every reset |
| CUSTOM-SCENIC or PRINT-005 | category proof tokens and cabinet sockets: suspect crest, object seal, room-key tile | P2-P5 | 3 tokens plus 3 sockets | 1 token set | `$10-60` | local fabrication, craft stores, Amazon, big-box hardware | Planning | 0-2 weeks | B/C | token lost, wrong socket fit, socket jam | staff proof token set | count and fit-test each reset |
| PRINT-005 or CUSTOM-SCENIC | public proof tray/evidence rail with token silhouettes | P2-P5 | 1 | 0-1 | `$5-30` | office supply, craft stores, big-box hardware | Planning | 0-1 week | C/B | tray becomes prop dump, token home unclear | operator calls proof state | count and reset photo |
| SCENIC-004 | acrylic mirror/window | P5 | 1-3 | 1 | `$10-60` | big-box hardware, local plastics | Planning | 0-1 week | B/C | scratch, glare, crack | printed reveal card | clean/replace |
| PRINT-001 | optional house echo cards / final mirror echo strip | P2-P5 | 4 | 1 set | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | echo mistaken for required clue, card wear | operator says echo is memory only | count/replace |
| SCENIC-003 | portrait/case board frames with truth-window backs | P2 | 4-6 | 1 | `$5-25` each | office supply, Amazon, thrift | Planning | 0-1 week | B/C | frame damage, sticky hinge, or swapped label | operator exclusion | reset photo check |
| PRINT-001 | portrait/evidence card set | P1-P2 | 1 set | 1 set | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | bent, marked, missing | duplicate set | count/replace |
| PRINT-003 | alibi/evidence tags/tokens with bold mirrorline marks | P2 | 8-12 | 1 set | `$5-20` | office supply, Amazon, print shop | Planning | 0-1 week | C | lost, bent, or swapped token | duplicate pouch | count after reset |
| PRINT-001 | P0 answer-set labels: five suspects, six objects, five rooms, operator key | P1-P5 | 1 set | 1 set | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | label confusion or accidental answer exposure | operator key only | count and hide key |
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

| Spare | Quantity | Stored where | Replace when |
|---|---:|---|---|
| evidence/portrait card set | 1 full set | operator kit | card missing, bent, or marked |
| alibi token set | 1 full set | operator kit | token missing, bent, or unreadable |
| object set | 1 partial/full set | reset bin | object missing or damaged |
| balance-plinth pointer/weight parts | 1 set | maintenance kit | pointer sticks or true object no longer balances |
| selector knob | 1-3 | maintenance kit | loosened or cracked |
| selector cover tab/hinge | 1 set | maintenance kit | cover sticks, bends, or opens without proof |
| proof token set | 1 full set | operator kit | token missing, damaged, or loose in socket |
| proof rail label/silhouette set | 1 set | operator kit | label peels, rail unclear, or prop dumping starts |
| house echo card set | 1 full set | operator kit | card missing, bent, marked, or too worn to read |
| LEDs/light strip | 1 strip/indicator set | maintenance kit | pre-run light test fails |
| room overlay set | 1 full set | operator kit | overlay scratched, bent, or missing |
| hinge/screw set | 1 set | maintenance kit | frame or panel loosens |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft pass | P1-P5 have proposed mechanisms plus proof-token convergence and proof-locked selector covers | Build cardboard sprint before scenic detail. |
| BOM line for every required component | Draft | inventory-backed BOM above | Verify quantities once suspect/object/location count is set. |
| Inventory ID or custom fabrication plan for every critical component | Draft | custom cabinet named; common IDs assigned | Price and sketch cabinet fabrication path. |
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



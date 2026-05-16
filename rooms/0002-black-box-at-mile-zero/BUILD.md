# The Black Box at Mile Zero - Build

## Budget summary

| Budget | Estimate | Notes |
|---|---:|---|
| Prototype | `$250-750` | Prioritize central box mockup, rail, transcript/audio loop, and low-voltage cabinet. |
| Production | `$750-2,500+` | Depends on final object size, latch quality, scenic finish, lighting, and enclosure fabrication. |
| Spares | `$75-250` | Evidence tags, transcript cards, latch parts, switches, bulbs/LEDs, printed backups. |
| Maintenance | `$25-100/run season` | Box latch inspection, cards/labels, batteries, small hardware replacement. |
| Transport | `$50-250` | Central object transport lock, foam, bins, tie-downs, and service kit. |

## Puzzle mechanism map

| Beat ID | Physical mechanism | Device class | Manual fallback | Reset state | Abuse case |
|---|---|---|---|---|---|
| P1 | evidence rail and removable tags | passive prop | operator names first match | tags rehung in seed order | tags pocketed/dropped |
| P2 | sliding witness windows on box | mechanical prop | operator sets first window | sliders reset closed | slider forced or jammed |
| P3 | audio recorder plus transcript cards | powered/passive hybrid | printed transcript only | audio rewound, cards replaced | audio ignored or device fails |
| P4 | low-voltage utility cabinet | powered prop | manual indicator card | switches reset off | rapid toggling or switch damage |
| P5 | central latch stack and one-handle reveal | mechanical/powered optional | operator manual release | latches relocked, reveal replaced | pulling/twisting latch |

## Bill of materials

| Inventory ID | Component | Beat IDs | Installed qty | Spare qty | Unit cost band | Source tier | Price confidence | Lead time | Durability class | Failure mode | Bypass | Maintenance |
|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| SCENIC-001 or SCENIC-002 | central black box enclosure | P2, P5 | 1 | 0-1 | `$25-100` prototype shell; `$100-250+` custom | Harbor Freight/Amazon/ULINE or local fabrication | Planning | 0-3 weeks | B/Unknown | latch/hinge damage, enclosure shift | manual release | inspect every run day |
| MECH-004 | sliding witness window track | P2 | 3-5 | 1 | `$12-45` | big-box hardware or McMaster-Carr | Planning | 0-1 week | B | jammed slider | operator sets state | clean/inspect tracks |
| SCENIC-004 | acrylic witness windows | P2 | 3-5 | 1 | `$10-60` | big-box hardware or local plastics | Planning | 0-1 week | B/C | scratched/cracked panel | operator announces state | replace rounded panels |
| CORE-002 | evidence rail/bracket stock | P1 | 1 | offcuts | `$8-35` | big-box hardware or McMaster-Carr | Planning | 0-1 week | A | tags detach or rail bends | spare tags | inspect mounting |
| PRINT-003 | evidence tags/cards | P1, P2 | 12-20 | 1 set | `$5-20` | office supply, Amazon, print shop | Planning | 0-1 week | C | lost, bent, marked | duplicate pouch | count after each run |
| ELEC-006 | rugged/simple audio player | P3 | 1 | 1 optional | `$15-50` | Amazon, Adafruit, thrift retail | Planning | 0-1 week | C | battery/audio failure | transcript | charge/test daily |
| PRINT-001 | transcript cards | P3 | 1 set | 1 set | `$5-30` | Staples, FedEx Office, local print shop | Planning | 0-1 week | C | missing card | spare set | count/reset |
| ELEC-002 | low-voltage switches | P4 | 4-8 | 2 | `$2-12` each | Digi-Key, Mouser, Adafruit, Amazon | Planning | 0-1 week | B | switch fault | manual cabinet card | pre-run switch test |
| ELEC-001 | cabinet LEDs/state lights | P4 | 4-8 | strip/LED spares | `$10-35` | Amazon, Adafruit, big-box lighting | Planning | 0-1 week | B | light fault | manual cabinet card | pre-run light test |
| ELEC-003 | low-voltage power supply | P4 | 1 | 1 | `$10-30` | Amazon, Digi-Key, Adafruit | Planning | 0-1 week | B | power failure | passive/manual card path | inspect/test daily |
| ELEC-004 | fused low-voltage distribution | P4 | 1 | fuse set | `$8-25` | automotive aisle, Amazon, electronics suppliers | Planning | 0-1 week | B | blown fuse, loose wire | passive/manual card path | inspect fuses |
| MECH-001 | final latch stack | P5 | 3-5 | 2 | `$5-25` each | big-box hardware, McMaster-Carr | Planning | 0-1 week | B | jam, pinch, misalignment | staff release | inspect and lubricate |
| MECH-005 | hinges for final reveal | P5 | 2-4 | 1 set | `$5-30` | big-box hardware, McMaster-Carr | Planning | 0-1 week | A/B | hinge sag or pinch | staff opens panel | inspect screws/clearance |
| OPS-002 | reset/spare organizer | all | 1 | 0 | `$5-25` | Harbor Freight, Walmart, Amazon | Planning | 0-1 week | B | spares disorganized | operator checklist | restock weekly |

## Reliability risks

| Risk | Affected beats | Detection | Operator response | Design revision |
|---|---|---|---|---|
| Central box jam | P2, P5 | player force or no state change | stop play, bypass, inspect | use robust hardware and visible feedback |
| Evidence loss | P1-P3 | reset count mismatch | use duplicate tag/card | tether or larger components |
| Audio unfairness | P3 | repeated replay with no progress | direct team to transcript | make transcript equivalent |
| Cabinet state drift | P4 | lights do not match switch state | manual indicator card | simpler passive alternative |

## Abuse cases

| Component/beat | Pull/twist/drop/spill/force scenario | Expected result | Design protection |
|---|---|---|---|
| central box | player pulls handle before solved | handle does not move dangerously | mechanical stop and no-force label in fiction |
| windows | player shoves slider | slider stops without pinch | rounded edges, serviceable tracks |
| evidence tags | player drops or pockets tag | operator can identify missing tag | countable tags and spare pouch |
| cabinet | player rapid-toggles switches | no unsafe electrical state | low-voltage only and fused circuit |

## Spare kit

| Spare | Quantity | Stored where | Replace when |
|---|---:|---|---|
| evidence tag set | 1 full set | operator kit | missing, bent, or marked |
| transcript card set | 1 full set | operator kit | missing or unreadable |
| latch parts | 2 latches plus matching screws | maintenance kit | any binding or visible wear |
| LEDs/switches | 2 switches plus LED strip/indicator spares | maintenance kit | pre-run test fails |
| low-voltage power supply | 1 | operator/maintenance kit | power fault or intermittent state |
| window panel/track parts | 1 panel/track section | maintenance kit | scratching, cracking, or sticking |

## Build readiness gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Physical mechanism for every required beat | Draft | P1-P5 have proposed mechanisms | Prototype central box before graph promotion. |
| BOM line for every required component | Draft | inventory-backed BOM above | Verify quantities during prototype. |
| Inventory ID or custom fabrication plan for every critical component | Draft | common inventory IDs assigned; central box may become custom | Decide whether black box is stock case, scenic build, or fabricated module. |
| Known cost band for critical components | Draft | planning bands assigned | Check current prices before prototype budget lock. |
| Manual bypass for powered/sensed/fragile beats | Draft | P3-P5 bypasses named | Test staff use under timing pressure. |
| Durability class not D for required components | TBD | no prototype | Assign durability class after mockup. |
| Reset action verifiable under time pressure | TBD | reset plan only | Run reset drill against 10-minute target. |


# Stormglass Station Puzzle Graph

## Phase mix

| Role | Phase | Base skills | Why it belongs in this room |
|---|---|---|---|
| Signature | instrument convergence | compare, align, physically prove | weather station fantasy lives in instruments agreeing |
| Support | sorting and mapping | categorize, sequence | wind and tide data can split across players |
| Support | final communication | verify, call, set | beacon payoff turns solved data into rescue action |

## Design pulse evidence

| Design pulse | Phase | Output | Evidence | Decision |
|---|---|---|---|---|
| v0.seed | instrument convergence | five-beat graph | design draft | prototype the console first |

## Hopper

| ID | State | Phase | Name | Technique/device | Skill tags | Aha | Physical action | Clue sources | Failure mode | Promotion test |
|---|---|---|---|---|---|---|---|---|---|---|
| H-006-01 | promoted | setup | Storm log roles | TECH-SORT-001 / printed log | role split, category frame | the station needs pressure, wind, tide, signal | assign role cards and open log | station log, colored instrument labels | players search everything before roles | confused team names three instrument jobs |
| H-006-02 | promoted | instrument | Falling pressure dial | TECH-ALIGN-001 / dial | compare, align | pressure is read by matching needle to storm warnings | rotate barometer dial and claim pressure cartridge | warning cards, dial face | dial becomes random spin | team explains low pressure clue |
| H-006-03 | promoted | instrument | Wind vane words | TECH-SORT-003 / card rack | sort, orientation | wind direction chooses a word path, not a map route | set vane and order cards | vane rose, wind cards | card rack order ambiguity | team sorts without operator naming answer |
| H-006-04 | promoted | instrument | Tide gauge float | TECH-FIT-003 / slider | physical test, exclusion | only the right tide height aligns harbor marks | move float and seat tide cartridge | gauge track, harbor overlay | slider tolerance drift | gauge clicks at one readable state |
| H-006-05 | promoted | finale | Stormglass route | TECH-TEAM-001 / socket console | team ritual, final verify | three proof cartridges unlock the safe route | seat cartridges, open shutter, set beacon | cartridges, console, signal panel | one player guesses final buttons | no final action until proofs are seated |

## Puzzle review

| Candidate | Verdict | Why it might be fun | Why it might fail | Alternative implementation | Backup/hint | User/design input needed | Next revision |
|---|---|---|---|---|---|---|---|
| Stormglass console | keep | tactile final convergence | sockets could jam | magnet board with printed route | staff proof card | builder socket tolerance | cardboard socket test |
| Tide gauge slider | keep | physical alignment is clear | small marks may be hard to read | larger float board | demonstrate one false tide | accessibility read distance | high-contrast mock |
| Wind vane words | keep | lets one player handle cards | too text-heavy | symbol cards | point to cardinal arrow | reading load playtest | icon pass |

## Alternatives

| Beat/device | Primary implementation | Alternative implementation | What stays the same | Tradeoff | Trigger to switch |
|---|---|---|---|---|---|
| Barometer dial | rotating pointer board | loose overlay cards | pressure cartridge output | dial is more scenic, cards easier reset | pointer drift in bench |
| Wind rack | ordered physical cards | magnetic word tiles | wind-word aha | cards cheaper, magnets durable | card loss after playtest |
| Tide gauge | slider with click detent | weighted float in tube | tide-height alignment | slider safer than fluid | slider unreadable |
| Console | proof sockets and shutter | operator-visible tray plus envelope | proof convergence | sockets more magical, tray easier | socket jams or cost too high |

## Technique play profile

| Beat/candidate | Technique(s) | Crowd profile | Advantage in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P1 storm log | sorting, role assignment | confused teams | creates bounded search | too much reading | short labels and operator category hint |
| P2 barometer | alignment | tactile players | obvious physical action | random spinning | detents and mismatch feedback |
| P3 wind | sort/orientation | verbal teams | good pair task | card ambiguity | color/back icons |
| P4 tide | fit/alignment | visual-spatial teams | clear exclusion | glare or tiny marks | high contrast and work light |
| P5 route | team ritual | all teams | final shared moment | one-player takeover | caller/checker/handler protocol |

## Nodes

| ID | Beat | Technique/device | Skill tags | Physical mechanism | Input | Aha | Output | Reset state |
|---|---|---|---|---|---|---|---|---|
| P1 | Storm log intake | TECH-SORT-001 / log and role cards | categorize, roles | log opens instrument jobs | team reads station alert | three data proofs are needed | role cards and instrument targets | log closed, roles stacked |
| P2 | Barometer pressure | TECH-ALIGN-001 / dial | compare, align | dial pointer and pressure cartridge | warning card and dial marks | falling pressure selects the storm code | pressure cartridge | dial home, cartridge hidden |
| P3 | Wind vane words | TECH-SORT-003 / vane rack | sort, orientation | vane arrow and ordered cards | wind rose and word cards | direction order spells beacon word | wind cartridge | cards sorted home |
| P4 | Tide gauge height | TECH-FIT-003 / slider | fit, exclude | float slider and harbor overlay | tide table and gauge | only one height aligns harbor marks | tide cartridge | slider home, cartridge hidden |
| P5 | Stormglass signal | TECH-TEAM-001 / proof sockets | verify, final call | cartridges unlock route shutter and signal panel | three cartridges | instruments agree to one safe harbor route | beacon call and exit token | cartridges removed, shutter closed |

## Edges

| From | To | Dependency | Failure fallback |
|---|---|---|---|
| P1 | P2 | pressure target introduced | operator names pressure station |
| P1 | P3 | wind target introduced | operator points to vane rose |
| P1 | P4 | tide target introduced | operator points to tide table |
| P2 | P5 | pressure cartridge required | staff proof cartridge after demonstrated solve |
| P3 | P5 | wind cartridge required | staff proof cartridge after card order |
| P4 | P5 | tide cartridge required | staff proof cartridge after aligned gauge |
| P5 | exit | beacon route set | operator manual release and story wrap |

## Hint ladder

| Stuck state | Observable signal | Hint 1 | Hint 2 | Acceleration |
|---|---|---|---|---|
| searching before roles | opening every prop | "The station asks for three readings first." | point to pressure, wind, tide labels | assign role cards |
| random barometer spin | repeated dial turns | "Warnings are not equal; find the falling one." | compare low-pressure card to dial | set one detent as example |
| wind card ambiguity | words shuffled repeatedly | "Let the arrow choose the order." | name first cardinal point | give first card position |
| tide glare | leaning close, no slider use | "The float is a measuring tool, not decoration." | show one false alignment | set float to adjacent wrong state |
| final guessing | pressing beacon buttons early | "The station will not transmit without proof." | seat already-earned cartridges visibly | staff opens shutter after proof call |

## Bottleneck check

- Required bottlenecks: final proof seating and beacon call.
- Accidental bottlenecks: unreadable tide overlay, ambiguous wind order, socket jam.
- Parallel work: P2, P3, and P4 can proceed after P1.
- Final convergence: P5 requires all three proof cartridges and team role split.

## Purpose mapping review

| Test | Pass? | Evidence | Required revision |
|---|---|---|---|
| Injection: distinct visible things have distinct understandable purposes | Draft | instrument labels map to P2-P4 | playtest clue noise |
| Surjection: every required purpose has a visible/documented home | Draft | every beat has a zone and prop | prototype photos |
| Bijection: used elements make sense after use | Draft | cartridges move from stations to console | final reveal script |
| Intentional overlap/share: reused elements layer cleanly | Draft | console reuses three cartridges | socket color coding |
| Build: earlier uses make later uses more meaningful | Draft | readings become final route | tabletop simulation |

| Visible element/zone | Purpose(s) | First use | Later reuse/build | Atmosphere or clue? | Risk |
|---|---|---|---|---|---|
| Storm log | role frame and category targets | P1 | hint reference | both | reading load |
| Instrument wall | pressure/wind/tide proofs | P2-P4 | source of cartridges | clue | crowding |
| Stormglass console | proof convergence | P5 | final reveal | both | socket jam |
| Signal panel | beacon payoff | P5 | exit story | clue | button guessing |

## Promotion gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Aha | Draft | nodes and hint ladder | tabletop test |
| Fairness | Draft | separate clues per instrument | playtest stuck log |
| Theme | Draft | weather station actions | scenic language pass |
| Physicality | Draft | every beat has a physical mechanism | cardboard prototype |
| Purpose mapping | Draft | mapping review above | zone labels |
| Economics | Draft | BOM ranges | vendor/source pass |
| Flow | Draft | floorplan zones | tape layout |
| Ops | Draft | reset and failure modes | reset drill |
| Safety | Draft | safety file | safety review |

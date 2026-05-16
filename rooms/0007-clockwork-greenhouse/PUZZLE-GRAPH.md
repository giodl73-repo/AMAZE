# Clockwork Greenhouse Puzzle Graph

## Phase mix

| Role | Phase | Base skills | Why it belongs in this room |
|---|---|---|---|
| Signature | growth-system convergence | sort, align, verify | greenhouse fantasy is systems balancing into bloom |
| Support | physical classification | categorize, compare | seed/soil makes early tactile work |
| Support | mechanical tuning | valves, light, final wheel | keeps puzzle intent physical |

## Design pulse evidence

| Design pulse | Phase | Output | Evidence | Decision |
|---|---|---|---|---|
| v0.seed | system convergence | five-beat graph | design draft | prototype bloom wheel first |

## Hopper

| ID | State | Phase | Name | Technique/device | Skill tags | Aha | Physical action | Clue sources | Failure mode | Promotion test |
|---|---|---|---|---|---|---|---|---|---|---|
| H-007-01 | promoted | setup | Gardener log | TECH-SORT-001 / log | role split, category frame | the orchid needs four balanced systems | assign role tags and read plant needs | gardener log, bench labels | search soup | confused team names four systems |
| H-007-02 | promoted | seed/soil | Seed bed pairing | TECH-SORT-003 / trays | classify, match | each seed wants one soil texture | place seed tokens into soil tile tray | seed shapes, soil icons | tiny piece loss | correct tray earns root token |
| H-007-03 | promoted | water | Irrigation valves | TECH-ALIGN-001 / valve board | sequence, align | watering order follows leaf vein arrows | turn valves and claim water token | leaf cards, valve colors | random valve turning | team explains order |
| H-007-04 | promoted | light | Prism shade path | TECH-ALIGN-003 / slider | overlay, compare | flower color appears only at one light angle | slide prism/shade cards and claim light token | shade cards, prism marks | glare or angle ambiguity | readable under work light |
| H-007-05 | promoted | finale | Bloom wheel | TECH-TEAM-001 / token sockets | verify, converge | four systems unlock one bloom season | seat tokens, rotate wheel, open flower | proof tokens, season dial | one-player final | all roles verify before reveal |

## Puzzle review

| Candidate | Verdict | Why it might be fun | Why it might fail | Alternative implementation | Backup/hint | User/design input needed | Next revision |
|---|---|---|---|---|---|---|---|
| Bloom wheel | keep | tangible payoff and group reveal | pinch/jam risk | envelope reveal card | manual bloom card | builder tolerance | cardboard wheel |
| Seed bed pairing | keep | tactile and family-friendly | small-piece loss | large magnetic tokens | staff root token | piece size | enlarge tokens |
| Prism shade path | keep | pretty visual aha | glare/accessibility | printed overlay only | work-light card | glare test | high contrast prototype |

## Alternatives

| Beat/device | Primary implementation | Alternative implementation | What stays the same | Tradeoff | Trigger to switch |
|---|---|---|---|---|---|
| Seed bed | physical trays and tokens | magnetic board | seed/soil classification | trays more tactile, magnets easier reset | piece loss |
| Valve board | rotating valves | slider levers | water sequence | valves more thematic | pointer drift |
| Prism path | acrylic slider | printed color wheel | light-angle aha | acrylic more magical | glare blocks accessibility |
| Bloom wheel | mechanical petals | printed reveal envelope | proof convergence | wheel stronger payoff | wheel jams |

## Technique play profile

| Beat/candidate | Technique(s) | Crowd profile | Advantage in this room | Frustration trigger | Safeguard |
|---|---|---|---|---|---|
| P1 log | sorting and roles | confused teams | bounds search | reading load | short checklist |
| P2 seed bed | tactile matching | families | physical early win | tiny pieces | oversized tokens |
| P3 valves | alignment/sequence | chaotic teams | durable action | random turning | colored home marks |
| P4 prism | overlay/compare | visual teams | delightful reveal | glare | work light/high contrast |
| P5 bloom | team ritual | all teams | shared final moment | one-player takeover | caller/checker/handler |

## Nodes

| ID | Beat | Technique/device | Skill tags | Physical mechanism | Input | Aha | Output | Reset state |
|---|---|---|---|---|---|---|---|---|
| P1 | Gardener log intake | TECH-SORT-001 / log and role tags | categorize, roles | log, role tags, proof rail | team reads orchid needs | four systems are required | system roles and proof targets | log closed, tags stacked |
| P2 | Seed bed pairing | TECH-SORT-003 / seed trays | classify, match | seed tokens and soil tiles | seed shapes and soil icons | seeds match textures, not colors | root token | tokens home, root hidden |
| P3 | Irrigation valves | TECH-ALIGN-001 / valve board | sequence, align | four valve pointers | leaf vein arrows | arrows define watering order | water token | valves home, token hidden |
| P4 | Prism shade path | TECH-ALIGN-003 / prism slider | overlay, compare | prism/shade slider | flower color and shade marks | one angle reveals true bloom color | light token | slider home, token hidden |
| P5 | Bloom wheel | TECH-TEAM-001 / token sockets | verify, converge | four token sockets, season dial, petals | root/water/light/time tokens | balanced systems unlock the orchid | bloom reveal and exit token | tokens removed, wheel home |

## Edges

| From | To | Dependency | Failure fallback |
|---|---|---|---|
| P1 | P2 | root system introduced | operator names seed bed |
| P1 | P3 | water system introduced | operator points to leaf arrows |
| P1 | P4 | light system introduced | operator points to shade marks |
| P2 | P5 | root token required | staff root token after explanation |
| P3 | P5 | water token required | staff water token after valve order |
| P4 | P5 | light token required | staff light token after prism state |
| P5 | exit | bloom reveal | manual bloom card |

## Hint ladder

| Stuck state | Observable signal | Hint 1 | Hint 2 | Acceleration |
|---|---|---|---|---|
| search before systems | props handled without roles | "The log names what the orchid needs." | point to four proof silhouettes | assign role tags |
| seed mismatch | color matching only | "The roots care about texture." | show one wrong soil texture | give first pairing |
| valve spinning | random turns | "Follow the leaf veins from stem to tip." | name first valve | set first valve |
| prism glare | avoids slider | "Use the inspection light and compare one mark at a time." | show one false angle | use printed light token card |
| final takeover | one player grabs tokens | "The flower requires a caller, a checker, and a gardener's hand." | assign roles | staff verifies and releases |

## Bottleneck check

- Required bottlenecks: final bloom wheel and four proof tokens.
- Accidental bottlenecks: tiny seed pieces, valve order ambiguity, prism glare, wheel jam.
- Parallel work: P2, P3, and P4 can proceed after P1.
- Final convergence: P5 requires root, water, light, and time/system verification.

## Purpose mapping review

| Test | Pass? | Evidence | Required revision |
|---|---|---|---|
| Injection: distinct visible things have distinct understandable purposes | Draft | zones and labels separate systems | playtest noise |
| Surjection: every required purpose has a visible/documented home | Draft | all nodes have mechanisms | prototype photos |
| Bijection: used elements make sense after use | Draft | tokens move to bloom wheel | final script |
| Intentional overlap/share: reused elements layer cleanly | Draft | proof rail then wheel | token silhouettes |
| Build: earlier uses make later uses more meaningful | Draft | systems converge into bloom | tabletop simulation |

| Visible element/zone | Purpose(s) | First use | Later reuse/build | Atmosphere or clue? | Risk |
|---|---|---|---|---|---|
| Gardener log | role frame | P1 | hint reference | both | reading load |
| Seed trays | root proof | P2 | root token | clue | piece loss |
| Valve board | water proof | P3 | water token | clue | random turning |
| Prism panel | light proof | P4 | light token | both | glare |
| Bloom wheel | final convergence | P5 | exit payoff | both | jam/pinch |

## Promotion gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Aha | Draft | graph and hint ladder | tabletop test |
| Fairness | Draft | separate clue sources | playtest |
| Theme | Draft | greenhouse systems | scenic pass |
| Physicality | Draft | all beats physical | cardboard prototype |
| Purpose mapping | Draft | review above | zone labels |
| Economics | Draft | BOM ranges | sourcing pass |
| Flow | Draft | floorplan | tape layout |
| Ops | Draft | OPS reset/failures | timed reset |
| Safety | Draft | SAFETY file | safety review |

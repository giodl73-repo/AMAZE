# The Black Box at Mile Zero - Puzzle Graph

Use this graph as a seed, not a locked design. The central object, player role,
and tone are now locked enough for first simulations: a sealed roadside evidence
recorder, roadside investigators, and an eerie-wonder reveal.

## Phase mix

| Role | Phase | Base skills | Why it belongs in this room |
|---|---|---|---|
| Signature | mechanical state | turning, latching, alignment, object state | The room is about physically understanding one central object. |
| Support | route/spatial transform | ordering, mapping, orientation | Mile Zero is a beautiful impossible road, not a normal trip itinerary. |
| Support | audio/reveal | listening, transcription, pattern matching | The recorder should feel like a witness, not a generic audio clue. |

## Design pulse evidence

| Design pulse | Phase | Output | Evidence | Decision |
|---|---|---|---|---|
| v0.theme | mechanical state | Central object as main promise. | User selected mystery trailer with strong physical object. | Seed graph created. |
| v0.theme-lock | mechanical state, route/spatial transform, audio/reveal | Roadside recorder, investigators, eerie-wonder final reveal. | User selected black box, roadside investigators, and beautiful impossible road tone. | Run first team matrix before adding more puzzle complexity. |

## Hopper

| ID | State | Phase | Name | Skill tags | Aha | Physical action | Clue sources | Failure mode | Promotion test |
|---|---|---|---|---|---|---|---|---|---|
| H1 | candidate | mechanical state | Witness windows | observation, alignment | box windows are not labels; they are state indicators | slide panels and read changing windows | black box, evidence rail | players treat windows as flavor text | amazing and confused teams both explain the state change |
| H2 | candidate | route/spatial transform | Mile tags | ordering, mapping | evidence tags mark distance from an impossible origin | hang tags on rail in route order | wall rail, box interior | search soup if tags are too similar | team finds order without operator hint in target time |
| H3 | candidate | audio/reveal | Recorder skip | audio, transcription | repeated noise is a usable index, not ambience | play recorder, match transcript fragments | audio player, printed backup | audio-dependent failure | quiet and accessibility-varied teams solve with backup |
| H4 | candidate | classification/indexing | Evidence bags | sorting, extraction | bag numbers are chain-of-custody, not puzzle order | sort and clip physical evidence | evidence bags, case board | too much reading | family/mixed team can assign child-friendly role |
| H5 | candidate | mechanical state | Final latch stack | collaboration, sequencing | each subsystem unlocks a safe latch, but opening requires restraint | set latches and open box with one handle | box, rail, recorder, cabinet | players force latches | no-force affordance works under chaotic team |

## Nodes

| ID | Beat | Skill tags | Physical mechanism | Input | Aha | Output | Reset state |
|---|---|---|---|---|---|---|---|
| P1 | Evidence intake | observation, role assignment | evidence rail, tags, case board | visible evidence and sealed roadside recorder | evidence is a route witness, not decoration | first box state and team roles | rehang evidence tags |
| P2 | Witness windows | mechanical state, alignment | sliding windows on black box | evidence tag marks | windows reveal which evidence matters now | active evidence subset | reset sliders |
| P3 | Recorder skip | audio, transcription, indexing | rugged audio recorder with printed transcript backup | active evidence subset | the skip pattern is the recorder trying to protect a route phrase | route phrase or coordinate fragment | reset audio and transcript cards |
| P4 | Utility cabinet | ordering, safe switching | low-voltage cabinet, lights, manual bypass | route phrase plus cabinet labels | cabinet energizes the correct latch path | final latch sequence | reset switches |
| P5 | Box opening | collaboration, restraint, metapuzzle | central box latches, one-handle opening, interior reveal | window state, recorder phrase, latch sequence | the recorder opens when the incident is reconstructed as a road, not a crash | Mile Zero reveal and exit | relock box and replace reveal |

## Edges

| From | To | Dependency | Failure fallback |
|---|---|---|---|
| P1 | P2 | evidence tags teach box-window interaction | operator points to matching tag shapes |
| P2 | P3 | active evidence subset narrows recorder/transcript | window color matches transcript border |
| P3 | P4 | route phrase identifies cabinet labels | operator gives first word of route phrase |
| P4 | P5 | latch sequence opens final box | manual latch bypass at hard-exit minus 4 |

## Hint ladder

| Stuck state | Observable signal | Hint 1 | Hint 2 | Acceleration |
|---|---|---|---|---|
| Treating evidence as loose search | random handling, no shared theory | "The case board is trying to sort the evidence for you." | "Look for marks that repeat on the box." | operator names the first matching mark |
| Box windows ignored | players solve around central object | "The box is not just locked; it is watching state." | "Move one window and compare what changes." | operator sets first window |
| Audio blocks progress | replaying audio without notes | "The transcript is official for a reason." | "The repeated skip is a count." | reveal one indexed transcript word |
| Latch force risk | tugging or twisting | "This evidence unit opens only when it is calm." | "If it resists, one setting is wrong." | operator freezes play and bypasses latch safely |

## Bottleneck check

- Required bottlenecks: final box opening only.
- Accidental bottlenecks: audio comprehension, central object crowding, latch mechanics.
- Parallel work: evidence rail, transcript fragments, and box observation can split early.
- Final convergence: window state, recorder phrase, and utility latch sequence open the box.

## Promotion gates

| Gate | Pass? | Evidence | Required revision |
|---|---|---|---|
| Aha | TBD | no team simulation yet | Prove players understand the object-state idea. |
| Fairness | TBD | no team simulation yet | Test evidence volume and transcript clarity. |
| Theme | Draft pass | black box, roadside investigators, eerie-wonder reveal locked | Simulate whether players feel "we found a road," not "we solved a filing cabinet." |
| Physicality | TBD | all proposed beats physical | Prototype box windows and final latch. |
| Economics | TBD | no BOM yet | Estimate central box cost and spares. |
| Flow | TBD | floorplan draft only | Verify crowding around central table. |
| Ops | TBD | OPS draft only | Validate 10-minute reset. |
| Safety | TBD | SAFETY draft only | Resolve object weight, latch force, egress. |


# The Brineworks at Low Tide - Scenes

## Party assumptions

| Team archetype | Personas/roles | Notes |
|---|---|---|
| Amazing team | sorter, route planner, prop handler, skeptic | May solve fast and need optional dry bubble gags. |
| Confused team | bin searcher, hesitant reader, quiet matcher | Needs categories and physical affordances to be obvious. |
| Fighting team | dominant planner, ignored tile handler, frustrated checker | Must split service jobs so one player cannot own all logic. |
| Chaotic team | tile dumper, bell ringer, speed guesser | Loose props, bell tokens, and final griddle need hard reset homes. |
| Accessibility-varied team | seated checker, low-light-sensitive player, companion handler | Must solve without high reach, wet/slippery effects, or audio-only bell clues. |
| Operator stress team | late/tired group, one disengaged player | Operator must see proof state and recover without handling every tile. |

## Team archetype probes

| Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Operator response | Design revision |
|---|---|---|---|---|---|
| Amazing team | May finish core thin. | P2-P5 | all proofs earned before atmosphere lands | offer dry bubble gags after proof earns | optional reward cards |
| Confused team | May search bins without category frame. | P1-P2 | no one names order/current/bell | point to shift card categories | stronger category plaques |
| Fighting team | One planner may dominate routing and final. | P2-P5 | others wait while one player decides | assign sorter/router/ringer/checker | split proofs physically |
| Chaotic team | May dump tiles, ring bells, force griddle. | P2-P5 | props scattered or token guessing | reset tray frame and pause griddle | oversized parts and gated sockets |
| Accessibility-varied team | Reach/audio may block. | P3-P4 | pipe or bell clue cannot be used | use waist-height route and visual ring | no audio-only solve |
| Operator stress team | Proof state may be hard to monitor. | Full run | operator cannot tell which proofs are earned | proof rail/state photo | add public proof homes |

## Group-game stakes

| Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|
| Sorter | agency | Tile handling must matter, not be busywork. | P2 | untested | edge shapes should reject wrong stack |
| Router | agency/access | Pipe logic should be readable from seated/checker role. | P3 | untested | high-contrast arrows |
| Bell tester | delight/access | Bell prop should be funny without audio dependence. | P4 | untested | visual clam closure |
| Operator | control | Can recover proof state and reset quickly. | all | untested | proof tokens and reset photos |

## Behavior probes

| Behavior | Team/persona | Scene/beat probe | Observable signal | Risk if unsupported | Design response |
|---|---|---|---|---|---|
| over-searching | Confused team | P1-P2 | rummaging bins without rail use | search soup | shift card and bounded bins |
| logic domination | Fighting team | P2-P5 | one player routes and sets everything | low group fun | station split and final caller/checker/handler |
| random guessing | Chaotic team | P5 | griddle token/socket attempts before proofs | false solve/breakage | proof sockets and staff-visible state |
| audio dependence | Accessibility-varied team | P4 | listening to bells instead of testing pearl fit | unfair solve | visual/tactile clam fit and ring mark |
| speedrunning | Amazing team | P2-P5 | fast solve misses comedy | thin delight | optional dry bubble gags |
| state tracking under pressure | Operator stress team | full run | operator loses proof state | bad hinting/reset | proof-token homes |

## Scene list

| Scene | Purpose | Entry condition | Exit condition | Clock | Team/behavior probes |
|---|---|---|---|---|---|
| S1 Shift starts | Establish visible exit, canteen panic, and three service jobs. | team enters snack works | team understands order/current/bell proofs | 0-6 | confused, over-searching |
| S2 Lunch repair | Split work across order rail, pipe board, and clam register. | service jobs learned | all three proofs earned or accelerated | 6-28 | fighting, chaotic, accessibility-varied |
| S3 Pressure griddle | Converge proofs and restart the canteen. | proofs ready | open token released | 28-38 | random guessing, operator stress |
| S4 Open/reset | Land goofy payoff, keep egress clear, reset stations. | open token released | team exits and reset starts | 38-45 | all teams |

## Beat cards

| Scene | Beat | Player action | Aha | Check | Behavior probe | Target min | Hint at min | Slow max min | Mechanism | Reliability risk | DC | Success | Partial | Stall/hint trigger | Reset effect |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| S1 | P1 Shift card intake | inspect shift card, plaques, and visible stations | canteen has three service jobs | team names order/current/bell | over-searching | 4 | 2 | 6 | shift card, plaques, bell flavor | reading load | 8 | categories understood | one category named | bin rummaging before category frame | reset card/plaques |
| S2 | P2 Singing order rail | sort ingredient tiles into order rail and claim service ticket | order is a physical stack | order proof earned | over-searching | 7 | 4 | 9 | ingredient tiles, rail, service ticket | tile loss/search soup | 11 | ticket placed in proof home | one correct tile pair | tile pile chaos | reset tiles/ticket |
| S2 | P3 Current pipe board | rotate chunky pipe elbows to route blue tide | current follows arrows to griddle | current proof earned | accessibility-varied | 8 | 5 | 10 | pipe board, arrows, current gear | fiddly alignment/reach | 12 | gear placed in proof home | route reaches one checkpoint | random elbow turning | reset elbows/gear |
| S2 | P4 Bell clam register | test chunky bell tokens in the clam's pearl cup and claim shell tag | pearl bell is the only token that closes the clam without force | bell proof earned | audio dependence | 6 | 4 | 8 | clam register, bell tokens, pearl hinge cup, visual ring mark | force, tolerance drift, or audio confusion | 10 | shell tag placed in proof home | one wrong bell rejected | ringing/listening only or forcing clam shut | reset bells/tag/cup |
| S3 | P5 Pressure griddle | seat ticket, gear, and shell tag; call/check service; lift open token | griddle restarts only with proofs | open token released | random guessing | 6 | 4 | 8 | proof sockets, pressure gauge, open token | token guessing/false open | 13 | open token and exit beat | two proofs seated | random socket attempts | reset griddle/tokens |

## Transformation states

| ID | Beat | Object/space | From state | Trigger | To state | Visible proof | Reset state | Failure/bypass |
|---|---|---|---|---|---|---|---|---|
| T1 | P2 | Service ticket rail | ticket hidden, order rail empty | ingredient tiles stacked in correct order | service ticket moves to proof home | stacked order visible | tiles/ticket reset | staff ticket after one rejected/accepted stack proof |
| T2 | P3 | Current gear board | gear locked beside pipe board | elbows route blue tide from intake to griddle | current gear moves to proof home | pipe route visible to seated checker | elbows/gear reset | staff gear after first elbow route proof |
| T3 | P4 | Clam register shell tag | shell tag hidden, clam cup open | pearl bell fits without force and closes clam | shell tag moves to proof home | visual ring mark and closed clam visible | bells/tag/cup reset | staff shell tag after wrong-token rejection proof |
| T4 | P5 | Pressure griddle | griddle closed and pressure gauge idle | ticket, gear, and shell tag seated in sockets | pressure gauge rises and open token releases | proof sockets occupied and token visible | griddle/tokens reset | staff seats proven proof token if socket jams |
| T5 | P5 | Open sign/token | open token hidden | griddle service call checked | open token available for exit beat | token can be handed to operator | token home | operator supplies token after verified griddle state |

## Unlock paths

| Path | Beats | Unlocks | Fast coherence | Slow coherence | Operator acceleration |
|---|---|---|---|---|---|
| Canteen service proofs | P2-P4 | Pressure griddle proof sockets | Fast teams still need order, current, and bell proofs before the griddle matters. | Slow teams can get one tactile demonstration without losing the three-job frame. | Staff grants one verified proof after the team explains the service job. |
| Griddle open | P5 | Open token and exit beat | Fast teams must seat all proofs and call/check service before token release. | Slow teams can use manual token release without allowing random sockets to win. | Operator supplies open token after verified proof seating and service call. |

## Session timing model

| Timing field | Meaning |
|---|---|
| Target min | Current planned puzzle-beat total is 31 minutes for a 45-minute staffed-hour profile. |
| Hint at min | Operator should intervene before prop chaos or social domination takes over. |
| Slow max min | Current slow-case ceiling is 41 minutes, leaving a small usable buffer. |

## Operator interventions

| Trigger | Intervention | Fiction-preserving phrasing | Escalation |
|---|---|---|---|
| Category confusion | Point to shift card | "The lunch rush has three jobs." | name order/current/bell |
| Tile chaos | Reframe rail | "The rail wants a stack, not soup." | show one rejected tile |
| Pipe stall | Start at intake | "The tide starts here and follows arrows." | set one elbow |
| Bell audio focus | Point to pearl cup | "The clam votes with its hinge, not its music." | demonstrate one wrong bell failing to fit |
| Final guessing | Pause griddle | "The griddle only opens for proven service." | seat staff proof for proven category |

## Prototype simulation focus

| Prototype question | Team most likely to expose it | Watch for | Pass condition |
|---|---|---|---|
| Do players understand order/current/bell as three proof categories? | Confused team | handling props before category frame | team names all three before P5 |
| Does P2 feel like cooking, not color homework? | Amazing team | "too easy" comments | rail is quick but satisfying |
| Can seated/checker role solve P3? | Accessibility-varied team | reaching, squinting, companion dependence | checker can guide route |
| Is P4 visual/tactile, not audio-only? | Accessibility-varied team | listening for bell tone instead of testing fit | pearl cup closure and ring mark carry solve |
| Can chaotic team force P5? | Chaotic team | wrong sockets, griddle pulling | no false open or damage |
| Can staff reset under 10 minutes? | Operator stress team | missing tiles/tokens | reset drill passes |

## Consequence log

| Beat | Consequence | Affected clock | Operator visibility | Revision |
|---|---|---|---|---|
| P1 | If categories are unclear, room becomes prop soup. | S1 | visible in movement | stronger plaques |
| P2 | If too many tiles exist, order rail becomes search soup. | S2 | bin camera/count | reduce tile count |
| P3 | If pipes are small/high, accessibility fails. | S2 | route camera | enlarge and lower board |
| P4 | If bell is audio-only or force-based, fairness fails. | S2 | register camera | oversized pearl cup, no-force stop, visual ring state |
| P5 | If griddle opens without proofs, finale fails. | S3 | socket camera | proof-gated sockets |

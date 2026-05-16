# The Manor in the Mirrorline - Scenes

## Party assumptions

| Team archetype | Personas/roles | Notes |
|---|---|---|
| Amazing team | deduction captain, prop handler, skeptic, notetaker | May solve categories too fast and want optional manor lore. |
| Confused team | searcher, hesitant reader, quiet observer | Needs categories made obvious without shame. |
| Fighting team | dominant logician, ignored prop handler, frustrated reader | Must prevent one-player logic-grid ownership. |
| Chaotic team | prop toucher, joke-maker, speed guesser | Prop inventory and no-guess final cabinet are critical. |
| Accessibility-varied team | seated player, low-light-sensitive player, companion handler | Must solve without darkness-only, audio-only, or reach-only clues. |
| Operator stress team | late/tired group, one disengaged player | Operator must track category state and prevent random final guessing. |

## Team archetype probes

| Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Operator response | Design revision |
|---|---|---|---|---|---|
| Amazing team | May solve too fast and feel thin. | P1-P4 | They identify all three categories before target time. | Offer optional house-echo reads after proofs are earned. | Add optional depth that does not affect required solve. |
| Confused team | May search props without deduction frame. | P1-P2 | They do not separate suspect/object/location. | Prompt house ledger categories. | Make category affordances stronger. |
| Fighting team | One player may dominate the deduction. | P2-P5 | Others wait while one player reasons aloud. | Assign caller/checker/handler roles. | Split evidence across physical stations. |
| Chaotic team | May guess or scatter objects. | P3-P5 | Random selector changes or misplaced objects. | Enforce "proved accusation only" protocol. | Add prop trays, tethers, selector locks. |
| Accessibility-varied team | Low light, audio, or reach may block. | P3-P5 | Visual/tactile alternatives carry solve. | Offer work-light and companion handling. | Duplicate all required reveal info. |
| Operator stress team | Category state may be hard to monitor. | Full run | Operator cannot tell what has been proven. | Use state map and category checkpoints. | Add operator-facing indicators. |

## Group-game stakes

| Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|
| Deduction captain | agency | Gets satisfying reasoning without taking over. | P2-P5 | untested | require physical verification by others |
| Prop handler | agency | Handles objects meaningfully, not as assistant labor. | P3, P5 | untested | object marks must matter |
| Quiet observer | recognition | Can notice broken mirrorlines and overlay mismatches. | P2, P4 | untested | make state changes visible |
| Operator | control | Can recover category/timing state. | all | untested | add state board if needed |

## Behavior probes

| Behavior | Team/persona | Scene/beat probe | Observable signal | Risk if unsupported | Design response |
|---|---|---|---|---|---|
| over-searching | Confused team | P1-P2 | scanning every prop with no category theory | search soup | ledger categories and bounded stations |
| logic domination | Fighting team | P2-P5 | one player controls deduction, tokens, and cabinet | low group fun | caller/checker/handler split with separate proof and selector jobs |
| random guessing | Chaotic team | P5 | selector-cover forcing or slider spinning without evidence | false solves, breakage | proof-locked selector covers |
| low-light/glare avoidance | Accessibility-varied team | P4 | cannot read overlay/window marks | unfair location solve | work-light inspection window with high-contrast marks |
| speedrunning | Amazing team | P1-P4 | solves before manor atmosphere lands | thin delight | optional house echoes after proof earns |
| state tracking under pressure | Operator stress team | full run | operator loses proven categories | bad hinting or unsafe rush | operator state map |

## Scene list

| Scene | Purpose | Entry condition | Exit condition | Clock | Team/behavior probes |
|---|---|---|---|---|---|
| S1 Arrival | Establish guests, visible exit, and three-question mystery. | team enters parlor trailer | team understands suspect/object/location categories | 0-7 | confused, over-searching |
| S2 Evidence rooms | Split category work across gallery, sideboard, and room stations. | category rule learned | at least two categories narrowed | 7-24 | fighting, chaotic, accessibility-varied |
| S3 Accusation | Converge on final suspect/object/location and set cabinet. | all categories narrowed | mirrorline reveal and exit token | 24-40 | random guessing, operator stress |
| S4 Exit/reset | Land reveal, optional house echo, keep egress clear, reset modular stations. | reveal complete | team exits and reset starts | 40-45 | all teams |

## P0 content assumptions

P0 uses the prototype-only answer set from `PUZZLE-GRAPH.md`: Nella Ash,
hand mirror, observatory. Scene timing should test whether players can follow
category proofs without needing to memorize those labels.

## Beat cards

| Scene | Beat | Player action | Aha | Check | Behavior probe | Target min | Hint at min | Slow max min | Mechanism | Reliability risk | DC | Success | Partial | Stall/hint trigger | Reset effect |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| S1 | P1 House ledger intake | inspect ledger, bell, and visible stations | mystery has three physical categories | team names suspect/object/location | over-searching | 5 | 3 | 7 | ledger, bell, category plaques | reading overload | 9 | categories understood | one category named | searching before category frame | reset ledger/bell |
| S2 | P2 Suspect gallery | open portrait backs, place alibi tokens in truth windows, and claim the suspect crest | four alibi tokens complete the mirrorline; Nella Ash's breaks it | suspect proof token earned | logic domination | 7 | 5 | 9 | portrait frames, alibi tokens, truth windows, suspect crest token | frame damage, token swap, or subtle line read | 12 | suspect crest placed in team tray | one true alibi line proven | rereading without token tests; at slow path demonstrate one true alibi line | reset portraits/tokens/crest |
| S2 | P3 Object sideboard | sort objects, test them on the mirrorline balance plinth, and claim the object seal | the true object is the only one that balances the mirrorline | object proof token earned | random guessing | 6 | 4 | 8 | counted prop tray, balance plinth, object seal token | lost object or plinth tolerance drift | 11 | object seal placed in team tray | correct object tested but seal not claimed | object pile chaos or pure visual searching | replace objects/seal and reset plinth |
| S2 | P4 Room stations | use work-light inspection windows and transparent proof overlays to compare rooms, then claim the room-key tile | the true location is the room that refuses the suspect/object alignment | location proof token earned | low-light/glare avoidance | 8 | 5 | 9 | miniature room panels, work-light inspection windows, transparent overlays, room-key tile | glare, swapped overlay, or unreadable marks | 13 | room-key tile placed in team tray | one false room excluded | no overlay use by 5 min; at slow path demonstrate one false alignment | reset panels/overlays/tile |
| S3 | P5 Mirrorline accusation | move crest, seal, and tile from proof rail to cabinet sockets; lift unlocked selector covers; call/check answers; then set sliders | proof opens access before accusation becomes possible | final accusation state set | random guessing | 6 | 4 | 8 | public proof rail, keyed proof-token sockets, selector covers, accusation sliders, mirror reveal | token missing from rail, socket jam, cover jam, slider slip, or false positive | 13 | reveal and exit token | tokens seated but one selector wrong | cover forcing, slider spinning, missing token, or argument | reset proof rail/cabinet/tokens/covers/sliders |

## Transformation states

| ID | Beat | Object/space | From state | Trigger | To state | Visible proof | Reset state | Failure/bypass |
|---|---|---|---|---|---|---|---|---|
| T1 | P2 | Suspect gallery crest | portrait backs closed, crest hidden | alibi tokens complete truth-window contradiction | suspect crest moves to team tray | broken mirrorline/alibi proof visible | portraits/tokens/crest reset | staff crest after one true alibi line is demonstrated |
| T2 | P3 | Object sideboard seal | seal hidden beside balance plinth | true object balances mirrorline plinth | object seal moves to team tray | balance result visible on plinth | objects/seal and plinth reset | staff seal after failed/true balance proof |
| T3 | P4 | Room-key tile station | tile hidden among room panels | overlay proves true location by refusal alignment | room-key tile moves to team tray | overlay/window proof visible under work light | panels/overlays/tile reset | staff tile after one false alignment is demonstrated |
| T4 | P5 | Accusation cabinet covers | selector covers locked | crest, seal, and tile seated in keyed sockets | selector covers lift and sliders become playable | proof rail empty and cabinet sockets occupied | proof rail/cabinet/tokens/covers reset | staff seats proven category token if socket jams |
| T5 | P5 | Mirrorline reveal | mirror reveal hidden | final suspect/object/location sliders set and called/checked | mirrorline reveal and exit token appear | reveal visible from group position | sliders/reveal/token reset | operator reveal after verified accusation call |

## Unlock paths

| Path | Beats | Unlocks | Fast coherence | Slow coherence | Operator acceleration |
|---|---|---|---|---|---|
| Category proofs | P2-P4 | Accusation cabinet selector covers | Fast teams still need suspect, object, and location tokens before accusation controls are available. | Slow teams can receive one category demonstration while preserving the three-proof frame. | Staff provides one proven category token after the team explains the physical proof. |
| Mirrorline accusation | P5 | Mirror reveal and exit token | Fast teams must seat proofs and split caller/checker/handler before setting sliders. | Slow teams can bypass a jammed cover without skipping the final accusation call. | Staff opens a stuck cover or triggers reveal after verified proof seating and accusation call. |

## Session timing model

| Timing field | Meaning |
|---|---|
| Target min | Current planned puzzle-beat total is 32 minutes for a 45-minute staffed-hour profile. |
| Hint at min | Operator should intervene before deduction becomes private or frustration-heavy. |
| Slow max min | Current slow-case ceiling is 41 minutes after P2/P4 acceleration, leaving a small but usable buffer. |

## Operator interventions

| Trigger | Intervention | Fiction-preserving phrasing | Escalation |
|---|---|---|---|
| Category confusion | Point to ledger categories | "The house will only accept three answers." | name suspect/object/location categories |
| One-player domination | Assign roles | "The manor requires a caller, a checker, and a hand at the cabinet." | ask dominant player to verify only |
| Object chaos | Reset tray frame | "The sideboard inventory is part of the testimony." | demonstrate one failed balance test |
| Low-light block | Offer work-light inspection | "The housekeeper kept a daylight lamp for inspections." | demonstrate one false room alignment |
| Final guessing | Pause selector covers/sliders | "The manor rejects accusations without evidence." | seat staff proof token for proven categories |
| Fast team wants more | Offer house echoes after proof earns | "The house remembers more than it needed to prove." | point to one optional station-back echo |

## Prototype simulation focus

| Prototype question | Team most likely to expose it | Watch for | Pass condition |
|---|---|---|---|
| Do players understand that crest/seal/tile are category proofs? | Confused team | carrying tokens without knowing what they prove | team says "we need all three proofs" before P5 hint |
| Does token seating split roles? | Fighting team | one player takes all tokens from proof rail, covers, and sliders | caller/checker/handler roles emerge or operator can assign them cleanly |
| Are sockets/covers resistant to guessing and force? | Chaotic team | wrong-token attempts, cover forcing, slider spinning, jamming | no damage, no false reveal, clear "proof first" feedback |
| Is P2 a real physical contradiction rather than reading? | Confused team | rereading portrait text without trying alibi tokens | team tests tokens in truth windows before hint time |
| Is P3 a real physical test rather than search? | Confused team | inspecting objects for hidden marks without using plinth | team uses the balance plinth as the obvious test surface |
| Is P4 readable without spooky darkness? | Accessibility-varied team | avoiding overlay/window clue or needing high reach | high-contrast overlay route works under work light |
| Can staff reset tokens quickly? | Operator stress team | missing token, wrong home, proof rail clutter, socket unclear | reset verifies under 10 minutes |

## Consequence log

| Beat | Consequence | Affected clock | Operator visibility | Revision |
|---|---|---|---|---|
| P1 | If categories are unclear, the room becomes search soup. | S1 | visible in early movement | strengthen ledger/category plaques |
| P2 | If gallery is too text-heavy or token windows are subtle, one player dominates. | S2 | group posture/audio | make truth-window line bold and token actions obvious |
| P3 | If sideboard becomes hidden-mark search, the object category is weak. | S2 | object handling and plinth use | make balance plinth the obvious test |
| P4 | If overlay reveal is glare-prone or fiddly, accessibility fails. | S2 | station camera | enlarge overlays and make false-room alignment obvious |
| P5 | If cabinet covers can be forced or one player controls all sliders, deduction loses meaning. | S3 | selector-cover and proof-socket camera | test category proof locks and caller/checker/handler ritual |
| Optional echoes | If house-echo copy reads like a clue, teams may slow down or overthink. | S2-S4 | observer notes and hint log | keep echoes after proof earns and label as memory/flavor |


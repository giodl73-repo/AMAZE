# Signal in the Silverstream - Scenes

This file uses `docs/scene-beat-harness.md`.

## Party assumptions

| Team archetype | Personas/roles | Notes |
|---|---|---|
| Amazing team | Scout, Scribe, Mechanic, Face, Navigator | Test best-case shared discovery and signature broadcast payoff. |
| Confused team | First-Date Duo plus quiet observer behavior | Test onboarding, clue hierarchy, and dignity-preserving hints. |
| Fighting team | Dominant solver plus reluctant collaborator | Test competing theories at route/table beats. |
| Speed team | Speedrun Enthusiasts | Test sequence breaks, brute force, and bypass risk. |
| Family/mixed team | Family Four | Test crowding, parallel work, reach, and mixed-age participation. |
| Anxious/trust team | Anxious Participant plus Floor Operator | Test exit trust, pressure, sound, and low-light reassurance. |
| Operator stress team | Floor Operator | Test monitoring, acceleration, reset, and hard-exit procedure. |

## Team archetype probes

Use `docs/team-testing.md`.

| Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Operator response | Design revision |
|---|---|---|---|---|---|
| Amazing team | Best-case team should get a strong "trailer itself is the puzzle" arc. | P5 radio convergence | team references route, power, galley, and bearing together | stay silent until payoff | preserve final broadcast as signature moment |
| Confused team | Team may treat postcards and props as scenery. | P1 route fragments | no route ordering after 4 minutes | point attention to previous crew's logged stops | add stronger intro artifact if repeated |
| Fighting team | Competing theories may stall at spatial/table step. | P4 fold-down table | argument over whether table is furniture or puzzle | neutral attention redirect to travel in the table | add evidence mark that resolves debate |
| Speed team | Experts may bypass story and brute-force radio. | P5 radio convergence | attempts final tuning before bearing word | require route/power/frequency/bearing confirmation | add anti-bypass gating if needed |
| Family/mixed team | Children may lack safe meaningful work or handle fragile UV objects. | P3 galley inventory | adults take over cabinet compare | assign object counting/spotting as safe parallel action | make object set robust and readable |
| Anxious/trust team | Pressure and audio may feel like real distress. | P5 final broadcast | player checks exit or asks if sound stops | reassure exit/volume control in fiction | add visible pause/exit language |
| Operator stress team | Staff must recover failed UV/audio/table without breaking fiction. | P2, P4, P5 | solved state but mechanism fails | use manual UV, compass clue, or playback bypass | prototype diagnostics before build |

## Group-game stakes

| Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|
| First-Date Duo | Social safety | Hints can preserve dignity if they point attention before interpretation. | P1, P4 | pending | Test hint wording aloud. |
| Family Four | Role coverage | The room gives scouts, scribes, mechanics, and faces meaningful work. | P1, P2, P3 | pending | Add child-safe parallel tasks if P2 bottlenecks. |
| Speedrun Enthusiasts | Agency | Expert teams can move fast without bypassing the signature broadcast. | P2, P5 | pending | Stress-test sequence breaks. |
| Floor Operator | Recoverability | Staff can rescue stalls without breaking fiction. | P3, P4, P5 | pending | Write exact acceleration cards. |

## Behavior probes

Use `docs/behavior-testing.md`.

| Behavior | Team/persona | Scene/beat probe | Observable signal | Risk if unsupported | Design response |
|---|---|---|---|---|---|
| shared convergence | Amazing team | P5 radio convergence | team verbally connects route, power, galley, table, and radio | final beat feels like a code entry instead of a room payoff | preserve silence before payoff and make broadcast respond to all systems |
| over-inspecting set dressing | Confused team | P1 route fragments | all cabinets searched before map/postcards | search soup and wasted clock | bounded intro clue and postcard wear marks |
| brute force | Speed team | P2, P5 | repeated switch/radio attempts without inputs | bypasses aha chain | confirmation feedback and operator redirect |
| dominant solver | Fighting team | P4 fold-down table | one player blocks table and rejects other theory | social friction and spectator boredom | require visible alignment others can verify |
| excited handling | Family/mixed team | P3 galley inventory | objects dropped, hidden, or pocketed | reset failure and breakage | duplicate objects, count card, robust props |
| exit trust | Anxious/trust team | P5 final broadcast | player checks exit during audio pressure | real distress | visible exit reassurance and volume control |
| component failure recovery | Operator stress team | P2, P4, P5 | solved state but switch, latch, or audio fails | operator cannot recover without breaking fiction | manual UV trigger, service-notch bearing, and final playback bypass |

## Scene list

| Scene | Purpose | Entry condition | Exit condition | Clock | Team/behavior probes |
|---|---|---|---|---|---|
| Arrival and briefing | teach that the trailer systems are playable | team enters trailer | team identifies route artifacts | intro clock | confused team, over-inspecting |
| Route reconstruction | produce the route code | postcards/map visible | ordered route code found | session clock | amazing/confused teams |
| Power and galley | turn route into safe power and frequency clues | route code solved | three frequency digits found | soft-warning clock | speed/family teams |
| Table orientation | convert frequencies into bearing | galley output found | bearing word found | session clock | fighting team, dominant solver |
| Final broadcast | combine systems into final radio response | route, power, frequency, bearing known | exit key/reveal | hard-exit clock | speed/anxious/operator stress |

## Beat cards

| Scene | Beat | Player action | Aha | Check | Behavior probe | Target min | Hint at min | Slow max min | Mechanism | Reliability risk | DC | Success | Partial | Stall/hint trigger | Reset effect |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| Route reconstruction | P1 route fragments | sort postcards against map and mileage plaques | postcards are stops | ordering | over-inspecting set dressing | 4 | 3 | 6 | laminated postcards and mounted map | card loss or markings | Medium | route code | two stops placed | team treats postcards as souvenirs for 4 minutes | shuffle postcards |
| Power and galley | P2 shore-power panel | set labeled prop switches in route order | stops map to safe power sequence | indexing | brute force | 4 | 3 | 6 | low-voltage prop switch panel | false negative or forced switch | Medium | UV strip powers | first switch confirmed | repeated random switching | reset switches |
| Power and galley | P3 galley inventory | compare lit cabinet to inventory | absences mark radio frequencies | extraction | excited handling | 5 | 3 | 6 | UV cabinet and removable objects | UV failure or missing object | Medium | frequency digits | one digit found | team keeps searching unrelated storage after first digit | replace objects |
| Table orientation | P4 fold-down table | rotate/latch table against compass mark | orientation changes dial reading | spatial reasoning | dominant solver | 5 | 3 | 6 | rotating table, latch, and service-notch bearing mark | jammed latch or excess force | High | bearing word | compass noticed and service notch aligned | table treated as normal furniture or argument continues | reset table latch |
| Final broadcast | P5 radio convergence | tune radio using route, power, frequency, bearing | all systems describe one trip | metapuzzle | exit trust | 4 | 3 | 6 | tactile radio dial and audio module | audio misfire or loose knob | Medium | final broadcast | correct station range | final data remains separate after soft warning | reset radio |

## Session timing model

Target session: 30-minute prototype. Also evaluate a 45-minute staffed-hour
profile before committing to a sellable format.

| Timing field | Meaning |
|---|---|
| Target min | Expected solve time for a healthy run of this beat. |
| Hint at min | Minute within the beat when the operator should intervene if no progress is visible. |
| Slow max min | Maximum acceptable beat duration before acceleration or bypass. |

Planned puzzle-beat target is 22 minutes. That fits the 30-minute prototype with
little slow-case slack and is underfilled for a 45-minute staffed-hour profile
unless the room adds optional depth, richer story beats, or a second parallel
thread.

## Operator interventions

| Trigger | Intervention | Fiction-preserving phrasing | Escalation |
|---|---|---|---|
| team ignores route artifacts | point attention | "Dispatch says the previous crew always logged stops." | name postcards as stops |
| random switch flipping | safety redirect | "The trailer only likes power in trip order." | reveal first switch |
| galley inventory stalls after first digit | acceleration | "The missing gear is easier to hear if you only count what changed under repair light." | reveal one frequency digit |
| table beat stalls | attention redirect | "That table has more travel in it than it should." | show compass mark, then service-notch bearing mark |
| hard-exit minus 3 | acceleration | "The broadcast is trying to lock onto a bearing." | provide final alignment card |

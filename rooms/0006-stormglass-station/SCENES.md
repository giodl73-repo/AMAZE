# Stormglass Station Scenes

## Party assumptions

| Team archetype | Personas/roles | Notes |
|---|---|---|
| Amazing team | systems thinker, dial handler, mapper, caller | May parallelize P2-P4 quickly and need optional storm lore. |
| Confused team | careful reader, nervous handler, quiet observer | Needs the three-instrument frame made explicit. |
| Fighting team | dominant caller, ignored prop handler, skeptical checker | Needs final caller/checker/handler ritual. |
| Chaotic team | card shuffler, button presser, speed guesser | Needs no-guess proof gates and durable cartridges. |
| Accessibility-varied team | seated caller, low-light-sensitive player, companion handler | Must solve without darkness, audio, crawling, or high reach. |
| Operator stress team | late group, tired operator, reset pressure | Operator must see proof state and recover jams. |

## Team archetype probes

| Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Operator response | Design revision |
|---|---|---|---|---|---|
| Amazing team | May finish instrument work too fast. | P2-P4 | all cartridges earned before atmosphere lands | offer optional storm log after P5 | add optional lore cards |
| Confused team | May not see three required readings. | P1 | searches without naming pressure/wind/tide | point to station checklist | strengthen category labels |
| Fighting team | One player may own final signal. | P5 | one player holds all cartridges and buttons | assign caller/checker/handler | split console controls |
| Chaotic team | May press beacon before proofs. | P5 | random button presses or cartridge forcing | enforce proof-first script | add covers/detents |
| Accessibility-varied team | Glare or reach may block tide/console. | P4-P5 | avoids overlay or cannot reach sockets | offer work light and companion handling | lower/duplicate controls |
| Operator stress team | Reset errors may hide a cartridge. | full run | operator cannot confirm home state | use reset photo and pouch count | add keyed cartridge colors |

## Group-game stakes

| Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|
| Dial handler | agency | Has a real physical instrument job. | P2 | untested | detents must feel intentional |
| Card sorter | recognition | Can solve wind path without being a reader only. | P3 | untested | icon backs on cards |
| Seated caller | control | Can coordinate final proof without reaching every socket. | P5 | untested | caller card and readable console |
| Operator | control | Can see and recover all proof states. | all | untested | state map and spare pouch |

## Behavior probes

| Behavior | Team/persona | Scene/beat probe | Observable signal | Risk if unsupported | Design response |
|---|---|---|---|---|---|
| over-searching | Confused team | P1-P2 | handling all props without instrument frame | search soup | station checklist and role cards |
| logic domination | Fighting team | P5 | one player seats all cartridges and presses beacon | low group fun | caller/checker/handler ritual |
| random guessing | Chaotic team | P5 | button pressing before proof | false final solve | proof-locked route shutter |
| low-light/glare avoidance | Accessibility-varied team | P4 | cannot read tide overlay | unfair tide solve | high-contrast overlay and work light |
| speedrunning | Amazing team | P2-P4 | solves before story lands | thin delight | optional storm log |
| reset under pressure | Operator stress team | reset | missing cartridge or wrong card order | bad next run | pouch count and reset photo |

## Scene list

| Scene | Purpose | Entry condition | Exit condition | Clock | Team/behavior probes |
|---|---|---|---|---|---|
| S1 Watch change | Establish safety, visible exit, and three required readings. | team enters station | team names pressure, wind, and tide jobs | 0-6 | confused, over-searching |
| S2 Instrument work | Split pressure, wind, and tide physical proofs. | role cards assigned | three cartridges earned | 6-26 | fighting, chaotic, accessibility-varied |
| S3 Stormglass route | Converge proofs and reveal safe harbor route. | cartridges earned | route shutter open and beacon code known | 26-37 | random guessing, operator stress |
| S4 Beacon and exit | Transmit signal, land payoff, keep egress clear. | route known | exit token delivered and team exits | 37-45 | all teams |

## Beat cards

| Scene | Beat | Player action | Aha | Check | Behavior probe | Target min | Hint at min | Slow max min | Mechanism | Reliability risk | DC | Success | Partial | Stall/hint trigger | Reset effect |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|
| S1 | P1 Storm log intake | inspect station log, take role cards, identify three missing readings | the station only needs pressure, wind, and tide before it can transmit | team names the three jobs | over-searching | 4 | 2 | 6 | log, role cards, checklist | reading overload or loose role cards | 8 | roles assigned and instrument jobs named | one job named | searching stations before naming jobs | restack log and role cards |
| S2 | P2 Barometer pressure | compare warning cards to dial, rotate pointer to low-pressure mark, claim cartridge | falling pressure selects the storm code | pressure cartridge earned | speedrunning | 6 | 4 | 8 | dial board, warning cards, cartridge latch | pointer drift or random spin | 11 | cartridge placed in tray | correct card found but dial not set | repeated spinning without comparing warnings | dial home, cartridge hidden |
| S2 | P3 Wind vane words | set vane direction and order wind cards by arrow path | wind direction chooses word order, not map geography | wind cartridge earned | logic domination | 6 | 4 | 8 | vane, card rack, word cards | card loss or ambiguous order | 11 | wind word and cartridge earned | first card correct | shuffling cards without using vane | cards sorted, vane home |
| S2 | P4 Tide gauge height | move tide float, align overlay marks, claim tide cartridge | only one tide height aligns all harbor marks | tide cartridge earned | low-light/glare avoidance | 7 | 5 | 9 | slider gauge, overlay, cartridge | glare, sticky slider, tolerance drift | 12 | cartridge placed in tray | adjacent height excluded | reading table but not moving float | slider home, overlay pouch |
| S3 | P5 Stormglass route | seat three cartridges, open route shutter, call/check route, set beacon | instruments agree to one safe harbor route | route visible and beacon set | random guessing | 7 | 5 | 9 | proof sockets, shutter, signal buttons | socket jam, lost cartridge, button forcing | 13 | beacon call and exit token | cartridges seated but beacon wrong | button pressing before proof or one-player takeover | cartridges removed, shutter closed, buttons reset |

## Transformation states

| ID | Beat | Object/space | From state | Trigger | To state | Visible proof | Reset state | Failure/bypass |
|---|---|---|---|---|---|---|---|---|
| T1 | P2 | Pressure cartridge latch | locked cartridge behind dial | low-pressure mark selected from warning cards | pressure cartridge releases to proof tray | cartridge label visible in tray | dial home, cartridge hidden | staff pressure cartridge after card explanation |
| T2 | P3 | Wind cartridge rack | cartridge covered behind wind cards | vane direction and card order verified | wind cartridge releases to proof tray | wind word/card order visible | cards sorted, vane home, cartridge hidden | staff wind cartridge after first card proof |
| T3 | P4 | Tide cartridge slot | cartridge hidden behind tide overlay | float height aligns all harbor marks | tide cartridge releases to proof tray | tide height and aligned marks visible | slider home, overlay pouch, cartridge hidden | staff tide cartridge with work-light/high-contrast proof |
| T4 | P5 | Stormglass route shutter | shutter closed and beacon buttons inert | pressure, wind, and tide cartridges seated in sockets | route shutter opens and beacon route is visible | safe harbor route can be called/checker verified | cartridges removed, shutter closed, buttons reset | manual route release after staff seats earned proof |
| T5 | P5 | Exit token drawer | latched token drawer | beacon route called and signal set | drawer opens with exit token | token handed to operator at exit | drawer latched, token home | operator validates route and supplies exit token |

## Unlock paths

| Path | Beats | Unlocks | Fast coherence | Slow coherence | Operator acceleration |
|---|---|---|---|---|---|
| Instrument convergence | P2-P4 | Route shutter and beacon controls | Fast teams still collect three named readings before the route appears, so the final signal is earned by public proof. | Slow teams can receive one instrument-specific nudge while preserving the need for all three cartridges. | Staff releases one verified cartridge after the team explains the instrument logic, then requires all sockets before shutter open. |
| Beacon finale | P5 | Exit token drawer | Fast teams must call/check the visible route before the token drawer opens. | Slow teams can bypass a jammed shutter with the same route call/check ritual. | Manual route release and operator-validated exit token preserve proof without forcing a stuck device. |

## Session timing model

| Timing field | Meaning |
|---|---|
| Target min | Planned puzzle-beat total is 30 minutes, leaving room for intro, payoff, and variance. |
| Hint at min | Operator intervenes before instrument work becomes private or random. |
| Slow max min | Slow-case ceiling is 40 minutes before acceleration and manual proof release. |

## Operator interventions

| Trigger | Intervention | Fiction-preserving phrasing | Escalation |
|---|---|---|---|
| Category confusion | point to station checklist | "The watch log asks for three readings before any transmission." | name pressure, wind, tide |
| Barometer random spin | point to warning cards | "The needle follows the warning, not your guess." | set one false detent to show mismatch |
| Wind card shuffle | point to vane arrow | "The wind chooses the reading order." | place first card |
| Tide glare | offer work light and overlay demo | "The harbor marks need daylight inspection." | demonstrate one false tide |
| Final guessing | block beacon until proofs | "The transmitter is locked until the instruments agree." | staff seats earned proof |
| Socket jam | use staff proof cartridge | "The station accepts the verified spare." | manual route release |

## Consequence log

| Beat | Consequence | Affected clock | Operator visibility | Revision |
|---|---|---|---|---|
| P1 | If roles are unclear, room becomes search soup. | S1-S2 | visible movement pattern | strengthen checklist |
| P2 | If dial lacks detents, players spin randomly. | S2 | pointer visible | add click stops |
| P3 | If wind cards are text-heavy, quiet players disengage. | S2 | card handling | add icons |
| P4 | If overlay glares, accessibility fails. | S2 | leaning/squinting | high-contrast test |
| P5 | If buttons work before proof, final loses meaning. | S3 | beacon presses visible | proof shutter or staff lockout |

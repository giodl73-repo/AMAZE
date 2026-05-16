# Signal in the Silverstream - Playtest

Use this file for simulations, prototype tests, and external playtests. Scores
are forward-only: keep old results and create a new entry when the room changes.

Design pulses belong to AMAZE's design cadence. Room rhythm is recorded through
runs, scenes, beats, clocks, stuck states, hints, reveals, and reset effects.

## Version history

| Version | Date | State | Major change | Decision |
|---|---|---|---|---|
| v0.graph | TBD | Graphed | Added prototype timing model and team/behavior probes. | Run optimizer and simulate required team archetypes. |
| v0.1-timing | TBD | Graphed | Added P3/P4 acceleration paths to fit 30-minute prototype slow case. | Simulate confused, fighting, and family/mixed teams first; evaluate 45-minute staffed-hour expansion. |
| v0.catalog-retrofit | 2026-05-16 | Graphed | Added technique/device/kit IDs, crowd profiles, risk bands, and bench-test plan. | Bench-test powered/audio/fold-table devices before simulation promotion. |

## Simulation and playtest runs

| Run ID | Version | Date | Type | Team archetype | Personas/roles | Players | Finish time | Hints | Stuck states | Physical failures | Safety flags | Reset time | Memorable beat |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---:|---|

## Design pulse evidence

| Design pulse | Phase | Wave | Persona/team | Base skill tested | Result | Revision |
|---|---|---|---|---|---|---|

## Group-game evidence

| Run ID | Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|---|

## Team archetype evidence

Use `docs/team-testing.md`.

| Run ID | Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Severity | Operator response | Design revision |
|---|---|---|---|---|---|---|---|

## Behavior evidence

Use `docs/behavior-testing.md`.

| Run ID | Behavior | Team/persona | Scene/beat probe | Observable signal | Severity | Design response | Revision |
|---|---|---|---|---|---|---|---|

## Variability matrix

Use `components/RELIABILITY.md`.

| Team/behavior | Beat(s) stressed | What changes | Reliability issue IDs | Mitigation | Evidence needed |
|---|---|---|---|---|---|
| Speed/chaotic | P2, P5 | team toggles switches/spins radio before route is understood | REL-FP, REL-STATE | visible accepted state, detents, manual bypass | BT-0001-P2-H, BT-0001-P5-C |
| Accessibility-varied | P3, P5 | UV/audio channels may exclude players | REL-SENSORY | printed frequency equivalent and final transcript/visual success | BT-0001-P3-X, BT-0001-P5-X |
| Chaotic/family | P1, P3 | postcards and galley objects scatter or go missing | REL-LOSS, REL-SOUP, REL-RESET | route silhouettes, object count, duplicate set | BT-0001-P1-C, BT-0001-R |
| Forceful handling | P4 | table is leaned on, rotated wrong, or latch forced | REL-FORCE, REL-JAM | no-load stop, staff compass fallback, hinge/stop inspection | BT-0001-P4-H |
| Operator stress | all | 30-minute version has no slow-case buffer | REL-OPS, REL-RESET | acceleration cards and reset photos | full reset/late-team simulation |

## Chaos protocol

Use `components/RELIABILITY.md`.

| Probe ID | Beat/device | Probe action | Watch signal | Expected recovery | Status |
|---|---|---|---|---|---|
| CHAOS-RAPID | P2 `DEV-SWITCH-001` | rapid switch toggling | unsafe state or false UV trigger | fused low voltage and manual UV card | pending |
| CHAOS-DUMP | P1/P3 route and galley props | dump postcards/objects | missing card/object or search soup | count cards and silhouettes recover state | pending |
| CHAOS-FORCE | P4 `DEV-FOLD-001` | lean/force table | pinch, false bearing, latch damage | no-load stop and staff fallback | pending |
| CHAOS-GUESS | P5 `DEV-DIAL-001` | spin/tune before solve | false final broadcast | detent/index rejects or operator withholds playback | pending |

## Stuck-state log

| Run ID | Beat | Observable signal | Cause | Hint used | Revision |
|---|---|---|---|---|---|

## Beat timing evidence

| Run ID | Beat | Target min | Actual min | Hint at min | Hint used at min | Slow max min | Timing decision |
|---|---|---:|---:|---:|---:|---:|---|

## Physical and reset failures

| Run ID | Component/beat | Failure | Player impact | Operator recovery | Recovery time | In time? | Post-run fix | Template lesson? |
|---|---|---|---|---|---:|---|---|---|

## Reliability issue log

| Run/Test ID | Beat/device | Issue ID | Failure mode | Player impact | Operator detection | Bypass | Post-run fix | Result |
|---|---|---|---|---|---|---|---|---|
| BT-0001-P2-H | P2 breaker panel | REL-FP, REL-STATE | wrong/rapid switch state might appear accepted | UV path opens early or players distrust panel | switch/state light mismatch | manual UV trigger | simplify accepted-state indicator | Not run |
| BT-0001-P4-H | P4 fold table | REL-FORCE, REL-JAM | table can be leaned on or latch can stick | unsafe force or blocked bearing | visible table stress/latch bind | staff compass mark | stronger stop or replace with wall panel | Not run |
| BT-0001-P5-X | P5 radio/audio | REL-SENSORY | final success could depend on hearing | excluded player misses finale | replay/transcript request | visual tuned state and transcript | add non-audio reveal | Not run |

## Admin replacement drills

Use `components\RELIABILITY.md`.

| Test ID | Component | Criticality | Failure simulated | Spare/bypass used | Target replacement min | Actual replacement min | Result | Required change |
|---|---|---|---|---|---:|---:|---|---|
| AR-0001-P1-CARD | laminated postcard set | C4 required | route card missing/not found | duplicate route pouch | 1 | TBD | Not run | stage route pouch and count card |
| AR-0001-P2-SWITCH | breaker switch path | C4 required | correct switch sequence fails | manual UV trigger | 2 | TBD | Not run | verify operator control can fire in time |
| AR-0001-P3-OBJ | galley object | C4 required | object pocketed/not found | duplicate object set | 2 | TBD | Not run | label P3 pouch |
| AR-0001-P4-TABLE | fold table latch | C5 showstopper | latch jams or unsafe force starts | staff compass mark fallback | 1 | TBD | Not run | make bypass script in-world |
| AR-0001-P5-RADIO | radio knob/audio | C4 required | knob slips or audio fails | spare knob/operator playback/transcript | 2 | TBD | Not run | stage knob and transcript |

## Device bench tests

Use `components/DEVICE-REVIEW.md`. Passing these moves devices from L0 idea to
L1 cardboard.

| Test ID | Version | Device | Risk band | Crowd profile | Pass type | Required observation | Status | Promotion impact |
|---|---|---|---|---|---|---|---|---|
| BT-0001-P1-C | v0.catalog-retrofit | `DEV-TRAY-001` route rail/map | Green | helper | correct-use | route order becomes public, not private reading | pending | blocks P1 promotion |
| BT-0001-P2-H | v0.catalog-retrofit | `DEV-SWITCH-001` breaker panel | Orange | tactile helper | chaotic-use | random switching creates no unsafe state and bypass works | pending | blocks P2 promotion |
| BT-0001-P3-X | v0.catalog-retrofit | `DEV-LED-001` UV cabinet | Orange | discovery helper | accessibility | printed/visible equivalent preserves frequency solve | pending | blocks P3 promotion |
| BT-0001-P4-H | v0.catalog-retrofit | `DEV-FOLD-001` table | Yellow | transformation pleaser | chaotic-use | force/lean attempts create no pinch or false bearing | pending | blocks P4 promotion |
| BT-0001-P5-C | v0.catalog-retrofit | `DEV-DIAL-001` radio | Orange | finale pleaser | correct-use | visual tuned state and audio payoff align | pending | blocks P5 promotion |
| BT-0001-P5-X | v0.catalog-retrofit | `DEV-AUDIO-001` final broadcast | Orange | finale pleaser | accessibility | final success does not depend on hearing | pending | blocks P5 promotion |
| BT-0001-R | v0.catalog-retrofit | full reset kit | Yellow | helper | reset/transport | operator resets route, switches, objects, table, and radio within target | pending | blocks playtest-ready |

## Safety and accessibility observations

| Run ID | Observation | Severity | Mitigation | Blocks promotion? |
|---|---|---|---|---|

## Scores

Use `scoring/RUBRIC.md`.

| Version | Date | Rubric | Theme/story | Puzzle graph | Physical/reliability | Spatial flow | Purpose mapping | Safety/accessibility | Operator support | Throughput/timing | Group-game | Delight | Total | Top revision moves |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| v0.graph | TBD | v0.1 | 7 | 9 | 8 | 5 | not scored | 8 | 8 | 6 | 5 | 3 | 59 | Declare envelope; prototype P4 table; solve safety/accessibility alternatives. |
| v0.1-timing | TBD | v0.1 | 7 | 9 | 8 | 5 | not scored | 8 | 9 | 7 | 5 | 3 | 61 | Simulate team archetypes; validate P3/P4 acceleration does not flatten aha. |
| v0.catalog-retrofit | 2026-05-16 | v0.2 | 7 | 9 | 9 | 5 | 6 | 8 | 9 | 7 | 5 | 3 | 68 | Bench-test breaker, UV cabinet, fold table, radio/audio path, and reset kit before simulation promotion. |

## Surprise log

| Surprise | Evidence | Decision |
|---|---|---|
| Slow-case timing exceeded prototype target before tuning. | Optimizer reported 33-minute slow case for 30-minute target. | Added P3 frequency reveal and P4 service-notch bearing acceleration. |
| Trailer-native furniture can be a pleaser and a hazard. | P4 fold-down table is memorable but creates force/reach/load risks. | Treat transform surfaces as devices with bench tests, not scenic furniture. |

## Promotion decision

| Version | Current state | Next state | Gate result | Required changes before promotion |
|---|---|---|---|---|
| v0.1-timing | Graphed | Simulated | Pending | Run amazing, confused, social-friction, speed/chaos, trust/access, and operator stress simulations. |
| v0.catalog-retrofit | Graphed | Simulated | Not ready | Complete L1 bench tests for route rail, breaker, UV cabinet, fold table, radio/audio accessibility, and reset. |

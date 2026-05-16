# The Black Box at Mile Zero - Playtest

Use this file for simulations, prototype tests, and external playtests. Scores
are forward-only: keep old results and create a new entry when the room changes.

Design pulses belong to AMAZE's design cadence. Room rhythm is recorded through
runs, scenes, beats, clocks, stuck states, hints, reveals, and reset effects.

## Version history

| Version | Date | State | Major change | Decision |
|---|---|---|---|---|
| v0.theme | 2026-05-15 | Seed | User chose mystery trailer with a strong physical object at the center. | Co-design exact theme before build detail. |
| v0.theme-lock | 2026-05-15 | Seed | Locked black box as sealed roadside evidence recorder, players as roadside investigators, and final tone as eerie wonder. | Run first simulation matrix before adding more puzzle complexity. |
| v0.catalog-retrofit | 2026-05-16 | Seed | Added technique/device/kit IDs, crowd profiles, risk bands, and bench-test plan. | Prototype central box windows/latch before adding puzzle complexity. |

## Simulation and playtest runs

| Run ID | Version | Date | Type | Team archetype | Personas/roles | Players | Finish time | Hints | Stuck states | Physical failures | Safety flags | Reset time | Memorable beat |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---:|---|

## Design pulse evidence

| Design pulse | Phase | Wave | Persona/team | Base skill tested | Result | Revision |
|---|---|---|---|---|---|---|
| v0.theme | mechanical state | central-object promise | user/co-designer | theme fit | selected as 0002 seed | keep object identity editable |
| v0.theme-lock | mechanical state, route/spatial transform, audio/reveal | object-role-tone lock | user/co-designer | theme fit | recorder/investigator/eerie-road spine selected | simulate whether players feel awe rather than paperwork |

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
| Amazing/speed | P2, P5 | players attack central box before route reconstruction | REL-FP, REL-FORCE | witness-window state gate and no-force latch script | BT-0002-P2-H, BT-0002-P5-H |
| Confused/quiet | P1-P3 | evidence tags and recorder feel like paperwork/audio loop | REL-SOUP, REL-SENSORY | public evidence rail and transcript-equivalent route | BT-0002-P1-C, BT-0002-P3-X |
| Chaotic/family | P1, P2 | evidence tags scatter and sliders get shoved | REL-LOSS, REL-JAM, REL-RESET | tag count, slider stops, reset photos | BT-0002-P2-H, BT-0002-R |
| Accessibility-varied | P3 | audio recorder cannot be required | REL-SENSORY | transcript path is official and complete | BT-0002-P3-X |
| Operator stress | P4, P5 | cabinet/latch state must recover under pressure | REL-OPS, REL-STATE | manual cabinet card and staff latch release | BT-0002-P4-H, BT-0002-P5-H |

## Chaos protocol

Use `components/RELIABILITY.md`.

| Probe ID | Beat/device | Probe action | Watch signal | Expected recovery | Status |
|---|---|---|---|---|---|
| CHAOS-DUMP | P1 evidence rail | remove/scatter tags | route state impossible or hidden | count tags and use duplicate pouch | pending |
| CHAOS-FORCE | P2/P5 box windows/latches | shove sliders and pull handle | jam, pinch, or false open | stops survive; staff release if needed | pending |
| CHAOS-RAPID | P3/P4 recorder/cabinet | replay audio and toggle switches quickly | audio loop frustration or light mismatch | transcript path and manual cabinet card | pending |
| CHAOS-GUESS | P5 latch stack | attempt final opening early | box opens before reconstruction | latches reject and no-force script lands | pending |

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
| BT-0002-P2-H | P2 witness windows | REL-JAM, REL-STATE | slider sticks or state unclear | team distrusts central object | window state not visible/reset photo mismatch | operator sets first window | larger windows or simpler reveal | Not run |
| BT-0002-P3-X | P3 recorder | REL-SENSORY | audio skip required without equivalent | team cannot progress without hearing | repeated replay/transcript confusion | printed transcript-only path | rewrite transcript index | Not run |
| BT-0002-P5-H | P5 latch stack | REL-FORCE, REL-FP | handle/latches false-open or invite unsafe pulling | finale breaks early or unsafe | visible force/early movement | staff release/freeze play | redesign latch stops | Not run |

## Admin replacement drills

Use `components\RELIABILITY.md`.

| Test ID | Component | Criticality | Failure simulated | Spare/bypass used | Target replacement min | Actual replacement min | Result | Required change |
|---|---|---|---|---|---:|---:|---|---|
| AR-0002-P1-TAG | evidence tag set | C4 required | active tag missing/not found | duplicate tag pouch | 1 | TBD | Not run | stage tag pouch and count card |
| AR-0002-P2-WINDOW | witness window slider | C4 required | slider jams | operator sets first window/spare panel | 2 | TBD | Not run | mark service access |
| AR-0002-P3-TRANSCRIPT | transcript card set | C4 required | audio player fails or card missing | spare transcript set | 1 | TBD | Not run | keep transcript equivalent |
| AR-0002-P4-CABINET | utility cabinet power/switch | C4 required | light/switch state fails | manual cabinet card | 2 | TBD | Not run | stage manual card at cabinet |
| AR-0002-P5-LATCH | final latch stack | C5 showstopper | latch jams or handle forced | staff release/manual open | 1 | TBD | Not run | prove release visible to operator |

## Device bench tests

Use `components/DEVICE-REVIEW.md`. Passing these moves devices from L0 idea to
L1 cardboard.

| Test ID | Version | Device | Risk band | Crowd profile | Pass type | Required observation | Status | Promotion impact |
|---|---|---|---|---|---|---|---|---|
| BT-0002-P1-C | v0.catalog-retrofit | `DEV-TRAY-001` evidence rail | Green | helper | correct-use | evidence state is public and not search soup | pending | blocks P1 promotion |
| BT-0002-P2-C | v0.catalog-retrofit | `DEV-WINDOW-001` witness windows | Yellow | central-object pleaser | correct-use | players explain the changed box state | pending | blocks P2 promotion |
| BT-0002-P2-H | v0.catalog-retrofit | `DEV-WINDOW-001` witness windows | Yellow | central-object pleaser | chaotic-use | sliders do not jam, pinch, or false-activate evidence | pending | blocks P2 promotion |
| BT-0002-P3-X | v0.catalog-retrofit | `DEV-AUDIO-001` recorder | Orange | atmosphere pleaser | accessibility | transcript-only path gives route phrase/index fairly | pending | blocks P3 promotion |
| BT-0002-P4-H | v0.catalog-retrofit | `DEV-SWITCH-001` cabinet | Orange | tactile helper | chaotic-use | rapid/wrong switching creates no unsafe state and is recoverable | pending | blocks P4 promotion |
| BT-0002-P5-H | v0.catalog-retrofit | `DEV-LATCH-001` box reveal | Yellow, Orange if powered | finale pleaser | chaotic-use | early pulling creates no false open and no unsafe force path | pending | blocks P5 promotion |
| BT-0002-R | v0.catalog-retrofit | full reset kit | Yellow | helper | reset/transport | one operator resets tags, windows, recorder, cabinet, and box under 10 minutes | pending | blocks playtest-ready |

## Safety and accessibility observations

| Run ID | Observation | Severity | Mitigation | Blocks promotion? |
|---|---|---|---|---|

## Scores

Use `scoring/RUBRIC.md`.

| Version | Date | Rubric | Theme/story | Puzzle graph | Physical/reliability | Spatial flow | Purpose mapping | Safety/accessibility | Operator support | Throughput/timing | Group-game | Delight | Total | Top revision moves |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| v0.theme | 2026-05-15 | v0.1 | TBD | TBD | TBD | TBD | not scored | TBD | TBD | TBD | TBD | TBD | TBD | Score after user theme pass and first harness run. |
| v0.catalog-retrofit | 2026-05-16 | v0.2 | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | Score after first team matrix; catalog retrofit only adds review structure. |

## Surprise log

| Surprise | Evidence | Decision |
|---|---|---|
| Central-object rooms live or die by force affordance. | Black box windows and latch stack are both pleasers and force risks. | Bench-test no-force state before adding more evidence complexity. |

## Promotion decision

| Version | Current state | Next state | Gate result | Required changes before promotion |
|---|---|---|---|---|
| v0.theme | Seed | Graphed | Not ready | User theme pass, then simulate amazing/confused/fighting/accessibility-varied teams. |
| v0.catalog-retrofit | Seed | Graphed | Not ready | Complete L1 cardboard bench tests for witness windows, recorder transcript path, utility cabinet, latch stack, and reset. |


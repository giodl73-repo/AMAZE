# Astral Archive Overnight Playtest

## Version history

| Version | Date | State | Major change | Decision |
|---|---|---|---|---|
| v0.seed | 2026-05-16 | seed | initial 90-minute room pack | validate lint/check and 90-minute timing |

## Simulation and playtest runs

| Run ID | Version | Date | Type | Team archetype | Personas/roles | Players | Finish time | Hints | Stuck states | Physical failures | Safety flags | Reset time | Memorable beat |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---:|---|
| SIM-0008-A | v0.seed | 2026-05-16 | design simulation | Amazing team | systems thinker, map handler, wheel handler, story keeper, caller | 5 | TBD | TBD | may skip act recaps | none observed; design-only | fatigue not yet tested | TBD | orrery vault dawn coordinate |
| SIM-0008-C | v0.seed | 2026-05-16 | design simulation | Confused team | careful reader, hesitant handler, quiet observer, role follower | 4 | TBD | TBD | memory overload risk | none observed; design-only | fatigue not yet tested | TBD | act board external memory |

## Design pulse evidence

| Design pulse | Phase | Wave | Persona/team | Base skill tested | Result | Revision |
|---|---|---|---|---|---|---|
| v0.seed | uber-session | wave 1 | mixed team | pacing, memory, stamina | not run | tabletop acts separately |

## Learning ledger

| Learning | Evidence | Scope | Decision | Follow-up |
|---|---|---|---|---|
| 90-minute mobile rooms need act contracts, not just more puzzles. | design review | whole room | state cards and act breaks are required | test act-board comprehension |

## Group-game evidence

| Run ID | Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|---|
| SIM-0008-A | story keeper | recognition | keeps public state without becoming note-taker | P1-P10 | not-run | test card length |
| SIM-0008-F | wheel handler | agency | safely operates astrolabe/vault | P5/P10 | not-run | detent/guard prototype |
| SIM-0008-X | seated player | control | can call/check from rest points | all acts | not-run | floorplan mock |

## Team archetype evidence

| Run ID | Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Severity | Operator response | Design revision |
|---|---|---|---|---|---|---|---|
| SIM-0008-C | Confused team | memory overload | P4/P7/P10 | uses state cards instead of asking old facts | high | point to act board | shorten/clarify cards |
| SIM-0008-F | Fighting team | one-player dominance | full run/P9 | roles rotate at act breaks | high | assign new roles | split final controls |
| SIM-0008-O | Operator stress team | long reset/pause | full run/reset | staff preserves state and resets under 25 | high | act bins and pause script | color-code bins |

## Behavior evidence

| Run ID | Behavior | Team/persona | Scene/beat probe | Observable signal | Severity | Design response | Revision |
|---|---|---|---|---|---|---|---|
| SIM-0008-FAT | fatigue drift | any long-session team | P4/P7 | team accepts watch-change pause | high | act break and seating | test full 90 |
| SIM-0008-MEM | memory overload | Confused team | P7-P10 | state cards carry long arc | high | public state board | card rewrite |
| SIM-0008-COL | color/glare avoidance | Accessibility-varied team | P6 | high-contrast backup solves | high | work-light/printed marks | glare test |

## Variability matrix

| Team/behavior | Beat(s) stressed | What changes | Reliability issue IDs | Mitigation | Evidence needed |
|---|---|---|---|---|---|
| Fatigued/late group | full run | slower transitions, lower energy | REL-0008-FATIGUE | act breaks and accelerators | full 90-minute simulation |
| Confused/memory overload | P7-P10 | earlier facts forgotten | REL-0008-MEMORY | state cards | confused-team run |
| Chaotic/random guessing | P2/P5/P9 | drawer/wheel/relic force | REL-0008-FORCE | detents, no-force script | abuse test |
| Accessibility/color/glare | P6/P10 | work-light backup required | REL-0008-GLARE | high-contrast marks | glare test |

## Chaos protocol

| Probe ID | Beat/device | Probe action | Watch signal | Expected recovery | Status |
|---|---|---|---|---|---|
| CHAOS-0008-01 | full run | force a comfort-pause interruption after Act I | state preserved and team resumes | operator script works | not-run |
| CHAOS-0008-02 | P9/P10 vault | wrong relic and light force | no seating, no damage | manual vault path works | not-run |
| CHAOS-0008-03 | reset | mix one tile into wrong act bin | staff catches count/photo mismatch | bin labels recover | not-run |
| CHAOS-0008-04 | P6 filters | run under glare/work-light conditions | high-contrast marks remain readable | printed backup works | not-run |

## Stuck-state log

| Run ID | Beat | Observable signal | Cause | Hint used | Revision |
|---|---|---|---|---|---|
| TBD | P1 | TBD | TBD | TBD | TBD |

## Beat timing evidence

| Run ID | Beat | Target min | Actual min | Hint at min | Hint used at min | Slow max min | Timing decision |
|---|---|---:|---:|---:|---:|---:|---|
| model | P1 | 5 | TBD | 4 | TBD | 7 | keep |
| model | P2 | 7 | TBD | 6 | TBD | 9 | keep |
| model | P3 | 7 | TBD | 6 | TBD | 9 | keep |
| model | P4 | 5 | TBD | 4 | TBD | 7 | keep |
| model | P5 | 7 | TBD | 6 | TBD | 9 | keep |
| model | P6 | 8 | TBD | 7 | TBD | 10 | keep |
| model | P7 | 8 | TBD | 7 | TBD | 10 | keep |
| model | P8 | 8 | TBD | 7 | TBD | 10 | keep |
| model | P9 | 7 | TBD | 6 | TBD | 8 | keep |
| model | P10 | 7 | TBD | 6 | TBD | 9 | keep |

## Physical and reset failures

| Run ID | Component/beat | Failure | Player impact | Operator recovery | Recovery time | In time? | Post-run fix | Template lesson? |
|---|---|---|---|---|---:|---|---|---|
| TBD | P10 orrery vault | TBD | final blocked | manual vault card | TBD | TBD | TBD | TBD |

## Reliability issue log

| Run/Test ID | Beat/device | Issue ID | Failure mode | Player impact | Operator detection | Bypass | Post-run fix | Result |
|---|---|---|---|---|---|---|---|---|
| BENCH-0008-01 | full run | REL-0008-FATIGUE | fatigue/drift | unsafe or boring long run | operator observation | watch-change pause | adjust act pacing | not-run |
| BENCH-0008-02 | act board | REL-0008-MEMORY | state card overload | unfair final memory | repeated old-question asks | operator recap | rewrite cards | not-run |
| BENCH-0008-03 | orrery vault | REL-0008-VAULT | socket/wheel jam | final blocked | relic/wheel stuck | manual vault card | tolerance fix | not-run |

## Admin replacement drills

| Test ID | Component | Criticality | Failure simulated | Spare/bypass used | Target replacement min | Actual replacement min | Result | Required change |
|---|---|---|---|---|---:|---:|---|---|
| ADMIN-0008-01 | orrery vault | C5 showstopper | final wheel/socket jam | manual vault card | 5 | TBD | not-run | write timed drill |
| ADMIN-0008-02 | act board/state cards | C4 required | state card lost | duplicate act pack | 2 | TBD | not-run | duplicate binder |
| ADMIN-0008-03 | spectrum filter rail | C4 required | glare/filter loss | printed high-contrast card | 3 | TBD | not-run | backup card pack |

## Device bench tests

| Test ID | Version | Device | Risk band | Crowd profile | Pass type | Required observation | Status | Follow-up |
|---|---|---|---|---|---|---|---|---|
| BENCH-0008-01 | v0.seed | orrery vault | high | chaotic/operator stress | abuse and recovery | wrong relic fails safely; manual vault under 5 minutes | not-run | cardboard vault |
| BENCH-0008-02 | v0.seed | act board/state cards | high | confused/fatigued | comprehension | team uses cards at P7/P10 without memory overload | not-run | text rewrite |
| BENCH-0008-03 | v0.seed | spectrum filters | medium-high | accessibility-varied | readability | high-contrast backup works under glare | not-run | work-light test |
| BENCH-0008-04 | v0.seed | reset bins | high | operator stress | reset | 25-minute reset catches one mixed item | not-run | bin labels/photos |

## Safety and accessibility observations

| Run ID | Observation | Severity | Mitigation | Blocks promotion? |
|---|---|---|---|---|
| design | 90-minute fatigue/temperature/restroom risk | high | act breaks, comfort-pause policy, temp checks | yes before paid play |
| design | final vault crowding/pinch risk | high | floor marks, guard ring, manual release | yes before build readiness |
| design | memory load risk | high | state cards and act board | yes before playtest |

## Scores

| Version | Date | Rubric | Theme/story | Puzzle graph | Physical/reliability | Spatial flow | Purpose mapping | Safety/accessibility | Operator support | Throughput/timing | Group-game | Delight | Total | Top revision moves |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| v0.seed | 2026-05-16 | AMAZE v0 | 4 | 4 | 2 | 2 | 3 | 2 | 3 | 2 | 3 | 4 | 33 | prototype vault, write pause policy, test state cards |

## Surprise log

| Surprise | Evidence | Decision |
|---|---|---|
| The hardest 90-minute puzzle is memory/stamina, not clue count. | design review | make act board a required component |

## Promotion decision

| Version | Current state | Next state | Gate result | Required changes before promotion |
|---|---|---|---|---|
| v0.seed | Seed | Concept | pending | pass lint/check, optimize for 90, validate comfort-pause policy |

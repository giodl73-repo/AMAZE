# Stormglass Station Playtest

## Version history

| Version | Date | State | Major change | Decision |
|---|---|---|---|---|
| v0.seed | 2026-05-16 | seed | initial full room pack | run lint/check and tabletop simulation |

## Simulation and playtest runs

| Run ID | Version | Date | Type | Team archetype | Personas/roles | Players | Finish time | Hints | Stuck states | Physical failures | Safety flags | Reset time | Memorable beat |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---:|---|
| SIM-0006-A | v0.seed | 2026-05-16 | design simulation | Amazing team | systems thinker, dial handler, mapper, caller | 4 | TBD | TBD | speedrun risk on P2-P4 | none observed; design-only | none observed; design-only | TBD | stormglass console route reveal |
| SIM-0006-C | v0.seed | 2026-05-16 | design simulation | Confused team | careful reader, nervous handler, quiet observer | 3 | TBD | TBD | category confusion at P1 | none observed; design-only | none observed; design-only | TBD | station checklist names three readings |

## Design pulse evidence

| Design pulse | Phase | Wave | Persona/team | Base skill tested | Result | Revision |
|---|---|---|---|---|---|---|
| v0.seed | instrument convergence | wave 1 | mixed team | align/sort/fit | not run | build cardboard console |

## Learning ledger

| Learning | Evidence | Scope | Decision | Follow-up |
|---|---|---|---|---|
| Final proof sockets are the riskiest part. | build review | P5 | prototype first | bench socket tolerances |

## Group-game evidence

| Run ID | Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|---|
| SIM-0006-A | caller | control | coordinates final route without taking props | P5 | not-run | test role ritual |
| SIM-0006-C | quiet observer | recognition | can notice tide alignment | P4 | not-run | high-contrast overlay |

## Team archetype evidence

| Run ID | Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Severity | Operator response | Design revision |
|---|---|---|---|---|---|---|---|
| SIM-0006-C | Confused team | may not see three required readings | P1 | names pressure/wind/tide before instrument work | medium | point to checklist | strengthen labels |
| SIM-0006-F | Fighting team | one player may own final | P5 | caller/checker/handler roles emerge | medium | assign roles | split controls |

## Behavior evidence

| Run ID | Behavior | Team/persona | Scene/beat probe | Observable signal | Severity | Design response | Revision |
|---|---|---|---|---|---|---|---|
| SIM-0006-X | low-light/glare avoidance | Accessibility-varied team | P4 | reads tide overlay under work light | high | matte high-contrast overlay | prototype glare test |
| SIM-0006-Q | random guessing | Chaotic team | P5 | no beacon action before proofs | high | proof shutter | socket/cover test |

## Variability matrix

| Team/behavior | Beat(s) stressed | What changes | Reliability issue IDs | Mitigation | Evidence needed |
|---|---|---|---|---|---|
| Confused/over-searching | P1-P2 | more time at role frame | REL-0006-ROLE | checklist and hint | tabletop run |
| Chaotic/random guessing | P5 | more force on sockets/buttons | REL-0006-SOCKET | durable sockets and proof lock | abuse test |
| Accessibility/glare | P4 | work-light route required | REL-0006-GLARE | matte overlay | glare test |

## Chaos protocol

| Probe ID | Beat/device | Probe action | Watch signal | Expected recovery | Status |
|---|---|---|---|---|---|
| CHAOS-0006-01 | P3 wind cards | remove one card during reset drill | staff catches count mismatch | duplicate card pouch restores | not-run |
| CHAOS-0006-02 | P5 proof console | try wrong cartridge and light force | no seating, no damage | operator says proof mismatch | not-run |
| CHAOS-0006-03 | P4 tide overlay | shine work light at angle | marks still readable | matte overlay if glare | not-run |

## Stuck-state log

| Run ID | Beat | Observable signal | Cause | Hint used | Revision |
|---|---|---|---|---|---|
| TBD | P1 | TBD | TBD | TBD | TBD |

## Beat timing evidence

| Run ID | Beat | Target min | Actual min | Hint at min | Hint used at min | Slow max min | Timing decision |
|---|---|---:|---:|---:|---:|---:|---|
| model | P1 | 4 | TBD | 2 | TBD | 6 | keep |
| model | P2 | 6 | TBD | 4 | TBD | 8 | keep |
| model | P3 | 6 | TBD | 4 | TBD | 8 | keep |
| model | P4 | 7 | TBD | 5 | TBD | 9 | keep |
| model | P5 | 7 | TBD | 5 | TBD | 9 | keep |

## Physical and reset failures

| Run ID | Component/beat | Failure | Player impact | Operator recovery | Recovery time | In time? | Post-run fix | Template lesson? |
|---|---|---|---|---|---:|---|---|---|
| TBD | P5 console | TBD | TBD | manual route card | TBD | TBD | TBD | TBD |

## Reliability issue log

| Run/Test ID | Beat/device | Issue ID | Failure mode | Player impact | Operator detection | Bypass | Post-run fix | Result |
|---|---|---|---|---|---|---|---|---|
| BENCH-0006-01 | P5 console | REL-0006-SOCKET | socket jam | final blocked | cartridge fails to seat | manual route card | adjust tolerance | not-run |
| BENCH-0006-02 | P4 tide | REL-0006-GLARE | overlay glare | clue unreadable | player avoids overlay | work light/printed answer | matte overlay | not-run |

## Admin replacement drills

| Test ID | Component | Criticality | Failure simulated | Spare/bypass used | Target replacement min | Actual replacement min | Result | Required change |
|---|---|---|---|---|---:|---:|---|---|
| ADMIN-0006-01 | stormglass proof console | C5 showstopper | shutter/socket jam | manual route card | 2 | TBD | not-run | write timed drill |
| ADMIN-0006-02 | tide overlay/cartridge | C4 required | overlay scratched | spare overlay and staff cartridge | 3 | TBD | not-run | package spare pouch |

## Device bench tests

| Test ID | Version | Device | Risk band | Crowd profile | Pass type | Required observation | Status | Follow-up |
|---|---|---|---|---|---|---|---|---|
| BENCH-0006-01 | v0.seed | proof socket console | high | chaotic/operator stress | abuse and recovery | wrong cartridge does not seat; earned cartridge seats; manual route works | not-run | cardboard socket |
| BENCH-0006-02 | v0.seed | tide gauge overlay | medium | accessibility-varied | readability | correct tide readable under work light without glare | not-run | matte prototype |
| BENCH-0006-03 | v0.seed | wind vane rack | medium | confused/fighting | reset and clue clarity | cards reset by count and arrow order is readable | not-run | icon backs |

## Safety and accessibility observations

| Run ID | Observation | Severity | Mitigation | Blocks promotion? |
|---|---|---|---|---|
| design | acrylic edge/glare risk | medium | rounded edges and matte overlay | yes before build readiness |
| design | final console crowding | medium | role ritual and three-sided access | yes before playtest |

## Scores

| Version | Date | Rubric | Theme/story | Puzzle graph | Physical/reliability | Spatial flow | Purpose mapping | Safety/accessibility | Operator support | Throughput/timing | Group-game | Delight | Total | Top revision moves |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| v0.seed | 2026-05-16 | AMAZE v0 | 3 | 3 | 2 | 3 | 3 | 3 | 3 | 3 | 3 | 4 | 33 | prototype console, glare test, reset drill |

## Surprise log

| Surprise | Evidence | Decision |
|---|---|---|
| Stormglass can be a tactile science fantasy without real glass. | design review | use acrylic/print, not fragile glass |

## Promotion decision

| Version | Current state | Next state | Gate result | Required changes before promotion |
|---|---|---|---|---|
| v0.seed | Seed | Concept | pending | pass lint/check, optimize timing, bench console mock |

# Clockwork Greenhouse Playtest

## Version history

| Version | Date | State | Major change | Decision |
|---|---|---|---|---|
| v0.seed | 2026-05-16 | seed | initial full room pack | run lint/check and tabletop simulation |

## Simulation and playtest runs

| Run ID | Version | Date | Type | Team archetype | Personas/roles | Players | Finish time | Hints | Stuck states | Physical failures | Safety flags | Reset time | Memorable beat |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---:|---|
| SIM-0007-A | v0.seed | 2026-05-16 | design simulation | Amazing team | systems thinker, tray sorter, valve handler, caller | 4 | TBD | TBD | speedrun risk on P2-P4 | none observed; design-only | none observed; design-only | TBD | bloom wheel reveal |
| SIM-0007-C | v0.seed | 2026-05-16 | design simulation | Confused team | careful reader, hesitant handler, quiet observer | 3 | TBD | TBD | system confusion at P1 | none observed; design-only | none observed; design-only | TBD | proof rail names root/water/light/time |

## Design pulse evidence

| Design pulse | Phase | Wave | Persona/team | Base skill tested | Result | Revision |
|---|---|---|---|---|---|---|
| v0.seed | growth-system convergence | wave 1 | mixed team | sort/align/verify | not run | build cardboard wheel |

## Learning ledger

| Learning | Evidence | Scope | Decision | Follow-up |
|---|---|---|---|---|
| Bloom wheel is the build-risk center. | design review | P5 | prototype first | bench wheel tolerances |

## Group-game evidence

| Run ID | Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|---|
| SIM-0007-A | caller | control | coordinates bloom without taking all props | P5 | not-run | role ritual |
| SIM-0007-C | tray sorter | agency | solves seed/soil physically | P2 | not-run | oversized tokens |

## Team archetype evidence

| Run ID | Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Severity | Operator response | Design revision |
|---|---|---|---|---|---|---|---|
| SIM-0007-C | Confused team | may miss four-system frame | P1 | names systems before station work | medium | point to proof rail | strengthen silhouettes |
| SIM-0007-F | Fighting team | one player may own final | P5 | caller/checker/handler roles emerge | medium | assign roles | split wheel controls |

## Behavior evidence

| Run ID | Behavior | Team/persona | Scene/beat probe | Observable signal | Severity | Design response | Revision |
|---|---|---|---|---|---|---|---|
| SIM-0007-X | low-light/glare avoidance | Accessibility-varied team | P4 | prism readable under work light | high | matte high-contrast panel | glare test |
| SIM-0007-Q | random guessing | Chaotic team | P3/P5 | no random valve/wheel solve | high | detents/proof lock | abuse test |

## Variability matrix

| Team/behavior | Beat(s) stressed | What changes | Reliability issue IDs | Mitigation | Evidence needed |
|---|---|---|---|---|---|
| Confused/over-searching | P1-P2 | more time at role frame | REL-0007-ROLE | proof rail silhouettes | tabletop run |
| Chaotic/random guessing | P3/P5 | more force on valves/wheel | REL-0007-WHEEL | detents and guard ring | abuse test |
| Accessibility/glare | P4 | work-light route required | REL-0007-GLARE | matte panel | glare test |

## Chaos protocol

| Probe ID | Beat/device | Probe action | Watch signal | Expected recovery | Status |
|---|---|---|---|---|---|
| CHAOS-0007-01 | P2 seed tray | remove one seed token during reset drill | staff catches count mismatch | duplicate pouch restores | not-run |
| CHAOS-0007-02 | P5 bloom wheel | try wrong token and light force | no seating, no damage | operator says proof mismatch | not-run |
| CHAOS-0007-03 | P4 prism panel | test glare/work-light angle | marks remain readable | matte revision if not | not-run |

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
| TBD | P5 bloom wheel | TBD | TBD | manual bloom card | TBD | TBD | TBD | TBD |

## Reliability issue log

| Run/Test ID | Beat/device | Issue ID | Failure mode | Player impact | Operator detection | Bypass | Post-run fix | Result |
|---|---|---|---|---|---|---|---|---|
| BENCH-0007-01 | P5 bloom wheel | REL-0007-WHEEL | wheel jam/pinch | final blocked | wheel/tokens fail | manual bloom card | adjust tolerances/guard | not-run |
| BENCH-0007-02 | P4 prism | REL-0007-GLARE | glare | clue unreadable | player avoids panel | work light/printed card | matte overlay | not-run |

## Admin replacement drills

| Test ID | Component | Criticality | Failure simulated | Spare/bypass used | Target replacement min | Actual replacement min | Result | Required change |
|---|---|---|---|---|---:|---:|---|---|
| ADMIN-0007-01 | bloom wheel | C5 showstopper | wheel jam | manual bloom card | 2 | TBD | not-run | write timed drill |
| ADMIN-0007-02 | seed tray/token set | C4 required | token missing | duplicate token pouch | 2 | TBD | not-run | package larger tokens |

## Device bench tests

| Test ID | Version | Device | Risk band | Crowd profile | Pass type | Required observation | Status | Follow-up |
|---|---|---|---|---|---|---|---|---|
| BENCH-0007-01 | v0.seed | bloom wheel | high | chaotic/operator stress | abuse and recovery | wrong token does not seat; manual bloom works | not-run | cardboard wheel |
| BENCH-0007-02 | v0.seed | prism shade panel | medium | accessibility-varied | readability | true mark readable under work light | not-run | matte prototype |
| BENCH-0007-03 | v0.seed | seed tray | medium | family/confused | reset and clarity | token count survives reset and texture match is clear | not-run | oversize tokens |

## Safety and accessibility observations

| Run ID | Observation | Severity | Mitigation | Blocks promotion? |
|---|---|---|---|---|
| design | bloom wheel pinch risk | high | finger gap, guard ring, manual release | yes before build readiness |
| design | prism glare risk | medium | matte high-contrast work-light path | yes before playtest |

## Scores

| Version | Date | Rubric | Theme/story | Puzzle graph | Physical/reliability | Spatial flow | Purpose mapping | Safety/accessibility | Operator support | Throughput/timing | Group-game | Delight | Total | Top revision moves |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| v0.seed | 2026-05-16 | AMAZE v0 | 4 | 3 | 2 | 3 | 3 | 3 | 3 | 3 | 3 | 4 | 34 | prototype wheel, glare test, token sizing |

## Surprise log

| Surprise | Evidence | Decision |
|---|---|---|
| Cozy botanical theme still supports real physical stakes. | design review | keep no-scent/no-soil safety constraint |

## Promotion decision

| Version | Current state | Next state | Gate result | Required changes before promotion |
|---|---|---|---|---|
| v0.seed | Seed | Concept | pending | pass lint/check, optimize timing, bench bloom wheel mock |

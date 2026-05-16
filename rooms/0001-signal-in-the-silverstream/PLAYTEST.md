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

## Stuck-state log

| Run ID | Beat | Observable signal | Cause | Hint used | Revision |
|---|---|---|---|---|---|

## Beat timing evidence

| Run ID | Beat | Target min | Actual min | Hint at min | Hint used at min | Slow max min | Timing decision |
|---|---|---:|---:|---:|---:|---:|---|

## Physical and reset failures

| Run ID | Component/beat | Failure | Player impact | Operator recovery | Post-run fix | Template lesson? |
|---|---|---|---|---|---|---|

## Safety and accessibility observations

| Run ID | Observation | Severity | Mitigation | Blocks promotion? |
|---|---|---|---|---|

## Scores

Use `scoring/RUBRIC.md`.

| Version | Date | Rubric | Theme/story | Puzzle graph | Physical/reliability | Spatial flow | Purpose mapping | Safety/accessibility | Operator support | Throughput/timing | Group-game | Delight | Total | Top revision moves |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| v0.graph | TBD | v0.1 | 7 | 9 | 8 | 5 | not scored | 8 | 8 | 6 | 5 | 3 | 59 | Declare envelope; prototype P4 table; solve safety/accessibility alternatives. |
| v0.1-timing | TBD | v0.1 | 7 | 9 | 8 | 5 | not scored | 8 | 9 | 7 | 5 | 3 | 61 | Simulate team archetypes; validate P3/P4 acceleration does not flatten aha. |

## Surprise log

| Surprise | Evidence | Decision |
|---|---|---|
| Slow-case timing exceeded prototype target before tuning. | Optimizer reported 33-minute slow case for 30-minute target. | Added P3 frequency reveal and P4 service-notch bearing acceleration. |

## Promotion decision

| Version | Current state | Next state | Gate result | Required changes before promotion |
|---|---|---|---|---|
| v0.1-timing | Graphed | Simulated | Pending | Run amazing, confused, social-friction, speed/chaos, trust/access, and operator stress simulations. |

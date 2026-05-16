# Scenes Template

Use `docs/scene-beat-harness.md` to complete this file.

## Party assumptions

| Team archetype | Personas/roles | Notes |
|---|---|---|

Required coverage comes from `docs/team-testing.md`: amazing, confused, one
social-friction archetype, one speed/chaos archetype, one trust/accessibility
archetype, and one operator stress pass.

## Team archetype probes

Use `docs/team-testing.md`.

| Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Operator response | Design revision |
|---|---|---|---|---|---|

## Group-game stakes

| Persona/role | Axis | Stake | Evidence beats | State | Revision |
|---|---|---|---|---|---|

## Behavior probes

Use `docs/behavior-testing.md`.

| Behavior | Team/persona | Scene/beat probe | Observable signal | Risk if unsupported | Design response |
|---|---|---|---|---|---|

## Scene list

| Scene | Purpose | Entry condition | Exit condition | Clock | Team/behavior probes |
|---|---|---|---|---|---|

## Beat cards

| Scene | Beat | Player action | Aha | Check | Behavior probe | Target min | Hint at min | Slow max min | Mechanism | Reliability risk | DC | Success | Partial | Stall/hint trigger | Reset effect |
|---|---|---|---|---|---|---:|---:|---:|---|---|---|---|---|---|---|

## Transformation states

Use this table for every visible object, prop, cabinet, wall, light state, or
room phase that changes because of player progress. `From state` and `To state`
describe what players can see/touch, while `Trigger` and `Failure/bypass`
separate design intent from build implementation.

| ID | Beat | Object/space | From state | Trigger | To state | Visible proof | Reset state | Failure/bypass |
|---|---|---|---|---|---|---|---|---|

## Unlock paths

Use this table to test whether connected tissue still works when one team opens
an act quickly and another reaches the same unlock late. `Beats` accepts lists
or ranges such as `P2-P4, P7`; `simulate` reports fast and slow unlock minutes.

| Path | Beats | Unlocks | Fast coherence | Slow coherence | Operator acceleration |
|---|---|---|---|---|---|

## Session timing model

Use the target session declared in `BRIEF.md`. See
`docs/session-duration.md` for 30-minute trailer, 45-minute staffed-hour, and
60-minute standard profiles.

| Timing field | Meaning |
|---|---|
| Target min | Expected solve time for a healthy run of this beat. |
| Hint at min | Minute within the beat when the operator should intervene if no progress is visible. |
| Slow max min | Maximum acceptable beat duration before acceleration or bypass. |

The sum of `Target min` should leave buffer for intro, transitions, final
payoff, human variance, and hard exit. Use the Rust harness optimizer before
promoting a room graph.

## Operator interventions

| Trigger | Intervention | Fiction-preserving phrasing | Escalation |
|---|---|---|---|

## Consequence log

| Beat | Consequence | Affected clock | Operator visibility | Revision |
|---|---|---|---|---|

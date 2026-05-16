# Signal in the Silverstream - Operations

## Staffing

- Operator count: 1.
- Required training: intro script, hint ladder, manual UV trigger, table bypass, final audio playback, emergency extraction.
- Monitoring position: outside trailer with audio/video monitoring and manual override controls.

## Run clock

- Intro: 2 minutes.
- Soft-warning: 20 minutes.
- Hard-exit: 30 minutes for prototype; 45 minutes for staffed-hour profile.
- Reset: 7 minutes prototype; 10-15 minutes staffed-hour turnover.

## Hint protocol

| Beat/stuck state | Observable signal | Hint 1: point attention | Hint 2: clarify relationship | Acceleration |
|---|---|---|---|---|
| P1 route fragments | team treats postcards as souvenirs for 3-4 minutes | "Dispatch says the previous crew always logged stops." | "The stops are in driving order." | name first stop |
| P2 random switch flipping | repeated switch attempts without route reference | "The trailer only likes power in trip order." | "Match the switch labels to the route stops." | confirm first switch |
| P3 unrelated search | team searches storage after UV powers or stalls after first digit | "The galley inventory looks different under repair light." | "What is missing matters more than what is present." | reveal one frequency digit |
| P4 table treated as furniture | table ignored or argued over after frequencies | "That table has more travel in it than it should." | "The compass mark changes how the dial should be read." | show compass mark, then service-notch bearing mark |
| P5 data remains separate | route/power/frequency/bearing not combined by hard-exit minus 3 | "The broadcast is trying to lock onto a bearing." | "Every repaired system is part of the same trip." | provide final alignment card |

## NPC and operator characters

| NPC/voice | Function | Trigger | Delivery mode | Sample line | Limits | Reset/ops burden |
|---|---|---|---|---|---|---|
| Dispatch | onboarding, hints, final broadcast recovery | P1 route confusion, P5 convergence, audio failure | live operator voice or radio playback | "Dispatch says the previous crew always logged stops." | does not name route order unless accelerating | script card and backup audio ready |
| Repair Light | diegetic confirmation and access hint | P2/P3 power state, UV failure, unrelated search | light cue plus operator line | "The repair light is stable; compare what changed." | does not reveal all frequency digits | UV test/reset and manual trigger |
| Roadside Safety Stop | safety break and anxious-player support | exit trust issue, volume distress, hard stop request | direct staff voice outside fiction | "Safety stop. The trailer is stable." | safety overrides story immediately | pause/extraction log |

## Multi-room operator rotation

| Scope | Coverage model | Max rooms | Safe when | Unsafe when | Handoff signal | Dedicated staff trigger |
|---|---|---:|---|---|---|---|
| Prototype trailer pair | roving operator with camera/audio wall | 2 | both rooms are before final broadcast and egress views are live | anxious player, final audio, manual UV/table/radio bypass, or reset | green/yellow/red room state card | safety stop, audio distress, or manual final playback |
| Escape-room building cluster | central desk plus hall runner | 3 | rooms are staggered and no more than one final is active | camera/audio blind spot, overlapping hard exits, or no hall runner | desk log with next hint time | egress issue, player distress, or stuck latch |

## Reset checklist

| Step | Action | Verify |
|---|---|---|
| 1 | collect and shuffle postcards | full postcard set present and unmarked |
| 2 | reset switch panel | all switches at start state |
| 3 | reset UV cabinet | UV test passes and object inventory complete |
| 4 | reset table | latch/orientation at start mark |
| 5 | reset radio | dial/audio module at start state |
| 6 | clear operator notes | manual bypasses reset and ready |

## Failure modes

| Failure | Player-facing recovery | Staff action | Post-session fix |
|---|---|---|---|
| UV does not trigger after correct switches | "The repair light is flickering; dispatch can stabilize it." | manual UV trigger or reveal backup labels | inspect panel/UV strip |
| galley object missing | "One inventory line was already checked by the last crew." | reveal one digit or replace from spare set | restore object set |
| table latch jams | "The table is stuck between bearings; use the service mark." | point to service-notch bearing mark or backup bearing clue | inspect/tighten latch |
| radio audio misfires | "The signal is coming through dispatch instead." | play final broadcast manually | inspect audio module/knob |
| anxious player needs pause/exit | "Safety stop. The trailer is stable." | pause, lights/volume up, extract if needed | log trigger and revise |

## Transport and maintenance

| Item | Before transport | After transport | Routine maintenance |
|---|---|---|---|
| postcards/map | pouch cards, cover map | inspect count and marks | wipe/replace worn cards |
| switch panel | secure caps | test each switch | tighten labels/caps |
| UV cabinet | stow objects, protect strip | test reveal | replace strip if flicker |
| table | lock for travel | verify latch/orientation | inspect force wear |
| radio | secure knob/module | test playback | replace loose knob |

## Operator stress tests

| Scenario | Expected staff action | Hard limit | Revision if failed |
|---|---|---|---|
| confused team misses P1 | attention-first route hint | 6 minutes on P1 | strengthen onboarding artifact |
| speed team brute-forces P5 | redirect to missing system relationship | no final bypass before P4 | add gating or confirmation state |
| UV failure | manual trigger or backup labels | 1 minute recovery | separate logic from circuit |
| anxious participant at final audio | pause/volume down/exit reassurance | immediate response | add visible pause/exit language |
| messy reset | use checklist and inventory cards | 7 minutes | reduce movable objects or add storage fixtures |

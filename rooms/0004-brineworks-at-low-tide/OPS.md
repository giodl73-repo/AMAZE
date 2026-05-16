# The Brineworks at Low Tide - Operations

## Staffing

- Operator count: one operator for simulation and prototype.
- Required training: intro/safety script, category-state tracking, no-force prop script, final griddle bypass, reset checklist.
- Monitoring position: outside trailer or protected nook with views of door, pressure griddle, proof rail, and high-touch stations.

## Run clock

- Intro: 3-5 minutes.
- Soft-warning: 35 minutes.
- Hard-exit: 45 minutes.
- Reset: 10 minutes target.

## Operator category-state map

| Category | Player-facing proof | Operator-visible state | Acceleration | Blocks final griddle? |
|---|---|---|---|---|
| Order | service ticket released from order rail | ticket in proof rail or griddle socket | demonstrate one rejected tile and seat staff ticket | yes |
| Current | blue current gear released from pipe board | gear in proof rail or griddle socket | set one elbow and seat staff gear | yes |
| Bell | shell tag revealed when pearl bell closes clam register without force | tag in proof rail or griddle socket | demonstrate wrong bell failing pearl cup and seat staff tag | yes |
| Final | all three proofs seated; gauge/open token released | sockets/gauge visible | staff release/open token | no, if bypass needed |

## Hint protocol

| Beat/stuck state | Observable signal | Hint 1: point attention | Hint 2: clarify relationship | Acceleration |
|---|---|---|---|---|
| P1 category confusion | players search before naming jobs | "The shift card has three jobs." | "Order, current, bell: fix one at a time." | name three categories |
| P2 tile chaos | tiles scattered or color-only guessing | "The rail wants a stack, not soup." | "Match the edge shapes before the colors." | show one wrong tile rejection |
| P3 pipe stall | random elbow turning | "The tide follows arrows." | "Start at the intake and make blue reach the griddle." | set one elbow |
| P4 bell/audio stall | players listen, ring repeatedly, or force shell | "The clam votes with its hinge." | "Test which bell fits the pearl cup without force." | demonstrate one wrong bell failing to fit |
| P5 griddle guessing | random token/socket attempts | "The griddle only opens for proven service." | "Seat one ticket, one current, one bell." | seat staff proof for proven category |

## NPC and operator characters

| NPC/voice | Function | Trigger | Delivery mode | Sample line | Limits | Reset/ops burden |
|---|---|---|---|---|---|---|
| Canteen Steward | onboarding, category hints, final proof protocol | P1 service-job confusion, P5 griddle guessing | live operator voice or order-window card | "The lunch rush has three jobs." | does not name all proofs unless accelerating | shift-card script and proof-state checklist |
| Clam Register | tactile/no-force reminder and bell hint | P4 audio focus, force on clam, accessibility issue | prop label plus operator line | "The clam votes with its hinge, not its music." | does not make sound a required clue | bell tokens counted and cup reset |
| Griddle Captain | final bypass and hard-exit voice | P5 socket jam, late group, token missing | live operator voice plus manual open token | "The griddle accepts the proof." | cannot release open token without proof-state verification | staff proof set and open token ready |

## Multi-room operator rotation

| Scope | Coverage model | Max rooms | Safe when | Unsafe when | Handoff signal | Dedicated staff trigger |
|---|---|---:|---|---|---|---|
| Snack-room trailer pair | roving operator with proof-rail/griddle camera | 2 | both rooms have clear proof-state visibility and no final griddle active | griddle/socket issue, tile dump, access support, or reset | service-state board and griddle light | manual open token, player force, or egress concern |
| Escape-room building cluster | central desk plus hall runner | 3 | rooms are staggered and each griddle has camera/audio | overlapping griddle finales or no runner | desk log with order/current/bell states | stuck griddle, player distress, or accessibility support |

## Reset checklist

| Step | Action | Verify |
|---|---|---|
| 1 | Reset shift card, category plaques, and visible exit path. | opening photo matches |
| 2 | Return ingredient tiles and service ticket. | tile count complete; ticket in rail |
| 3 | Reset pipe elbows and current gear. | pipe-board photo matches |
| 4 | Reset bell tokens, clam register pearl cup, and shell tag. | token count complete; cup empty; clam open/tag hidden |
| 5 | Reset proof rail. | all three homes empty/visible |
| 6 | Reset pressure griddle sockets, gauge, open token, and dry bubble effect. | sockets empty; open token hidden |
| 7 | Clear egress and inspect loose props. | floor/door path clear |

## Reset timing budget

| Reset segment | Target min | Stop/fix if over | Likely simplification |
|---|---:|---|---|
| Shift card and plaques | 0.5 | 1.0 | fixed plaques |
| Ingredient tiles and rail | 2.0 | 3.0 | fewer tiles, larger bin labels |
| Pipe board and gear | 2.0 | 3.0 | fewer elbows, larger start-state marks |
| Clam register, pearl cup, and bell tokens | 1.5 | 2.5 | fewer bell tokens, stronger pockets, larger fit marks |
| Proof rail and griddle | 2.0 | 3.0 | simplify sockets/open token |
| Egress/loose-prop sweep | 1.0 | 1.5 | remove floor-adjacent storage |
| Total | 9.0 | 12.0 | cut optional reward reset before safety checks |

## Failure modes

| Failure | Player-facing recovery | Staff action | Post-session fix |
|---|---|---|---|
| tile missing | "The canteen has a backup ingredient." | provide duplicate or staff ticket | replace/retether tile |
| order rail sticks | "The rail is sticky; the steward can certify the order." | release/give ticket | inspect rail |
| pipe elbow falls/loosens | "The tide coupler is loose." | set elbow or give staff gear | repair mount |
| clam register sticks or pearl cup misreads | "The clam is being dramatic." | demonstrate intended fit or give staff tag | inspect hinge/stop/cup |
| bell token missing | "The register has a spare bell." | provide duplicate/staff tag | replace token |
| griddle socket jams | "The griddle accepts the proof." | seat staff proof/release token | inspect socket |
| dry bubble light fails | "The reef is applauding quietly." | use printed open token | repair light |

## Proof-token reset drill

| Check | Expected state | Failure response |
|---|---|---|
| Service ticket | in order rail, not griddle | replace from staff proof set |
| Current gear | in pipe-board home, elbows reset | replace gear; reset route |
| Shell tag and pearl cup | tag hidden in clam register, cup empty, bell tokens counted | replace tag/token/cup insert |
| Proof rail | empty and visible | clear unrelated props |
| Griddle sockets | empty, clean, visible to camera | clear and test fit |
| Staff proofs | full duplicate set in kit | block run until restored |

## Transport and maintenance

| Item | Before transport | After transport | Routine maintenance |
|---|---|---|---|
| pressure griddle | engage transport lock | inspect sockets/gauge/open token | daily socket/gauge check |
| ingredient bins | latch or bag tiles | count tiles | replace worn tiles |
| pipe board | lock/remove elbows | compare to reset photo | tighten mounts |
| clam register | secure hinge and tokens | inspect closure | hinge/stop check |
| proof rail | cover or remove loose tokens | inspect labels | replace peeling labels |

## Operator stress tests

| Scenario | Expected staff action | Hard limit | Revision if failed |
|---|---|---|---|
| confused team searches bins | redirect to shift card | no aimless search past P1 hint | stronger category plaques |
| fighting team has one planner | assign sorter/router/ringer/checker | no player idle for whole scene | split evidence more physically |
| chaotic team dumps tiles | pause and restore bin frame | no missing tile past beat | reduce/tether tiles |
| accessibility-varied team cannot route pipe | use seated checker and high-contrast arrows | no high reach required | lower/enlarge board |
| late/tired group reaches hard-exit | accelerate one proof and final safely | no rushed unsafe griddle use | simplify final sequence |

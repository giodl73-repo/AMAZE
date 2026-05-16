# AMAZE Device Catalog

Devices are reusable build modules. A device may implement many techniques, and
one technique may be implemented by many devices. Room `BUILD.md` files should
reference inventory IDs for parts and device IDs for assemblies when useful.

## Device card fields

| Field | Meaning |
|---|---|
| Device ID | Stable module identifier. |
| Device family | What kind of assembly it is. |
| Player interface | What players touch/see/hear. |
| Typical techniques | Technique IDs this device commonly supports. |
| Common inventory | Parts from `INVENTORY.md`. |
| Bypass | Staff action if the device fails. |
| Reset check | What must be verified between teams. |
| Play profile | Whether the device tends to delight, frustrate, or split teams. |

## Passive and analog devices

| Device ID | Device family | Player interface | Typical techniques | Common inventory | Bypass | Reset check |
|---|---|---|---|---|---|---|
| DEV-BIN-001 | Counted prop bin | oversized objects in labeled bins | `TECH-SORT-001`, `TECH-SORT-004` | `PRINT-005`, `OPS-002`, `PRINT-002` | operator supplies missing item or staff proof | count matches reset card |
| DEV-RAIL-001 | Token/order rail | shaped tokens slide into a rail | `TECH-FIT-002`, `TECH-SORT-002`, `TECH-REVEAL-001` | `CORE-001`, `MECH-004`, `PRINT-005` | staff releases proof token | rail clear; token order reset |
| DEV-SOCKET-001 | Keyed proof socket | category token seats in shaped socket | `TECH-FIT-001`, `TECH-REVEAL-001`, `TECH-TEAM-002` | `CORE-001`, `SCENIC-004`, `SCENIC-005` | staff proof token or manual unlock | wrong tokens rejected; socket empty |
| DEV-COVER-001 | Proof-locked cover | cover opens only after valid token/state | `TECH-FIT-004`, `TECH-REVEAL-003` | `MECH-005`, `MECH-003`, `CORE-002` | staff opens/holds cover | cover closed; stop intact |
| DEV-BALANCE-001 | Balance plinth | object placed on seesaw/pointer/rest | `TECH-FIT-003` | `CORE-001`, `MECH-004`, `SCENIC-005` | staff demonstrates intended result | pointer zero; true object passes |
| DEV-WINDOW-001 | Inspection window | players view/compare through framed opening | `TECH-ALIGN-001`, `TECH-ALIGN-003` | `SCENIC-004`, `CORE-001`, `PRINT-001` | printed comparison card | window clean; card/panel reset |
| DEV-OVERLAY-001 | Transparent overlay set | acetate/acrylic overlays align to targets | `TECH-ALIGN-001`, `TECH-ALIGN-004` | `SCENIC-004`, `PRINT-001`, `OPS-002` | printed answer/exclusion card | overlays counted and unscratched |
| DEV-PIPE-001 | Chunky route board | elbows/tiles rotate or snap into path | `TECH-ALIGN-002` | `CORE-001`, `SCENIC-005`, `MECH-005` | staff sets one segment or gives proof | route pieces in start state |
| DEV-FLAG-001 | Mechanical flag/reveal | flag, card, or token rises/slides/flips | `TECH-REVEAL-003` | `MECH-004`, `MECH-005`, `PRINT-001` | staff hands reveal token | flag hidden; guide clear |
| DEV-TRAY-001 | Public proof rail | earned tokens rest in public silhouettes | `TECH-TEAM-001` | `PRINT-005`, `SCENIC-003`, `PRINT-002` | operator names proof state | rail clear; silhouettes visible |
| DEV-DIAL-001 | Tactile dial/tuner | knob, pointer, scale, or radio-style face | `TECH-REVEAL-003`, `TECH-TEAM-002` | `MECH-006`, `PRINT-002`, `CORE-001` | operator sets/announces tuned state | dial reset to start mark |
| DEV-FOLD-001 | Fold/transform surface | hinged/latching table, panel, or map surface changes orientation | `TECH-ALIGN-004`, `TECH-REVEAL-003` | `MECH-005`, `MECH-001`, `CORE-002` | operator points to transformed mark | latch/orientation reset |
| DEV-LATCH-001 | Latch stack | several latches or hasps must be set/opened without force | `TECH-FIT-004`, `TECH-TEAM-002`, `TECH-REVEAL-003` | `MECH-001`, `MECH-005`, `MECH-010` | staff opens latch path | latches relocked and stops checked |

## Powered low-voltage devices

| Device ID | Device family | Player interface | Typical techniques | Common inventory | Bypass | Reset check |
|---|---|---|---|---|---|---|
| DEV-LED-001 | State light strip | colored/white light indicates state | `TECH-REVEAL-002`, `TECH-REVEAL-004` | `ELEC-001`, `ELEC-003`, `ELEC-004` | printed state card or operator narration | power test passes |
| DEV-SWITCH-001 | Physical switch panel | toggles/buttons/levers | `TECH-REVEAL-002`, `TECH-ALIGN-002` | `ELEC-002`, `ELEC-004`, `CORE-001` | operator sets state manually | switches start state photo |
| DEV-SENSOR-001 | Reed/microswitch detector | hidden state detection for panels/tokens | `TECH-REVEAL-001`, `TECH-REVEAL-002` | `ELEC-008`, `ELEC-005`, `ELEC-004` | manual proof token or staff release | sensor test and false positive check |
| DEV-AUDIO-001 | Ambience/audio prompt | speaker or local audio effect | `TECH-REVEAL-004` | `ELEC-006`, `ELEC-007`, `ELEC-003` | printed transcript/visual equivalent | volume and backup checked |
| DEV-FAN-001 | Dry air/bubble gag | fan/flap/streamer motion, no liquid | `TECH-REVEAL-004` | `ELEC-003`, `ELEC-002`, custom fan | printed reward card | fan clear; no loose fabric hazard |
| DEV-CONTROLLER-001 | Microcontroller state box | handles sensors, lights, timed reveal | `TECH-REVEAL-002`, `TECH-REVEAL-003` | `ELEC-005`, `ELEC-004`, `SCENIC-001` | manual bypass per output | flashed spare and reset script |

## Locking and final-convergence devices

| Device ID | Device family | Player interface | Typical techniques | Common inventory | Bypass | Reset check |
|---|---|---|---|---|---|---|
| DEV-FINAL-001 | Three-proof final cabinet | three sockets plus reveal/open token | `TECH-REVEAL-001`, `TECH-TEAM-002`, `TECH-REVEAL-003` | `CORE-001`, `MECH-004`, `MECH-005`, `SCENIC-004` | staff release/open token | sockets empty; reveal reset |
| DEV-FINAL-002 | Proof-locked selector face | tokens open selector covers/sliders | `TECH-FIT-001`, `TECH-FIT-004`, `TECH-TEAM-002` | `MECH-006`, `MECH-005`, `CORE-002` | staff opens cover or sets selector | covers closed; sliders reset |
| DEV-FINAL-003 | Pressure/gauge finale | proof tokens change gauge/open state | `TECH-REVEAL-003`, `TECH-REVEAL-004` | `MECH-006`, `ELEC-001`, `CORE-001` | staff moves gauge/open token | gauge zero; open token hidden |

## Device selection rules

1. Prefer analog/passive devices until the room proves the aha.
2. Use powered devices for feedback, delight, or operator visibility; do not let
   them be the only way to exit or understand a required clue.
3. Every device must have a manual bypass and reset check before promotion.
4. Every loose device/token needs a home, a count, and a spare/substitute plan.
5. Any device with pinch, force, wet/slip, heat, smoke, darkness, or audio-only
   reliance must go through safety review before build readiness.

## Device visual references

Use these editable diagrams from `components\VISUALS.md` when reviewing a device
or room BOM. A visual reference is required for C4-C5 components before build
readiness, either by using one of these diagrams or creating a room-specific
diagram.

| Device ID | Visual reference | Covers |
|---|---|---|
| `DEV-SOCKET-001` | `components\diagrams\dev-socket-proof-token.excalidraw` | keyed token fit, reject path, camera-visible state, staff proof pouch, loss/jam simulation fields |
| `DEV-RAIL-001` | `components\diagrams\dev-rail-proof-order.excalidraw` | public ordering rail, channel/stops, tile loss, jam, false release, staff proof recovery |
| `DEV-WINDOW-001` | `components\diagrams\dev-overlay-inspection-window.excalidraw` | inspection aperture, visibility, glare, crowding, printed backup |
| `DEV-OVERLAY-001` | `components\diagrams\dev-overlay-inspection-window.excalidraw` | overlay orientation, glare/scratch, sleeve reset, spare overlay/card |
| `DEV-FINAL-001`, `DEV-FINAL-002` | `components\diagrams\dev-final-lite-vs-cabinet.excalidraw` | visible board vs proof-locked cabinet tradeoff, final proof state, build pressure |
| `DEV-SWITCH-001`, `DEV-LED-001` | `components\diagrams\dev-low-voltage-control.excalidraw` | safe low-voltage player controls, fused build, manual state card, power/light fault recovery |
| `DEV-BIN-001` and counted card/prop sets | `components\diagrams\dev-counted-prop-card-set.excalidraw` | count card, duplicate pouch, reset photo, missing/bent/marked item recovery |
| `DEV-FOLD-001`, `DEV-DIAL-001` | `components\diagrams\dev-transform-dial-surface.excalidraw` | transform/tuning action, stops, detents, start marks, no-load script, knob/latch recovery |
| `DEV-LATCH-001` | `components\diagrams\dev-latch-final-box.excalidraw` | central sealed object, latch alignment, hinge stops, staff release, pinch/force risks |
| `DEV-PIPE-001`, `DEV-COVER-001`, `DEV-FINAL-003` | `components\diagrams\dev-pipe-cover-final-machine.excalidraw` | route boards, proof covers, pressure/gauge finales, safe removable pieces, manual open token |

## Device play profiles

Use these profiles when choosing devices for a room. A device is a crowd pleaser
only when the player-facing success signal is immediate, public, and fair.

### Passive and analog devices

| Device ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| DEV-BIN-001 | Accessible helper | Fast onboarding, easy reset, good for kids/families, makes categories visible. | Can become chore sorting, search soup, or a dumping ground. | Few oversized items have obvious homes and funny labels. | Too many tiny/identical props or hidden inventory logic. | Keep counts low, use silhouettes/labels, add reset card and duplicate set. |
| DEV-RAIL-001 | Strong tactile pleaser | Satisfying slide/sequence action, public progress, easy to theme. | Can jam, become color matching, or hide the aha inside tolerances. | Wrong pieces reject visibly and the final stack releases a proof. | Players cannot tell whether order, orientation, or force is wrong. | Oversize pieces, asymmetric edges, no-force stops, operator-visible release. |
| DEV-SOCKET-001 | High-trust proof gate | Clear "this belongs here" moment, reduces final guessing, supports team ritual. | False fits, jammed tokens, or too-similar proofs break trust. | Tokens are chunky, distinct, and seat with a visible/feelable click. | Every token almost fits or success depends on exact fine motor alignment. | Distinct silhouettes, rounded reject paths, spare token set, staff proof bypass. |
| DEV-COVER-001 | Comic/payoff pleaser | Creates anticipation, reveal, and satisfying closure/opening. | Force, pinch, and ambiguous resistance risk. | Correct input removes resistance and reveals something physical. | Players push harder because wrong state feels like sticky hardware. | Soft stops, hinge guards, visible "no force" behavior, staff-open bypass. |
| DEV-BALANCE-001 | Discovery pleaser | Physical truth feels magical and explainable; good for object identity. | Drift, calibration, hidden weight clues, and false positives. | The right object behaves obviously differently from all wrong objects. | Tiny weight differences or table angle determine success. | Wide tolerance gap, reset zero mark, robust base, non-weight clue backup. |
| DEV-WINDOW-001 | Focused aha device | Frames attention, reduces search space, works well in small trailers. | Glare, parallax, crowding at a peephole, low-light dependence. | The window makes one clue suddenly obvious to the group. | One person blocks the view or alignment is too subtle. | Accessible height, large aperture, matte surfaces, duplicate printed view. |
| DEV-OVERLAY-001 | Puzzle-lover pleaser | Great "of course" alignment reveal; cheap to prototype. | Scratches, glare, orientation ambiguity, private solve for one player. | Alignment has bold landmarks and the result is readable at arm's length. | Players rotate/flip endlessly without feedback. | Corner keys, orientation icons, high contrast, sleeve storage, spare overlay. |
| DEV-PIPE-001 | Family/team pleaser | Big physical routing, easy table talk, strong cause/effect. | Fiddly rotations, high reach, route ambiguity, piece loss. | Pieces are chunky and a checker can call the path from a seated position. | Small elbows drift or players need precision instead of reasoning. | Waist-height board, magnets/stops, high-contrast arrows, start-state photo. |
| DEV-FLAG-001 | Universal payoff | Immediate visible reveal, low electronics burden, strong operator visibility. | Jams, weak motion, anticlimax if too subtle. | Flag/card moves enough that everyone notices. | Reveal is tiny, slow, hidden, or easy to miss. | Big travel distance, bright target, clear track, manual reveal fallback. |
| DEV-TRAY-001 | Operator/team helper | Makes progress public, supports final convergence, simplifies reset. | Can feel like admin if not themed; may collect unrelated props. | Proof homes are scenic and make the final ritual obvious. | Players treat it as random storage. | Silhouettes, labels, camera-visible placement, "only proofs live here" rule. |
| DEV-DIAL-001 | Classic interaction pleaser | Familiar, tactile, good final tuning/payoff affordance. | Slippage, tiny labels, random spinning, or audio-only confirmation. | Dial has detents/clear marks and success is visible. | Players fine-tune endlessly without feedback. | Big marks, set-screw knob, detents/index, visual success state. |
| DEV-FOLD-001 | Transformation pleaser | Uses trailer furniture as puzzle, creates memorable physical shift. | Pinch, force, weight, latch jams, or accessibility/reach problems. | Transformation changes meaning clearly without strength. | Players lean on it or fight sticky hardware. | Light-duty prototype first, stops, two-handed safe script, staff bypass. |
| DEV-LATCH-001 | Tension/finale pleaser | Opening a real object feels earned and dramatic. | Force, pinch, misalignment, and false-open risks. | Latches show "not yet" clearly and open smoothly when solved. | Players pull harder because the latch feels stuck. | No-force stops, robust hardware, clear status, staff release. |

### Powered low-voltage devices

| Device ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| DEV-LED-001 | Easy delight | Fast feedback, strong mood, visible from across trailer. | False positives, color-only clues, power/controller failure. | Light confirms an already-understood physical solve. | Players need color perception or subtle light changes to progress. | Text/shape backup, fused power, pre-run test, printed state fallback. |
| DEV-SWITCH-001 | Familiar tactile control | Buttons/levers are approachable and satisfying. | Random flipping, brute force, unclear start state. | Switches map clearly to visible room states. | It becomes a trial-and-error panel with no consequence or logic. | Start-state photo, labels tied to fiction, lockout/feedback, operator reset check. |
| DEV-SENSOR-001 | Invisible magic when done well | Lets analog props trigger feedback without visible electronics. | False negatives/positives are hard for players to diagnose. | Sensor confirms a clear physical state, not a hidden exact placement. | Correct players are told "no" by hardware. | Wide tolerances, operator indicator, manual proof, bench false-state test. |
| DEV-AUDIO-001 | Atmosphere pleaser | Adds personality, comedy, urgency, and reward. | Audio-only clues exclude players and fail in noisy rooms. | Sound is flavor, confirmation, or optional clue with printed equivalent. | Required information exists only in the speaker. | Transcript/visual backup, volume check, captions/cards, no required audio-only beats. |
| DEV-FAN-001 | Delight/spectacle pleaser | Dry motion can feel like bubbles, steam, wind, or machine life. | Loose fabric, noise, hair/eye irritation, mistaken clue importance. | Effect rewards progress and can be ignored safely. | Players put hands/faces near moving parts or wait for hidden timing. | Guarded fan, low power, no loose hazards, printed/LED fallback. |
| DEV-CONTROLLER-001 | Powerful but risky | Coordinates sensors, lights, timed reveals, and operator indicators. | Debug burden, hidden state bugs, boot failures, brittle dependencies. | It makes staff state clearer while players still solve physically. | A software fault blocks exit or makes the room inscrutable. | Manual mode, flashed spare, visible diagnostics, keep required logic simple. |

### Locking and final-convergence devices

| Device ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| DEV-FINAL-001 | Reliable finale | Clear three-proof convergence, strong "we earned this" structure. | Can feel like token dumping if proofs lack meaning. | Each proof is remembered and has a distinct final home. | Any token can be guessed or one player silently finishes it. | Distinct sockets, role split, proof recap, operator-visible final state. |
| DEV-FINAL-002 | Premium deduction finale | Proof-locked selectors make accusation/choice feel earned. | Too many covers/sliders create fiddly final admin. | Proofs unlock choices in a dramatic staged order. | Players solve logic but fight the hardware. | Fewer selectors, large labels, detents, staff-open bypass, reset photo. |
| DEV-FINAL-003 | Signature machine pleaser | Gauge/open-token payoff gives a trailer room a memorable central object. | Gauge ambiguity, false open, force, or theatrical effect over puzzle clarity. | Proof tokens visibly change pressure/open state and everyone sees the machine wake. | Players guess sockets or cannot tell whether the machine accepted proof. | Keyed sockets, big gauge states, no-force stops, open token manual release. |

## Device mix guidance

| Mix problem | Smell | Better mix |
|---|---|---|
| Too many bins/cards | Room feels like chores or classroom sorting. | Add one rail/socket/reveal so sorting produces physical consequence. |
| Too many hidden sensors | Players distrust the room because success is invisible. | Make the physical state visible first; use sensors only for feedback. |
| Too many one-person windows/overlays | One player solves while others wait. | Pair with caller/checker roles and a public proof tray. |
| Too many rails/sockets | Room becomes matching toy rather than puzzle. | Add route, contradiction, or team-role logic. |
| Too many powered effects | Spectacle outruns reliability and reset. | Keep powered effects optional or operator-visible, not required. |
| Too much final hardware | The finale becomes admin. | Reduce final actions to proof recap, role split, and one memorable reveal. |

## Device-to-room examples

| Room | Device examples | Notes |
|---|---|---|
| 0003 Manor in the Mirrorline | `DEV-WINDOW-001`, `DEV-OVERLAY-001`, `DEV-BALANCE-001`, `DEV-FINAL-002` | physical deduction proofs and proof-locked cabinet |
| 0004 Brineworks at Low Tide | `DEV-RAIL-001`, `DEV-PIPE-001`, `DEV-COVER-001`, `DEV-FINAL-003` | service proofs, current routing, pearl-fit clam, pressure griddle |

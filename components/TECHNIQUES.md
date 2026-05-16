# AMAZE Technique Catalog

Techniques are reusable puzzle/build patterns. They are not room concepts by
themselves; they describe what players do, what the aha is, what can fail, and
what device classes commonly implement the beat.

Use this catalog when filling a room hopper or reviewing whether a puzzle is
physical enough.

## Technique card fields

| Field | Meaning |
|---|---|
| Technique ID | Stable ID used in room `PUZZLE-GRAPH.md` or hopper notes. |
| Player action | What the team physically does. |
| Aha | What the team realizes. |
| Good for | Room functions this technique supports. |
| Device families | Reusable device classes that can implement it. |
| Failure modes | Common ways it becomes unfair, fragile, or boring. |
| Promotion test | What must be proven before build detail. |
| Play profile | Whether the technique tends to delight, support, or frustrate teams. |

## Physical manipulation techniques

| ID | Technique | Player action | Aha | Good for | Device families | Failure modes | Promotion test |
|---|---|---|---|---|---|---|---|
| TECH-FIT-001 | Keyed fit | Try objects/tokens in shaped homes. | only the correct object fits cleanly | proof tokens, final gates, prop classification | `DEV-SOCKET`, `DEV-RAIL`, `DEV-COVER` | forcing, tiny tolerances, false fits | wrong attempts fail safely and visibly |
| TECH-FIT-002 | Edge matching | Align edges, cuts, tabs, or silhouettes. | the correct sequence shares a continuous physical edge | order building, map repair, evidence reconstruction | `DEV-RAIL`, `DEV-OVERLAY`, `DEV-PANEL` | subtle marks, orientation ambiguity | confused team can explain the matching rule |
| TECH-FIT-003 | Weight/balance test | Place objects on balance/rest surface. | only the right object has the right physical behavior | object proof, material identity, cause/effect | `DEV-BALANCE`, `DEV-PLINTH` | tolerance drift, hidden-mark search, false positives | true object succeeds; false objects visibly fail |
| TECH-FIT-004 | No-force closure | Close a door/shell/cover only when state is right. | correct input removes resistance | comedy props, final locks, bell/register beats | `DEV-COVER`, `DEV-HINGE`, `DEV-SOCKET` | players force it, pinch, unclear rejection | wrong inputs cannot be forced to success |

## Spatial and alignment techniques

| ID | Technique | Player action | Aha | Good for | Device families | Failure modes | Promotion test |
|---|---|---|---|---|---|---|---|
| TECH-ALIGN-001 | Overlay comparison | Place transparent/printed overlays on targets. | one target agrees or refuses alignment | location proof, map solve, suspect/object reuse | `DEV-OVERLAY`, `DEV-WINDOW`, `DEV-LIGHT` | glare, tiny alignment, swapped overlays | work-light route is readable |
| TECH-ALIGN-002 | Route continuity | Rotate/snap route pieces to connect endpoints. | flow follows a continuous path | pipes, circuits, roads, ventilation, currents | `DEV-PIPE`, `DEV-TRACK`, `DEV-MAGNETIC-PANEL` | fiddly rotations, high reach, route ambiguity | seated/checker role can guide route |
| TECH-ALIGN-003 | Window reveal | Look through an aperture or inspection window. | framing selects the relevant information | location, map, hidden-state reveal | `DEV-WINDOW`, `DEV-PANEL`, `DEV-LIGHT` | low-light dependence, parallax unfairness | readable at accessible height |
| TECH-ALIGN-004 | Symmetry/reflection check | Compare mirrored, reversed, or paired states. | the mismatch is the answer | manor/mirror beats, quality control, calibration | `DEV-MIRROR`, `DEV-OVERLAY`, `DEV-PANEL` | subtle reflection, glare, visual fatigue | non-reflective duplicate path exists |

## Classification and ordering techniques

| ID | Technique | Player action | Aha | Good for | Device families | Failure modes | Promotion test |
|---|---|---|---|---|---|---|---|
| TECH-SORT-001 | Bounded sort | Sort a small set into labeled homes. | the category frame matters more than searching | onboarding, ingredient bins, evidence classes | `DEV-BIN`, `DEV-RAIL`, `DEV-TAG` | too many loose parts, label overload | reset count and confused-team pass |
| TECH-SORT-002 | Physical order stack | Build a sequence with shaped tokens. | order is encoded by fit/shape, not text | food orders, rituals, production lines | `DEV-RAIL`, `DEV-SOCKET`, `DEV-TRAY` | color-only triviality, search soup | stack releases proof or obvious state |
| TECH-SORT-003 | Contradiction test | Compare a claim token against a physical state. | one claim fails its own evidence | suspect alibis, quality checks, witness statements | `DEV-WINDOW`, `DEV-CARD`, `DEV-SOCKET` | reading puzzle, private logic | physical contradiction is visible to group |
| TECH-SORT-004 | Inventory count | Confirm all items are present and in homes. | reset/inventory state is part of the puzzle | kitchens, labs, evidence kits | `DEV-BIN`, `DEV-TAG`, `DEV-RAIL` | bookkeeping not fun, tiny objects | count supports both play and reset |

## Signal, feedback, and reveal techniques

| ID | Technique | Player action | Aha | Good for | Device families | Failure modes | Promotion test |
|---|---|---|---|---|---|---|---|
| TECH-REVEAL-001 | Proof-token release | Earn a visible token from a station. | solved stations produce portable proof | final convergence, public progress | `DEV-SOCKET`, `DEV-RAIL`, `DEV-COVER` | loose-part reset debt | token has home, spare, and socket |
| TECH-REVEAL-002 | State light | Cause a clear light/state change. | device accepted or rejected the input | feedback, final payoff, operator visibility | `DEV-LED`, `DEV-SWITCH`, `DEV-CONTROLLER` | false positives, power failure | manual equivalent exists |
| TECH-REVEAL-003 | Mechanical flag | Raise, slide, flip, or uncover a physical result. | the room answers physically | finales, station approval, progress | `DEV-FLAG`, `DEV-SLIDER`, `DEV-COVER` | jam, subtle motion, fragile reset | operator can see final state |
| TECH-REVEAL-004 | Dry sensory gag | Use cards, lights, fans, or sound for delight. | theme rewards progress without clue load | atmosphere, fast-team reward | `DEV-LED`, `DEV-AUDIO`, `DEV-FAN`, `DEV-CARD` | mistaken for clue, wet/mess risk | optional and bypassable |

## Team-flow techniques

| ID | Technique | Player action | Aha | Good for | Device families | Failure modes | Promotion test |
|---|---|---|---|---|---|---|---|
| TECH-TEAM-001 | Proof rail | Place earned proofs in public homes. | progress is shared and operator-visible | final convergence, reset control | `DEV-RAIL`, `DEV-TOKEN`, `DEV-CAMERA-ZONE` | prop dumping, aisle blockage | camera can read proof state |
| TECH-TEAM-002 | Caller/checker/handler | Split final action across roles. | no one player owns the finale | social safety, final rituals | `DEV-SOCKET`, `DEV-SLIDER`, `DEV-COVER` | role confusion, crowding | fighting team stays engaged |
| TECH-TEAM-003 | Parallel stations | Let subgroups solve independent proofs. | team can divide and reconverge | throughput, group agency | any station family | bottlenecks, operator state loss | matrix pass covers team archetypes |

## Technique play profiles

Use this before picking devices. A technique can be sound but wrong for the room's
audience, team size, or reset clock.

### Physical manipulation profiles

| Technique ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| TECH-FIT-001 | tactile pleaser | Fast to understand, satisfying proof of correctness, good for mixed ages. | Force, false fits, and precision tolerance can make players blame hardware. | Correct fit is chunky, obvious, and wrong fits fail cleanly. | Every object nearly fits or success depends on exact hand pressure. | Oversized shapes, visible reject paths, soft stops, staff bypass. |
| TECH-FIT-002 | puzzle-lover pleaser | Creates an "of course" physical aha without code math. | Subtle edges become tedious orientation fiddling. | The edge/shape rule is visible after one good match. | Players rotate pieces randomly with no feedback. | Asymmetric tabs, orientation icons, low piece count, example reject. |
| TECH-FIT-003 | discovery pleaser | Physical truth feels magical and thematic. | Drift and calibration failures feel unfair. | Differences are large enough for public use. | Tiny weight differences decide the solve. | Wide tolerance gap, reset zero mark, non-weight confirmation. |
| TECH-FIT-004 | comic/reveal pleaser | Makes props feel alive and prevents brute-force openings. | Can invite pushing, pinching, or sticky-hardware confusion. | Correct state removes resistance and reveals a clear result. | Wrong state feels like a jam instead of a no. | No-force script, soft stops, hinge guards, visible "not yet" state. |

### Spatial and alignment profiles

| Technique ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| TECH-ALIGN-001 | aha pleaser | Cheap, portable, and creates strong reveal moments. | Glare, parallax, private solving, and orientation ambiguity. | Landmarks snap into meaning at arm's length. | One player squints while everyone else waits. | Large marks, corner keys, matte surfaces, public result card. |
| TECH-ALIGN-002 | team pleaser | Big spatial reasoning supports caller/checker/handler roles. | Fiddly parts, high reach, and ambiguous paths. | The path is visible to the group and pieces are chunky. | Players fight hardware rather than reason about flow. | Waist-height board, high-contrast arrows, end-state proof, reset photo. |
| TECH-ALIGN-003 | focus helper | Frames attention and reduces search soup. | Can become a bottleneck peephole. | The framed view is large enough for group discussion. | Only one person can see or low light hides the answer. | Accessible height, duplicate framed card, work-light readability. |
| TECH-ALIGN-004 | thematic specialist | Strong for mirror, lab, quality-control, or before/after concepts. | Reflection/glare can exclude or fatigue players. | Mismatch is bold and thematically motivated. | Players need visual acuity, darkness, or subtle reflection. | Non-reflective duplicate route, high contrast, no required darkness. |

### Classification and ordering profiles

| Technique ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| TECH-SORT-001 | onboarding helper | Gives teams vocabulary and quick early wins. | Becomes chores if categories are arbitrary or numerous. | Sorting teaches the room's jobs and unlocks action. | Players feel like they are tidying props. | Few categories, funny labels, physical consequence, clear end state. |
| TECH-SORT-002 | tactile pleaser | Turns sequence/order into a physical build. | Can become color-only homework or bin rummaging. | Shape/order creates a release, reveal, or group-recognized object. | Too many loose tiles or no feedback until the end. | 5-7 pieces, edge logic, bounded bins, duplicate set. |
| TECH-SORT-003 | deduction pleaser | Lets players catch lies through evidence instead of codes. | Can collapse into reading comprehension. | Contradiction is physically visible to the group. | The "aha" lives only in paragraphs or private notes. | Tokenized claims, evidence windows, group proof moment. |
| TECH-SORT-004 | operations helper | Supports reset and puzzle fairness together. | Bookkeeping can feel like staff work. | Count confirms progress or makes absence meaningful. | Players are simply inventorying objects. | Count only meaningful categories; keep tiny parts out. |

### Signal, feedback, and reveal profiles

| Technique ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| TECH-REVEAL-001 | progress pleaser | Proof tokens create public achievement and final convergence. | Loose-token reset debt and prop dumping. | Each token is meaningful and has a visible home. | Tokens feel like arbitrary keys. | Distinct silhouettes, proof tray, spare set, final recap. |
| TECH-REVEAL-002 | feedback helper | Clear confirmation reduces uncertainty. | False states and color/audio-only dependency. | It confirms a physical solve players already understand. | Players hunt for hidden electronic rules. | Manual equivalent, operator indicator, text/shape backup. |
| TECH-REVEAL-003 | universal pleaser | Mechanical motion is readable and satisfying. | Jams or tiny reveals are anticlimactic. | Reveal is large, physical, and staff-visible. | Motion is subtle or misses the group's attention. | Big travel, bright flag/card, clear track, manual reveal. |
| TECH-REVEAL-004 | delight support | Adds memorability without puzzle burden. | Players may mistake flavor for clue or effects add safety debt. | It rewards progress and is safely optional. | Teams wait on it or search it for hidden logic. | Mark as reward, no wet/heat/mist, printed fallback. |

### Team-flow profiles

| Technique ID | Crowd profile | Advantages | Pitfalls | Crowd pleaser when | Frustrater when | Design safeguards |
|---|---|---|---|---|---|---|
| TECH-TEAM-001 | flow helper | Makes progress and reset public. | Can feel like admin or collect junk props. | Proof homes are scenic and clearly final-relevant. | Players treat it as a dumping tray. | Silhouettes, labels, camera view, only proofs allowed. |
| TECH-TEAM-002 | finale pleaser | Prevents one-player endings and creates shared ritual. | Role confusion and crowding at final device. | Caller, checker, and handler each matter briefly. | One person finishes while others watch. | Large controls, role prompts, one final action per proof. |
| TECH-TEAM-003 | throughput helper | Lets groups split and recombine. | Parallelism can hide state from staff or isolate players. | Stations are visible and reconverge cleanly. | Subgroups solve silently with no shared payoff. | Public proof rail, operator state map, final recap. |

## Technique mix guidance

| Mix problem | Smell | Better mix |
|---|---|---|
| Too many fit tests | Room feels like a children's shape sorter. | Add contradiction, route continuity, or role-based final convergence. |
| Too many overlays/windows | One player owns the solve. | Add public proof tokens or a handler/checker split. |
| Too much sorting | Room feels like chores. | Make sorted categories trigger a rail, reveal, or final machine state. |
| Too many reveals | Players chase spectacle without thinking. | Ensure each reveal pays off a clear aha. |
| Too much parallelism | Team fragments and operator loses state. | Add proof rail, midpoint recap, or explicit convergence. |

## Promotion checklist

A technique is ready to promote into a room beat only when:

1. The player action is physical and visible.
2. The aha is not just "read this text."
3. The station has a reset state and staff-visible proof state.
4. Wrong attempts fail safely and informatively.
5. Accessibility path does not rely on darkness, audio-only signals, high reach,
   crawling, wet/slippery surfaces, or force.
6. A manual bypass exists for powered or fragile implementations.

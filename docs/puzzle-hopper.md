# Puzzle Hopper

AMAZE rooms need puzzle-hunt-level craft before they need props. The hopper is
the inventory of possible aha moments, clue systems, transformations, and
physical interactions that a room can draw from.

The hopper is not a backlog of things to build. It is a design laboratory.
Seed the hopper from puzzle-type phases in `docs/puzzle-phases.md`, then promote
only the candidates that survive physicalization, simulation, scoring, and
room-specific gates.

## Candidate card

| Field | Question |
|---|---|
| Name | What is the short handle? |
| Phase | Which puzzle-type phase from `docs/puzzle-phases.md` does this candidate develop? |
| Design wave/pulse | Which AMAZE design wave and test pulse produced this idea? |
| Aha | What must players realize? |
| Inputs | What do players see, hear, touch, sort, count, align, or infer? |
| Transform | What puzzle-hunt skill converts input to output? |
| Output | What does the puzzle yield? |
| Physical action | What satisfying trailer-scale action expresses the transform? |
| Build economics | What component, cost band, breakage risk, and spare plan does it imply? |
| Clue sources | How does the room teach the aha fairly? |
| Red herrings | What might players reasonably but wrongly try? |
| Failure mode | How does this break, confuse, bottleneck, or overrun? |
| Reset state | What must staff restore? |
| Promotion test | What evidence moves it into a room graph? |

## Puzzle-hunt skills

Use these skill tags to make difficulty explicit:

| Skill | Escape-room use |
|---|---|
| Observation | noticing differences, wear marks, light, labels, absences |
| Association | connecting object, place, phrase, sound, or story references |
| Ordering | route, chronology, weight, color, frequency, map path |
| Extraction | selecting letters, numbers, positions, symbols, or sounds |
| Transformation | cipher, substitution, folding, overlay, rotation, reflection |
| Indexing | using one artifact to pick from another |
| Classification | grouping by category, material, era, owner, function |
| Spatial reasoning | maps, sightlines, hidden alignments, body position |
| Audio reasoning | rhythm, pitch, radio tuning, voice fragments, silence |
| Tactile reasoning | texture, weight, magnetism, pressure, latch behavior |
| Collaboration | two-player sync, call-and-response, separated viewpoints |
| Metapuzzle | combining solved outputs into a final realization |

Avoid stacking too many hidden skills in one beat. A candidate with three
unstated skills is usually under-clued.

## Hopper states

| State | Meaning |
|---|---|
| Raw | interesting seed, not yet a puzzle |
| Carded | candidate card is complete |
| Clued | fair clue sources are identified |
| Physicalized | one or more trailer-scale actions are plausible |
| Simulated | tested in the scene/beat harness |
| Promoted | added to a room puzzle graph |
| Parked | good idea, wrong room or wrong constraints |
| Retired | repeated evidence says do not use |

## Promotion gates

A candidate promotes only when it passes all gates:

1. **Aha gate** - the realization is specific and satisfying.
2. **Fairness gate** - players can know what to do without mind-reading.
3. **Theme gate** - the action belongs in the room's fiction.
4. **Physical gate** - it can be touched, seen, heard, or embodied in the trailer.
5. **Economics gate** - the BOM, budget, durability, reliability, and spare plan
   are credible for the room.
6. **Flow gate** - it does not create an accidental bottleneck.
7. **Ops gate** - staff can monitor, hint, reset, and recover it.
8. **Safety gate** - it does not require unsafe movement, force, darkness, heat,
   electrical exposure, or panic pressure.

## Review pass

Every hopper needs an explicit editorial review before candidates promote. This
is where AMAZE says whether a puzzle is actually good, merely thematic, too
fragile, or not yet fair.

| Field | Question |
|---|---|
| Candidate | Which hopper item is being reviewed? |
| Verdict | Keep, rework, park, cut, or promote. |
| Why it might be fun | What player experience could make this worth building? |
| Why it might fail | What is the likely boredom, confusion, bottleneck, safety, or reset failure? |
| User/design input needed | What creative choice must be made before the puzzle gets more build detail? |
| Next revision | What concrete change improves the candidate? |

## Hopper table

Use this table in room docs or future `hopper/` files:

| ID | State | Phase | Name | Skill tags | Aha | Physical action | Risk | Next test |
|---|---|---|---|---|---|---|---|---|

## Examples for trailer rooms

| Seed | Better candidate |
|---|---|
| "Find a hidden key" | Notice that only road-worn keys match map scratches, then index scratches into mile markers. |
| "Use a radio" | Tune to stations in route order; static gaps encode missing stops. |
| "Open a cabinet" | Balance galley items by weight to reveal that the trailer is "leaning" toward a clue. |
| "Use the table" | Rotate the fold-down table so etched route lines align with window sightlines. |

The better candidate names the aha and the transform. The prop is only the
delivery vehicle.

---
name: Simulation Evidence Analyst
slug: simulation-evidence-analyst
tier: playtest
applies_to: [simulation, playtest, scoring, personas, evidence]
---

# Simulation Evidence Analyst

## Mandate

This role turns puzzlehunt-style test discipline into escape-room evidence. It
does not ask whether the author can explain the room; it asks what simulated and
real teams actually did.

The analyst protects the loop:

```text
DESIGN -> SIMULATE -> SCORE -> LOG SURPRISES -> IMPROVE -> LOCK NEXT VERSION
```

## Hard Question

*"What did the run prove, what did it merely suggest, and what must change before
the next version?"*

## What to Inspect

- Simulations use at least one novice/social team, one fast expert team, one
  mixed family/accessibility team, and one operator stress pass.
- `SCENES.md` records party roles, checks, clocks, group-game stakes, operator
  interventions, consequences, and reset effects.
- `PLAYTEST.md` records finish time, hints, stuck states, physical failures,
  safety flags, reset time, memorable beats, and revision decisions.
- Scores in `scoring/RUBRIC.md` cite evidence from runs instead of author taste.
- Repeated surprises become design-system candidates only after they recur.
- Old scores remain intact; new room versions get new entries.

## Failure Modes to Catch

- Treating a simulation as a story pitch rather than a test.
- Declaring a puzzle fair because the author can solve it.
- Ignoring wrong solves, bypasses, hint shame, crowding, or reset friction.
- Advancing a room with only one team type tested.
- Adding new puzzles before resolving the top evidence-backed revisions.

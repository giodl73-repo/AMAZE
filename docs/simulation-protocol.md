# Simulation Protocol

AMAZE simulations are dry runs before fabrication. They are meant to find
failure while changes are still cheap.

The default simulation model is the D&D-style scene and beat harness in
`docs/scene-beat-harness.md`. Use this file as the operating checklist and the
harness as the play model.

## Inputs

- `rooms/<slug>/BRIEF.md`
- `rooms/<slug>/FLOORPLAN.md`
- `rooms/<slug>/PUZZLE-GRAPH.md`
- `rooms/<slug>/SCENES.md`
- `rooms/<slug>/BUILD.md`
- `rooms/<slug>/SAFETY.md`
- `rooms/<slug>/OPS.md`
- selected personas from `personas/`
- team archetypes from `docs/team-testing.md`
- behavior probes from `docs/behavior-testing.md`

## Procedure

1. Set the clock: target session time, soft-warning time, hard-exit time, and
   reset target.
2. Pick a team archetype, select personas inside that team, assign party roles,
   draft group-game stakes, and read the room from that point of view.
3. Choose the behavior probes for this run: search, solve, social, physical,
   sensory, pressure, trust, or reset/ops behavior.
4. Walk scenes and beats in time order, recording:
    - discovery time;
    - beat target time, actual time, hint time, and slow maximum;
    - skill checks;
    - behavior signals;
    - group-game stakes earned, contested, refuted, or silent;
   - stuck states;
   - hint triggers;
   - physical mechanism reliability;
   - BOM, durability, and breakage concerns;
   - physical crowding;
   - safety discomfort;
    - operator intervention;
    - reset consequences.
5. Repeat across the required team archetypes in `docs/team-testing.md`.
6. Run one operator stress pass: late team, failed component, messy reset, or
    anxious participant.
7. Score with `scoring/RUBRIC.md`.
8. Move puzzle ideas through `docs/puzzle-hopper.md`: promote, park, retire, or
    revise.
9. Write top revisions before adding new puzzles.

## Output

Each simulation produces a table:

| Persona | Party roles | Finish time | Hints | Stuck states | Physical failures | Safety flags | Reset notes | Memorable beat |
|---|---|---:|---:|---|---|---|---|---|

Beat timing evidence:

| Beat | Target min | Actual min | Hint at min | Hint used at min | Slow max min | Timing decision |
|---|---:|---:|---:|---:|---:|---|

And team/behavior matrices:

| Team archetype | Team promise/risk | Scene/beat probe | Observable signal | Operator response | Design revision |
|---|---|---|---|---|---|

| Behavior | Persona/team | Scene/beat probe | Observable signal | Risk if unsupported | Design response | Evidence result |
|---|---|---|---|---|---|---|

## Timing rules

- A room should have a designed acceleration path before the soft-warning time.
- A hard-exit procedure should preserve the story where possible.
- Reset verification must be shorter than reset performance. A reset checklist
  nobody can verify under time pressure is not operational.

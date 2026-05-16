# AMAZE

**A mobile escape-room design workshop for trailer-scale builds.**

AMAZE designs escape rooms that can fit inside Airstream-size or larger trailers:
portable, inspectable, resettable game spaces where puzzle craft, physical layout,
safety, operations, and build logistics are designed together from the start.

AMAZE belongs to the Games Design family, alongside puzzle hunts, tabletop
adventures, and board-game systems. Its medium is different: the final artifact
is a compact built environment that has to move, reset, survive teams, and keep
players safe.

## Design promise

Every room should be:

1. **Playable** - puzzles produce fair aha moments, not prop-search frustration.
2. **Buildable** - components fit, mount, reset, and travel inside a real trailer.
3. **Safe** - egress, fire, electrical, visibility, accessibility, and operator
   controls are design inputs, not late checklists.
4. **Operable** - staff can reset, monitor, hint, repair, and transport the room.
5. **Memorable** - the space has a reason to exist beyond being a box of locks.

## First target

The first design target is a trailer-scale escape room, roughly Airstream-size or
larger. The repo should treat exact dimensions, axle/weight limits, power
availability, staffing, jurisdictional rules, and venue constraints as explicit
brief variables.

## Repo layout

```text
AMAZE/
  README.md
  CLAUDE.md
  docs/specs/                  design specs and constraints
  rooms/                       one directory per room concept
  components/                  reusable prop, lock, sensor, lighting, and scenic modules
  operations/                  reset, staffing, maintenance, transport, and safety checklists
  research/                    escape-room, trailer, accessibility, and build references
  .roles/                      governance and creative review lenses
```

## Initial workflow

```text
BRIEF -> FLOORPLAN -> PUZZLE GRAPH -> BUILD PLAN -> SAFETY REVIEW -> PLAYTEST -> OPS PACK
```

The workflow is intentionally physical. A puzzle graph that cannot fit in the
room, survive transport, or reset cleanly is not ready.

## Current status

Foundation scaffold. No room concepts have shipped yet.

## License

Private repository. License decision pending.

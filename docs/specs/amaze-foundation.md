# AMAZE Foundation Spec

## Goal

Create a private workshop for designing trailer-scale escape rooms where puzzle
craft, floorplan constraints, build systems, safety, operations, and playtest
evidence are reviewed together.

## Medium boundary

AMAZE designs mobile escape rooms for Airstream-size or larger trailers. It does
not assume a permanent storefront, warehouse-scale installation, or purely
digital room.

## Core surfaces

| Surface | Purpose |
|---|---|
| Room brief | Theme, audience, duration, capacity, trailer constraints, success criteria. |
| Floorplan | Dimensions, circulation, sightlines, egress, operator zones, reset paths. |
| Puzzle graph | Dependencies, aha beats, hint ladder, bottlenecks, reset state. |
| Scene harness | Party roles, group-game stakes, beat checks, clocks, interventions. |
| Build plan | BOM, budget, props, locks, sensors, durability, reliability, mounting, power, transport. |
| Safety review | Egress, fire, electrical, accessibility, emergency procedures. |
| Playtest record | Team behavior, stuck points, hint use, reset timing, operator notes. |
| Ops pack | Staff script, reset checklist, maintenance plan, failure-mode handling. |

## System specs

| Spec | Purpose |
|---|---|
| `docs/specs/design-system.md` | Principles, invariants, pitfalls, and puzzlehunt lessons used across rooms. |
| `docs/specs/room-lifecycle.md` | Required room template files, lifecycle states, promotion gates, and template evolution rules. |
| `docs/puzzle-phases.md` | Puzzle-type phases, waves, pulses, base skills, and theme fits used to seed room hoppers. |

## Non-goals

- No final engineering certification or code-compliance claim.
- No vendor, insurance, or venue commitments in public/shared docs.
- No permanent-location room assumptions unless a brief explicitly declares them.

## First validation

A room concept is ready for deeper build planning only when it has:

1. a declared trailer envelope;
2. a floorplan with egress and staff access;
3. a puzzle graph with no unintentional bottlenecks and every required beat
   physically embodied;
4. a scene simulation with puzzle-hunt checks and group-game stakes;
5. a BOM/budget and durability review for required mechanisms;
6. a reset path under the target reset time;
7. a safety review with open risks named rather than hidden.

These checks are elaborated in `docs/specs/design-system.md` and
`docs/specs/room-lifecycle.md`.

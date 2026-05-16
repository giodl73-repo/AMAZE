# AMAZE - repo guide

You are in AMAZE, a private Games Design repo for trailer-scale escape-room
design.

## House rules

1. Treat safety as a design constraint, not an afterthought.
2. Keep physical dimensions, reset paths, staff controls, and accessibility
   visible in every room brief.
3. Separate puzzle intent from build implementation: the same beat may have many
   prop, lock, sensor, or scenic solutions.
4. Do not assume a specific trailer model unless the brief declares it.
5. Do not publish private venue, vendor, pricing, or personal contact details.

## Expected artifacts

- `rooms/<slug>/BRIEF.md` - theme, audience, capacity, duration, constraints.
- `rooms/<slug>/FLOORPLAN.md` - dimensions, zones, flow, egress, operator sightlines.
- `rooms/<slug>/PUZZLE-GRAPH.md` - puzzle dependencies, aha beats, hints, reset state.
- `rooms/<slug>/BUILD.md` - components, mounting, power, transport, maintenance.
- `rooms/<slug>/SAFETY.md` - egress, fire, electrical, accessibility, emergency stop.
- `rooms/<slug>/PLAYTEST.md` - team observations, stuck points, reset timing.
- `rooms/<slug>/OPS.md` - staff script, reset checklist, hint protocol, failure modes.

## Roles

Read `.roles/ROLE.md` before reviewing a room, component, or workflow change.
Use parliament for governance and the peer directories for creative, spatial,
build, and playtest lenses.

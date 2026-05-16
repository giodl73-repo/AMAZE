# AMAZE Personas

Personas are used for simulation and review. They are not governance roles; the
governance roles live in `.roles/parliament/`.

Personas are individual or role lenses. Use `docs/team-testing.md` for
whole-team archetypes and `docs/behavior-testing.md` for specific behavior
probes. A simulation run should name all three: team archetype, personas/roles,
and behaviors under test.

## Player personas

| Persona | What they test |
|---|---|
| `first-date-duo.md` | Social comfort, onboarding, embarrassment risk, cooperative fun. |
| `speedrun-enthusiasts.md` | Sequence breaking, bypasses, clue economy, timing pressure. |
| `family-four.md` | Mixed ages, crowding, accessibility, parallel tasks. |
| `anxious-participant.md` | confinement, darkness, noise, pressure, exit trust. |

## Operator and build personas

| Persona | What they test |
|---|---|
| `floor-operator.md` | monitoring, hint timing, reset, customer experience. |
| `mobile-builder.md` | trailer envelope, mounting, power, transport, maintenance. |
| `venue-booker.md` | throughput, reliability, staffing, weather/parking constraints. |

## Simulation use

For every room concept, run the required team archetypes in
`docs/team-testing.md`, including at least:

1. one novice/social team;
2. one fast expert team;
3. one mixed family/accessibility team;
4. one operator stress pass.

Record time, hints, stuck states, safety flags, reset friction, and what each
team would tell a friend.

Also record team and behavior probes: the team dynamic being tested, the
behavior being tested, the scene or beat that probes it, the observable signal,
and the design response.

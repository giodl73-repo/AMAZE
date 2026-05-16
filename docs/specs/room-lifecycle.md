# Room Lifecycle and Template Contract

This spec defines what every new escape-room folder contains and how a room
evolves from idea to field-ready design.

## New room folder

Create each room as `rooms/<number>-<slug>/` and start from `rooms/TEMPLATE/`.
The number preserves development order; the slug names the concept, not the
current version.

Required files:

| File | Purpose |
|---|---|
| `BRIEF.md` | Concept, audience, trailer envelope, capacity, duration, constraints, success criteria, and current status. |
| `FLOORPLAN.md` | Dimensions, zones, circulation, egress, operator sightlines, staff access, reset paths, and accessibility notes. |
| `PUZZLE-GRAPH.md` | Hopper candidates, promoted nodes, dependencies, hint ladder, bottleneck check, and reset state. |
| `SCENES.md` | Persona simulations using scenes, beats, checks, clocks, stakes, interventions, and consequences. |
| `BUILD.md` | Physical mechanisms, BOM, budget bands, durability, reliability, abuse cases, maintenance, and spares. |
| `SAFETY.md` | Egress, hazards, mitigations, accessibility alternatives, operator controls, emergency stop, and extraction. |
| `PLAYTEST.md` | Simulation/playtest runs, evidence, scores, surprises, revision decisions, and version history. |
| `OPS.md` | Staffing, run clock, hint protocol, monitoring, reset checklist, failure modes, transport, and maintenance. |

## Lifecycle states

| State | Meaning | Minimum evidence |
|---|---|---|
| Seed | Interesting premise or puzzle cluster. | `BRIEF.md` has concept, audience, trailer assumptions, and signature moment hypothesis. |
| Briefed | Room has constraints and success criteria. | Brief names envelope, team size, clocks, staff model, accessibility assumptions, and open verification items. |
| Mapped | Space is playable on paper. | `FLOORPLAN.md` names dimensions, zones, egress, sightlines, crowding risks, reset paths, and staff access. |
| Graphed | Puzzle structure is explicit. | `PUZZLE-GRAPH.md` has nodes, edges, hint ladder, bottlenecks, physical mechanisms, and reset states. |
| Simulated | The room has survived dry runs. | `SCENES.md` includes at least three player personas and one operator stress pass; `PLAYTEST.md` records outcomes. |
| Build-candidate | Mechanisms are credible enough to prototype. | `BUILD.md` has BOM, budget bands, durability classes, reliability risks, bypasses, and spares for required beats. |
| Safety-reviewed | Known risks are visible and mitigated or blocking. | `SAFETY.md` covers egress, override, hazards, accessibility, communication, and extraction. |
| Playtest-ready | A rough physical run is safe and operable. | Ops script, reset checklist, hint ladder, hard-exit plan, and component bypasses are usable by staff. |
| Field-ready | The room can run for real teams under schedule pressure. | Repeated playtests meet score, safety, reset, reliability, and operator thresholds with no blocking risks. |
| Parked | Good idea, wrong time or constraints. | Open blockers and resume conditions are named. |
| Retired | Evidence says not to continue. | Retire reason and reusable lessons are logged. |

## Promotion gates

A room advances only when all gates for the next state pass.

| Gate | Blocking failure |
|---|---|
| Safety gate | Unresolved egress, emergency exit, staff override, unsafe movement, panic pressure, electrical/fire hazard, or inaccessible required path. |
| Envelope gate | Unknown or impossible dimensions, weight, power, mounting, staff access, reset, transport, or venue assumptions. |
| Puzzle gate | Missing aha, unfair leap, search soup, accidental bottleneck, unclear output, or no hintable stuck signal. |
| Physicality gate | Required beat lacks a physical mechanism, reset state, BOM line, durability review, or manual fallback. |
| Operator gate | Staff cannot monitor, hint, accelerate, reset, recover failure, or hard-exit gracefully. |
| Evidence gate | Advancement is based on author confidence rather than simulation, playtest, score, or explicit risk acceptance. |

## Versioning

Use lightweight version labels in `PLAYTEST.md`.

| Version | Use |
|---|---|
| `v0.seed` | Premise, audience, and signature moment only. |
| `v0.graph` | Puzzle graph and floorplan are coherent enough to simulate. |
| `v0.sim` | First complete scene/beat simulation set. |
| `v0.proto` | Prototype mechanisms and rough ops flow. |
| `v1.playtest` | First external playtestable room. |
| `v1.field` | Field-ready baseline. |

Scores are forward-only. Do not rewrite old results when a room changes; add a
new version entry and explain what changed.

## Evolution loop

Each room evolves through the same loop:

1. Design or revise a small set of beats.
2. Simulate with personas and operator stress.
3. Score with `scoring/RUBRIC.md`.
4. Log surprises, stuck states, failures, and reset friction in `PLAYTEST.md`.
5. Promote, revise, park, or retire hopper items.
6. Update the brief, floorplan, graph, scenes, build, safety, and ops docs only where evidence requires it.
7. Lock the next version before adding more scope.

## Template evolution

The room template is a living contract. Change `rooms/TEMPLATE/` when a lesson
should affect every future room, not just the current concept.

Template changes should happen when:

| Trigger | Template response |
|---|---|
| A pitfall repeats across two or more rooms or simulations. | Add a field, checklist item, or review question that catches it earlier. |
| Operators need the same hidden knowledge repeatedly. | Add it to `OPS.md`, `SCENES.md`, or the hint-ladder structure. |
| Reset failures recur. | Add explicit reset verification fields. |
| Safety/accessibility questions recur. | Add the question to `SAFETY.md` and `FLOORPLAN.md`. |
| Build failures recur. | Add abuse-case, durability, spare, bypass, or maintenance detail to `BUILD.md`. |
| Puzzlehunt-style testing reveals hidden difficulty. | Add skill-tag, clue-source, red-herring, or stuck-signal fields. |

Do not make the template more complex for a one-off issue. Park room-specific
lessons in that room's `PLAYTEST.md` until they repeat.

## Starting checklist

When creating a new room, answer these before writing puzzles in detail:

1. What trailer envelope are we assuming, and what remains unknown?
2. What is the signature moment players will describe afterward?
3. What native trailer affordance carries that moment?
4. What team size, session clock, reset clock, and staff model are designed for?
5. What player personas could fail in different ways?
6. What is the first safety or accessibility concern?
7. What would make this concept unbuildable or unoperable?

# AMAZE Design System

This spec keeps trailer escape-room design coherent across rooms. Principles
guide tradeoffs, invariants protect the experience, and pitfalls preserve what
the workshop learns from simulations and playtests.

## Principles

| Principle | Meaning |
|---|---|
| Safety is part of the magic | Egress, override, accessibility, fire, electrical, and operator control are designed into the fiction instead of patched on later. |
| The trailer is playable | Native trailer affordances, constraints, movement, storage, power, sightlines, and travel become design material. |
| Aha before artifact | A prop is not a puzzle until the player realization, clue path, and physical action are clear. |
| Physical beats beat paper beats | Required puzzle progress must be touched, moved, aligned, heard, revealed, coordinated, or spatially embodied. |
| Operators are part of the system | Monitoring, hinting, acceleration, safety pause, reset, and graceful exit are first-class design surfaces. |
| Reset is design | A magical moment that cannot be verified and reset under schedule pressure is not field-ready. |
| Evidence beats taste | Simulations, playtests, stuck states, hint timing, reset timing, and failure logs decide whether a room advances. |
| Mobile means rugged | Mounting, vibration, weather, transport, spares, weight, power, and venue constraints shape the design from the first brief. |
| Group play is designed | Roles, parallelism, social safety, spectatorship, and table talk matter as much as individual puzzle difficulty. |
| Revisions are forward-only | A scored version stays scored; later lessons create the next version rather than rewriting history. |

## Invariants

Every room version must keep these true.

| Invariant | Required evidence |
|---|---|
| Declared envelope | `BRIEF.md` names the trailer envelope, team size, session target, reset target, staff model, power assumptions, and constraints to verify. |
| Spatial safety | `FLOORPLAN.md` names zones, egress, operator sightlines, crowding risks, accessibility routes, reset paths, and staff access. |
| Physical puzzle graph | `PUZZLE-GRAPH.md` maps every required beat to a physical mechanism, input, aha, output, dependency, hint, and reset state. |
| Scene simulation | `SCENES.md` runs beats through party roles, checks, clocks, group-game stakes, operator interventions, and consequences. |
| Build economics | `BUILD.md` includes BOM, budget bands, durability class, device reliability, bypass, maintenance, breakage path, and spare plan. |
| Safety review | `SAFETY.md` names egress, emergency exit, staff override, hazards, mitigations, accessibility alternatives, communication, and extraction script. |
| Playtest evidence | `PLAYTEST.md` records team behavior, finish time, hints, stuck states, safety flags, physical failures, reset friction, and revision decisions. |
| Operations pack | `OPS.md` defines staffing, run clock, monitoring, hint protocol, reset checklist, failure handling, maintenance, and transport notes. |
| Persona coverage | Each simulation or playtest cycle includes a novice/social team, fast expert team, mixed family/accessibility team, and operator stress pass. |
| Bypass without denial | Every powered, sensed, lock, latch, or fragile required beat has a manual operator recovery path that preserves play when possible. |
| No hidden safety debt | Open risks are named in the room docs; unknowns block promotion instead of being treated as solved. |

## Pitfalls

Use this table during review. Add to it when repeated simulations or playtests
produce the same failure.

| Pitfall | Symptom | Prevention |
|---|---|---|
| Search soup | Players inspect everything because no clue tells them what matters. | Name the aha, clue sources, red herrings, and observable stuck signal before adding props. |
| Lock chain | Progress becomes a sequence of codes with little embodied action. | Require each promoted beat to have a trailer-scale physical mechanism and story purpose. |
| One-person puzzle | One player solves while everyone else watches without useful inference. | Test role coverage, spectatorship, and parallelism in `SCENES.md`. |
| Accidental bottleneck | A non-signature beat blocks the whole room. | Mark required bottlenecks, add redundant clueing, and provide acceleration paths. |
| Fragile magic prop | A satisfying mechanism cannot survive public use, transport, or reset. | Run durability, abuse-case, spare, and manual-bypass review before build readiness. |
| Invisible operator state | Staff cannot tell whether players are stuck, succeeding, or facing device failure. | Define operator view and false-positive/false-negative detection for each risky beat. |
| Reset theater | Staff can reset only if they remember hidden state or inspect too many details. | Use checklist steps with explicit verification and target reset time. |
| Fake accessibility | The room is technically enterable but key beats require crawling, climbing, low light, fine hearing, or standing reach. | Provide participation alternatives in safety and floorplan docs before playtest. |
| Trailer-as-backdrop | The room could be reskinned into any box. | Make native trailer systems, movement, storage, route, power, and travel constraints meaningful. |
| Late safety review | Hazards are found after story and build choices harden. | Review safety from first brief and block advancement on unresolved egress or override issues. |
| Device trust gap | Players solve correctly but the device fails silently. | Define player-visible success, operator-visible success, failure state, and fiction-preserving bypass. |
| Hint shame | Hints make players feel corrected rather than guided. | Write hint ladders that point attention before interpretation and preserve dignity. |

## Puzzlehunt lessons to import

AMAZE borrows puzzlehunt rigor but adapts it to physical rooms.

| Puzzlehunt lesson | Escape-room adaptation |
|---|---|
| Fair extraction matters | Players should be able to explain why an answer was earned once solved. |
| Skill tags expose hidden difficulty | Observation, ordering, extraction, indexing, transformation, and collaboration tags keep puzzle load visible. |
| Metas need clean convergence | Final room beats should combine prior discoveries without making players recalculate the whole room. |
| Test solvers beat author confidence | Simulations and playtests reveal false assumptions earlier than build reviews. |
| Errata are design data | Stuck states, wrong solves, bypasses, and repeated hints become revision inputs. |
| Red herrings must be bounded | Plausible wrong paths are allowed only when clueing and operator view can recover them. |

## Review questions

Ask these before a room advances.

1. What is the signature physical moment?
2. What must players realize, and how does the room teach it fairly?
3. Who is doing something useful when one player manipulates a prop?
4. What does the operator see when the team is stuck?
5. What breaks, jams, misreads, vanishes, or resets badly?
6. How does the team exit if a device dies?
7. What safety reassurance is visible to players without breaking immersion?
8. What changed because of simulation or playtest evidence?

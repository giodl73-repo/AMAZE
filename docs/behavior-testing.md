# Behavior Testing Harness

AMAZE tests people behavior as a first-class design surface. Personas and team
archetypes are useful bundles, but the harness should also track specific
behaviors that can appear in many teams.

The goal is not to label players. The goal is to design rooms that stay fair,
safe, operable, and fun across different ways people actually behave.

## Core idea

| Layer | Meaning |
|---|---|
| Behavior | A repeatable player or staff pattern to test, such as aggressive search, social hesitation, hint avoidance, anxiety spike, or prop forcing. |
| Persona | A scenario bundle that combines several behaviors into a team lens. |
| Team archetype | A whole-party dynamic from `docs/team-testing.md`, such as amazing, confused, fighting, quiet, chaotic, or speed team. |
| Probe | A beat, scene, or operator action that intentionally tests a behavior. |
| Signal | What the operator or playtest observer can see when the behavior appears. |
| Design response | The room feature, hint, reset path, safety reassurance, or build change that handles the behavior. |

## Behavior families

| Family | Behaviors to test | Common signals | Design response |
|---|---|---|---|
| Search behavior | careful scanner, chaotic searcher, pocketing, over-inspecting, ignoring set dressing | objects displaced, repeated opening, players split attention, missed obvious clue | bounded search areas, clue hierarchy, reset inventory, "not a clue" affordances |
| Solve behavior | pattern matcher, brute-forcer, overthinker, under-clued guesser, spreadsheet scribe | early code attempts, long theory loops, skipped aha, too much note-taking | lockout/bypass policy, redundant clueing, confirmation feedback, smaller extraction chunks |
| Social behavior | dominant solver, quiet observer, embarrassment avoidance, date-night politeness, child-led enthusiasm | one voice controls room, silence, reluctance to ask hint, playful or awkward handoffs | role coverage, dignity-preserving hints, spectator inference, parallel low-risk tasks |
| Physical behavior | forceful handling, fine-motor difficulty, low reach, excited handling, seated participation | prop stress, repeated twisting, inaccessible controls, dropped parts | abuse-case design, reach checks, durable tactile affordances, staff-visible misuse signals |
| Sensory behavior | audio difficulty, color confusion, low-light discomfort, sensory overload, tactile avoidance | repeated replay requests, wrong color reads, distress, disengagement | multimodal clueing, captions, symbol/texture backup, volume control, light alternatives |
| Pressure behavior | speedrunning, hint avoidance, panic under clock, giving up early, hard-exit resistance | rapid bypass attempts, no hint requests, rising distress, morale drop, negotiation at exit | soft-warning design, visible exits, opt-out language, graceful acceleration, fiction-preserving hard exit |
| Trust behavior | exit distrust, device distrust, staff distrust, false-success confusion | players test doors, question sensors, retry solved steps, ignore hint | visible safety reassurance, player-visible success state, operator-visible diagnostics, clear hint voice |
| Reset/ops behavior | messy team, late arrival, component failure, staff shortcut, venue interruption | reset overrun, missing props, skipped checklist, noisy environment | reset verification, spare kit, transport checklist, fallback scripts, run-clock policy |

## Behavior matrix

Use this matrix during simulation setup alongside `docs/team-testing.md`.

| Behavior | Persona/team | Scene/beat probe | Observable signal | Risk if unsupported | Design response | Evidence result |
|---|---|---|---|---|---|---|

## Minimum behavior coverage

Every room simulation cycle should include at least:

1. **A novice/social behavior pass** - tests onboarding, embarrassment risk, hint dignity, and shared fun.
2. **An expert/speed behavior pass** - tests sequence breaks, brute force, skipped story, and bypass risk.
3. **A mixed-age/access behavior pass** - tests reach, reading load, crowding, parallel tasks, and prop durability.
4. **An anxiety/trust behavior pass** - tests visible exits, staff reassurance, darkness, sound, pressure, and opt-out paths.
5. **An operator/failure behavior pass** - tests monitoring, hint timing, reset friction, component failure, late teams, and hard exit.

## Probe design

A good probe is specific enough to observe.

| Weak probe | Better probe |
|---|---|
| "Test if players like the radio." | "When P5 starts, does the quiet player have a role besides watching the dial?" |
| "Test anxiety." | "At first low-light reveal, does the anxious participant know how to exit and how to ask for lights up?" |
| "Test speedrunners." | "Can the team brute-force the final lock before understanding the route/power/frequency relationship?" |
| "Test kids." | "Can a shorter player safely contribute to P3 without handling fragile UV hardware?" |
| "Test reset." | "After a chaotic search run, can staff verify all movable objects in under two minutes?" |

## Behavior severity

| Severity | Meaning | Response |
|---|---|---|
| Watch | Behavior appeared but did not hurt fun, safety, fairness, or reset. | Keep observing. |
| Revise | Behavior caused confusion, delay, exclusion, or operator friction. | Update clueing, physical design, hint protocol, or reset path. |
| Block | Behavior created safety risk, inaccessible required progress, unrecoverable failure, or broken trust. | Do not promote until redesigned. |

## Behavior-driven review questions

Ask these before promotion:

1. Who is likely to touch this first, and how might they misuse it?
2. Who can solve this without touching it, and is that okay?
3. Who is excluded by sound, color, darkness, reach, reading, or fine motor load?
4. What does a dominant player do here, and what does everyone else do?
5. What does a speed team skip, brute-force, or bypass?
6. What does a cautious or anxious player need to trust the room?
7. What does the operator see when this behavior appears?
8. What reset damage does this behavior leave behind?

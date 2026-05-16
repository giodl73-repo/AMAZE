# Scene and Beat Harness

AMAZE uses a tabletop-RPG style harness to simulate escape rooms before
fabrication. The room is a one-shot adventure. Players are the party. The
operator is the DM with monitoring, hints, clocks, and safety override.

Puzzle ideas come from `docs/puzzle-hopper.md`. Group-game quality is argued
with `docs/group-game-axes.md`. Physical credibility is checked with
`docs/build-economics.md`. Behavior probes come from
`docs/behavior-testing.md`. Team archetypes come from `docs/team-testing.md`.

## Core objects

| Object | Escape-room meaning |
|---|---|
| Party | The team persona and individual player roles. |
| Team archetype | The whole-party dynamic being tested. |
| Scene | A physical zone or phase of the room. |
| Beat | A playable moment inside a scene: discovery, aha, action, reveal, transition. |
| Check | A test of a skill: observation, ordering, extraction, collaboration, etc. |
| Clock | Time pressure: session clock, hint clock, reset clock, safety clock. |
| Stake | A persona's claim on a group-game axis. |
| Behavior | A repeatable player or staff pattern being tested. |
| Probe | A beat, scene, or operator action that intentionally tests a behavior. |
| Mechanism | The physical object, device, surface, or embodied action carrying the beat. |
| Intervention | Operator hint, acceleration, reset assist, safety pause, or graceful exit. |
| Consequence | Lost time, crowding, wrong path, prop stress, story reveal, or delight. |

## Party roles

Assign roles during simulation. One player may hold more than one role.

| Role | Tests |
|---|---|
| Scout | notices surfaces, absences, hidden access, visual clues |
| Scribe | tracks codes, routes, symbols, and prior discoveries |
| Mechanic | manipulates props, latches, switches, magnets, locks |
| Face | listens to story/audio, asks for hints, manages group morale |
| Navigator | manages space, map, route, order, and final convergence |

## Scene card

| Field | Question |
|---|---|
| Scene name | What zone or phase is being played? |
| Purpose | What must this scene teach or unlock? |
| Entry condition | How do players arrive here? |
| Beats | What happens in order or in parallel? |
| Checks | Which puzzle-hunt skills are tested? |
| Team/behavior probes | Which team archetypes and player/staff behaviors are being tested? |
| Clocks | What time pressure applies? |
| Operator view | What can staff observe? |
| Safety note | What could go wrong physically or emotionally? |
| Exit condition | What moves the party onward? |

## Beat card

| Field | Question |
|---|---|
| Beat | What is the moment? |
| Player action | What do players physically do? |
| Aha | What do they realize? |
| Check | Which skill is tested? |
| Behavior probe | Which behavior might this beat reveal? |
| Target min | Expected solve time for a healthy run of this beat. |
| Hint at min | Minute within the beat when the operator should intervene if no progress is visible. |
| Slow max min | Maximum acceptable beat duration before acceleration or bypass. |
| Mechanism | What physical mechanism carries the beat? |
| Reliability | What breaks, misreads, jams, or needs bypass? |
| DC | How hard is it for this persona? Use Low, Medium, High, or Expert. |
| Success | What happens when they solve it? |
| Partial | What progress or clue do they get? |
| Failure/stall | What happens if they are stuck? |
| Hint trigger | What observable state tells the operator to intervene? |
| Reset effect | What state changes for staff? |

## Running a simulation

1. Choose a party persona from `personas/`.
2. Choose a team archetype from `docs/team-testing.md`.
3. Assign party roles.
4. Draft group-game axis stakes from `docs/group-game-axes.md`.
5. Choose behavior probes from `docs/behavior-testing.md`.
6. Start the session, soft-warning, hard-exit, and reset clocks.
7. Walk scene by scene.
8. For each beat, ask:
   - Who engages first?
   - What skill are they using?
   - What clue makes the aha fair?
   - What happens on partial progress?
   - How many minutes should this beat take?
   - When should the first hint fire?
   - What is the maximum time before acceleration?
   - Which group-game stakes are earning or failing?
   - Which team dynamic is this probing?
   - Which behavior is this probing?
   - What observable signal tells us the behavior appeared?
   - What physical mechanism carries the beat?
   - What can break, and what is the manual bypass?
   - What does the operator see?
   - What does this do to reset?
9. Log interventions, team signals, behavior signals, and consequences.
10. Score with `scoring/RUBRIC.md`.

## Difficulty notation

Use persona-relative difficulty:

| DC | Meaning |
|---|---|
| Low | most teams should get it quickly |
| Medium | fair challenge; one hint may be normal |
| High | likely bottleneck; needs redundant clueing |
| Expert | reserve for optional shortcuts or expert delight |

If a required beat is High or Expert for novice/social teams, add clueing,
parallel work, or an acceleration path.

## Operator as DM

The operator does not solve the room for players. The operator manages pace,
safety, and fun:

- foreshadow instead of answer;
- point attention before giving interpretation;
- accelerate without shaming;
- preserve the fiction when ending a session;
- stop the game immediately for safety.

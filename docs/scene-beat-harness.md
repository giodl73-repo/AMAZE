# Scene and Beat Harness

AMAZE uses a tabletop-RPG style harness to simulate escape rooms before
fabrication. The room is a one-shot adventure. Players are the party. The
operator is the DM with monitoring, hints, clocks, and safety override.

Puzzle ideas come from `docs/puzzle-hopper.md`. Group-game quality is argued
with `docs/group-game-axes.md`. Physical credibility is checked with
`docs/build-economics.md`.

## Core objects

| Object | Escape-room meaning |
|---|---|
| Party | The team persona and individual player roles. |
| Scene | A physical zone or phase of the room. |
| Beat | A playable moment inside a scene: discovery, aha, action, reveal, transition. |
| Check | A test of a skill: observation, ordering, extraction, collaboration, etc. |
| Clock | Time pressure: session clock, hint clock, reset clock, safety clock. |
| Stake | A persona's claim on a group-game axis. |
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
2. Assign party roles.
3. Draft group-game axis stakes from `docs/group-game-axes.md`.
4. Start the session, soft-warning, hard-exit, and reset clocks.
5. Walk scene by scene.
6. For each beat, ask:
   - Who engages first?
   - What skill are they using?
   - What clue makes the aha fair?
   - What happens on partial progress?
   - Which group-game stakes are earning or failing?
   - What physical mechanism carries the beat?
   - What can break, and what is the manual bypass?
   - What does the operator see?
   - What does this do to reset?
7. Log interventions and consequences.
8. Score with `scoring/RUBRIC.md`.

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

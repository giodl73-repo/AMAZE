---
name: AMAZE Escape Room Rubric
slug: amaze-escape-room-rubric
version: v0.1
status: draft
---

# AMAZE Escape Room Rubric

This rubric scores trailer-scale escape rooms on 100 points. It is
forward-only: a new rubric version applies only to rooms or playtests scored
after the change. Earlier scores are not silently rewritten.

## Hard gates

A room cannot advance to build planning when any hard gate fails:

1. **Safety gate** - egress, staff override, emergency procedure, and major
   physical hazards are unresolved.
2. **Envelope gate** - the design does not fit the declared trailer dimensions,
   power, weight, access, or reset constraints.
3. **Operator gate** - staff cannot monitor, hint, reset, or extract players.
4. **Fun gate** - the room is mostly search, locks, or chores with no memorable
   aha chain.
5. **Physicality gate** - every required puzzle beat has a physical mechanism,
   BOM line, durability review, reliability/bypass plan, and reset state.

## Score dimensions

| Dimension | Points | Core question |
|---|---:|---|
| Theme and story | 8 | Does the trailer feel like a place with a reason to exist? |
| Puzzle graph | 14 | Are aha beats fair, connected, and free of accidental bottlenecks? |
| Physical interaction and reliability | 16 | Are mechanisms satisfying, durable, budgeted, and recoverable? |
| Spatial flow | 12 | Can bodies move, search, gather, turn, and exit in the declared trailer? |
| Safety and accessibility | 18 | Are egress, override, hazards, and accessible participation designed in? |
| Operator support | 12 | Can staff monitor, hint, accelerate, reset, and recover failures? |
| Throughput and timing | 8 | Does the room fit session length, reset window, and exit rhythm? |
| Group-game quality | 8 | Do agency, role coverage, interaction texture, and social safety hold up? |
| Delight and memorability | 4 | Will players tell someone what happened afterward? |

## Score bands

Use the same bands for each dimension:

- **0-3**: fails the dimension; must revise.
- **4-6**: partially works; usable idea with visible risk.
- **7-8**: strong; should survive a focused playtest.
- **9-10**: exceptional; evidence shows the dimension creates value.

## Dimension anchors

### Theme and story (8)

- **0-3**: theme is paint on a lock box.
- **4-6**: theme explains props but not puzzle logic.
- **7**: story, space, and puzzle actions reinforce each other.
- **8**: the room's best puzzle could not be reskinned without losing its meaning.

### Puzzle graph (14)

- **0-4**: linear lock chain, search soup, or unearned leaps.
- **5-8**: fair local puzzles, weak dependency graph or obvious bottlenecks.
- **9-12**: visible aha chain with redundant clueing and recoverable stuck points.
- **13-14**: multiple teams can solve through different paths while converging cleanly.

### Physical interaction and reliability (16)

- **0-4**: required beats lack physical mechanisms, BOM, bypasses, or durability review.
- **5-9**: mechanisms are plausible, but cost, reset, breakage, or reliability risks remain.
- **10-13**: interactions feel tactile, readable, budgeted, repeatable, and recoverable.
- **14-16**: the physical act is the aha, and the BOM/spares/bypass plan can survive public use.

### Spatial flow (12)

- **0-3**: unsafe, cramped, inaccessible, or unplayable at target team size.
- **4-6**: fits on paper but creates crowding or blind corners.
- **7-10**: zones, sightlines, movement, and gathering points are deliberate.
- **11-12**: the compact space creates tension without making bodies the puzzle.

### Safety and accessibility (18)

- **0-4**: unresolved egress, override, fire, electrical, trip, ventilation, or accessibility risks.
- **5-10**: risks named but not fully designed out or mitigated.
- **11-15**: safety procedures and accessible participation are integrated.
- **16-18**: safety systems preserve immersion while staying obvious to staff.

### Operator support (12)

- **0-3**: staff cannot see, hint, reset, repair, or extract reliably.
- **4-6**: staff can operate, but only with hidden knowledge.
- **7-10**: monitoring, hinting, reset, and failure recovery are documented.
- **11-12**: operators can keep teams moving gracefully under real schedule pressure.

### Throughput and timing (8)

- **0-2**: likely overrun; no acceleration or hard-exit plan.
- **3-5**: target time plausible but reset or hint timing is vague.
- **6-7**: session, hint, exit, and reset clocks are designed together.
- **8**: slow and fast teams both have satisfying arcs inside the schedule.

### Group-game quality (8)

- **0-2**: one person plays while others watch, social pressure turns negative, or roles are unclear.
- **3-5**: the team can cooperate, but agency, parallelism, or teachability is uneven.
- **6-7**: scouts, scribes, mechanics, faces, and navigators all get meaningful work.
- **8**: the room reliably creates useful table talk, shared discovery, and fair group agency.

### Delight and memorability (4)

- **0-1**: competent but forgettable.
- **2**: one good moment.
- **3**: multiple "tell a friend" beats.
- **4**: a signature moment players will describe in one sentence.

## Scoring protocol

1. Read the room brief, floorplan, puzzle graph, scene harness, build/BOM file,
   safety review, and ops pack.
2. Run at least three simulated teams from `personas/`.
3. Draft group-game stakes from `docs/group-game-axes.md`.
4. Record stuck points, hints, elapsed time, safety flags, reset friction,
   physical failures, device reliability risks, and budget/durability issues.
5. Score each dimension with evidence.
6. Name the top three revision moves.
7. Log any repeated surprise as a candidate rubric amendment.

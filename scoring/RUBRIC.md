---
name: AMAZE Escape Room Rubric
slug: amaze-escape-room-rubric
version: v0.2
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
   technique/device/kit mapping, BOM line, durability review,
   reliability/bypass plan, and reset state.
6. **Reliability gate** - every required yellow/orange/red device has named
   likely failure modes, chaos probes, staff detection, bypass, and retest path.
7. **Transformation/unlock gate** - every meaningful cabinet opening, prop
   reveal, room phase, or state change has a declared source beat, visible
   before/after state, reset state, failure/bypass path, and fast/slow unlock
   coherence check.

## Score dimensions

| Dimension | Points | Core question |
|---|---:|---|
| Theme and story | 8 | Does the trailer feel like a place with a reason to exist? |
| Puzzle graph | 12 | Are aha beats, unlocks, and dependencies fair, connected, and free of accidental bottlenecks? |
| Physical interaction and reliability | 14 | Are mechanisms, transformations, reveals, and recoveries satisfying, durable, budgeted, and resettable? |
| Spatial flow | 10 | Can bodies move, search, gather, turn, and exit in the declared trailer? |
| Purpose mapping and integration | 8 | Is the trailer fully, coherently, and intentionally used? |
| Safety and accessibility | 18 | Are egress, override, hazards, and accessible participation designed in? |
| Operator support | 10 | Can staff monitor, hint, accelerate, reset, and recover failures? |
| Throughput and timing | 8 | Does the room fit session length, reset window, exit rhythm, and fast/slow unlock pacing? |
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

### Puzzle graph (12)

- **0-4**: linear lock chain, search soup, or unearned leaps.
- **5-8**: fair local puzzles, weak dependency graph, hidden unlock logic, or obvious bottlenecks.
- **9-11**: visible aha chain with declared unlocks, redundant clueing, and recoverable stuck points.
- **12**: multiple teams can solve through different paths and unlock rates while converging cleanly.

### Physical interaction and reliability (14)

- **0-4**: required beats lack physical mechanisms, declared transformation states, technique/device mapping, BOM, bypasses, or durability review.
- **5-9**: mechanisms are plausible, but visible before/after state, cost, reset, breakage, kit support, or reliability risks remain.
- **10-12**: interactions and reveals feel tactile, readable, budgeted, repeatable, cataloged, chaos-tested, resettable, and recoverable.
- **13-14**: the physical state change is the aha, and the device review, reliability evidence, kits, BOM/spares/bypass plan can survive public use.

### Spatial flow (10)

- **0-3**: unsafe, cramped, inaccessible, or unplayable at target team size.
- **4-6**: fits on paper but creates crowding or blind corners.
- **7-9**: zones, sightlines, movement, and gathering points are deliberate.
- **10**: the compact space creates tension without making bodies the puzzle.

### Purpose mapping and integration (8)

This dimension asks whether the trailer behaves like a coherent designed system.
Use injection/surjection/bijection as design tests, not as strict math.

- **0-2**: many props/zones are unused, duplicated accidentally, or become misleading clue noise.
- **3-5**: most visible elements have purpose, but some spaces are underused, overburdened, or only decorative without being signaled as atmosphere.
- **6-7**: zones, props, clues, reveals, reset states, and staff controls are intentionally allocated; overlaps are useful and legible.
- **8**: the trailer feels bijective at play scale: every important thing has a reason, every required purpose has a visible home, and reused elements build meaning over time.

#### Mapping tests

| Test | Design question | Healthy answer | Failure smell |
|---|---|---|---|
| Injection | Do distinct visible things map to distinct player-understandable purposes? | Similar props either have distinct roles or are clearly atmosphere. | duplicate-looking objects create accidental red herrings or two props do the same job for no reason. |
| Surjection | Does every required purpose map onto something players can perceive and use? | Every clue, beat, reset state, staff control, and safety function has a visible or documented home. | abstract logic exists only in designer notes; trailer zones are empty or unused. |
| Bijection | Do player-visible elements and game purposes line up cleanly at the moment they matter? | Players later say "of course that belonged there." | after use, props feel arbitrary, decorative, or like puzzle furniture. |
| Overlap/share | Are reused elements intentionally layered? | A prop can teach, clue, reset, and reveal if each layer is staged. | one object carries too many unrelated meanings or causes premature solves. |
| Build | Do earlier uses make later uses more meaningful? | The trailer accumulates understanding; final convergence reuses learned affordances. | puzzles feel interchangeable, episodic, or disconnected. |

### Safety and accessibility (18)

- **0-4**: unresolved egress, override, fire, electrical, trip, ventilation, or accessibility risks.
- **5-10**: risks named but not fully designed out or mitigated.
- **11-15**: safety procedures and accessible participation are integrated.
- **16-18**: safety systems preserve immersion while staying obvious to staff.

### Operator support (10)

- **0-3**: staff cannot see, hint, reset, repair, or extract reliably.
- **4-6**: staff can operate, but only with hidden knowledge.
- **7-9**: monitoring, hinting, reset, transformation state, NPC/operator voice, rotation limits, and failure recovery are documented.
- **10**: operators can keep teams moving gracefully under real schedule pressure, including manual unlock/reveal recovery and safe multi-room coverage limits.

#### NPC and multi-room operation

NPCs are operator tools, not just story flavor. Score them as support only when
they declare function, trigger, delivery mode, sample line, limits, and reset/ops
burden. A shared operator can rotate between two or three trailers/rooms only
when the room pack declares the coverage model, maximum rooms, safe/unsafe
conditions, handoff signal, and dedicated-staff triggers.

| Ops layer | Healthy answer | Failure smell |
|---|---|---|
| NPC/operator voice | The persona preserves fiction while delivering onboarding, hints, acceleration, safety breaks, or payoff. | The character becomes an answer dispenser or hides safety-critical authority. |
| Multi-room rotation | Shared coverage is allowed only under declared camera/audio/state/handoff conditions. | One operator is assumed to watch multiple rooms with no phase visibility or escalation rule. |
| Dedicated staff trigger | Comfort pauses, accessibility support, C5 jams, finals, and emergencies force staff presence. | The business model requires unattended high-risk moments. |

### Throughput and timing (8)

- **0-2**: likely overrun; no acceleration or hard-exit plan.
- **3-5**: target time plausible but reset, hint timing, or unlock pacing is vague.
- **6-7**: session, hint, unlock, exit, and reset clocks are designed together.
- **8**: slow and fast teams both unlock satisfying arcs inside the schedule.

## Transformation and unlock coherence

Use this cross-cutting evidence whenever a room changes physical state: a
cabinet opens, a wall or panel reveals, a prop changes shape/position/light, a
room phase begins, or a final mechanism becomes available. These rows primarily
inform Puzzle graph, Physical interaction and reliability, Operator support, and
Throughput and timing.

| Evidence | Required question | Failure smell |
|---|---|---|
| Transformation states | Can players and staff see the before state, trigger, after state, reset state, and bypass? | the "magic" only exists in designer notes, or staff cannot tell what state the room is in |
| Unlock paths | Does each unlock still make narrative and puzzle sense for fast and slow teams? | fast teams open content without earned context, or slow teams reach an unlock after its story purpose has expired |
| Connected tissue | Does the new room phase explain why the next object/space is now relevant? | cabinet opens as a reward but does not teach, redirect, or pay off prior action |
| Operator acceleration | Can staff advance the room without breaking safety, fairness, or fiction? | bypass reveals an answer with no player-verifiable proof |
| Reset and failure path | Can the transformation be restored and recovered under schedule pressure? | one jammed reveal blocks the room, or reset depends on hidden memory |

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

## Variant quality and build leverage

Use this when comparing a full room to a lite, portable, cheaper, faster, or
lower-risk variant.

Do **not** award quality points for being cheaper or faster to build. Instead:

1. Score the variant independently using the 100-point rubric above.
2. Record build leverage beside the score: build hours, slow build case, admin
   recoveries, reset target, and top bottleneck.
3. Explain every score delta from the source room.
4. Treat a simplification as a quality loss if it turns a physical action into a
   worksheet, removes a group role, hides operator state, or makes the signature
   moment less memorable.
5. Treat a simplification as a quality gain if it removes force/pinch risk,
   improves sightlines, shortens reset, makes state more public, or preserves the
   aha with fewer fragile parts.

| Comparison field | Question |
|---|---|
| Quality retained | Which rubric dimensions stayed equal? |
| Quality lost | Which dimensions dropped because the physical/social/theatrical experience is weaker? |
| Quality gained | Which dimensions improved because the simpler build is safer, clearer, more reliable, or more operable? |
| Build leverage | How many build hours and bottleneck risks were removed without reducing the total score? |
| Prototype burden | What must be physically tested before claiming the score is real? |

Two variants can have the same total score for different reasons. That is a
useful finding, not a contradiction.

## Scoring protocol

1. Read the room brief, floorplan, puzzle graph, scene harness, build/BOM file,
   safety review, and ops pack.
2. Run at least three simulated teams from `personas/`.
3. Draft group-game stakes from `docs/group-game-axes.md`.
4. Record stuck points, hints, elapsed time, safety flags, reset friction,
   physical failures, chaos probes, device reliability risks, and
   budget/durability issues.
5. Run the purpose-mapping tests: injection, surjection, bijection, intentional
   overlap/share, and build.
6. Score each dimension with evidence.
7. Name the top three revision moves.
8. Log any repeated surprise as a candidate rubric amendment.
